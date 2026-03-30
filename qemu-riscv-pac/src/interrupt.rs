pub use riscv::interrupt::Exception;
pub use riscv::interrupt::Interrupt as CoreInterrupt;
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
    #[doc = "10 - UART0 interrupt (PLIC source 10)"]
    UART0 = 10,
}
