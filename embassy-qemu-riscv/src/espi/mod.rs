use crate::interrupt::typelevel::{Binding, Interrupt};
use crate::pac::espi0;
use crate::{pac, peripherals, plic};
use core::future::poll_fn;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicU8, Ordering};
use core::task::Poll;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;

/// Default shared-memory base address (matches QEMU virt machine).
const SHMEM_BASE: usize = 0x10200000;

/// Default shared-memory region size in bytes.
const SHMEM_SIZE: usize = 4096;

// Enable ESPI0 interrupt source in the PLIC.
//
// Sets the ESPI0 priority to P1 and enables the source in PLIC context 0.
// This should be called when creating an async eSPI instance.
pub(crate) fn plic_enable_espi0() {
    use pac::interrupt::{ExternalInterrupt, Priority};

    // SAFETY: We have sole control of PLIC configuration for ESPI0
    unsafe {
        plic()
            .priorities()
            .set_priority(ExternalInterrupt::ESPI0, Priority::P1);
        plic().ctx0().enables().enable(ExternalInterrupt::ESPI0);
    }
}

/// eSPI interrupt handler.
pub struct InterruptHandler<T: Instance> {
    _phantom: PhantomData<T>,
}

impl<T: Instance> crate::interrupt::typelevel::Handler<T::Interrupt> for InterruptHandler<T> {
    unsafe fn on_interrupt() {
        let info = T::info();
        let reg = info.reg();
        let status = reg.int_status().read();

        #[cfg(feature = "defmt")]
        defmt::trace!("eSPI ISR: int_status={:#X}, int_enable={:#X}",
            status.bits(), reg.int_enable().read().bits());

        if status.periph_pending().bit() {
            // Don't W1C here — let the task read PERIPH_ADDR first, then clear
            reg.int_enable().modify(|_, w| w.periph_en().clear_bit());
            #[cfg(feature = "defmt")]
            defmt::trace!("eSPI ISR: periph_pending, waking task");
            info.periph_waker.wake();
        }

        if status.vwire_pending().bit() {
            // Don't W1C here — let the task read VWIRE_IN first, then clear
            reg.int_enable().modify(|_, w| w.vwire_en().clear_bit());
            #[cfg(feature = "defmt")]
            defmt::trace!("eSPI ISR: vwire_pending, waking task");
            info.vwire_waker.wake();
        }
    }
}

/// eSPI error.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// Shared memory access out of bounds.
    OutOfBounds,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OutOfBounds => write!(f, "Shared memory access out of bounds"),
        }
    }
}
impl core::error::Error for Error {}

fn init(reg: &espi0::RegisterBlock) {
    // Clear any pending interrupts (W1C — write all 1s to clear)
    // SAFETY: Writing 1s to W1C register clears pending bits.
    reg.int_status().write(|w| unsafe { w.bits(0xFFFF_FFFF) });
    // Disable all interrupts
    reg.int_enable().reset();
}

/// eSPI driver.
pub struct Espi<'d, M: Mode> {
    info: &'static Info,
    _phantom: PhantomData<&'d M>,
}

impl<'d, M: Mode> Espi<'d, M> {
    fn new_inner<T: Instance>() -> Self {
        let info = T::info();
        init(info.reg());
        info.refcount.fetch_add(1, Ordering::AcqRel);
        Self {
            info,
            _phantom: PhantomData,
        }
    }

    /// Read the current value of the VWIRE_IN register.
    pub fn blocking_read_vwire(&self) -> u32 {
        self.info.reg().vwire_in().read().data().bits()
    }

    /// Write a value to the VWIRE_OUT register, sending it to the peer.
    pub fn blocking_write_vwire(&mut self, val: u32) {
        // SAFETY: Any u32 value is valid for the VWIRE_OUT register.
        self.info
            .reg()
            .vwire_out()
            .write(|w| unsafe { w.data().bits(val) });
    }

    /// Block until the peripheral-channel interrupt fires (peer wrote to shared memory).
    ///
    /// Returns the shared-memory offset the peer wrote to.
    pub fn blocking_wait_periph(&self) -> u32 {
        while !self.info.reg().int_status().read().periph_pending().bit() {}
        let offset = self.info.reg().periph_addr().read().offset().bits();
        // W1C clear
        // SAFETY: Writing 1 to W1C bits is the correct clearing protocol.
        self.info
            .reg()
            .int_status()
            .write(|w| unsafe { w.bits(1 << 0) });
        offset
    }

