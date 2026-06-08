//! GPIO driver.
//!
//! Drives the socket-backed bidirectional GPIO peripheral (`GPIO`) and implements the
//! [`embedded-hal`](embedded_hal) digital traits
//! ([`InputPin`](embedded_hal::digital::InputPin),
//! [`OutputPin`](embedded_hal::digital::OutputPin),
//! [`StatefulOutputPin`](embedded_hal::digital::StatefulOutputPin)) for blocking use and the
//! [`embedded-hal-async`](embedded_hal_async) [`Wait`](embedded_hal_async::digital::Wait) trait
//! for awaiting input edges/levels.
//!
//! Each of the 32 pins is an independent wire bridged to a peer: the guest drives an output
//! level through [`Output`]/[`Flex::set_level`] and observes the peer's level through
//! [`Input`]/[`Flex::is_high`]. The pins share a single PLIC interrupt; the HAL fans it out to
//! one waker per pin so any number of pins can be awaited concurrently.
//!
//! As all the pins are bidirectional and have no pull configuration, every method is infallible.

use crate::interrupt::typelevel::Binding;
use crate::pac::gpio;
use crate::{pac, peripherals, plic};
use core::convert::Infallible;
use core::future::poll_fn;
use core::marker::PhantomData;
use core::task::Poll;
use embassy_hal_internal::{Peri, PeripheralType};
use embassy_sync::waitqueue::AtomicWaker;
use embedded_hal::digital::{ErrorType, InputPin, OutputPin, StatefulOutputPin};
use embedded_hal_async::digital::Wait;

/// Number of GPIO pins exposed by the peripheral.
pub const PIN_COUNT: usize = 32;

// One waker per pin, woken by the shared interrupt handler.
static WAKERS: [AtomicWaker; PIN_COUNT] = [const { AtomicWaker::new() }; PIN_COUNT];

#[inline(always)]
fn reg() -> &'static gpio::RegisterBlock {
    // SAFETY: The pointer is to a valid MMIO register block for the entire program lifetime.
    unsafe { &*pac::Gpio::ptr() }
}

// Enable the shared GPIO interrupt source in the PLIC.
//
// Sets the GPIO priority to P1 and enables the source in PLIC context 0.
// Called (idempotently) when creating an async pin.
pub(crate) fn plic_enable_gpio() {
    use pac::interrupt::{ExternalInterrupt, Priority};

    // SAFETY: We have sole control of PLIC configuration for GPIO
    unsafe {
        plic()
            .priorities()
            .set_priority(ExternalInterrupt::GPIO, Priority::P1);
        plic().ctx0().enables().enable(ExternalInterrupt::GPIO);
    }
}

/// GPIO interrupt handler.
///
/// Bind the shared `GPIO` interrupt to this handler to use the async [`Wait`] API:
///
/// ```rust,ignore
/// bind_interrupts!(struct Irqs {
///     GPIO => gpio::InterruptHandler;
/// });
/// ```
pub struct InterruptHandler {
    _priv: (),
}

impl crate::interrupt::typelevel::Handler<crate::interrupt::typelevel::GPIO> for InterruptHandler {
    unsafe fn on_interrupt() {
        let reg = reg();
        let active = reg.irq_pend().read().bits() & reg.irq_en().read().bits();
        if active == 0 {
            return;
        }

        // Disable the enables for the firing pins. This de-asserts the shared (level-triggered)
        // line and stops it re-firing; the awaiting task observes the cleared enable bit,
        // services the event, and re-arms the next time it waits.
        reg.irq_en()
            .modify(|r, w| unsafe { w.bits(r.bits() & !active) });
        // Clear the edge-latched pending bits (write-1-to-clear).
        reg.irq_pend().write(|w| unsafe { w.bits(active) });

        let mut bits = active;
        while bits != 0 {
            let pin = bits.trailing_zeros() as usize;
            WAKERS[pin].wake();
            bits &= bits - 1;
        }
    }
}

