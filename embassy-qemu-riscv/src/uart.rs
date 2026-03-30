use crate::pac::uart0;
use crate::{pac, peripherals};
use core::marker::PhantomData;
use core::sync::atomic::{AtomicU8, Ordering};
use embassy_hal_internal::{Peri, PeripheralType};

/// 16550A TX/RX FIFO size in bytes.
const FIFO_SZ: usize = 16;

/// QEMU virt 16550A UART input clock frequency in Hz.
const UART_CLK: u32 = 3_686_400;

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

fn init<T: Instance>(config: Config) -> Result<(), Error> {
    let info = T::info();

    // Calculate baud rate divisor: baud = clock / (16 * divisor)
    let divisor = UART_CLK
        .checked_div(16 * config.baudrate)
        .ok_or(Error::InvalidBaud)?;

    if divisor == 0 || divisor > u16::MAX as u32 {
        return Err(Error::InvalidBaud);
    }
    let divisor = divisor as u16;

    // Set LCR: 8 data bits, 1 stop bit, no parity, DLAB=1
    info.reg.data().lcr().write(|w| {
        // SAFETY: 0b11 is a valid 2-bit WLS value (8 data bits).
        unsafe { w.wls().bits(0b11) };
        w.dlab().set_bit()
    });

    // Write baud rate divisor
    info.reg
        .dlab()
        .dll()
        // SAFETY: Any u8 value is a valid divisor latch LSB.
        .write(|w| unsafe { w.bits(divisor as u8) });
    info.reg
        .dlab()
        .dlm()
        // SAFETY: Any u8 value is a valid divisor latch MSB.
        .write(|w| unsafe { w.bits((divisor >> 8) as u8) });

    // Clear DLAB to access normal registers
    info.reg.dlab().lcr().modify(|_, w| w.dlab().clear_bit());

    // Enable and clear both FIFOs
    info.reg.data().fcr().write(|w| {
        w.fifoe().set_bit();
        w.rfifor().set_bit();
        w.xfifor().set_bit()
    });

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
        init::<T>(config)?;
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

/// RX-only UART driver.
pub struct UartRx<'d, M: Mode> {
    info: Info,
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

        Ok(self.info.reg.data().rbr().read().bits())
    }

    fn blocking_read_byte(&mut self) -> Result<u8, Error> {
        // LSR clears error bits on read so want to make sure only read it once after data ready
        let lsr = loop {
            let lsr = self.info.reg.data().lsr().read();
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
        init::<T>(config)?;
        Ok(Self::new_inner::<T>())
    }
}

/// TX-only UART driver.
#[allow(dead_code)]
pub struct UartTx<'d, M: Mode> {
    info: Info,
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
                .reg
                .data()
                .thr()
                .write(|w| unsafe { w.bits(*byte) });
        }
    }

    /// Writes bytes to TX FIFO, blocking if full.
    pub fn blocking_write(&mut self, buf: &[u8]) {
        for chunk in buf.chunks(FIFO_SZ) {
            while !self.info.reg.data().lsr().read().thre().bit() {}
            self.write_chunk(chunk);
        }
    }

    /// Blocks until both the transmit holding and shift registers are empty.
    pub fn blocking_flush(&mut self) {
        // Note: The register for some reason is named "Transmit Error" but it really
        // reflects the status of both the transmit and shift registers aka "Busy" status
        while !self.info.reg.data().lsr().read().temt().bit() {}
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
        init::<T>(config)?;
        Ok(Self::new_inner::<T>())
    }
}

struct Info {
    reg: &'static uart0::RegisterBlock,
    rx_tx_refcount: AtomicU8,
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
    fn info() -> Info;
}

/// UART instance trait.
#[allow(private_bounds)]
pub trait Instance: SealedInstance + PeripheralType + 'static + Send {}

macro_rules! impl_instance {
    ($peri:ident, $rb:ident) => {
        impl SealedInstance for peripherals::$peri {
            #[inline(always)]
            fn info() -> Info {
                Info {
                    // SAFETY: We are the sole users of the pointer and use it safely.
                    reg: unsafe { &*pac::$rb::ptr() },
                    rx_tx_refcount: AtomicU8::new(0),
                }
            }
        }

        impl Instance for peripherals::$peri {}
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

// Note: We don't currently have async support, but need embedded-io-async for embedded-services
impl embedded_io_async::Read for Uart<'_, Blocking> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.rx.blocking_read(buf)?;
        Ok(buf.len())
    }
}

impl embedded_io_async::Read for UartRx<'_, Blocking> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.blocking_read(buf)?;
        Ok(buf.len())
    }
}

impl embedded_io_async::Write for Uart<'_, Blocking> {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.tx.blocking_write(buf);
        Ok(buf.len())
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        self.tx.blocking_flush();
        Ok(())
    }
}

impl embedded_io_async::Write for UartTx<'_, Blocking> {
    async fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.blocking_write(buf);
        Ok(buf.len())
    }

    async fn flush(&mut self) -> Result<(), Self::Error> {
        self.blocking_flush();
        Ok(())
    }
}
