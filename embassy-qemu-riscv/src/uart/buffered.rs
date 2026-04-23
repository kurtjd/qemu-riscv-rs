//! Buffered UART driver.
//!
//! Unlike the unbuffered [`uart`](crate::uart) driver which reads directly from the hardware
//! FIFO, this driver uses an interrupt handler that continuously drains incoming bytes into
//! a user-provided ring buffer. This significantly reduces the chance of hardware FIFO overruns
//! when the application cannot service received data quickly enough.
//!
//! The user provides a `&mut [u8]` buffer at construction time. The larger the buffer, the more
//! time the application has to read data before the buffer fills and new data is lost.

use super::{
    Async, Blocking, Config, Error, FIFO_SZ, InterruptType, Mode, int_flag, plic_enable_uart0,
};
use crate::interrupt::typelevel::{Binding, Interrupt};
use crate::pac::uart0;
use crate::{pac, peripherals};
use core::future::poll_fn;
use core::marker::PhantomData;
use core::sync::atomic::{AtomicU8, Ordering};
use core::task::Poll;
use embassy_hal_internal::atomic_ring_buffer::RingBuffer;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;

/// Buffered UART interrupt handler.
///
/// Unlike the unbuffered [`uart::InterruptHandler`](crate::uart::InterruptHandler), this handler
/// drains all available bytes from the hardware FIFO into a software ring buffer on every
/// RX interrupt. RX interrupts stay enabled permanently.
pub struct InterruptHandler<T: Instance> {
    _phantom: PhantomData<T>,
}

impl<T: Instance> crate::interrupt::typelevel::Handler<T::Interrupt> for InterruptHandler<T> {
    unsafe fn on_interrupt() {
        let info = T::info();

        let iir = info.reg().data().iir().read();
        let intid = InterruptType::try_from(iir.iid().bits());

        match intid {
            Ok(InterruptType::LineStatus) => {
                // Read LSR to acknowledge the error condition.
                // The RxAvailable path will encounter the error byte when draining.
                let _lsr = info.reg().data().lsr().read();
            }

            Ok(InterruptType::RxAvailable) | Ok(InterruptType::CharacterTimeout) => {
                // Drain ALL bytes from HW FIFO into ring buffer
                let mut pushed_any = false;
                while info.reg().data().lsr().read().dr().bit() {
                    let byte = info.reg().data().rbr().read().bits();
                    // SAFETY: Writer is only created in ISR context; there is one ISR.
                    let mut writer = unsafe { info.buffer.writer() };
                    if writer.push_one(byte) {
                        pushed_any = true;
                    } else {
                        break; // Ring buffer full
                    }
                }

                if pushed_any {
                    info.rx_waker.wake();
                }
            }

            // TX empty: same pattern as unbuffered driver
            Ok(InterruptType::TxEmpty) => {
                // SAFETY: Any u8 value is valid for the SCR register.
                info.reg()
                    .data()
                    .scr()
                    .modify(|r, w| unsafe { w.bits(r.bits() | int_flag::TX_EMPTY) });
                info.reg().data().ier().modify(|_, w| w.etbei().clear_bit());
                info.tx_waker.wake();
            }

            _ => (),
        }
    }
}

/// Buffered UART driver.
pub struct Uart<'d, M: Mode> {
    rx: UartRx<'d, M>,
    tx: UartTx<'d, M>,
}

impl<'d, M: Mode> Uart<'d, M> {
    fn new_inner<T: Instance>(rx_buf: &'d mut [u8], config: Config) -> Result<Self, Error> {
        let info = T::info();
        super::init(info.reg(), config)?;
        let rx = UartRx::new_inner::<T>(rx_buf);
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
    /// Create a new blocking buffered UART driver instance with given configuration.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidBaud`] if the supplied baud rate can not be represented.
    pub fn new_blocking<T: Instance>(
        _peri: Peri<'d, T>,
        rx_buf: &'d mut [u8],
        config: Config,
    ) -> Result<Self, Error> {
        Self::new_inner::<T>(rx_buf, config)
    }
}

impl<'d> Uart<'d, Async> {
    /// Create a new async buffered UART driver instance with given configuration.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidBaud`] if the supplied baud rate can not be represented.
    pub fn new_async<T: Instance>(
        _peri: Peri<'d, T>,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>>,
        rx_buf: &'d mut [u8],
        config: Config,
    ) -> Result<Self, Error> {
        let uart = Self::new_inner::<T>(rx_buf, config)?;
        plic_enable_uart0();
        Ok(uart)
    }

    /// Read bytes from the ring buffer.
    ///
    /// Returns the number of bytes read (at least 1). Waits asynchronously if no data is available.
    pub async fn read(&mut self, buf: &mut [u8]) -> usize {
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

/// Buffered RX-only UART driver.
///
/// The interrupt handler drains the hardware FIFO into a software ring buffer.
/// Async reads consume data from this ring buffer, not directly from hardware.
pub struct UartRx<'d, M: Mode> {
    info: &'static Info,
    _phantom: PhantomData<&'d M>,
}

impl<'d, M: Mode> UartRx<'d, M> {
    fn new_inner<T: Instance>(rx_buf: &'d mut [u8]) -> Self {
        let info = T::info();
        info.rx_tx_refcount.fetch_add(1, Ordering::AcqRel);

        // SAFETY: The buffer lives for 'd which outlives the driver instance.
        // No other code initializes this buffer concurrently.
        unsafe { info.buffer.init(rx_buf.as_mut_ptr(), rx_buf.len()) }

        // Set FIFO trigger to 1 byte — the ISR will drain all available bytes anyway
        info.reg().data().fcr().write(|w| {
            w.fifoe().set_bit();
            // SAFETY: 0b00 is a valid 2-bit trigger level value (1 byte).
            unsafe { w.rftl().bits(0b00) }
        });

        // Enable RX interrupts permanently — the ISR drains bytes into the ring buffer
        critical_section::with(|_| {
            info.reg().data().ier().modify(|_, w| {
                w.elsi().set_bit();
                w.erbfi().set_bit()
            });
        });

        Self {
            info,
            _phantom: PhantomData,
        }
    }

