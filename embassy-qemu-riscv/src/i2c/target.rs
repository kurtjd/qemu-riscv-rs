//! I2C target driver.
//!
//! Drives the socket-backed I2C target peripheral (`I2C_TARGET`) and implements the
//! [`embedded-mcu-hal`](embedded_mcu_hal) I2C target traits
//! ([`blocking`](embedded_mcu_hal::i2c::target::blocking::I2c) and
//! [`asynch`](embedded_mcu_hal::i2c::target::asynch::I2c)).
//!
//! The application drives the device with the `listen` → `respond` → re-`listen` loop:
//! [`listen`](Self::listen) waits for the controller to address the target and reports
//! the [`Request`], then [`respond_to_write`](Self::respond_to_write) /
//! [`respond_to_read`](Self::respond_to_read) drain or fill bytes until the transfer
//! terminates.
//!
//! The device matches a single configured 7-bit address. An unmatched address is
//! silently ignored by the hardware, so address NACKs are never observed and all
//! methods are infallible.

use crate::interrupt::typelevel::{Binding, Interrupt};
use crate::pac::i2c_target;
use crate::{pac, peripherals, plic};
use core::convert::Infallible;
use core::future::poll_fn;
use core::marker::PhantomData;
use core::task::Poll;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;
use embedded_mcu_hal::i2c::target::{ReadStatus, Request, WriteStatus};

// Enable the I2C_TARGET interrupt source in the PLIC.
//
// Sets the I2C_TARGET priority to P1 and enables the source in PLIC context 0.
// Called when creating an async target instance.
pub(crate) fn plic_enable_i2c_target() {
    use pac::interrupt::{ExternalInterrupt, Priority};

    // SAFETY: We have sole control of PLIC configuration for I2C_TARGET
    unsafe {
        plic()
            .priorities()
            .set_priority(ExternalInterrupt::I2C_TARGET, Priority::P1);
        plic().ctx0().enables().enable(ExternalInterrupt::I2C_TARGET);
    }
}

/// I2C target interrupt handler.
pub struct InterruptHandler<T: Instance> {
    _phantom: PhantomData<T>,
}

impl<T: Instance> crate::interrupt::typelevel::Handler<T::Interrupt> for InterruptHandler<T> {
    unsafe fn on_interrupt() {
        let info = T::info();

        // Disable all event interrupt-enable bits to de-assert the level-triggered IRQ.
        // The waiting task observes STATUS, services the event, and re-enables the
        // interrupts the next time it waits.
        info.reg().ctrl().modify(|_, w| {
            w.int_start().clear_bit();
            w.int_stop().clear_bit();
            w.int_rx_rdy().clear_bit();
            w.int_tx().clear_bit()
        });
        info.waker.wake();
    }
}

// Reset the device to a known-clean baseline, preserving the match address and any
// configured interrupt enables (the hardware `recover` semantics).
fn recover_reg(reg: &i2c_target::RegisterBlock) {
    reg.ctrl().write(|w| w.reset().set_bit());
}

/// I2C target driver.
pub struct I2c<'d, M: Mode> {
    info: &'static Info,
    address: u8,
    restart_pending: bool,
    _phantom: PhantomData<&'d M>,
}

impl<'d, M: Mode> I2c<'d, M> {
    fn new_inner<T: Instance>(address: u8) -> Self {
        let info = T::info();

        // Baseline the device, then program the address we answer to.
        recover_reg(info.reg());
        info.reg().addr().write(|w| {
            // SAFETY: `address` is masked to the 7-bit address field by the hardware.
            unsafe { w.addr().bits(address & 0x7f) }
        });

        Self {
            info,
            address: address & 0x7f,
            restart_pending: false,
            _phantom: PhantomData,
        }
    }

    // Clear a write-1-to-clear STATUS bit without disturbing the others.
    #[inline]
    fn clear_start(&mut self) {
        self.info
            .reg()
            .status()
            .write(|w| w.start().clear_bit_by_one());
    }

    #[inline]
    fn clear_stop(&mut self) {
        self.info
            .reg()
            .status()
            .write(|w| w.stop().clear_bit_by_one());
    }

    #[inline]
    fn clear_restart(&mut self) {
        self.info
            .reg()
            .status()
            .write(|w| w.restart().clear_bit_by_one());
    }

    #[inline]
    fn clear_tx_done(&mut self) {
        self.info
            .reg()
            .status()
            .write(|w| w.tx_done().clear_bit_by_one());
    }

    // Inspect the latched START status and turn it into a Request, advancing the
    // internal restart bookkeeping. Returns `None` if START is not set.
    fn take_start_request(&mut self, status: &i2c_target::status::R) -> Option<Request> {
        if !status.start().bit() {
            return None;
        }

        // A repeated START sets both START and RESTART. Report the edge first
        // (RepeatedStart), then on the next listen report the new direction.
        if status.restart().bit() && !self.restart_pending {
            self.clear_restart();
            self.restart_pending = true;
            return Some(Request::RepeatedStart(self.address));
        }

        self.restart_pending = false;
        let read = status.rw().bit();
        self.clear_start();
        Some(if read {
            Request::Read(self.address)
        } else {
            Request::Write(self.address)
        })
    }
}