/// Digital level of a pin.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum Level {
    /// Logic low (0).
    Low,
    /// Logic high (1).
    High,
}

impl From<bool> for Level {
    fn from(b: bool) -> Self {
        if b { Level::High } else { Level::Low }
    }
}

impl From<Level> for bool {
    fn from(l: Level) -> bool {
        matches!(l, Level::High)
    }
}

/// A flexible pin that can read its input level and drive its output level.
///
/// Because the underlying wire is bidirectional, a `Flex` pin is simultaneously an input and an
/// output: the `set_*` methods drive the level seen by the peer, while the `is_*` methods report
/// the level the peer is driving.
pub struct Flex<'d, M: Mode> {
    pin: u8,
    _phantom: PhantomData<&'d mut M>,
}

impl<'d> Flex<'d, Blocking> {
    /// Create a new blocking flexible pin.
    pub fn new_blocking(pin: Peri<'d, impl Pin>) -> Self {
        Self {
            pin: pin.pin_number(),
            _phantom: PhantomData,
        }
    }
}

impl<'d> Flex<'d, Async> {
    /// Create a new async flexible pin, enabling the shared GPIO interrupt.
    pub fn new_async<T: Pin>(
        pin: Peri<'d, T>,
        _irq: impl Binding<crate::interrupt::typelevel::GPIO, InterruptHandler>,
    ) -> Self {
        plic_enable_gpio();
        Self {
            pin: pin.pin_number(),
            _phantom: PhantomData,
        }
    }
}

impl<M: Mode> Flex<'_, M> {
    #[inline(always)]
    fn mask(&self) -> u32 {
        1u32 << self.pin
    }

    /// Returns `true` if the pin's input level is high.
    #[inline]
    pub fn is_high(&self) -> bool {
        reg().in_().read().bits() & self.mask() != 0
    }

    /// Returns `true` if the pin's input level is low.
    #[inline]
    pub fn is_low(&self) -> bool {
        !self.is_high()
    }

    /// Returns the pin's input level.
    #[inline]
    pub fn level(&self) -> Level {
        self.is_high().into()
    }

    /// Drive the pin's output high.
    #[inline]
    pub fn set_high(&mut self) {
        let mask = self.mask();
        critical_section::with(|_| {
            reg()
                .out()
                .modify(|r, w| unsafe { w.bits(r.bits() | mask) });
        });
    }

    /// Drive the pin's output low.
    #[inline]
    pub fn set_low(&mut self) {
        let mask = self.mask();
        critical_section::with(|_| {
            reg()
                .out()
                .modify(|r, w| unsafe { w.bits(r.bits() & !mask) });
        });
    }

    /// Drive the pin's output to `level`.
    #[inline]
    pub fn set_level(&mut self, level: Level) {
        match level {
            Level::High => self.set_high(),
            Level::Low => self.set_low(),
        }
    }

    /// Toggle the pin's output level.
    #[inline]
    pub fn toggle(&mut self) {
        let mask = self.mask();
        critical_section::with(|_| {
            reg()
                .out()
                .modify(|r, w| unsafe { w.bits(r.bits() ^ mask) });
        });
    }

    /// Returns `true` if the pin's output is set high.
    #[inline]
    pub fn is_set_high(&self) -> bool {
        reg().out().read().bits() & self.mask() != 0
    }

    /// Returns `true` if the pin's output is set low.
    #[inline]
    pub fn is_set_low(&self) -> bool {
        !self.is_set_high()
    }

    /// Returns the pin's currently driven output level.
    #[inline]
    pub fn output_level(&self) -> Level {
        self.is_set_high().into()
    }
}

