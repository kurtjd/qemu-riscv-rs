//! I2C controller driver.
//!
//! Drives the socket-backed I2C controller peripheral (`I2C0`). The device exposes a
//! tiny command-based interface through three registers (`CTRL`/`STATUS`/`DATA`): each
//! transfer is built from `START`, `TX`, `RX` and `STOP` commands written to `DATA`, and
//! every command completes by latching the `CMD_DONE` status bit (raising an interrupt
//! when enabled). Blocking drivers spin on `CMD_DONE`; async drivers wait on it via the
//! interrupt handler.

use crate::interrupt::typelevel::{Binding, Interrupt};
use crate::pac::i2c0;
use crate::{pac, peripherals, plic};
use core::future::poll_fn;
use core::marker::PhantomData;
use core::task::Poll;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;

/// Maximum number of bytes that can be requested in a single `RX` burst.
///
/// The hardware encodes the count as `count - 1` in a single byte and its RX FIFO is
/// 256 bytes deep, so a burst can move at most 256 bytes. Larger reads are split into
/// multiple bursts, each preceded by a repeated `START`.
const MAX_RX_BURST: usize = 256;

/// `DATA` register command codes (bits 9:8).
mod cmd {
    pub(super) const START: u8 = 0;
    pub(super) const STOP: u8 = 1;
    pub(super) const RX: u8 = 2;
    pub(super) const TX: u8 = 3;
}

// Enable the I2C0 interrupt source in the PLIC.
//
// Sets the I2C0 priority to P1 and enables the source in PLIC context 0.
// Called when creating an async I2C instance.
pub(crate) fn plic_enable_i2c0() {
    use pac::interrupt::{ExternalInterrupt, Priority};

    // SAFETY: We have sole control of PLIC configuration for I2C0
    unsafe {
        plic()
            .priorities()
            .set_priority(ExternalInterrupt::I2C0, Priority::P1);
        plic().ctx0().enables().enable(ExternalInterrupt::I2C0);
    }
}

/// I2C interrupt handler.
pub struct InterruptHandler<T: Instance> {
    _phantom: PhantomData<T>,
}

impl<T: Instance> crate::interrupt::typelevel::Handler<T::Interrupt> for InterruptHandler<T> {
    unsafe fn on_interrupt() {
        let info = T::info();

        // Disable the command-complete interrupt to de-assert the level-triggered IRQ.
        // The waiting task observes STATUS.cmd_done, clears it, and re-enables the
        // interrupt the next time it waits.
        info.reg().ctrl().modify(|_, w| w.cmd_done_ie().clear_bit());
        info.waker.wake();
    }
}

/// I2C error.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// The target did not acknowledge its address or a written byte (or the link dropped).
    NoAcknowledge,
    /// The controller reported a protocol error (illegal command ordering).
    Protocol,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NoAcknowledge => write!(f, "No acknowledge"),
            Self::Protocol => write!(f, "Protocol error"),
        }
    }
}
impl core::error::Error for Error {}

impl embedded_hal::i2c::Error for Error {
    fn kind(&self) -> embedded_hal::i2c::ErrorKind {
        use embedded_hal::i2c::{ErrorKind, NoAcknowledgeSource};
        match self {
            Self::NoAcknowledge => ErrorKind::NoAcknowledge(NoAcknowledgeSource::Unknown),
            Self::Protocol => ErrorKind::Bus,
        }
    }
}

// Reset the device back to idle: clears the RX FIFO and status and aborts any
// in-flight transfer.
fn reset(reg: &i2c0::RegisterBlock) {
    reg.ctrl().write(|w| w.reset().set_bit());
}

/// I2C controller driver.
pub struct I2c<'d, M: Mode> {
    info: &'static Info,
    _phantom: PhantomData<&'d M>,
}

impl<'d, M: Mode> I2c<'d, M> {
    fn new_inner<T: Instance>() -> Self {
        let info = T::info();
        reset(info.reg());
        Self {
            info,
            _phantom: PhantomData,
        }
    }

    // Write a command + data byte to the DATA register.
    #[inline]
    fn write_cmd(&mut self, command: u8, data: u8) {
        self.info.reg().data().write(|w| {
            // SAFETY: `data` is a full 8-bit field and `command` is a valid 2-bit command.
            unsafe {
                w.data().bits(data);
                w.cmd().bits(command)
            }
        });
    }

    // Inspect the status after a command completed, then clear all latched W1C bits
    // (including CMD_DONE).
    fn take_status(&mut self) -> Result<(), Error> {
        let status = self.info.reg().status().read();
        let result = if status.nak().bit() {
            Err(Error::NoAcknowledge)
        } else if status.proto_err().bit() {
            Err(Error::Protocol)
        } else {
            Ok(())
        };

        self.info.reg().status().write(|w| {
            w.nak().clear_bit_by_one();
            w.proto_err().clear_bit_by_one();
            w.cmd_done().clear_bit_by_one()
        });

        result
    }

