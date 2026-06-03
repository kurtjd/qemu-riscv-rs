#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    int_status: IntStatus,
    int_enable: IntEnable,
    vwire_out: VwireOut,
    vwire_in: VwireIn,
    periph_addr: PeriphAddr,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt Status Register (write-1-to-clear)"]
    #[inline(always)]
    pub const fn int_status(&self) -> &IntStatus {
        &self.int_status
    }
    #[doc = "0x04 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn int_enable(&self) -> &IntEnable {
        &self.int_enable
    }
    #[doc = "0x08 - Virtual Wire Output Register"]
    #[inline(always)]
    pub const fn vwire_out(&self) -> &VwireOut {
        &self.vwire_out
    }
    #[doc = "0x0c - Virtual Wire Input Register (read-only)"]
    #[inline(always)]
    pub const fn vwire_in(&self) -> &VwireIn {
        &self.vwire_in
    }
    #[doc = "0x10 - Peripheral Address Register (offset of last peer shared memory write)"]
    #[inline(always)]
    pub const fn periph_addr(&self) -> &PeriphAddr {
        &self.periph_addr
    }
}
#[doc = "INT_STATUS (rw) register accessor: Interrupt Status Register (write-1-to-clear)\n\nYou can [`read`](crate::Reg::read) this register and get [`int_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_status`] module"]
#[doc(alias = "INT_STATUS")]
pub type IntStatus = crate::Reg<int_status::IntStatusSpec>;
#[doc = "Interrupt Status Register (write-1-to-clear)"]
pub mod int_status;
#[doc = "INT_ENABLE (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_enable`] module"]
#[doc(alias = "INT_ENABLE")]
pub type IntEnable = crate::Reg<int_enable::IntEnableSpec>;
#[doc = "Interrupt Enable Register"]
pub mod int_enable;
#[doc = "VWIRE_OUT (rw) register accessor: Virtual Wire Output Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vwire_out::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwire_out::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwire_out`] module"]
#[doc(alias = "VWIRE_OUT")]
pub type VwireOut = crate::Reg<vwire_out::VwireOutSpec>;
#[doc = "Virtual Wire Output Register"]
pub mod vwire_out;
#[doc = "VWIRE_IN (r) register accessor: Virtual Wire Input Register (read-only)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwire_in::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@vwire_in`] module"]
#[doc(alias = "VWIRE_IN")]
pub type VwireIn = crate::Reg<vwire_in::VwireInSpec>;
#[doc = "Virtual Wire Input Register (read-only)"]
pub mod vwire_in;
#[doc = "PERIPH_ADDR (r) register accessor: Peripheral Address Register (offset of last peer shared memory write)\n\nYou can [`read`](crate::Reg::read) this register and get [`periph_addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@periph_addr`] module"]
#[doc(alias = "PERIPH_ADDR")]
pub type PeriphAddr = crate::Reg<periph_addr::PeriphAddrSpec>;
#[doc = "Peripheral Address Register (offset of last peer shared memory write)"]
pub mod periph_addr;
