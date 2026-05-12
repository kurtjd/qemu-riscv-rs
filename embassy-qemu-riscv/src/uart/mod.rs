pub mod buffered;

use crate::interrupt::typelevel::{Binding, Interrupt};
use crate::pac::uart0;
use crate::{pac, peripherals, plic};
use core::future::poll_fn;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicU8, Ordering};
use core::task::Poll;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;

/// 16550A TX/RX FIFO size in bytes.
const FIFO_SZ: usize = 16;

/// QEMU virt 16550A UART input clock frequency in Hz.
const UART_CLK: u32 = 3_686_400;

// Enable UART0 interrupt source in the PLIC.
//
// Sets the UART0 priority to P1 and enables the source in PLIC context 0.
// This should be called when creating an async UART instance.
pub(crate) fn plic_enable_uart0() {
    use pac::interrupt::{ExternalInterrupt, Priority};

    // SAFETY: We have sole control of PLIC configuration for UART0
    unsafe {
        plic()
            .priorities()
            .set_priority(ExternalInterrupt::UART0, Priority::P1);
        plic().ctx0().enables().enable(ExternalInterrupt::UART0);
    }
}

// The HW gives us a convenient scratch register which we use to hold interrupt flags.
mod int_flag {
    pub(crate) const RX_AVAILABLE: u8 = 1 << 0;
    pub(crate) const TIMEOUT: u8 = 1 << 1;
    pub(crate) const TX_EMPTY: u8 = 1 << 2;
}

/// UART interrupt handler.
pub struct InterruptHandler<T: Instance> {
    _phantom: PhantomData<T>,
}

impl<T: Instance> crate::interrupt::typelevel::Handler<T::Interrupt> for InterruptHandler<T> {
    unsafe fn on_interrupt() {
        let info = T::info();

        let iir = info.reg().data().iir().read();
        let intid = InterruptType::try_from(iir.iid().bits());

        match intid {
            // In case of RxAvailable or LineStatus, we set the RX_AVAILABLE flag
            // and disable the interrupt to clear it (which clears it in IIR).
            // The RX task will re-enable the interrupt when it goes back to waiting.
            Ok(InterruptType::LineStatus) | Ok(InterruptType::RxAvailable) => {
                // SAFETY: Any u8 value is valid for the SCR register.
                info.reg()
                    .data()
                    .scr()
                    .modify(|r, w| unsafe { w.bits(r.bits() | int_flag::RX_AVAILABLE) });
                info.reg().data().ier().modify(|_, w| {
                    w.elsi().clear_bit();
                    w.erbfi().clear_bit()
                });
                info.rx_waker.wake();
            }

            // Character timeout means data is ready but FIFO trigger level wasn't reached.
            // Same handling as RxAvailable but with a different flag so the task can
            // fall back to byte-by-byte reading.
            Ok(InterruptType::CharacterTimeout) => {
                // SAFETY: Any u8 value is valid for the SCR register.
                info.reg()
                    .data()
                    .scr()
                    .modify(|r, w| unsafe { w.bits(r.bits() | int_flag::TIMEOUT) });
                info.reg().data().ier().modify(|_, w| {
                    w.elsi().clear_bit();
                    w.erbfi().clear_bit()
                });
                info.rx_waker.wake();
            }

            // We mark TX empty in the SCR flag rather than checking LSR in the TX task,
            // because reading LSR would clear error bits that the RX task might need to see.
            Ok(InterruptType::TxEmpty) => {
                // SAFETY: Any u8 value is valid for the SCR register.
                info.reg()
                    .data()
                    .scr()
                    .modify(|r, w| unsafe { w.bits(r.bits() | int_flag::TX_EMPTY) });
                info.reg().data().ier().modify(|_, w| w.etbei().clear_bit());
                info.tx_waker.wake();
            }

            // Unknown/unsupported interrupt type, so ignore
            _ => (),
        }
    }
}

enum InterruptType {
    LineStatus,
    RxAvailable,
    CharacterTimeout,
    TxEmpty,
    // MODEM is not currently supported
    _Modem,
}