impl Flex<'_, Async> {
    // Wait until the pin's input matches `level` using a level-triggered interrupt.
    async fn wait_for_level(&mut self, level: Level) {
        let mask = self.mask();
        let want_high: bool = level.into();

        poll_fn(|cx| {
            WAKERS[self.pin as usize].register(cx.waker());

            if (reg().in_().read().bits() & mask != 0) == want_high {
                // Already at the desired level; make sure our interrupt is disarmed.
                critical_section::with(|_| {
                    reg()
                        .irq_en()
                        .modify(|r, w| unsafe { w.bits(r.bits() & !mask) });
                });
                return Poll::Ready(());
            }

            // Arm: level trigger, requested polarity, enable.
            critical_section::with(|_| {
                reg()
                    .irq_trig()
                    .modify(|r, w| unsafe { w.bits(r.bits() & !mask) });
                reg().irq_pol().modify(|r, w| unsafe {
                    let v = r.bits();
                    w.bits(if want_high { v | mask } else { v & !mask })
                });
                reg()
                    .irq_en()
                    .modify(|r, w| unsafe { w.bits(r.bits() | mask) });
            });
            Poll::Pending
        })
        .await
    }

    // Wait for a single input edge using an edge-triggered interrupt. When both `rising` and
    // `falling` are requested (any-edge), the hardware single-polarity edge is armed for the
    // transition away from the pin's current level.
    async fn wait_for_edge(&mut self, rising: bool, falling: bool) {
        let mask = self.mask();
        let want_rising = if rising && falling {
            // Any edge: wait for the transition away from the current level.
            reg().in_().read().bits() & mask == 0
        } else {
            rising
        };

        // Arm: edge trigger, requested polarity, clear any stale latch, enable.
        critical_section::with(|_| {
            reg()
                .irq_trig()
                .modify(|r, w| unsafe { w.bits(r.bits() | mask) });
            reg().irq_pol().modify(|r, w| unsafe {
                let v = r.bits();
                w.bits(if want_rising { v | mask } else { v & !mask })
            });
            reg().irq_pend().write(|w| unsafe { w.bits(mask) });
            reg()
                .irq_en()
                .modify(|r, w| unsafe { w.bits(r.bits() | mask) });
        });

        poll_fn(|cx| {
            WAKERS[self.pin as usize].register(cx.waker());
            // The handler clears our enable bit once the edge fires.
            if reg().irq_en().read().bits() & mask == 0 {
                Poll::Ready(())
            } else {
                Poll::Pending
            }
        })
        .await
    }

    /// Wait until the pin's input level is high.
    pub async fn wait_for_high(&mut self) {
        self.wait_for_level(Level::High).await;
    }

    /// Wait until the pin's input level is low.
    pub async fn wait_for_low(&mut self) {
        self.wait_for_level(Level::Low).await;
    }

    /// Wait for a rising edge on the pin's input.
    pub async fn wait_for_rising_edge(&mut self) {
        self.wait_for_edge(true, false).await;
    }

    /// Wait for a falling edge on the pin's input.
    pub async fn wait_for_falling_edge(&mut self) {
        self.wait_for_edge(false, true).await;
    }

    /// Wait for any edge (rising or falling) on the pin's input.
    pub async fn wait_for_any_edge(&mut self) {
        self.wait_for_edge(true, true).await;
    }
}

impl<M: Mode> Drop for Flex<'_, M> {
    fn drop(&mut self) {
        // Disarm the pin's interrupt and clear any latched pending bit.
        let mask = self.mask();
        critical_section::with(|_| {
            reg()
                .irq_en()
                .modify(|r, w| unsafe { w.bits(r.bits() & !mask) });
            reg().irq_pend().write(|w| unsafe { w.bits(mask) });
        });
    }
}

/// An input-only pin.
pub struct Input<'d, M: Mode> {
    pin: Flex<'d, M>,
}

impl<'d> Input<'d, Blocking> {
    /// Create a new blocking input pin.
    pub fn new_blocking(pin: Peri<'d, impl Pin>) -> Self {
        Self {
            pin: Flex::new_blocking(pin),
        }
    }
}