    /// Reads bytes from RX FIFO until buffer is full, blocking if empty.
    ///
    /// Note: In blocking mode, this reads directly from hardware (the ring buffer is not used).
    ///
    /// # Errors
    ///
    /// Returns [`Error::Overrun`] if FIFO overrun error occurred during read.
    pub fn blocking_read(&mut self, buf: &mut [u8]) -> Result<(), Error> {
        for byte in buf.iter_mut() {
            let lsr = loop {
                let lsr = self.info.reg().data().lsr().read();
                if lsr.dr().bit() {
                    break lsr;
                }
            };

            if lsr.oe().bit() {
                return Err(Error::Overrun);
            }

            *byte = self.info.reg().data().rbr().read().bits();
        }

        Ok(())
    }
}

impl<'d> UartRx<'d, Blocking> {
    /// Create a new blocking buffered RX-only UART driver instance with given configuration.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidBaud`] if the supplied baud rate can not be represented by
    /// given clock source.
    pub fn new_blocking<T: Instance>(
        _peri: Peri<'d, T>,
        rx_buf: &'d mut [u8],
        config: Config,
    ) -> Result<Self, Error> {
        super::init(T::info().reg(), config)?;
        Ok(Self::new_inner::<T>(rx_buf))
    }
}

impl<'d> UartRx<'d, Async> {
    /// Create a new async buffered RX-only UART driver instance with given configuration.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidBaud`] if the supplied baud rate can not be represented by
    /// given clock source.
    pub fn new_async<T: Instance>(
        _peri: Peri<'d, T>,
        _irq: impl Binding<T::Interrupt, InterruptHandler<T>>,
        rx_buf: &'d mut [u8],
        config: Config,
    ) -> Result<Self, Error> {
        super::init(T::info().reg(), config)?;
        plic_enable_uart0();
        Ok(Self::new_inner::<T>(rx_buf))
    }

    /// Read bytes from the ring buffer.
    ///
    /// Returns the number of bytes read (at least 1, up to `buf.len()`).
    /// Waits asynchronously if no data is currently available.
    pub async fn read(&mut self, buf: &mut [u8]) -> usize {
        poll_fn(|cx| {
            self.info.rx_waker.register(cx.waker());
            let available = self.info.buffer.available();

            if available > 0 {
                let to_read = core::cmp::min(available, buf.len());
                critical_section::with(|_| {
                    // SAFETY: Only one reader exists (UartRx has &mut self).
                    // Ring buffer is initialized.
                    let mut reader = unsafe { self.info.buffer.reader() };
                    let mut copied = 0;
                    while copied < to_read {
                        copied += reader.pop(|data| {
                            let len = core::cmp::min(data.len(), to_read - copied);
                            buf[copied..copied + len].copy_from_slice(&data[..len]);
                            len
                        });
                    }
                });
                Poll::Ready(to_read)
            } else {
                Poll::Pending
            }
        })
        .await
    }
}

impl<'d, M: Mode> Drop for UartRx<'d, M> {
    fn drop(&mut self) {
        // SAFETY: No other reader/writer exists after drop.
        unsafe { self.info.buffer.deinit() }
        super::drop_rx_tx(self.info.reg(), &self.info.rx_tx_refcount);
    }
}

/// Buffered TX-only UART driver.
///
/// TX is not buffered — it uses the same interrupt-driven approach as the unbuffered driver.
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
        super::init(T::info().reg(), config)?;
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
        super::init(T::info().reg(), config)?;
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
        super::drop_rx_tx(self.info.reg(), &self.info.rx_tx_refcount);
    }
}

struct Info {
    reg: *const uart0::RegisterBlock,
    buffer: RingBuffer,
    rx_tx_refcount: AtomicU8,
    rx_waker: AtomicWaker,
    tx_waker: AtomicWaker,
}

// SAFETY: The register block pointer is to memory-mapped I/O valid for the program lifetime.
// AtomicU8, AtomicWaker, and RingBuffer are all Send+Sync.
unsafe impl Send for Info {}
unsafe impl Sync for Info {}

impl Info {
    #[inline(always)]
    fn reg(&self) -> &uart0::RegisterBlock {
        // SAFETY: The pointer is to a valid MMIO register block for the entire program lifetime.
        unsafe { &*self.reg }
    }
}

trait SealedInstance {
    fn info() -> &'static Info;
}

/// Buffered UART instance trait.
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
                    buffer: RingBuffer::new(),
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
        Ok(self.read(buf).await)
    }
}

impl embedded_io_async::Read for UartRx<'_, Async> {
    async fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        Ok(self.read(buf).await)
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
