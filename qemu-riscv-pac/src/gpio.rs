#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    in_: In,
    out: Out,
    irq_trig: IrqTrig,
    irq_pol: IrqPol,
    irq_en: IrqEn,
    irq_pend: IrqPend,
}
impl RegisterBlock {
    #[doc = "0x00 - Input level (one bit per pin, driven by each pin's peer)"]
    #[inline(always)]
    pub const fn in_(&self) -> &In {
        &self.in_
    }
    #[doc = "0x04 - Output level (one bit per pin, driven to each pin's peer)"]
    #[inline(always)]
    pub const fn out(&self) -> &Out {
        &self.out
    }
    #[doc = "0x08 - Interrupt trigger type (0 = level, 1 = edge)"]
    #[inline(always)]
    pub const fn irq_trig(&self) -> &IrqTrig {
        &self.irq_trig
    }
    #[doc = "0x0c - Interrupt polarity (0 = low/falling, 1 = high/rising)"]
    #[inline(always)]
    pub const fn irq_pol(&self) -> &IrqPol {
        &self.irq_pol
    }
    #[doc = "0x10 - Per-pin interrupt enable"]
    #[inline(always)]
    pub const fn irq_en(&self) -> &IrqEn {
        &self.irq_en
    }
    #[doc = "0x14 - Per-pin interrupt pending (write-1-to-clear)"]
    #[inline(always)]
    pub const fn irq_pend(&self) -> &IrqPend {
        &self.irq_pend
    }
}
#[doc = "IN (r) register accessor: Input level (one bit per pin, driven by each pin's peer)\n\nYou can [`read`](crate::Reg::read) this register and get [`in_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@in_`] module"]
#[doc(alias = "IN")]
pub type In = crate::Reg<in_::InSpec>;
#[doc = "Input level (one bit per pin, driven by each pin's peer)"]
pub mod in_;
#[doc = "OUT (rw) register accessor: Output level (one bit per pin, driven to each pin's peer)\n\nYou can [`read`](crate::Reg::read) this register and get [`out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@out`] module"]
#[doc(alias = "OUT")]
pub type Out = crate::Reg<out::OutSpec>;
#[doc = "Output level (one bit per pin, driven to each pin's peer)"]
pub mod out;
#[doc = "IRQ_TRIG (rw) register accessor: Interrupt trigger type (0 = level, 1 = edge)\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_trig::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_trig::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_trig`] module"]
#[doc(alias = "IRQ_TRIG")]
pub type IrqTrig = crate::Reg<irq_trig::IrqTrigSpec>;
#[doc = "Interrupt trigger type (0 = level, 1 = edge)"]
pub mod irq_trig;
#[doc = "IRQ_POL (rw) register accessor: Interrupt polarity (0 = low/falling, 1 = high/rising)\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_pol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_pol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_pol`] module"]
#[doc(alias = "IRQ_POL")]
pub type IrqPol = crate::Reg<irq_pol::IrqPolSpec>;
#[doc = "Interrupt polarity (0 = low/falling, 1 = high/rising)"]
pub mod irq_pol;
#[doc = "IRQ_EN (rw) register accessor: Per-pin interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_en::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_en::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_en`] module"]
#[doc(alias = "IRQ_EN")]
pub type IrqEn = crate::Reg<irq_en::IrqEnSpec>;
#[doc = "Per-pin interrupt enable"]
pub mod irq_en;
#[doc = "IRQ_PEND (rw) register accessor: Per-pin interrupt pending (write-1-to-clear)\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_pend::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_pend::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_pend`] module"]
#[doc(alias = "IRQ_PEND")]
pub type IrqPend = crate::Reg<irq_pend::IrqPendSpec>;
#[doc = "Per-pin interrupt pending (write-1-to-clear)"]
pub mod irq_pend;
