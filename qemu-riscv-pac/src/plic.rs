#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    priority: [Priority; 96],
    _reserved1: [u8; 0x0e80],
    pending: [Pending; 3],
    _reserved2: [u8; 0x0ff4],
    m_enable: [MEnable; 3],
    _reserved3: [u8; 0x74],
    s_enable: [SEnable; 3],
    _reserved4: [u8; 0x001f_df74],
    m_threshold: MThreshold,
    m_claim: MClaim,
    _reserved6: [u8; 0x0ff8],
    s_threshold: SThreshold,
    s_claim: SClaim,
}
impl RegisterBlock {
    #[doc = "0x00..0x180 - Priority for interrupt source %s (source 0 is reserved)"]
    #[inline(always)]
    pub const fn priority(&self, n: usize) -> &Priority {
        &self.priority[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x180 - Priority for interrupt source %s (source 0 is reserved)"]
    #[inline(always)]
    pub fn priority_iter(&self) -> impl Iterator<Item = &Priority> {
        self.priority.iter()
    }
    #[doc = "0x1000..0x100c - Interrupt pending bits"]
    #[inline(always)]
    pub const fn pending(&self, n: usize) -> &Pending {
        &self.pending[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x1000..0x100c - Interrupt pending bits"]
    #[inline(always)]
    pub fn pending_iter(&self) -> impl Iterator<Item = &Pending> {
        self.pending.iter()
    }
    #[doc = "0x2000..0x200c - M-mode interrupt enable bits (context 0)"]
    #[inline(always)]
    pub const fn m_enable(&self, n: usize) -> &MEnable {
        &self.m_enable[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2000..0x200c - M-mode interrupt enable bits (context 0)"]
    #[inline(always)]
    pub fn m_enable_iter(&self) -> impl Iterator<Item = &MEnable> {
        self.m_enable.iter()
    }
    #[doc = "0x2080..0x208c - S-mode interrupt enable bits (context 1)"]
    #[inline(always)]
    pub const fn s_enable(&self, n: usize) -> &SEnable {
        &self.s_enable[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2080..0x208c - S-mode interrupt enable bits (context 1)"]
    #[inline(always)]
    pub fn s_enable_iter(&self) -> impl Iterator<Item = &SEnable> {
        self.s_enable.iter()
    }
    #[doc = "0x200000 - M-mode priority threshold (context 0)"]
    #[inline(always)]
    pub const fn m_threshold(&self) -> &MThreshold {
        &self.m_threshold
    }
    #[doc = "0x200004 - M-mode interrupt claim/complete (context 0). Read to claim, write to complete."]
    #[inline(always)]
    pub const fn m_claim(&self) -> &MClaim {
        &self.m_claim
    }
    #[doc = "0x201000 - S-mode priority threshold (context 1)"]
    #[inline(always)]
    pub const fn s_threshold(&self) -> &SThreshold {
        &self.s_threshold
    }
    #[doc = "0x201004 - S-mode interrupt claim/complete (context 1). Read to claim, write to complete."]
    #[inline(always)]
    pub const fn s_claim(&self) -> &SClaim {
        &self.s_claim
    }
}
#[doc = "PRIORITY (rw) register accessor: Priority for interrupt source %s (source 0 is reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`priority::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@priority`] module"]
#[doc(alias = "PRIORITY")]
pub type Priority = crate::Reg<priority::PrioritySpec>;
#[doc = "Priority for interrupt source %s (source 0 is reserved)"]
pub mod priority;
#[doc = "PENDING (r) register accessor: Interrupt pending bits\n\nYou can [`read`](crate::Reg::read) this register and get [`pending::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pending`] module"]
#[doc(alias = "PENDING")]
pub type Pending = crate::Reg<pending::PendingSpec>;
#[doc = "Interrupt pending bits"]
pub mod pending;
#[doc = "M_ENABLE (rw) register accessor: M-mode interrupt enable bits (context 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`m_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m_enable`] module"]
#[doc(alias = "M_ENABLE")]
pub type MEnable = crate::Reg<m_enable::MEnableSpec>;
#[doc = "M-mode interrupt enable bits (context 0)"]
pub mod m_enable;
#[doc = "S_ENABLE (rw) register accessor: S-mode interrupt enable bits (context 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`s_enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s_enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s_enable`] module"]
#[doc(alias = "S_ENABLE")]
pub type SEnable = crate::Reg<s_enable::SEnableSpec>;
#[doc = "S-mode interrupt enable bits (context 1)"]
pub mod s_enable;
#[doc = "M_THRESHOLD (rw) register accessor: M-mode priority threshold (context 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`m_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m_threshold`] module"]
#[doc(alias = "M_THRESHOLD")]
pub type MThreshold = crate::Reg<m_threshold::MThresholdSpec>;
#[doc = "M-mode priority threshold (context 0)"]
pub mod m_threshold;
#[doc = "M_CLAIM (rw) register accessor: M-mode interrupt claim/complete (context 0). Read to claim, write to complete.\n\nYou can [`read`](crate::Reg::read) this register and get [`m_claim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m_claim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m_claim`] module"]
#[doc(alias = "M_CLAIM")]
pub type MClaim = crate::Reg<m_claim::MClaimSpec>;
#[doc = "M-mode interrupt claim/complete (context 0). Read to claim, write to complete."]
pub mod m_claim;
#[doc = "S_THRESHOLD (rw) register accessor: S-mode priority threshold (context 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`s_threshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s_threshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s_threshold`] module"]
#[doc(alias = "S_THRESHOLD")]
pub type SThreshold = crate::Reg<s_threshold::SThresholdSpec>;
#[doc = "S-mode priority threshold (context 1)"]
pub mod s_threshold;
#[doc = "S_CLAIM (rw) register accessor: S-mode interrupt claim/complete (context 1). Read to claim, write to complete.\n\nYou can [`read`](crate::Reg::read) this register and get [`s_claim::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s_claim::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@s_claim`] module"]
#[doc(alias = "S_CLAIM")]
pub type SClaim = crate::Reg<s_claim::SClaimSpec>;
#[doc = "S-mode interrupt claim/complete (context 1). Read to claim, write to complete."]
pub mod s_claim;