    /// Block until the virtual-wire interrupt fires (peer updated vwires).
    pub fn blocking_wait_vwire(&self) {
        while !self.info.reg().int_status().read().vwire_pending().bit() {}
        // W1C clear
        // SAFETY: Writing 1 to W1C bits is the correct clearing protocol.
        self.info
            .reg()
            .int_status()
            .write(|w| unsafe { w.bits(1 << 1) });
    }

    /// Read bytes from the shared memory region.
    ///
    /// `offset` and `buf.len()` must both be 4-byte aligned because each MMIO
    /// access corresponds to a single eSPI protocol message.
    ///
    /// # Errors
    ///
    /// Returns [`Error::OutOfBounds`] if `offset + buf.len()` exceeds the shared memory size.
    ///
    /// # Panics
    ///
    /// Panics if `offset` or `buf.len()` is not 4-byte aligned.
    pub fn read_shmem(&self, offset: usize, buf: &mut [u8]) -> Result<(), Error> {
        assert!(offset.is_multiple_of(4) && buf.len().is_multiple_of(4), "shmem access must be 4-byte aligned");
        let end = offset.checked_add(buf.len()).ok_or(Error::OutOfBounds)?;
        if end > SHMEM_SIZE {
            return Err(Error::OutOfBounds);
        }

        let base = SHMEM_BASE + offset;
        for (i, chunk) in buf.chunks_exact_mut(4).enumerate() {
            let addr = base + i * 4;
            // SAFETY: Address is within the MMIO shared memory region (validated above)
            // and is 4-byte aligned.
            let val = unsafe { core::ptr::read_volatile(addr as *const u32) };
            chunk.copy_from_slice(&val.to_le_bytes());
        }
        Ok(())
    }

    /// Write bytes to the shared memory region.
    ///
    /// `offset` and `data.len()` must both be 4-byte aligned because each MMIO
    /// access corresponds to a single eSPI protocol message.
    ///
    /// # Errors
    ///
    /// Returns [`Error::OutOfBounds`] if `offset + data.len()` exceeds the shared memory size.
    ///
    /// # Panics
    ///
    /// Panics if `offset` or `data.len()` is not 4-byte aligned.
    pub fn write_shmem(&mut self, offset: usize, data: &[u8]) -> Result<(), Error> {
        assert!(offset.is_multiple_of(4) && data.len().is_multiple_of(4), "shmem access must be 4-byte aligned");
        let end = offset.checked_add(data.len()).ok_or(Error::OutOfBounds)?;
        if end > SHMEM_SIZE {
            return Err(Error::OutOfBounds);
        }

        let base = SHMEM_BASE + offset;
        for (i, chunk) in data.chunks_exact(4).enumerate() {
            let addr = base + i * 4;
            let val = u32::from_le_bytes(chunk.try_into().unwrap());
            // SAFETY: Address is within the MMIO shared memory region (validated above)
            // and is 4-byte aligned.
            unsafe { core::ptr::write_volatile(addr as *mut u32, val) };
        }
        Ok(())
    }
}

impl<'d> Espi<'d, Blocking> {
    /// Create a new blocking eSPI driver instance.
    pub fn new_blocking<T: Instance>(_peri: Peri<'d, T>) -> Self {
        Self::new_inner::<T>()
    }
}

impl<'d> Espi<'d, Async> {
    /// Create a new async eSPI driver instance.
    pub fn new_async<T: Instance>(
        _peri: Peri<'d, T>,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>>,
    ) -> Self {
        let espi = Self::new_inner::<T>();
        plic_enable_espi0();
        espi
    }

