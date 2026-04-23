#![no_std]
mod interrupt_macros;
#[cfg(feature = "time-driver")]
mod time_driver;
pub mod uart;

// Generate the typelevel interrupt module for all PLIC external interrupts.
interrupt_mod!(UART0);

mod chip {
    #[rustfmt::skip]
    embassy_hal_internal::peripherals!(
        UART0,
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
