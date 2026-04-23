//! Peripheral Interrupts
//!
//! A lot of this is taken from `embassy-hal-internal` but tweaked for RISC-V.
//! Ideally most of this could be merged back into `embassy-hal-internal`, but RISC-V interrupt
//! handling can be a bit platform-dependent so will need to consider how to make it more
//! flexible for all RISC-V platforms.
//!
//! On this platform, peripheral interrupts are external interrupts managed by a PLIC,
//! as opposed to platforms like the NEORV32 where they are custom core interrupts.

/// Macro to bind external (PLIC) interrupts to handlers.
///
/// This defines the right interrupt handlers, and creates a unit struct (like `struct Irqs;`)
/// and implements the right binding for it. You can pass this struct to drivers to
/// prove at compile-time that the right interrupts have been bound.
///
/// Example of how to bind one interrupt:
///
/// ```rust,ignore
/// use embassy_qemu_riscv::{bind_interrupts, uart, peripherals};
///
/// bind_interrupts!(struct Irqs {
///     UART0 => uart::InterruptHandler<peripherals::UART0>;
/// });
/// ```
#[macro_export]
macro_rules! bind_interrupts {
    ($vis:vis struct $name:ident { $($irq:ident => $($handler:ty),*;)* }) => {
            #[derive(Copy, Clone)]
            $vis struct $name;

        $(
            #[allow(non_snake_case)]
            #[riscv_rt::external_interrupt($crate::pac::interrupt::ExternalInterrupt::$irq)]
            fn $irq() {
                $(
                    // SAFETY: This macro ensures the given handler is being called from the correct IRQ
                    unsafe { <$handler as $crate::interrupt::typelevel::Handler<$crate::interrupt::typelevel::$irq>>::on_interrupt(); }
                )*
            }

            $(
                // SAFETY: This macro ensures the given IRQ is bounded to given handler
                unsafe impl $crate::interrupt::typelevel::Binding<$crate::interrupt::typelevel::$irq, $handler> for $name {}
            )*
        )*
    };
}

/// Generate a standard `mod interrupt` for a RISC-V HAL using PLIC external interrupts.
#[macro_export]
macro_rules! interrupt_mod {
    ($($irqs:ident),* $(,)?) => {
        /// Interrupt definitions.
        pub mod interrupt {
            pub use $crate::pac::interrupt::ExternalInterrupt;

            /// Type-level interrupt infrastructure.
            ///
            /// This module contains one *type* per interrupt. This is used for checking at compile time that
            /// the interrupts are correctly bound to HAL drivers.
            ///
            /// As an end user, you shouldn't need to use this module directly.
            /// Use the [`crate::bind_interrupts!`] macro to bind interrupts.
            pub mod typelevel {
                trait SealedInterrupt {}

                /// Type-level interrupt.
                ///
                /// This trait is implemented for all typelevel interrupt types in this module.
                #[allow(private_bounds)]
                pub trait Interrupt: SealedInterrupt {
                    /// External interrupt enum variant.
                    ///
                    /// This allows going from typelevel interrupts (one type per interrupt) to
                    /// non-typelevel interrupts (a single `ExternalInterrupt` enum type, with one variant per interrupt).
                    const IRQ: super::ExternalInterrupt;
                }

                $(
                    #[allow(non_camel_case_types)]
                    #[doc=stringify!($irqs)]
                    #[doc=" typelevel interrupt."]
                    pub enum $irqs {}

                    impl SealedInterrupt for $irqs {}
                    impl Interrupt for $irqs {
                        const IRQ: super::ExternalInterrupt = super::ExternalInterrupt::$irqs;
                    }
                )*

                /// Interrupt handler trait.
                ///
                /// Drivers that need to handle interrupts implement this trait.
                /// The user must ensure `on_interrupt()` is called every time the interrupt fires.
                /// Drivers must use use [`Binding`] to assert at compile time that the user has done so.
                pub trait Handler<I: Interrupt> {
                    /// Interrupt handler function.
                    ///
                    /// Must be called every time the `I` interrupt fires, synchronously from
                    /// the interrupt handler context.
                    ///
                    /// # Safety
                    ///
                    /// This function must ONLY be called from the interrupt handler for `I`.
                    unsafe fn on_interrupt();
                }

                /// Compile-time assertion that an interrupt has been bound to a handler.
                ///
                /// For the vast majority of cases, you should use the `bind_interrupts!`
                /// macro instead of writing `unsafe impl`s of this trait.
                ///
                /// # Safety
                ///
                /// By implementing this trait, you are asserting that you have arranged for `H::on_interrupt()`
                /// to be called every time the `I` interrupt fires.
                ///
                /// This allows drivers to check bindings at compile-time.
                pub unsafe trait Binding<I: Interrupt, H: Handler<I>> {}
            }
        }
    };
}
