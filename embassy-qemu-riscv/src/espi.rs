//! Driver for the QEMU EC's eSPI target and two fixed 4-KiB RAM mailboxes.
//! QEMU handles wire framing, link negotiation and host IRQ virtual wires.
//!
//! One driver owns the register block and both fixed EC mailbox memory windows.
//! Configurable host addresses are never used as EC pointers. Only doorbells
//! notify firmware; payload layout and message pacing are application-defined.
//! Waits leave doorbells pending until explicitly acknowledged. Construction,
//! reconfiguration and drop do not erase shared memory or pending notifications.

use crate::interrupt::typelevel::{Binding, Interrupt};
use crate::pac::espi_target;
use crate::{pac, peripherals, plic};
use core::future::poll_fn;
use core::marker::PhantomData;
use core::ops::Range;
use core::sync::atomic::{Ordering, compiler_fence};
use core::task::Poll;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;

/// Number of bytes in each fixed EC mailbox window.
pub const MAILBOX_CAPACITY: usize = 4096;
/// Maximum single host memory transfer; it must not cross a 64-byte boundary.
pub const MAX_PAYLOAD: usize = 64;

/// Mailbox identifier.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Mailbox {
    Mailbox0,
    Mailbox1,
}

impl Mailbox {
    pub const ALL: [Self; 2] = [Self::Mailbox0, Self::Mailbox1];

    pub const fn index(self) -> usize {
        match self {
            Self::Mailbox0 => 0,
            Self::Mailbox1 => 1,
        }
    }
}

/// Firmware-owned host decode configuration.
///
/// Defaults match the ARM virt controller windows, not hardware reset: QEMU
/// resets all bases to zero and leaves the target disabled.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    pub enabled: bool,
    /// Host mailbox bases, each 4-KiB aligned and distinct from the other bases.
    pub mailbox_addresses: [u32; 2],
    /// Host control-window base, not an EC pointer; also 4-KiB aligned.
    pub control_address: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            enabled: true,
            // Host decode bases used by QEMU's ARM virt eSPI controller.
            mailbox_addresses: [0x090f_0000, 0x0910_0000],
            control_address: 0x0911_0000,
        }
    }
}

impl Config {
    /// Validate the whole layout without touching hardware.
    pub fn validate(&self) -> Result<(), Error> {
        if self.control_address & 0xfff != 0 {
            return Err(Error::UnalignedControlAddress);
        }
        for mailbox in Mailbox::ALL {
            let address = self.mailbox_addresses[mailbox.index()];
            if address & 0xfff != 0 {
                return Err(Error::UnalignedMailboxAddress(mailbox));
            }
            if address == self.control_address {
                return Err(Error::OverlappingRegions);
            }
        }
        if self.mailbox_addresses[0] == self.mailbox_addresses[1] {
            return Err(Error::OverlappingRegions);
        }
        Ok(())
    }
}

/// Configuration and buffer errors. The target exposes no transport-error status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    UnalignedMailboxAddress(Mailbox),
    UnalignedControlAddress,
    OverlappingRegions,
    ConfigurationRejected,
    OutOfBounds,
}

/// State of a mailbox and its host notification controls.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct MailboxStatus {
    pub doorbell: bool,
    pub host_irq_pending: bool,
    pub shared_status: u32,
}

/// Coalescing host-to-EC notifications, indexed by [`Mailbox::index`].
/// Data reads and writes do not generate notifications themselves.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Events {
    pub doorbells: [bool; 2],
}

impl Events {
    pub const fn is_empty(&self) -> bool {
        !self.doorbells[0] && !self.doorbells[1]
    }
}

/// An exclusively owned eSPI target and its two mailbox memory windows.
pub struct Espi<'d, Mode: self::Mode> {
    info: &'static Info,
    _phantom: PhantomData<&'d mut Mode>,
}

