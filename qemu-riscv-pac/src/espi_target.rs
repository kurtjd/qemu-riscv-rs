#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    control: Control,
    mailbox0_addr: Mailbox0Addr,
    mailbox1_addr: Mailbox1Addr,
    control_addr: ControlAddr,
    mailbox_control: [MailboxControl; 2],
}
impl RegisterBlock {
    #[doc = "0x00 - Enable peripheral-channel mailbox access. Program all three decode bases while disabled; enabling requires distinct, 4-KiB-aligned bases."]
    #[inline(always)]
    pub const fn control(&self) -> &Control {
        &self.control
    }
    #[doc = "0x04 - Mailbox zero host decode base, not an EC pointer. Must be 4-KiB aligned and distinct from the other bases. Change only while CONTROL.ENABLE is zero."]
    #[inline(always)]
    pub const fn mailbox0_addr(&self) -> &Mailbox0Addr {
        &self.mailbox0_addr
    }
    #[doc = "0x08 - Mailbox one host decode base, not an EC pointer. Must be 4-KiB aligned and distinct from the other bases. Change only while CONTROL.ENABLE is zero."]
    #[inline(always)]
    pub const fn mailbox1_addr(&self) -> &Mailbox1Addr {
        &self.mailbox1_addr
    }
    #[doc = "0x0c - Host control-window decode base, not an EC pointer. Must be 4-KiB aligned and distinct from the mailbox bases. Change only while CONTROL.ENABLE is zero."]
    #[inline(always)]
    pub const fn control_addr(&self) -> &ControlAddr {
        &self.control_addr
    }
    #[doc = "0x10..0x28 - EC mailbox-control registers with 0x0c stride. Host control groups use 0x20 stride and expose IRQ acknowledgement instead of IRQ set."]
    #[inline(always)]
    pub const fn mailbox_control(&self, n: usize) -> &MailboxControl {
        &self.mailbox_control[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10..0x28 - EC mailbox-control registers with 0x0c stride. Host control groups use 0x20 stride and expose IRQ acknowledgement instead of IRQ set."]
    #[inline(always)]
    pub fn mailbox_control_iter(&self) -> impl Iterator<Item = &MailboxControl> {
        self.mailbox_control.iter()
    }
}
#[doc = "CONTROL (rw) register accessor: Enable peripheral-channel mailbox access. Program all three decode bases while disabled; enabling requires distinct, 4-KiB-aligned bases.\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`] module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "Enable peripheral-channel mailbox access. Program all three decode bases while disabled; enabling requires distinct, 4-KiB-aligned bases."]
pub mod control;
#[doc = "MAILBOX0_ADDR (rw) register accessor: Mailbox zero host decode base, not an EC pointer. Must be 4-KiB aligned and distinct from the other bases. Change only while CONTROL.ENABLE is zero.\n\nYou can [`read`](crate::Reg::read) this register and get [`mailbox0_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mailbox0_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox0_addr`] module"]
#[doc(alias = "MAILBOX0_ADDR")]
pub type Mailbox0Addr = crate::Reg<mailbox0_addr::Mailbox0AddrSpec>;
#[doc = "Mailbox zero host decode base, not an EC pointer. Must be 4-KiB aligned and distinct from the other bases. Change only while CONTROL.ENABLE is zero."]
pub mod mailbox0_addr;
#[doc = "MAILBOX1_ADDR (rw) register accessor: Mailbox one host decode base, not an EC pointer. Must be 4-KiB aligned and distinct from the other bases. Change only while CONTROL.ENABLE is zero.\n\nYou can [`read`](crate::Reg::read) this register and get [`mailbox1_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mailbox1_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mailbox1_addr`] module"]
#[doc(alias = "MAILBOX1_ADDR")]
pub type Mailbox1Addr = crate::Reg<mailbox1_addr::Mailbox1AddrSpec>;
#[doc = "Mailbox one host decode base, not an EC pointer. Must be 4-KiB aligned and distinct from the other bases. Change only while CONTROL.ENABLE is zero."]
pub mod mailbox1_addr;
#[doc = "CONTROL_ADDR (rw) register accessor: Host control-window decode base, not an EC pointer. Must be 4-KiB aligned and distinct from the mailbox bases. Change only while CONTROL.ENABLE is zero.\n\nYou can [`read`](crate::Reg::read) this register and get [`control_addr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control_addr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control_addr`] module"]
#[doc(alias = "CONTROL_ADDR")]
pub type ControlAddr = crate::Reg<control_addr::ControlAddrSpec>;
#[doc = "Host control-window decode base, not an EC pointer. Must be 4-KiB aligned and distinct from the mailbox bases. Change only while CONTROL.ENABLE is zero."]
pub mod control_addr;
#[doc = "EC mailbox-control registers with 0x0c stride. Host control groups use 0x20 stride and expose IRQ acknowledgement instead of IRQ set."]
pub use self::mailbox_control::MailboxControl;
#[doc = r"Cluster"]
#[doc = "EC mailbox-control registers with 0x0c stride. Host control groups use 0x20 stride and expose IRQ acknowledgement instead of IRQ set."]
pub mod mailbox_control;