    /// Wait for a virtual wire update from the peer, then return the new VWIRE_IN value.
    pub async fn read_vwire(&mut self) -> u32 {
        poll_fn(|cx| {
            self.info.vwire_waker.register(cx.waker());

            // Check if a vwire interrupt has already been processed by the ISR
            let status = self.info.reg().int_status().read();
            #[cfg(feature = "defmt")]
            defmt::trace!("read_vwire poll: int_status={:#X}", status.bits());
            if status.vwire_pending().bit() {
                // Clear it ourselves (shouldn't normally get here since ISR clears, but be safe)
                // SAFETY: Writing 1 to W1C bits is the correct clearing protocol.
                self.info
                    .reg()
                    .int_status()
                    .write(|w| unsafe { w.bits(1 << 1) });
                Poll::Ready(())
            } else {
                // Re-enable the interrupt so the ISR will wake us
                critical_section::with(|_| {
                    self.info
                        .reg()
                        .int_enable()
                        .modify(|_, w| w.vwire_en().set_bit());
                });
                Poll::Pending
            }
        })
        .await;

        self.info.reg().vwire_in().read().data().bits()
    }

    /// Write a value to the VWIRE_OUT register, sending it to the peer.
    ///
    /// This is synchronous (the write completes immediately), but is
    /// provided on the async driver for convenience.
    pub async fn write_vwire(&mut self, val: u32) {
        self.blocking_write_vwire(val);
    }

    /// Wait for a peripheral-channel interrupt (peer wrote to shared memory).
    ///
    /// Returns the shared-memory offset the peer wrote to.
    pub async fn wait_periph(&mut self) -> u32 {
        poll_fn(|cx| {
            self.info.periph_waker.register(cx.waker());

            let status = self.info.reg().int_status().read();
            #[cfg(feature = "defmt")]
            defmt::trace!("wait_periph poll: int_status={:#X}", status.bits());

            if status.periph_pending().bit() {
                let offset = self.info.reg().periph_addr().read().offset().bits();
                // SAFETY: Writing 1 to W1C bits is the correct clearing protocol.
                self.info
                    .reg()
                    .int_status()
                    .write(|w| unsafe { w.bits(1 << 0) });
                Poll::Ready(offset)
            } else {
                critical_section::with(|_| {
                    self.info
                        .reg()
                        .int_enable()
                        .modify(|_, w| w.periph_en().set_bit());
                });
                Poll::Pending
            }
        })
        .await
    }
}

impl<'d, M: Mode> Drop for Espi<'d, M> {
    fn drop(&mut self) {
        if self.info.refcount.fetch_sub(1, Ordering::AcqRel) == 1 {
            // Disable all eSPI interrupts
            self.info.reg().int_enable().reset();
            // Disable ESPI0 in the PLIC
            use pac::interrupt::ExternalInterrupt;
            plic().ctx0().enables().disable(ExternalInterrupt::ESPI0);
        }
    }
}

struct Info {
    reg: *const espi0::RegisterBlock,
    refcount: AtomicU8,
    periph_waker: AtomicWaker,
    vwire_waker: AtomicWaker,
}

// SAFETY: The register block pointer is to memory-mapped I/O valid for the program lifetime.
// AtomicU8 and AtomicWaker are both Send+Sync.
unsafe impl Send for Info {}
unsafe impl Sync for Info {}

impl Info {
    #[inline(always)]
    fn reg(&self) -> &espi0::RegisterBlock {
        // SAFETY: The pointer is to a valid MMIO register block for the entire program lifetime.
        unsafe { &*self.reg }
    }
}

trait SealedMode {}

/// Blocking mode.
pub struct Blocking;
impl SealedMode for Blocking {}
impl Mode for Blocking {}

/// Async mode.
pub struct Async;
impl SealedMode for Async {}
impl Mode for Async {}

/// Driver mode.
#[allow(private_bounds)]
pub trait Mode: SealedMode {}

trait SealedInstance {
    fn info() -> &'static Info;
}

/// eSPI instance trait.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {
    /// The typelevel interrupt for this instance.
    type Interrupt: Interrupt;
}

macro_rules! impl_instance {
    ($peri:ident, $rb:ident) => {
        impl SealedInstance for peripherals::$peri {
            #[inline(always)]
            fn info() -> &'static Info {
                static INFO: Info = Info {
                    reg: pac::$rb::ptr(),
                    refcount: AtomicU8::new(0),
                    periph_waker: AtomicWaker::new(),
                    vwire_waker: AtomicWaker::new(),
                };
                &INFO
            }
        }

        impl Instance for peripherals::$peri {
            type Interrupt = crate::interrupt::typelevel::$peri;
        }
    };
}

impl_instance!(ESPI0, Espi0);
