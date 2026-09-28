#[repr(C)]
#[doc = "EC mailbox-control registers with 0x0c stride. Host control groups use 0x20 stride and expose IRQ acknowledgement instead of IRQ set."]
#[doc(alias = "MAILBOX_CONTROL")]
pub struct MailboxControl {
    shared_status: SharedStatus,
    doorbell: Doorbell,
    irq_set: IrqSet,
}
impl MailboxControl {
    #[doc = "0x00 - Application-defined shared status, writable by EC and host"]
    #[inline(always)]
    pub const fn shared_status(&self) -> &SharedStatus {
        &self.shared_status
    }
    #[doc = "0x04 - Host-to-EC doorbell pending. EC writes bit zero as one to clear; pending doorbells assert PLIC source 5."]
    #[inline(always)]
    pub const fn doorbell(&self) -> &Doorbell {
        &self.doorbell
    }
    #[doc = "0x08 - Read host IRQ pending; write one to assert host IRQ"]
    #[inline(always)]
    pub const fn irq_set(&self) -> &IrqSet {
        &self.irq_set
    }
}
#[doc = "SHARED_STATUS (rw) register accessor: Application-defined shared status, writable by EC and host\n\nYou can [`read`](crate::Reg::read) this register and get [`shared_status::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shared_status::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shared_status`] module"]
#[doc(alias = "SHARED_STATUS")]
pub type SharedStatus = crate::Reg<shared_status::SharedStatusSpec>;
#[doc = "Application-defined shared status, writable by EC and host"]
pub mod shared_status;
#[doc = "DOORBELL (rw) register accessor: Host-to-EC doorbell pending. EC writes bit zero as one to clear; pending doorbells assert PLIC source 5.\n\nYou can [`read`](crate::Reg::read) this register and get [`doorbell::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doorbell::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@doorbell`] module"]
#[doc(alias = "DOORBELL")]
pub type Doorbell = crate::Reg<doorbell::DoorbellSpec>;
#[doc = "Host-to-EC doorbell pending. EC writes bit zero as one to clear; pending doorbells assert PLIC source 5."]
pub mod doorbell;
#[doc = "IRQ_SET (rw) register accessor: Read host IRQ pending; write one to assert host IRQ\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_set::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_set::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq_set`] module"]
#[doc(alias = "IRQ_SET")]
pub type IrqSet = crate::Reg<irq_set::IrqSetSpec>;
#[doc = "Read host IRQ pending; write one to assert host IRQ"]
pub mod irq_set;