impl TryFrom<u8> for InterruptType {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        // Note: This always assumes we are in FIFO mode (the driver does not support non-FIFO mode)
        Ok(match value {
            0b011 => Self::LineStatus,
            0b010 => Self::RxAvailable,
            0b110 => Self::CharacterTimeout,
            0b001 => Self::TxEmpty,
            0b000 => Self::_Modem,
            _ => Err(())?,
        })
    }
}

enum RxFifoTrigger {
    _1,
    _4,
    _8,
    _14,
}

impl From<RxFifoTrigger> for u8 {
    fn from(trigger: RxFifoTrigger) -> Self {
        match trigger {
            RxFifoTrigger::_1 => 0b00,
            RxFifoTrigger::_4 => 0b01,
            RxFifoTrigger::_8 => 0b10,
            RxFifoTrigger::_14 => 0b11,
        }
    }
}

/// UART configuration.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    /// Baudrate in bits per second (BPS).
    pub baudrate: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self { baudrate: 115_200 }
    }
}

/// UART error.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Error {
    /// A baud rate was supplied which can not be represenetd based on chosen clock source.
    InvalidBaud,
    /// RX FIFO overrun occurred.
    Overrun,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidBaud => write!(f, "Invalid baud rate"),
            Self::Overrun => write!(f, "RX FIFO overrun"),
        }
    }
}
impl core::error::Error for Error {}

fn init(reg: &uart0::RegisterBlock, config: Config) -> Result<(), Error> {
    // Calculate baud rate divisor: baud = clock / (16 * divisor)
    let divisor = UART_CLK
        .checked_div(16 * config.baudrate)
        .ok_or(Error::InvalidBaud)?;

    if divisor == 0 || divisor > u16::MAX as u32 {
        return Err(Error::InvalidBaud);
    }
    let divisor = divisor as u16;

    // Set LCR: 8 data bits, 1 stop bit, no parity, DLAB=1
    reg.data().lcr().write(|w| {
        // SAFETY: 0b11 is a valid 2-bit WLS value (8 data bits).
        unsafe { w.wls().bits(0b11) };
        w.dlab().set_bit()
    });

    // Write baud rate divisor
    reg.dlab()
        .dll()
        // SAFETY: Any u8 value is a valid divisor latch LSB.
        .write(|w| unsafe { w.bits(divisor as u8) });
    reg.dlab()
        .dlm()
        // SAFETY: Any u8 value is a valid divisor latch MSB.
        .write(|w| unsafe { w.bits((divisor >> 8) as u8) });

    // Clear DLAB to access normal registers
    reg.dlab().lcr().modify(|_, w| w.dlab().clear_bit());

    // Enable and clear both FIFOs
    reg.data().fcr().write(|w| {
        w.fifoe().set_bit();
        w.rfifor().set_bit();
        w.xfifor().set_bit()
    });

    // Clear scratch register which we use for interrupt flags
    // SAFETY: Any u8 value is valid for the SCR register.
    reg.data().scr().write(|w| unsafe { w.bits(0) });

    Ok(())
}

/// UART driver.
#[allow(dead_code)]
pub struct Uart<'d, M: Mode> {
    rx: UartRx<'d, M>,
    tx: UartTx<'d, M>,
}

impl<'d, M: Mode> Uart<'d, M> {
    fn new_inner<T: Instance>(config: Config) -> Result<Self, Error> {
        init(T::info().reg(), config)?;
        let rx = UartRx::new_inner::<T>();
        let tx = UartTx::new_inner::<T>();
        Ok(Self { rx, tx })
    }

    /// Reads bytes from RX FIFO until buffer is full, blocking if empty.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Overrun`] if FIFO overrun error occurred during read.
    pub fn blocking_read(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        self.rx.blocking_read(buf)
    }

    /// Writes bytes to TX FIFO, blocking if full.
    pub fn blocking_write(&mut self, bytes: &[u8]) {
        self.tx.blocking_write(bytes)
    }

    /// Blocks until both the transmit holding and shift registers are empty.
    pub fn blocking_flush(&mut self) {
        self.tx.blocking_flush()
    }

