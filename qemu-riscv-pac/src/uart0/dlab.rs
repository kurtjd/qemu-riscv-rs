#[repr(C)]
#[doc = "UART registers when DLAB=1"]
#[doc(alias = "DLAB")]
pub struct Dlab {
    dll: Dll,
    dlm: Dlm,
    _reserved_2_fcr: [u8; 0x01],
    lcr: Lcr,
    mcr: Mcr,
    lsr: Lsr,
    msr: Msr,
    scr: Scr,
}
impl Dlab {
    #[doc = "0x00 - Divisor Latch LSB (DLAB=1)"]
    #[inline(always)]
    pub const fn dll(&self) -> &Dll {
        &self.dll
    }
    #[doc = "0x01 - Divisor Latch MSB (DLAB=1)"]
    #[inline(always)]
    pub const fn dlm(&self) -> &Dlm {
        &self.dlm
    }
    #[doc = "0x02 - FIFO Control Register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &Fcr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2).cast() }
    }
    #[doc = "0x02 - Interrupt Identification Register"]
    #[inline(always)]
    pub const fn iir(&self) -> &Iir {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2).cast() }
    }
    #[doc = "0x03 - Line Control Register"]
    #[inline(always)]
    pub const fn lcr(&self) -> &Lcr {
        &self.lcr
    }
    #[doc = "0x04 - Modem Control Register"]
    #[inline(always)]
    pub const fn mcr(&self) -> &Mcr {
        &self.mcr
    }
    #[doc = "0x05 - Line Status Register"]
    #[inline(always)]
    pub const fn lsr(&self) -> &Lsr {
        &self.lsr
    }
    #[doc = "0x06 - Modem Status Register"]
    #[inline(always)]
    pub const fn msr(&self) -> &Msr {
        &self.msr
    }
    #[doc = "0x07 - Scratch Register"]
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        &self.scr
    }
}
#[doc = "DLL (rw) register accessor: Divisor Latch LSB (DLAB=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`dll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll`] module"]
#[doc(alias = "DLL")]
pub type Dll = crate::Reg<dll::DllSpec>;
#[doc = "Divisor Latch LSB (DLAB=1)"]
pub mod dll;
#[doc = "DLM (rw) register accessor: Divisor Latch MSB (DLAB=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`dlm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlm`] module"]
#[doc(alias = "DLM")]
pub type Dlm = crate::Reg<dlm::DlmSpec>;
#[doc = "Divisor Latch MSB (DLAB=1)"]
pub mod dlm;
#[doc = "IIR (r) register accessor: Interrupt Identification Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iir`] module"]
#[doc(alias = "IIR")]
pub type Iir = crate::Reg<iir::IirSpec>;
#[doc = "Interrupt Identification Register"]
pub mod iir;
#[doc = "FCR (w) register accessor: FIFO Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`] module"]
#[doc(alias = "FCR")]
pub type Fcr = crate::Reg<fcr::FcrSpec>;
#[doc = "FIFO Control Register"]
pub mod fcr;
#[doc = "LCR (rw) register accessor: Line Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcr`] module"]
#[doc(alias = "LCR")]
pub type Lcr = crate::Reg<lcr::LcrSpec>;
#[doc = "Line Control Register"]
pub mod lcr;
#[doc = "MCR (rw) register accessor: Modem Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`] module"]
#[doc(alias = "MCR")]
pub type Mcr = crate::Reg<mcr::McrSpec>;
#[doc = "Modem Control Register"]
pub mod mcr;
#[doc = "LSR (r) register accessor: Line Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsr`] module"]
#[doc(alias = "LSR")]
pub type Lsr = crate::Reg<lsr::LsrSpec>;
#[doc = "Line Status Register"]
pub mod lsr;
#[doc = "MSR (r) register accessor: Modem Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr`] module"]
#[doc(alias = "MSR")]
pub type Msr = crate::Reg<msr::MsrSpec>;
#[doc = "Modem Status Register"]
pub mod msr;
#[doc = "SCR (rw) register accessor: Scratch Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`] module"]
#[doc(alias = "SCR")]
pub type Scr = crate::Reg<scr::ScrSpec>;
#[doc = "Scratch Register"]
pub mod scr;