    // Pop one byte from the RX FIFO.
    #[inline]
    fn pop_byte(&mut self) -> u8 {
        self.info.reg().data().read().data().bits()
    }
}

impl<'d> I2c<'d, Blocking> {
    /// Create a new blocking I2C controller driver instance.
    pub fn new_blocking<T: Instance>(_peri: Peri<'d, T>) -> Self {
        Self::new_inner::<T>()
    }

    // Spin until the current command completes, then return its status.
    fn finish(&mut self) -> Result<(), Error> {
        while !self.info.reg().status().read().cmd_done().bit() {}
        self.take_status()
    }

    fn start(&mut self, address: u8, read: bool) -> Result<(), Error> {
        self.write_cmd(cmd::START, (address << 1) | read as u8);
        self.finish()
    }

    fn write_byte(&mut self, byte: u8) -> Result<(), Error> {
        self.write_cmd(cmd::TX, byte);
        self.finish()
    }

    fn read_burst(&mut self, chunk: &mut [u8]) -> Result<(), Error> {
        self.write_cmd(cmd::RX, (chunk.len() - 1) as u8);
        self.finish()?;
        for slot in chunk.iter_mut() {
            *slot = self.pop_byte();
        }
        Ok(())
    }

    fn stop(&mut self) -> Result<(), Error> {
        self.write_cmd(cmd::STOP, 0);
        self.finish()
    }

    /// Execute a sequence of I2C operations against `address`.
    ///
    /// Operations are framed with `START`/`STOP` like [`embedded_hal::i2c::I2c::transaction`]:
    /// a (repeated) `START` is issued before the first operation, whenever the transfer
    /// direction changes, and before every read burst.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NoAcknowledge`] if the target does not acknowledge, or
    /// [`Error::Protocol`] on an illegal command ordering.
    pub fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Error> {
        use embedded_hal::i2c::Operation;

        let mut started = false;
        let mut prev_read: Option<bool> = None;

        for op in operations.iter_mut() {
            match op {
                Operation::Write(bytes) => {
                    if prev_read != Some(false) {
                        self.start(address, false)?;
                        started = true;
                    }
                    for &byte in bytes.iter() {
                        self.write_byte(byte)?;
                    }
                    prev_read = Some(false);
                }
                Operation::Read(buf) => {
                    for chunk in buf.chunks_mut(MAX_RX_BURST) {
                        // Each read burst must be preceded by a (repeated) START.
                        self.start(address, true)?;
                        started = true;
                        self.read_burst(chunk)?;
                    }
                    prev_read = Some(true);
                }
            }
        }

        if started {
            self.stop()?;
        }

        Ok(())
    }

    /// Read bytes from `address` into `buf`.
    ///
    /// # Errors
    ///
    /// See [`Self::transaction`].
    pub fn read(&mut self, address: u8, buf: &mut [u8]) -> Result<(), Error> {
        self.transaction(address, &mut [embedded_hal::i2c::Operation::Read(buf)])
    }

    /// Write `bytes` to `address`.
    ///
    /// # Errors
    ///
    /// See [`Self::transaction`].
    pub fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Error> {
        self.transaction(address, &mut [embedded_hal::i2c::Operation::Write(bytes)])
    }

    /// Write `bytes` to `address`, then read into `buf` after a repeated `START`.
    ///
    /// # Errors
    ///
    /// See [`Self::transaction`].
    pub fn write_read(&mut self, address: u8, bytes: &[u8], buf: &mut [u8]) -> Result<(), Error> {
        self.transaction(
            address,
            &mut [
                embedded_hal::i2c::Operation::Write(bytes),
                embedded_hal::i2c::Operation::Read(buf),
            ],
        )
    }
}

impl<'d> I2c<'d, Async> {
    /// Create a new async I2C controller driver instance.
    pub fn new_async<T: Instance>(
        _peri: Peri<'d, T>,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>>,
    ) -> Self {
        let i2c = Self::new_inner::<T>();
        plic_enable_i2c0();
        i2c
    }

    // Wait until the current command completes, then return its status.
    async fn finish(&mut self) -> Result<(), Error> {
        poll_fn(|cx| {
            self.info.waker.register(cx.waker());

            if self.info.reg().status().read().cmd_done().bit() {
                Poll::Ready(())
            } else {
                // (Re-)enable the command-complete interrupt and wait for the handler.
                critical_section::with(|_| {
                    self.info
                        .reg()
                        .ctrl()
                        .modify(|_, w| w.cmd_done_ie().set_bit());
                });
                Poll::Pending
            }
        })
        .await;

        self.take_status()
    }