impl<'d, Mode: self::Mode> Espi<'d, Mode> {
    fn new_inner<Peripheral: Instance>(config: Config) -> Result<Self, Error> {
        config.validate()?;
        let mut driver = Self {
            info: Peripheral::info(),
            _phantom: PhantomData,
        };
        driver.configure(config)?;
        Ok(driver)
    }

    /// Apply a validated configuration without clearing RAM or resetting the link.
    ///
    /// The host must be quiescent while mappings change. Interrupts are masked and
    /// traffic is temporarily disabled; a hardware failure leaves traffic disabled.
    /// Existing doorbells, shared status and host IRQ levels are preserved.
    pub fn configure(&mut self, config: Config) -> Result<(), Error> {
        config.validate()?;
        self.info.mask_interrupts();
        self.set_enabled(false)?;
        let registers = self.info.reg();
        registers
            .mailbox0_addr()
            .write(|writer| unsafe { writer.bits(config.mailbox_addresses[0]) });
        registers
            .mailbox1_addr()
            .write(|writer| unsafe { writer.bits(config.mailbox_addresses[1]) });
        registers
            .control_addr()
            .write(|writer| unsafe { writer.bits(config.control_address) });
        io_fence();
        let expected = Config {
            enabled: false,
            ..config
        };
        if self.config() != expected {
            return Err(Error::ConfigurationRejected);
        }
        self.set_enabled(config.enabled)
    }

    /// Read back the firmware-owned host decode configuration.
    pub fn config(&self) -> Config {
        let registers = self.info.reg();
        Config {
            enabled: registers.control().read().enable().bit(),
            mailbox_addresses: [
                registers.mailbox0_addr().read().bits(),
                registers.mailbox1_addr().read().bits(),
            ],
            control_address: registers.control_addr().read().bits(),
        }
    }

    /// Enable or disable target traffic without resetting configuration or memory.
    pub fn set_enabled(&mut self, enabled: bool) -> Result<(), Error> {
        if enabled {
            self.config().validate()?;
        }
        self.info
            .reg()
            .control()
            .write(|writer| writer.enable().bit(enabled));
        io_fence();
        if self.is_enabled() != enabled {
            return Err(Error::ConfigurationRejected);
        }
        Ok(())
    }

    /// Whether host decode is enabled, not whether a peer is connected or ready.
    pub fn is_enabled(&self) -> bool {
        self.info.reg().control().read().enable().bit()
    }

    pub fn mailbox_status(&self, mailbox: Mailbox) -> MailboxStatus {
        let control = self.info.reg().mailbox_control(mailbox.index());
        let status = MailboxStatus {
            doorbell: control.doorbell().read().pending().bit(),
            host_irq_pending: control.irq_set().read().pending().bit(),
            shared_status: control.shared_status().read().bits(),
        };
        io_fence();
        status
    }

    /// Copy shared bytes into a caller-owned buffer. The range must fit the
    /// fixed 4096-byte mailbox. Host writes are not an atomic snapshot: use a
    /// host/firmware handshake when the buffer must remain stable during copying.
    pub fn read_mailbox(
        &mut self,
        mailbox: Mailbox,
        offset: usize,
        buffer: &mut [u8],
    ) -> Result<(), Error> {
        let range = checked_range(offset, buffer.len(), MAILBOX_CAPACITY)?;
        let memory = self.info.memory(mailbox);
        io_fence();
        for (destination, index) in buffer.iter_mut().zip(range) {
            *destination = memory.data(index).read().bits();
        }
        io_fence();
        Ok(())
    }

    /// Copy bytes to the fixed EC window with volatile accesses. This does not
    /// itself notify the host; publish shared status or raise its IRQ afterwards.
    /// The host must not concurrently use the same bytes.
    pub fn write_mailbox(
        &mut self,
        mailbox: Mailbox,
        offset: usize,
        buffer: &[u8],
    ) -> Result<(), Error> {
        let range = checked_range(offset, buffer.len(), MAILBOX_CAPACITY)?;
        let memory = self.info.memory(mailbox);
        io_fence();
        for (&byte, index) in buffer.iter().zip(range) {
            memory
                .data(index)
                .write(|writer| unsafe { writer.bits(byte) });
        }
        io_fence();
        Ok(())
    }

