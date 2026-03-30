#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_data: [u8; 0x08],
}
impl RegisterBlock {
    #[doc = "0x00..0x08 - UART registers when DLAB=1"]
    #[inline(always)]
    pub const fn dlab(&self) -> &Dlab {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00..0x08 - UART registers when DLAB=0"]
    #[inline(always)]
    pub const fn data(&self) -> &Data {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
}
#[doc = "UART registers when DLAB=0"]
pub use self::data::Data;
#[doc = r"Cluster"]
#[doc = "UART registers when DLAB=0"]
pub mod data;
#[doc = "UART registers when DLAB=1"]
pub use self::dlab::Dlab;
#[doc = r"Cluster"]
#[doc = "UART registers when DLAB=1"]
pub mod dlab;