impl<'d> Input<'d, Async> {
    /// Create a new async input pin, enabling the shared GPIO interrupt.
    pub fn new_async<T: Pin>(
        pin: Peri<'d, T>,
        irq: impl Binding<crate::interrupt::typelevel::GPIO, InterruptHandler>,
    ) -> Self {
        Self {
            pin: Flex::new_async(pin, irq),
        }
    }
}

impl<M: Mode> Input<'_, M> {
    /// Returns `true` if the pin's input level is high.
    #[inline]
    pub fn is_high(&self) -> bool {
        self.pin.is_high()
    }

    /// Returns `true` if the pin's input level is low.
    #[inline]
    pub fn is_low(&self) -> bool {
        self.pin.is_low()
    }

    /// Returns the pin's input level.
    #[inline]
    pub fn level(&self) -> Level {
        self.pin.level()
    }
}

impl Input<'_, Async> {
    /// Wait until the pin's input level is high.
    pub async fn wait_for_high(&mut self) {
        self.pin.wait_for_high().await;
    }

    /// Wait until the pin's input level is low.
    pub async fn wait_for_low(&mut self) {
        self.pin.wait_for_low().await;
    }

    /// Wait for a rising edge on the pin's input.
    pub async fn wait_for_rising_edge(&mut self) {
        self.pin.wait_for_rising_edge().await;
    }

    /// Wait for a falling edge on the pin's input.
    pub async fn wait_for_falling_edge(&mut self) {
        self.pin.wait_for_falling_edge().await;
    }

    /// Wait for any edge (rising or falling) on the pin's input.
    pub async fn wait_for_any_edge(&mut self) {
        self.pin.wait_for_any_edge().await;
    }
}

/// An output-only pin.
pub struct Output<'d> {
    pin: Flex<'d, Blocking>,
}

impl<'d> Output<'d> {
    /// Create a new output pin driving the given initial level.
    pub fn new(pin: Peri<'d, impl Pin>, initial_level: Level) -> Self {
        let mut pin = Flex::new_blocking(pin);
        pin.set_level(initial_level);
        Self { pin }
    }

    /// Drive the pin's output high.
    #[inline]
    pub fn set_high(&mut self) {
        self.pin.set_high();
    }

    /// Drive the pin's output low.
    #[inline]
    pub fn set_low(&mut self) {
        self.pin.set_low();
    }

    /// Drive the pin's output to `level`.
    #[inline]
    pub fn set_level(&mut self, level: Level) {
        self.pin.set_level(level);
    }

    /// Toggle the pin's output level.
    #[inline]
    pub fn toggle(&mut self) {
        self.pin.toggle();
    }

    /// Returns `true` if the pin's output is set high.
    #[inline]
    pub fn is_set_high(&self) -> bool {
        self.pin.is_set_high()
    }

    /// Returns `true` if the pin's output is set low.
    #[inline]
    pub fn is_set_low(&self) -> bool {
        self.pin.is_set_low()
    }

    /// Returns the pin's currently driven output level.
    #[inline]
    pub fn output_level(&self) -> Level {
        self.pin.output_level()
    }
}

// -------------------------------------------------------------------------------------------------
// embedded-hal trait implementations
// -------------------------------------------------------------------------------------------------

impl<M: Mode> ErrorType for Flex<'_, M> {
    type Error = Infallible;
}

impl<M: Mode> InputPin for Flex<'_, M> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Flex::is_high(self))
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(Flex::is_low(self))
    }
}

impl<M: Mode> OutputPin for Flex<'_, M> {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        Flex::set_high(self);
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        Flex::set_low(self);
        Ok(())
    }
}

impl<M: Mode> StatefulOutputPin for Flex<'_, M> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Flex::is_set_high(self))
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(Flex::is_set_low(self))
    }
}

impl Wait for Flex<'_, Async> {
    async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
        Flex::wait_for_high(self).await;
        Ok(())
    }

    async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
        Flex::wait_for_low(self).await;
        Ok(())
    }

    async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
        Flex::wait_for_rising_edge(self).await;
        Ok(())
    }

    async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
        Flex::wait_for_falling_edge(self).await;
        Ok(())
    }

    async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
        Flex::wait_for_any_edge(self).await;
        Ok(())
    }
}