    /// Publish an application-defined status after preceding mailbox accesses.
    /// The host can also write this register, so ownership is application-defined.
    pub fn set_shared_status(&mut self, mailbox: Mailbox, value: u32) {
        io_fence();
        self.info
            .reg()
            .mailbox_control(mailbox.index())
            .shared_status()
            .write(|writer| unsafe { writer.bits(value) });
    }

    /// Clear this mailbox's doorbell after consuming its payload. Repeated host
    /// rings coalesce, so the host must wait for the application's completion.
    pub fn acknowledge_doorbell(&mut self, mailbox: Mailbox) {
        io_fence();
        self.info
            .reg()
            .mailbox_control(mailbox.index())
            .doorbell()
            .write(|writer| writer.pending().clear_bit_by_one());
        io_fence();
    }

    /// Assert this mailbox's host IRQ after preceding memory/status writes.
    /// Only the host can acknowledge it; repeated assertions coalesce. Delivery
    /// requires the host's virtual-wire channel, and is not confirmed by this call.
    pub fn raise_host_irq(&mut self, mailbox: Mailbox) {
        io_fence();
        self.info
            .reg()
            .mailbox_control(mailbox.index())
            .irq_set()
            .write(|writer| writer.pending().set_bit());
        io_fence();
    }

    /// Inspect both doorbells without acknowledging them or consuming data.
    pub fn pending_events(&self) -> Events {
        let events = Events {
            doorbells: Mailbox::ALL.map(|mailbox| {
                self.info
                    .reg()
                    .mailbox_control(mailbox.index())
                    .doorbell()
                    .read()
                    .pending()
                    .bit()
            }),
        };
        io_fence();
        events
    }
}

impl<'d> Espi<'d, Blocking> {
    /// Construct a polling driver without enabling a PLIC source.
    pub fn new_blocking<Peripheral: Instance>(
        _peripheral: Peri<'d, Peripheral>,
        config: Config,
    ) -> Result<Self, Error> {
        Self::new_inner::<Peripheral>(config)
    }

    /// Wait for either doorbell without acknowledging it. The target exposes no
    /// link status; this can wait indefinitely if the host is absent.
    pub fn wait_for_events(&mut self) -> Events {
        loop {
            let events = self.pending_events();
            if !events.is_empty() {
                return events;
            }
            core::hint::spin_loop();
        }
    }
}

impl<'d> Espi<'d, Async> {
    /// Construct an interrupt-driven driver using a type-checked IRQ binding.
    pub fn new_async<Peripheral: Instance>(
        _peripheral: Peri<'d, Peripheral>,
        _irq: impl Binding<Peripheral::Interrupt, InterruptHandler<Peripheral>>,
        config: Config,
    ) -> Result<Self, Error> {
        let driver = Self::new_inner::<Peripheral>(config)?;
        use pac::interrupt::{ExternalInterrupt, Priority};
        unsafe {
            plic()
                .priorities()
                .set_priority(ExternalInterrupt::ESPI_TARGET, Priority::P1);
        }
        Ok(driver)
    }

    /// Wait for either doorbell without acknowledging it. Cancelling a pending
    /// wait masks PLIC source 5 without clearing notifications or changing data.
    /// The target exposes no disconnect notification; use an application timeout
    /// when waiting indefinitely for a host is undesirable.
    pub async fn wait_for_events(&mut self) -> Events {
        let _guard = InterruptGuard(self.info);
        poll_fn(|context| {
            self.info.waker.register(context.waker());
            let events = self.pending_events();
            if !events.is_empty() {
                return Poll::Ready(events);
            }
            self.info.set_interrupt_enabled(true);
            let events = self.pending_events();
            if events.is_empty() {
                Poll::Pending
            } else {
                Poll::Ready(events)
            }
        })
        .await
    }
}