    /// Splits the UART driver into separate [`UartRx`] and [`UartTx`] drivers.
    ///
    /// Helpful for sharing the UART among receiver/transmitter tasks.
    pub fn split(self) -> (UartRx<'d, M>, UartTx<'d, M>) {
        (self.rx, self.tx)
    }

    /// Splits the UART driver into separate [`UartRx`] and [`UartTx`] drivers by mutable reference.
    ///
    /// Helpful for sharing the UART among receiver/transmitter tasks without destroying the original [`Uart`] instance.
    pub fn split_ref(&mut self) -> (&mut UartRx<'d, M>, &mut UartTx<'d, M>) {
        (&mut self.rx, &mut self.tx)
    }
}

impl<'d> Uart<'d, Blocking> {
    /// Create a new blocking UART driver instance with given configuration.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidBaud`] if the supplied baud rate can not be represented.
    pub fn new_blocking<T: Instance>(_peri: Peri<'d, T>, config: Config) -> Result<Self, Error> {
        Self::new_inner::<T>(config)
    }
}

impl<'d> Uart<'d, Async> {
    /// Create a new async UART driver instance with given configuration.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidBaud`] if the supplied baud rate can not be represented.
    pub fn new_async<T: Instance>(
        _peri: Peri<'d, T>,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>>,
        config: Config,
    ) -> Result<Self, Error> {
        let uart = Self::new_inner::<T>(config)?;
        plic_enable_uart0();
        Ok(uart)
    }

    /// Reads bytes from RX FIFO until buffer is full.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Overrun`] if FIFO overrun error occurred during read.
    pub async fn read(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        self.rx.read(buf).await
    }

    /// Writes bytes to TX FIFO.
    pub async fn write(&mut self, bytes: &[u8]) {
        self.tx.write(bytes).await
    }

    /// Waits until both the transmit holding and shift registers are empty.
    pub async fn flush(&mut self) {
        self.tx.flush().await
    }
}

/// RX-only UART driver.
pub struct UartRx<'d, M: Mode> {
    info: &'static Info,
    _phantom: PhantomData<&'d M>,
}

impl<'d, M: Mode> UartRx<'d, M> {
    fn new_inner<T: Instance>() -> Self {
        T::info().rx_tx_refcount.fetch_add(1, Ordering::AcqRel);

        Self {
            info: T::info(),
            _phantom: PhantomData,
        }
    }

    fn read_inner(&mut self, lsr: uart0::data::lsr::R) -> Result<u8, Error> {
        // An overrun error is not associated with particular character like others
        // We just bail out early, and it might be hard to recover correctly from this
        //
        // The onus will likely need to be on caller to handle an overrun how they see fit
        if lsr.oe().bit() {
            return Err(Error::Overrun);
        }

        Ok(self.info.reg().data().rbr().read().bits())
    }

    fn blocking_read_byte(&mut self) -> Result<u8, Error> {
        // LSR clears error bits on read so want to make sure only read it once after data ready
        let lsr = loop {
            let lsr = self.info.reg().data().lsr().read();
            if lsr.dr().bit() {
                break lsr;
            }
        };

        self.read_inner(lsr)
    }

    /// Reads bytes from RX FIFO until buffer is full, blocking if empty.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Overrun`] if FIFO overrun error occurred during read.
    pub fn blocking_read(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        for byte in buf.iter_mut() {
            *byte = self.blocking_read_byte()?
        }

        Ok(())
    }
}

impl<'d> UartRx<'d, Blocking> {
    /// Create a new blocking RX-only UART driver instance with given configuration.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidBaud`] if the supplied baud rate can not be represented by
    /// given clock source.
    pub fn new_blocking<T: Instance>(_peri: Peri<'d, T>, config: Config) -> Result<Self, Error> {
        init(T::info().reg(), config)?;
        Ok(Self::new_inner::<T>())
    }
}