impl<M: Mode> ErrorType for Input<'_, M> {
    type Error = Infallible;
}

impl<M: Mode> InputPin for Input<'_, M> {
    fn is_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Input::is_high(self))
    }

    fn is_low(&mut self) -> Result<bool, Self::Error> {
        Ok(Input::is_low(self))
    }
}

impl Wait for Input<'_, Async> {
    async fn wait_for_high(&mut self) -> Result<(), Self::Error> {
        Input::wait_for_high(self).await;
        Ok(())
    }

    async fn wait_for_low(&mut self) -> Result<(), Self::Error> {
        Input::wait_for_low(self).await;
        Ok(())
    }

    async fn wait_for_rising_edge(&mut self) -> Result<(), Self::Error> {
        Input::wait_for_rising_edge(self).await;
        Ok(())
    }

    async fn wait_for_falling_edge(&mut self) -> Result<(), Self::Error> {
        Input::wait_for_falling_edge(self).await;
        Ok(())
    }

    async fn wait_for_any_edge(&mut self) -> Result<(), Self::Error> {
        Input::wait_for_any_edge(self).await;
        Ok(())
    }
}

impl ErrorType for Output<'_> {
    type Error = Infallible;
}

impl OutputPin for Output<'_> {
    fn set_high(&mut self) -> Result<(), Self::Error> {
        Output::set_high(self);
        Ok(())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        Output::set_low(self);
        Ok(())
    }
}

impl StatefulOutputPin for Output<'_> {
    fn is_set_high(&mut self) -> Result<bool, Self::Error> {
        Ok(Output::is_set_high(self))
    }

    fn is_set_low(&mut self) -> Result<bool, Self::Error> {
        Ok(Output::is_set_low(self))
    }
}

// -------------------------------------------------------------------------------------------------
// Mode + Pin sealed traits
// -------------------------------------------------------------------------------------------------

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

trait SealedPin {
    fn pin_number(&self) -> u8;
}

/// GPIO pin instance trait.
#[allow(private_bounds)]
pub trait Pin: PeripheralType + SealedPin + Sized + 'static {}

macro_rules! impl_pin {
    ($peri:ident, $n:literal) => {
        impl SealedPin for peripherals::$peri {
            #[inline(always)]
            fn pin_number(&self) -> u8 {
                $n
            }
        }

        impl Pin for peripherals::$peri {}
    };
}

impl_pin!(GPIO0, 0);
impl_pin!(GPIO1, 1);
impl_pin!(GPIO2, 2);
impl_pin!(GPIO3, 3);
impl_pin!(GPIO4, 4);
impl_pin!(GPIO5, 5);
impl_pin!(GPIO6, 6);
impl_pin!(GPIO7, 7);
impl_pin!(GPIO8, 8);
impl_pin!(GPIO9, 9);
impl_pin!(GPIO10, 10);
impl_pin!(GPIO11, 11);
impl_pin!(GPIO12, 12);
impl_pin!(GPIO13, 13);
impl_pin!(GPIO14, 14);
impl_pin!(GPIO15, 15);
impl_pin!(GPIO16, 16);
impl_pin!(GPIO17, 17);
impl_pin!(GPIO18, 18);
impl_pin!(GPIO19, 19);
impl_pin!(GPIO20, 20);
impl_pin!(GPIO21, 21);
impl_pin!(GPIO22, 22);
impl_pin!(GPIO23, 23);
impl_pin!(GPIO24, 24);
impl_pin!(GPIO25, 25);
impl_pin!(GPIO26, 26);
impl_pin!(GPIO27, 27);
impl_pin!(GPIO28, 28);
impl_pin!(GPIO29, 29);
impl_pin!(GPIO30, 30);
impl_pin!(GPIO31, 31);