impl<Mode: self::Mode> Drop for Espi<'_, Mode> {
    fn drop(&mut self) {
        self.info.mask_interrupts();
    }
}

/// Masks PLIC source 5 and wakes the waiting task without clearing doorbells.
pub struct InterruptHandler<Peripheral: Instance> {
    _phantom: PhantomData<Peripheral>,
}

impl<Peripheral: Instance> crate::interrupt::typelevel::Handler<Peripheral::Interrupt>
    for InterruptHandler<Peripheral>
{
    unsafe fn on_interrupt() {
        Peripheral::info().on_interrupt();
    }
}

struct Info {
    reg: *const espi_target::RegisterBlock,
    memory: [*const pac::espi_mailbox0::RegisterBlock; 2],
    waker: AtomicWaker,
    #[cfg(test)]
    interrupt_enabled: core::sync::atomic::AtomicBool,
}

unsafe impl Send for Info {}
unsafe impl Sync for Info {}

impl Info {
    fn reg(&self) -> &espi_target::RegisterBlock {
        unsafe { &*self.reg }
    }

    fn mask_interrupts(&self) {
        self.set_interrupt_enabled(false);
    }

    fn set_interrupt_enabled(&self, enabled: bool) {
        #[cfg(test)]
        self.interrupt_enabled.store(enabled, Ordering::SeqCst);
        #[cfg(not(test))]
        critical_section::with(|_| {
            let enables = plic().ctx0().enables();
            if enabled {
                unsafe { enables.enable(pac::interrupt::ExternalInterrupt::ESPI_TARGET) };
            } else {
                enables.disable(pac::interrupt::ExternalInterrupt::ESPI_TARGET);
            }
        });
        io_fence();
    }

    fn on_interrupt(&self) {
        self.mask_interrupts();
        self.waker.wake();
    }

    fn memory(&self, mailbox: Mailbox) -> &pac::espi_mailbox0::RegisterBlock {
        unsafe { &*self.memory[mailbox.index()] }
    }
}

struct InterruptGuard<'a>(&'a Info);

impl Drop for InterruptGuard<'_> {
    fn drop(&mut self) {
        self.0.mask_interrupts();
    }
}

fn checked_range(offset: usize, length: usize, limit: usize) -> Result<Range<usize>, Error> {
    if offset > limit || length > limit - offset {
        return Err(Error::OutOfBounds);
    }
    Ok(offset..offset + length)
}

fn io_fence() {
    compiler_fence(Ordering::SeqCst);
    #[cfg(any(target_arch = "riscv32", target_arch = "riscv64"))]
    riscv::asm::fence();
}

trait SealedMode {}

/// Blocking wait mode.
pub struct Blocking;
impl SealedMode for Blocking {}
impl Mode for Blocking {}

/// Interrupt-driven async wait mode.
pub struct Async;
impl SealedMode for Async {}
impl Mode for Async {}

#[allow(private_bounds)]
pub trait Mode: SealedMode {}

trait SealedInstance {
    fn info() -> &'static Info;
}

#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {
    type Interrupt: Interrupt;
}

impl SealedInstance for peripherals::ESPI_TARGET {
    fn info() -> &'static Info {
        static INFO: Info = Info {
            reg: pac::EspiTarget::ptr(),
            memory: [pac::EspiMailbox0::ptr(), pac::EspiMailbox1::ptr()],
            waker: AtomicWaker::new(),
            #[cfg(test)]
            interrupt_enabled: core::sync::atomic::AtomicBool::new(false),
        };
        &INFO
    }
}