impl<'d> UartRx<'d, Async> {
    /// Create a new async RX-only UART driver instance with given configuration.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidBaud`] if the supplied baud rate can not be represented by
    /// given clock source.
    pub fn new_async<T: Instance>(
        _peri: Peri<'d, T>,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>>,
        config: Config,
    ) -> Result<Self, Error> {
        init(T::info().reg(), config)?;
        plic_enable_uart0();
        Ok(Self::new_inner::<T>())
    }

    async fn wait_rx_ready(&mut self) -> bool {
        poll_fn(|cx| {
            self.info.rx_waker.register(cx.waker());
            let int_flags = self.info.reg().data().scr().read().bits();

            if int_flags & int_flag::TIMEOUT != 0 {
                critical_section::with(|_| {
                    // SAFETY: Any u8 value is valid for the SCR register.
                    self.info
                        .reg()
                        .data()
                        .scr()
                        .modify(|r, w| unsafe { w.bits(r.bits() & !int_flag::TIMEOUT) });
                });
                // Indicates a byte is ready to read, but we didn't trigger the FIFO
                // level before timeout
                Poll::Ready(false)
            } else if int_flags & int_flag::RX_AVAILABLE != 0 {
                critical_section::with(|_| {
                    // SAFETY: Any u8 value is valid for the SCR register.
                    self.info
                        .reg()
                        .data()
                        .scr()
                        .modify(|r, w| unsafe { w.bits(r.bits() & !int_flag::RX_AVAILABLE) });
                });
                Poll::Ready(true)
            } else {
                critical_section::with(|_| {
                    self.info.reg().data().ier().modify(|_, w| {
                        w.elsi().set_bit();
                        w.erbfi().set_bit()
                    });
                });
                Poll::Pending
            }
        })
        .await
    }

    fn set_fifo_trigger(&mut self, trigger: RxFifoTrigger) {
        self.info.reg().data().fcr().write(|w| {
            // Must always set FIFOE when writing FCR, even if previously set
            w.fifoe().set_bit();
            // SAFETY: FIFO trigger level values are valid 2-bit values.
            unsafe { w.rftl().bits(trigger.into()) }
        });
    }

    async fn read_byte(&mut self) -> Result<u8, Error> {
        // LSR clears error bits on read so want to make sure only read it once when
        // data ready then check bits
        let lsr = {
            let lsr = self.info.reg().data().lsr().read();
            if !lsr.dr().bit() {
                let _ = self.wait_rx_ready().await;
                self.info.reg().data().lsr().read()
            } else {
                lsr
            }
        };

        self.read_inner(lsr)
    }

    fn nb_read_byte(&mut self) -> Result<u8, Error> {
        let lsr = self.info.reg().data().lsr().read();
        self.read_inner(lsr)
    }

    async fn read_chunk(&mut self, chunk: &mut [u8]) -> Result<(), Error> {
        self.set_fifo_trigger(RxFifoTrigger::_1);
        for byte in chunk.iter_mut() {
            *byte = self.read_byte().await?;
        }
        Ok(())
    }

    async fn read_chunk_batched(&mut self, chunk: &mut [u8]) -> Result<(), Error> {
        // If our FIFO level was reached without timeout, we can read all the bytes
        // in one go, but still need to check each byte for an error
        if self.wait_rx_ready().await {
            for byte in chunk.iter_mut() {
                *byte = self.nb_read_byte()?;
            }
        // However, if a timeout occurred, our assumptions about the number of bytes
        // in the FIFO no longer holds (since we have to read a byte to clear the
        // timeout interrupt), meaning we have no choice but to fall back on
        // byte-by-byte interrupts
        } else {
            self.read_chunk(chunk).await?;
        }
        Ok(())
    }

    async fn read_chunks<const N: usize>(
        &mut self,
        chunks: &mut [[u8; N]],
        trigger: RxFifoTrigger,
    ) -> Result<(), Error> {
        self.set_fifo_trigger(trigger);
        for chunk in chunks.iter_mut() {
            self.read_chunk_batched(chunk).await?;
        }
        Ok(())
    }