    async fn start(&mut self, address: u8, read: bool) -> Result<(), Error> {
        self.write_cmd(cmd::START, (address << 1) | read as u8);
        self.finish().await
    }

    async fn write_byte(&mut self, byte: u8) -> Result<(), Error> {
        self.write_cmd(cmd::TX, byte);
        self.finish().await
    }

    async fn read_burst(&mut self, chunk: &mut [u8]) -> Result<(), Error> {
        self.write_cmd(cmd::RX, (chunk.len() - 1) as u8);
        self.finish().await?;
        for slot in chunk.iter_mut() {
            *slot = self.pop_byte();
        }
        Ok(())
    }

    async fn stop(&mut self) -> Result<(), Error> {
        self.write_cmd(cmd::STOP, 0);
        self.finish().await
    }

    /// Execute a sequence of I2C operations against `address`.
    ///
    /// Operations are framed with `START`/`STOP` like
    /// [`embedded_hal_async::i2c::I2c::transaction`]: a (repeated) `START` is issued before
    /// the first operation, whenever the transfer direction changes, and before every read
    /// burst.
    ///
    /// # Errors
    ///
    /// Returns [`Error::NoAcknowledge`] if the target does not acknowledge, or
    /// [`Error::Protocol`] on an illegal command ordering.
    pub async fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Error> {
        use embedded_hal::i2c::Operation;

        let mut started = false;
        let mut prev_read: Option<bool> = None;

        for op in operations.iter_mut() {
            match op {
                Operation::Write(bytes) => {
                    if prev_read != Some(false) {
                        self.start(address, false).await?;
                        started = true;
                    }
                    for &byte in bytes.iter() {
                        self.write_byte(byte).await?;
                    }
                    prev_read = Some(false);
                }
                Operation::Read(buf) => {
                    for chunk in buf.chunks_mut(MAX_RX_BURST) {
                        // Each read burst must be preceded by a (repeated) START.
                        self.start(address, true).await?;
                        started = true;
                        self.read_burst(chunk).await?;
                    }
                    prev_read = Some(true);
                }
            }
        }

        if started {
            self.stop().await?;
        }

        Ok(())
    }

    /// Read bytes from `address` into `buf`.
    ///
    /// # Errors
    ///
    /// See [`Self::transaction`].
    pub async fn read(&mut self, address: u8, buf: &mut [u8]) -> Result<(), Error> {
        self.transaction(address, &mut [embedded_hal::i2c::Operation::Read(buf)])
            .await
    }

    /// Write `bytes` to `address`.
    ///
    /// # Errors
    ///
    /// See [`Self::transaction`].
    pub async fn write(&mut self, address: u8, bytes: &[u8]) -> Result<(), Error> {
        self.transaction(address, &mut [embedded_hal::i2c::Operation::Write(bytes)])
            .await
    }

    /// Write `bytes` to `address`, then read into `buf` after a repeated `START`.
    ///
    /// # Errors
    ///
    /// See [`Self::transaction`].
    pub async fn write_read(
        &mut self,
        address: u8,
        bytes: &[u8],
        buf: &mut [u8],
    ) -> Result<(), Error> {
        self.transaction(
            address,
            &mut [
                embedded_hal::i2c::Operation::Write(bytes),
                embedded_hal::i2c::Operation::Read(buf),
            ],
        )
        .await
    }
}

impl<M: Mode> Drop for I2c<'_, M> {
    fn drop(&mut self) {
        // Return the device to idle and disable its PLIC source.
        reset(self.info.reg());
        use pac::interrupt::ExternalInterrupt;
        plic().ctx0().enables().disable(ExternalInterrupt::I2C0);
    }
}

struct Info {
    reg: *const i2c0::RegisterBlock,
    waker: AtomicWaker,
}

// SAFETY: The register block pointer is to memory-mapped I/O valid for the program lifetime.
// AtomicWaker is Send+Sync.
unsafe impl Send for Info {}
unsafe impl Sync for Info {}

impl Info {
    #[inline(always)]
    fn reg(&self) -> &i2c0::RegisterBlock {
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

/// I2C instance trait.
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

impl_instance!(I2C0, I2c0);

impl<M: Mode> embedded_hal::i2c::ErrorType for I2c<'_, M> {
    type Error = Error;
}

impl embedded_hal::i2c::I2c for I2c<'_, Blocking> {
    fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.transaction(address, operations)
    }
}

impl embedded_hal_async::i2c::I2c for I2c<'_, Async> {
    async fn transaction(
        &mut self,
        address: u8,
        operations: &mut [embedded_hal::i2c::Operation<'_>],
    ) -> Result<(), Self::Error> {
        self.transaction(address, operations).await
    }
}
