#![no_std]
#[cfg(feature = "time-driver")]
mod time_driver;
pub mod uart;

mod chip {
    #[rustfmt::skip]
    embassy_hal_internal::peripherals!(
        UART0,
    );
}

pub use chip::{Peripherals, peripherals};
pub use qemu_riscv_pac as pac;

/// Initialize the HAL.
///
/// # Panics
///
/// Panics if this has already been called once before.
pub fn init() -> Peripherals {
    // Attempt to take first so we panic before doing anything else
    let p = Peripherals::take();

    // SAFETY: We're not worried about breaking any critical sections here
    unsafe { riscv::interrupt::enable() }

    #[cfg(feature = "time-driver")]
    time_driver::init();

    p
}