impl Instance for peripherals::ESPI_TARGET {
    type Interrupt = crate::interrupt::typelevel::ESPI_TARGET;
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use core::future::Future;
    use core::mem::MaybeUninit;
    use core::sync::atomic::{AtomicBool, AtomicUsize};
    use core::task::{Context, Waker};
    use std::boxed::Box;
    use std::sync::Arc;
    use std::task::Wake;

    fn register_fixture<Mode: self::Mode>() -> Espi<'static, Mode> {
        let registers = Box::leak(Box::new(unsafe {
            MaybeUninit::<espi_target::RegisterBlock>::zeroed().assume_init()
        }));
        let info = Box::leak(Box::new(Info {
            reg: registers,
            memory: Mailbox::ALL.map(|_| {
                Box::leak(Box::new(unsafe {
                    MaybeUninit::<pac::espi_mailbox0::RegisterBlock>::zeroed().assume_init()
                })) as *const _
            }),
            waker: AtomicWaker::new(),
            interrupt_enabled: AtomicBool::new(false),
        }));
        Espi {
            info,
            _phantom: PhantomData,
        }
    }

    fn set_doorbell(info: &Info, mailbox: Mailbox, pending: bool) {
        unsafe {
            info.reg()
                .mailbox_control(mailbox.index())
                .doorbell()
                .as_ptr()
                .write_volatile(u32::from(pending));
        }
    }

    struct WakeCounter(AtomicUsize);

    impl Wake for WakeCounter {
        fn wake(self: Arc<Self>) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn default_layout_is_valid() {
        let config = Config::default();
        assert_eq!(config.validate(), Ok(()));
        assert!(config.enabled);
        assert_eq!(config.mailbox_addresses, [0x090f_0000, 0x0910_0000]);
        assert_eq!(config.control_address, 0x0911_0000);
    }

    #[test]
    fn every_decode_base_requires_alignment() {
        for mailbox in Mailbox::ALL {
            let mut config = Config::default();
            config.mailbox_addresses[mailbox.index()] += 1;
            assert_eq!(
                config.validate(),
                Err(Error::UnalignedMailboxAddress(mailbox))
            );
        }
        let mut config = Config::default();
        config.control_address += 1;
        assert_eq!(config.validate(), Err(Error::UnalignedControlAddress));
    }

    #[test]
    fn rejects_duplicate_bases_even_while_disabled() {
        let mut config = Config::default();
        config.mailbox_addresses[1] = config.mailbox_addresses[0];
        assert_eq!(config.validate(), Err(Error::OverlappingRegions));
        config.enabled = false;
        assert_eq!(config.validate(), Err(Error::OverlappingRegions));
        for mailbox in Mailbox::ALL {
            let mut config = Config::default();
            config.control_address = config.mailbox_addresses[mailbox.index()];
            assert_eq!(config.validate(), Err(Error::OverlappingRegions));
        }
    }

    #[test]
    fn accepts_first_last_and_adjacent_pages() {
        let config = Config {
            mailbox_addresses: [0, 4096],
            control_address: 0xffff_f000,
            ..Config::default()
        };
        assert_eq!(config.validate(), Ok(()));
    }

    #[test]
    fn reconfiguration_preserves_mailboxes_and_notifications() {
        let mut driver = register_fixture::<Blocking>();
        driver.configure(Config::default()).unwrap();
        for mailbox in Mailbox::ALL {
            driver.write_mailbox(mailbox, 4095, &[0xa5]).unwrap();
            driver.set_shared_status(mailbox, 0x1234_5678);
            driver.raise_host_irq(mailbox);
            set_doorbell(driver.info, mailbox, true);
        }
        let config = Config {
            enabled: false,
            mailbox_addresses: [0x0910_0000, 0x0911_0000],
            control_address: 0x090f_0000,
        };
        driver.configure(config).unwrap();
        assert_eq!(driver.config(), config);
        driver.set_enabled(true).unwrap();
        assert!(driver.is_enabled());
        for mailbox in Mailbox::ALL {
            assert_eq!(
                driver.mailbox_status(mailbox),
                MailboxStatus {
                    doorbell: true,
                    host_irq_pending: true,
                    shared_status: 0x1234_5678,
                }
            );
            let mut buffer = [0];
            driver.read_mailbox(mailbox, 4095, &mut buffer).unwrap();
            assert_eq!(buffer, [0xa5]);
        }
    }

    #[test]
    fn invalid_configuration_does_not_touch_hardware() {
        let mut driver = register_fixture::<Blocking>();
        driver.configure(Config::default()).unwrap();
        driver.info.set_interrupt_enabled(true);
        let mut invalid = Config::default();
        invalid.mailbox_addresses[1] += 1;
        assert_eq!(
            driver.configure(invalid),
            Err(Error::UnalignedMailboxAddress(Mailbox::Mailbox1))
        );
        assert_eq!(driver.config(), Config::default());
        assert!(driver.info.interrupt_enabled.load(Ordering::SeqCst));
    }

    #[test]
    fn cannot_enable_reset_decode_bases() {
        let mut driver = register_fixture::<Blocking>();
        assert_eq!(driver.set_enabled(true), Err(Error::OverlappingRegions));
        assert!(!driver.is_enabled());
        driver.set_enabled(false).unwrap();
    }

    #[test]
    fn buffer_ranges_reject_overflow_and_allow_empty_end() {
        assert_eq!(checked_range(4096, 0, 4096), Ok(4096..4096));
        assert_eq!(checked_range(4095, 1, 4096), Ok(4095..4096));
        assert_eq!(checked_range(4096, 1, 4096), Err(Error::OutOfBounds));
        assert_eq!(checked_range(usize::MAX, 1, 4096), Err(Error::OutOfBounds));
        assert_eq!(checked_range(1, usize::MAX, 4096), Err(Error::OutOfBounds));
    }

    #[test]
    fn mailbox_io_uses_full_fixed_windows() {
        let mut driver = register_fixture::<Blocking>();
        driver.configure(Config::default()).unwrap();
        let mut buffer = [0; MAILBOX_CAPACITY];
        for mailbox in Mailbox::ALL {
            let value = 0x5a + mailbox.index() as u8;
            driver
                .write_mailbox(mailbox, 0, &[value; MAILBOX_CAPACITY])
                .unwrap();
        }
        for mailbox in Mailbox::ALL {
            driver.read_mailbox(mailbox, 0, &mut buffer).unwrap();
            assert_eq!(buffer, [0x5a + mailbox.index() as u8; MAILBOX_CAPACITY]);
            driver
                .write_mailbox(mailbox, MAILBOX_CAPACITY, &[])
                .unwrap();
            driver
                .read_mailbox(mailbox, MAILBOX_CAPACITY, &mut [])
                .unwrap();
            assert_eq!(
                driver.write_mailbox(mailbox, MAILBOX_CAPACITY - 1, &[1, 2]),
                Err(Error::OutOfBounds)
            );
            assert_eq!(
                driver.read_mailbox(mailbox, usize::MAX, &mut buffer),
                Err(Error::OutOfBounds)
            );
            assert_eq!(buffer, [0x5a + mailbox.index() as u8; MAILBOX_CAPACITY]);
        }
        assert!(driver.pending_events().is_empty());
    }

    #[test]
    fn blocking_wait_preserves_both_doorbells() {
        let mut driver = register_fixture::<Blocking>();
        assert!(driver.pending_events().is_empty());
        for mailbox in Mailbox::ALL {
            set_doorbell(driver.info, mailbox, true);
        }
        let events = driver.wait_for_events();
        assert_eq!(events.doorbells, [true; 2]);
        assert_eq!(driver.pending_events(), events);
    }

    #[test]
    fn notification_methods_emit_only_the_write_one_bit() {
        let mut driver = register_fixture::<Blocking>();
        for mailbox in Mailbox::ALL {
            let control = driver.info.reg().mailbox_control(mailbox.index());
            control
                .doorbell()
                .write(|writer| unsafe { writer.bits(0xffff_fffe) });
            control
                .irq_set()
                .write(|writer| unsafe { writer.bits(0xffff_fffe) });
            driver.acknowledge_doorbell(mailbox);
            assert_eq!(control.doorbell().read().bits(), 1);
            driver.raise_host_irq(mailbox);
            assert_eq!(control.irq_set().read().bits(), 1);
        }
    }

    #[test]
    fn async_interrupt_wakes_without_acknowledging() {
        let mut driver = register_fixture::<Async>();
        let info = driver.info;
        let counter = Arc::new(WakeCounter(AtomicUsize::new(0)));
        let waker = Waker::from(counter.clone());
        let mut context = Context::from_waker(&waker);
        let mut wait = Box::pin(driver.wait_for_events());
        assert_eq!(wait.as_mut().poll(&mut context), Poll::Pending);
        assert!(info.interrupt_enabled.load(Ordering::SeqCst));
        for mailbox in Mailbox::ALL {
            set_doorbell(info, mailbox, true);
        }
        info.on_interrupt();
        assert!(!info.interrupt_enabled.load(Ordering::SeqCst));
        assert_eq!(counter.0.load(Ordering::SeqCst), 1);
        assert_eq!(
            wait.as_mut().poll(&mut context),
            Poll::Ready(Events {
                doorbells: [true; 2],
            })
        );
        drop(wait);
        assert_eq!(driver.pending_events().doorbells, [true; 2]);
        assert!(!info.interrupt_enabled.load(Ordering::SeqCst));
        for mailbox in Mailbox::ALL {
            set_doorbell(info, mailbox, false);
        }
        let mut wait = Box::pin(driver.wait_for_events());
        assert_eq!(wait.as_mut().poll(&mut context), Poll::Pending);
        assert!(info.interrupt_enabled.load(Ordering::SeqCst));
    }

    #[test]
    fn cancelling_wait_masks_but_does_not_acknowledge() {
        let mut driver = register_fixture::<Async>();
        let info = driver.info;
        let mut context = Context::from_waker(Waker::noop());
        let mut wait = Box::pin(driver.wait_for_events());
        assert_eq!(wait.as_mut().poll(&mut context), Poll::Pending);
        assert!(info.interrupt_enabled.load(Ordering::SeqCst));
        set_doorbell(info, Mailbox::Mailbox1, true);
        drop(wait);
        assert!(!info.interrupt_enabled.load(Ordering::SeqCst));
        assert_eq!(driver.pending_events().doorbells, [false, true]);
        let mut wait = Box::pin(driver.wait_for_events());
        assert_eq!(
            wait.as_mut().poll(&mut context),
            Poll::Ready(Events {
                doorbells: [false, true],
            })
        );
        assert!(!info.interrupt_enabled.load(Ordering::SeqCst));
    }

    #[test]
    fn dropping_driver_only_masks_interrupts() {
        let mut driver = register_fixture::<Blocking>();
        driver.configure(Config::default()).unwrap();
        driver.set_shared_status(Mailbox::Mailbox0, 42);
        driver.write_mailbox(Mailbox::Mailbox0, 0, &[0xa5]).unwrap();
        set_doorbell(driver.info, Mailbox::Mailbox0, true);
        let info = driver.info;
        info.set_interrupt_enabled(true);
        drop(driver);
        assert!(!info.interrupt_enabled.load(Ordering::SeqCst));
        assert!(info.reg().control().read().enable().bit());
        assert_eq!(
            info.reg().mailbox_control(0).shared_status().read().bits(),
            42
        );
        assert!(
            info.reg()
                .mailbox_control(0)
                .doorbell()
                .read()
                .pending()
                .bit()
        );
        assert_eq!(info.memory(Mailbox::Mailbox0).data(0).read().bits(), 0xa5);
    }
}