    /// Reads bytes from RX FIFO until buffer is full.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Overrun`] if FIFO overrun error occurred during read.
    pub async fn read(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        // The idea here is that the HW provides us 4 FIFO level triggers (14, 8, 4, 1),
        // so to minimize the number of interrupts, we split the buffer up greedily into
        // chunks of the highest trigger level we can, and then know when the interrupt is
        // triggered, we can read that number of bytes in one shot.
        const TRIG_14: usize = 14;
        const TRIG_8: usize = 8;
        const TRIG_4: usize = 4;

        let (c14, rem) = buf.as_chunks_mut::<TRIG_14>();
        let (c8, rem) = rem.as_chunks_mut::<TRIG_8>();
        let (c4, c1) = rem.as_chunks_mut::<TRIG_4>();

        self.read_chunks(c14, RxFifoTrigger::_14).await?;
        self.read_chunks(c8, RxFifoTrigger::_8).await?;
        self.read_chunks(c4, RxFifoTrigger::_4).await?;

        // The last chunk is smaller than 4, so we have no choice but to interrupt
        // for each byte
        self.read_chunk(c1).await?;

        Ok(())
    }
}

impl<'d, M: Mode> Drop for UartRx<'d, M> {
    fn drop(&mut self) {
        drop_rx_tx(self.info.reg(), &self.info.rx_tx_refcount);
    }
}

/// TX-only UART driver.
pub struct UartTx<'d, M: Mode> {
    info: &'static Info,
    _phantom: PhantomData<&'d M>,
}

impl<'d, M: Mode> UartTx<'d, M> {
    fn new_inner<T: Instance>() -> Self {
        T::info().rx_tx_refcount.fetch_add(1, Ordering::AcqRel);

        Self {
            info: T::info(),
            _phantom: PhantomData,
        }
    }

    fn write_chunk(&mut self, chunk: &[u8]) {
        for byte in chunk {
            // SAFETY: Any u8 value is valid for the THR register.
            self.info
                .reg()
                .data()
                .thr()
                .write(|w| unsafe { w.bits(*byte) });
        }
    }

    /// Writes bytes to TX FIFO, blocking if full.
    pub fn blocking_write(&mut self, buf: &[u8]) {
        for chunk in buf.chunks(FIFO_SZ) {
            while !self.info.reg().data().lsr().read().thre().bit() {}
            self.write_chunk(chunk);
        }
    }

    /// Blocks until both the transmit holding and shift registers are empty.
    pub fn blocking_flush(&mut self) {
        // Note: The register for some reason is named "Transmit Error" but it really
        // reflects the status of both the transmit and shift registers aka "Busy" status
        while !self.info.reg().data().lsr().read().temt().bit() {}
    }
}

impl<'d> UartTx<'d, Blocking> {
    /// Create a new blocking TX-only UART driver instance with given configuration.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidBaud`] if the supplied baud rate can not be represented by
    /// given clock source.
    pub fn new_blocking<T: Instance>(_peri: Peri<'d, T>, config: Config) -> Result<Self, Error> {
        init(T::info().reg(), config)?;
        Ok(Self::new_inner::<T>())
    }
}

impl<'d> UartTx<'d, Async> {
    /// Create a new async TX-only UART driver instance with given configuration.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidBaud`] if the supplied baud rate can not be represented by
    /// given clock source.
    pub fn new_async<T: Instance>(
        _peri: Peri<'d, T>,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>>,
        config: Config,
    ) -> Result<Self, Error> {
        init(T::info().reg(), config)?;
        plic_enable_uart0();
        Ok(Self::new_inner::<T>())
    }

    async fn wait_tx_empty(&mut self) {
        poll_fn(|cx| {
            self.info.tx_waker.register(cx.waker());
            if self.info.reg().data().scr().read().bits() & int_flag::TX_EMPTY != 0 {
                critical_section::with(|_| {
                    // SAFETY: Any u8 value is valid for the SCR register.
                    self.info
                        .reg()
                        .data()
                        .scr()
                        .modify(|r, w| unsafe { w.bits(r.bits() & !int_flag::TX_EMPTY) });
                });
                Poll::Ready(())
            } else {
                critical_section::with(|_| {
                    self.info
                        .reg()
                        .data()
                        .ier()
                        .modify(|_, w| w.etbei().set_bit());
                });
                Poll::Pending
            }
        })
        .await
    }

