#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    data: [Data; 4096],
}
impl RegisterBlock {
    #[doc = "0x00..0x1000 - Shared mailbox byte; the host may update memory independently of the EC"]
    #[inline(always)]
    pub const fn data(&self, n: usize) -> &Data {
        &self.data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x1000 - Shared mailbox byte; the host may update memory independently of the EC"]
    #[inline(always)]
    pub fn data_iter(&self) -> impl Iterator<Item = &Data> {
        self.data.iter()
    }
}
#[doc = "DATA (rw) register accessor: Shared mailbox byte; the host may update memory independently of the EC\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`] module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Shared mailbox byte; the host may update memory independently of the EC"]
pub mod data;