impl<'d> I2c<'d, Blocking> {
    /// Create a new blocking I2C target driver answering to `address` (7-bit).
    pub fn new_blocking<T: Instance>(_peri: Peri<'d, T>, address: u8) -> Self {
        Self::new_inner::<T>(address)
    }

    /// Bring the target back to a known-clean baseline after a wedged transfer.
    ///
    /// Drops any in-flight bytes and clears latched status while preserving the
    /// configured address. Idempotent.
    pub fn recover(&mut self) {
        recover_reg(self.info.reg());
        self.restart_pending = false;
    }

    /// Wait for the next event from the controller.
    pub fn listen(&mut self) -> Request {
        loop {
            let status = self.info.reg().status().read();

            if status.stop().bit() {
                self.clear_stop();
                self.restart_pending = false;
                return Request::Stop(self.address);
            }

            if let Some(req) = self.take_start_request(&status) {
                return req;
            }
        }
    }

    /// Drain incoming bytes for an in-flight write transfer into `buf`.
    pub fn respond_to_write(&mut self, buf: &mut [u8]) -> WriteStatus {
        let mut n = 0;
        loop {
            let status = self.info.reg().status().read();

            if status.rx_rdy().bit() {
                let byte = self.info.reg().data().read().data().bits();
                if n < buf.len() {
                    buf[n] = byte;
                    n += 1;
                    // ACK the byte so the controller may send the next one.
                    self.info.reg().ctrl().modify(|_, w| w.rx_ack().set_bit());
                    if n == buf.len() {
                        return WriteStatus::BufferFull(n);
                    }
                } else {
                    // No room: reject the byte and end the response.
                    self.info.reg().ctrl().modify(|_, w| w.rx_nak().set_bit());
                    return WriteStatus::BufferFull(n);
                }
            } else if status.stop().bit() {
                self.clear_stop();
                self.restart_pending = false;
                return WriteStatus::Stopped(n);
            } else if status.start().bit() {
                // Repeated start: leave it latched for the next listen().
                return WriteStatus::Restarted(n);
            }
        }
    }

    /// Supply outgoing bytes from `buf` for an in-flight read transfer.
    pub fn respond_to_read(&mut self, buf: &[u8]) -> ReadStatus {
        let mut sent = 0;
        for &byte in buf {
            // SAFETY: Any u8 value is valid for the DATA register.
            self.info
                .reg()
                .data()
                .write(|w| unsafe { w.data().bits(byte) });

            loop {
                let status = self.info.reg().status().read();

                if status.tx_done().bit() {
                    sent += 1;
                    let nak = status.tx_nak().bit();
                    self.clear_tx_done();
                    if nak {
                        return if sent == buf.len() {
                            ReadStatus::Complete(sent)
                        } else {
                            ReadStatus::EarlyStop(sent)
                        };
                    }
                    break;
                } else if status.stop().bit() {
                    self.clear_stop();
                    self.restart_pending = false;
                    return ReadStatus::EarlyStop(sent);
                } else if status.start().bit() {
                    return ReadStatus::EarlyStop(sent);
                }
            }
        }

        // Buffer exhausted and every byte was acknowledged: the controller wants more.
        ReadStatus::NeedMore(sent)
    }
}

impl<'d> I2c<'d, Async> {
    /// Create a new async I2C target driver answering to `address` (7-bit).
    pub fn new_async<T: Instance>(
        _peri: Peri<'d, T>,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>>,
        address: u8,
    ) -> Self {
        let i2c = Self::new_inner::<T>(address);
        plic_enable_i2c_target();
        i2c
    }

    // Wait until the device latches an actionable status bit (START/STOP/RX_RDY/TX_DONE).
    async fn wait(&mut self) {
        poll_fn(|cx| {
            self.info.waker.register(cx.waker());

            let status = self.info.reg().status().read();
            if status.start().bit()
                || status.stop().bit()
                || status.rx_rdy().bit()
                || status.tx_done().bit()
            {
                Poll::Ready(())
            } else {
                // (Re-)enable all event interrupts and wait for the handler.
                critical_section::with(|_| {
                    self.info.reg().ctrl().modify(|_, w| {
                        w.int_start().set_bit();
                        w.int_stop().set_bit();
                        w.int_rx_rdy().set_bit();
                        w.int_tx().set_bit()
                    });
                });
                Poll::Pending
            }
        })
        .await
    }

    /// Bring the target back to a known-clean baseline after a wedged or cancelled
    /// transfer.
    ///
    /// Drops any in-flight bytes and clears latched status while preserving the
    /// configured address. Idempotent.
    pub async fn recover(&mut self) {
        recover_reg(self.info.reg());
        self.restart_pending = false;
    }

    /// Wait for the next event from the controller.
    pub async fn listen(&mut self) -> Request {
        loop {
            let status = self.info.reg().status().read();

            if status.stop().bit() {
                self.clear_stop();
                self.restart_pending = false;
                return Request::Stop(self.address);
            }

            if let Some(req) = self.take_start_request(&status) {
                return req;
            }

            self.wait().await;
        }
    }