    /// Writes bytes to TX FIFO.
    pub async fn write(&mut self, buf: &[u8]) {
        for chunk in buf.chunks(FIFO_SZ) {
            self.wait_tx_empty().await;
            self.write_chunk(chunk);
        }
    }

    /// Waits until both the transmit holding and shift registers are empty.
    pub async fn flush(&mut self) {
        // We can wait for an interrupt to know when the TX FIFO is empty,
        // but there does not appear to be an interrupt for when the TX shift reg
        // is empty, so have to block
        self.wait_tx_empty().await;
        self.blocking_flush();
    }
}

impl<'d, M: Mode> Drop for UartTx<'d, M> {
    fn drop(&mut self) {
        drop_rx_tx(self.info.reg(), &self.info.rx_tx_refcount);
    }
}

fn drop_rx_tx(reg: &uart0::RegisterBlock, refcount: &AtomicU8) {
    // Only disable UART once both UartRx and UartTx have been dropped
    if refcount.fetch_sub(1, Ordering::AcqRel) == 1 {
        // Disable all UART interrupts
        reg.data().ier().reset();
        // Disable UART0 in the PLIC
        use pac::interrupt::ExternalInterrupt;
        plic().ctx0().enables().disable(ExternalInterrupt::UART0);
    }
}

struct Info {
    reg: *const uart0::RegisterBlock,
    rx_tx_refcount: AtomicU8,
    rx_waker: AtomicWaker,
    tx_waker: AtomicWaker,
}

// SAFETY: The register block pointer is to memory-mapped I/O valid for the program lifetime.
// AtomicU8 and AtomicWaker are both Send+Sync.
unsafe impl Send for Info {}
unsafe impl Sync for Info {}

impl Info {
    #[inline(always)]
    fn reg(&self) -> &uart0::RegisterBlock {
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

/// UART instance trait.
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
                    rx_tx_refcount: AtomicU8::new(0),
                    rx_waker: AtomicWaker::new(),
                    tx_waker: AtomicWaker::new(),
                };
                &INFO
            }
        }

        impl Instance for peripherals::$peri {
            type Interrupt = crate::interrupt::typelevel::$peri;
        }
    };
}

impl_instance!(UART0, Uart0);

impl embedded_io::Error for Error {
    fn kind(&self) -> embedded_io::ErrorKind {
        embedded_io::ErrorKind::Other
    }
}

impl<'d, M: Mode> embedded_io::ErrorType for Uart<'d, M> {
    type Error = Error;
}

impl<'d, M: Mode> embedded_io::ErrorType for UartRx<'d, M> {
    type Error = Error;
}

impl<'d, M: Mode> embedded_io::ErrorType for UartTx<'d, M> {
    type Error = Error;
}

impl embedded_io::Read for Uart<'_, Blocking> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.rx.blocking_read(buf)?;
        Ok(buf.len())
    }
}

impl embedded_io::Read for UartRx<'_, Blocking> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.blocking_read(buf)?;
        Ok(buf.len())
    }
}

impl embedded_io::Write for Uart<'_, Blocking> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.tx.blocking_write(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.tx.blocking_flush();
        Ok(())
    }
}

impl embedded_io::Write for UartTx<'_, Blocking> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.blocking_write(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.blocking_flush();
        Ok(())
    }
}

impl embedded_io_async::Read for Uart<'_, Async> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.read(buf).await?;
        Ok(buf.len())
    }
}

impl embedded_io_async::Read for UartRx<'_, Async> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.read(buf).await?;
        Ok(buf.len())
    }
}

impl embedded_io_async::Write for Uart<'_, Async> {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.write(buf).await;
        Ok(buf.len())
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        self.flush().await;
        Ok(())
    }
}

impl embedded_io_async::Write for UartTx<'_, Async> {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.write(buf).await;
        Ok(buf.len())
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        self.flush().await;
        Ok(())
    }
}
