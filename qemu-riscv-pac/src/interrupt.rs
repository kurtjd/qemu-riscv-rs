pub use riscv::interrupt::Exception;
pub use riscv::interrupt::Interrupt as CoreInterrupt;
#[doc = r" Priority levels in the device"]
# [riscv :: pac_enum (unsafe PriorityNumber)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Priority {
    #[doc = "0 - Priority 0 (disabled)"]
    P0 = 0,
    #[doc = "1 - Priority 1 (lowest)"]
    P1 = 1,
    #[doc = "2 - Priority 2"]
    P2 = 2,
    #[doc = "3 - Priority 3"]
    P3 = 3,
    #[doc = "4 - Priority 4"]
    P4 = 4,
    #[doc = "5 - Priority 5"]
    P5 = 5,
    #[doc = "6 - Priority 6"]
    P6 = 6,
    #[doc = "7 - Priority 7 (highest)"]
    P7 = 7,
}
#[doc = r" HARTs in the device"]
# [riscv :: pac_enum (unsafe HartIdNumber)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hart {
    #[doc = "0 - Hart 0"]
    H0 = 0,
}
pub use riscv::{
    ExceptionNumber, HartIdNumber, InterruptNumber, PriorityNumber,
    interrupt::{disable, enable, free, nested},
};
pub type Trap = riscv::interrupt::Trap<CoreInterrupt, Exception>;
#[doc = r" Retrieves the cause of a trap in the current hart."]
#[doc = r""]
#[doc = r" If the raw cause is not a valid interrupt or exception for the target, it returns an error."]
#[inline]
pub fn try_cause() -> riscv::result::Result<Trap> {
    riscv::interrupt::try_cause()
}
#[doc = r" Retrieves the cause of a trap in the current hart (machine mode)."]
#[doc = r""]
#[doc = r" If the raw cause is not a valid interrupt or exception for the target, it panics."]
#[inline]
pub fn cause() -> Trap {
    try_cause().unwrap()
}
#[doc = r" External interrupts. These interrupts are handled by the external peripherals."]
# [riscv :: pac_enum (unsafe ExternalInterruptNumber)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalInterrupt {
    #[doc = "1 - UART0 interrupt (PLIC source 1)"]
    UART0 = 1,
    #[doc = "2 - I2C0 controller interrupt (PLIC source 2)"]
    I2C0 = 2,
    #[doc = "3 - I2C target interrupt (PLIC source 3)"]
    I2C_TARGET = 3,
}
#[cfg(feature = "rt")]
#[riscv_rt::core_interrupt(CoreInterrupt::MachineExternal)]
unsafe fn plic_handler() {
    let plic = unsafe { crate::Plic::steal() };
    let claim = plic.ctx(Hart::H0).claim();
    if let Some(s) = claim.claim::<ExternalInterrupt>() {
        unsafe { _dispatch_external_interrupt(s.number()) }
        claim.complete(s);
    }
}