    /// Drain incoming bytes for an in-flight write transfer into `buf`.
    pub async fn respond_to_write(&mut self, buf: &mut [u8]) -> WriteStatus {
        let mut n = 0;
        loop {
            let status = self.info.reg().status().read();

            if status.rx_rdy().bit() {
                let byte = self.info.reg().data().read().data().bits();
                if n < buf.len() {
                    buf[n] = byte;
                    n += 1;
                    // ACK the byte so the controller may send the next one.
                    self.info.reg().ctrl().modify(|_, w| w.rx_ack().set_bit());
                    if n == buf.len() {
                        return WriteStatus::BufferFull(n);
                    }
                } else {
                    // No room: reject the byte and end the response.
                    self.info.reg().ctrl().modify(|_, w| w.rx_nak().set_bit());
                    return WriteStatus::BufferFull(n);
                }
            } else if status.stop().bit() {
                self.clear_stop();
                self.restart_pending = false;
                return WriteStatus::Stopped(n);
            } else if status.start().bit() {
                // Repeated start: leave it latched for the next listen().
                return WriteStatus::Restarted(n);
            } else {
                self.wait().await;
            }
        }
    }

    /// Supply outgoing bytes from `buf` for an in-flight read transfer.
    pub async fn respond_to_read(&mut self, buf: &[u8]) -> ReadStatus {
        let mut sent = 0;
        for &byte in buf {
            // SAFETY: Any u8 value is valid for the DATA register.
            self.info
                .reg()
                .data()
                .write(|w| unsafe { w.data().bits(byte) });

            loop {
                let status = self.info.reg().status().read();

                if status.tx_done().bit() {
                    sent += 1;
                    let nak = status.tx_nak().bit();
                    self.clear_tx_done();
                    if nak {
                        return if sent == buf.len() {
                            ReadStatus::Complete(sent)
                        } else {
                            ReadStatus::EarlyStop(sent)
                        };
                    }
                    break;
                } else if status.stop().bit() {
                    self.clear_stop();
                    self.restart_pending = false;
                    return ReadStatus::EarlyStop(sent);
                } else if status.start().bit() {
                    return ReadStatus::EarlyStop(sent);
                } else {
                    self.wait().await;
                }
            }
        }

        // Buffer exhausted and every byte was acknowledged: the controller wants more.
        ReadStatus::NeedMore(sent)
    }
}

impl<M: Mode> Drop for I2c<'_, M> {
    fn drop(&mut self) {
        // Return the device to idle and disable its PLIC source.
        recover_reg(self.info.reg());
        use pac::interrupt::ExternalInterrupt;
        plic().ctx0().enables().disable(ExternalInterrupt::I2C_TARGET);
    }
}

struct Info {
    reg: *const i2c_target::RegisterBlock,
    waker: AtomicWaker,
}

// SAFETY: The register block pointer is to memory-mapped I/O valid for the program lifetime.
// AtomicWaker is Send+Sync.
unsafe impl Send for Info {}
unsafe impl Sync for Info {}

impl Info {
    #[inline(always)]
    fn reg(&self) -> &i2c_target::RegisterBlock {
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

/// I2C target instance trait.
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
                    waker: AtomicWaker::new(),
                };
                &INFO
            }
        }

        impl Instance for peripherals::$peri {
            type Interrupt = crate::interrupt::typelevel::$peri;
        }
    };
}

impl_instance!(I2C_TARGET, I2cTarget);

impl<M: Mode> embedded_mcu_hal::i2c::target::ErrorType for I2c<'_, M> {
    type Error = Infallible;
}

impl embedded_mcu_hal::i2c::target::blocking::I2c for I2c<'_, Blocking> {
    fn recover(&mut self) -> Result<(), Self::Error> {
        self.recover();
        Ok(())
    }

    fn listen(&mut self) -> Result<Request, Self::Error> {
        Ok(self.listen())
    }

    fn respond_to_read(&mut self, buf: &[u8]) -> Result<ReadStatus, Self::Error> {
        Ok(self.respond_to_read(buf))
    }

    fn respond_to_write(&mut self, buf: &mut [u8]) -> Result<WriteStatus, Self::Error> {
        Ok(self.respond_to_write(buf))
    }
}

impl embedded_mcu_hal::i2c::target::asynch::I2c for I2c<'_, Async> {
    async fn recover(&mut self) -> Result<(), Self::Error> {
        self.recover().await;
        Ok(())
    }

    async fn listen(&mut self) -> Result<Request, Self::Error> {
        Ok(self.listen().await)
    }

    async fn respond_to_read(&mut self, buf: &[u8]) -> Result<ReadStatus, Self::Error> {
        Ok(self.respond_to_read(buf).await)
    }

    async fn respond_to_write(&mut self, buf: &mut [u8]) -> Result<WriteStatus, Self::Error> {
        Ok(self.respond_to_write(buf).await)
    }
}
