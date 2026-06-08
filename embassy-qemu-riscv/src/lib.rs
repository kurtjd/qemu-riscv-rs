#![no_std]
pub mod gpio;
pub mod i2c;
mod interrupt_macros;
#[cfg(feature = "time-driver")]
mod time_driver;
pub mod uart;

// Generate the typelevel interrupt module for all PLIC external interrupts.
interrupt_mod!(UART0, I2C0, I2C_TARGET, GPIO);

mod chip {
    #[rustfmt::skip]
    embassy_hal_internal::peripherals!(
        UART0,
        I2C0,
        I2C_TARGET,
        GPIO0,
        GPIO1,
        GPIO2,
        GPIO3,
        GPIO4,
        GPIO5,
        GPIO6,
        GPIO7,
        GPIO8,
        GPIO9,
        GPIO10,
        GPIO11,
        GPIO12,
        GPIO13,
        GPIO14,
        GPIO15,
        GPIO16,
        GPIO17,
        GPIO18,
        GPIO19,
        GPIO20,
        GPIO21,
        GPIO22,
        GPIO23,
        GPIO24,
        GPIO25,
        GPIO26,
        GPIO27,
        GPIO28,
        GPIO29,
        GPIO30,
        GPIO31,
    );
}

pub use chip::{Peripherals, peripherals};
pub use qemu_riscv_pac as pac;

pub(crate) fn plic() -> pac::Plic {
    // SAFETY: We are the sole users of the PLIC and manage it safely
    unsafe { pac::Plic::steal() }
}

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

    // Enable PLIC (sets MachineExternal bit in mie CSR)
    // and reset threshold to 0 so all priority levels can trigger
    // SAFETY: We have sole control of the PLIC from this point
    unsafe { plic().enable() };
    plic().ctx0().threshold().reset();

    #[cfg(feature = "time-driver")]
    time_driver::init();

    p
}
