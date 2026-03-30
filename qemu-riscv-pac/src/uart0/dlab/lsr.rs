#[doc = "Register `LSR` reader"]
pub type R = crate::R<LsrSpec>;
#[doc = "Field `DR` reader - Data ready"]
pub type DrR = crate::BitReader;
#[doc = "Field `OE` reader - Overrun error"]
pub type OeR = crate::BitReader;
#[doc = "Field `PE` reader - Parity error"]
pub type PeR = crate::BitReader;
#[doc = "Field `FE` reader - Framing error"]
pub type FeR = crate::BitReader;
#[doc = "Field `BI` reader - Break interrupt"]
pub type BiR = crate::BitReader;
#[doc = "Field `THRE` reader - Transmitter holding register empty"]
pub type ThreR = crate::BitReader;
#[doc = "Field `TEMT` reader - Transmitter empty"]
pub type TemtR = crate::BitReader;
#[doc = "Field `EIRF` reader - Error in receiver FIFO"]
pub type EirfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Data ready"]
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun error"]
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity error"]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Framing error"]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Break interrupt"]
    #[inline(always)]
    pub fn bi(&self) -> BiR {
        BiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmitter holding register empty"]
    #[inline(always)]
    pub fn thre(&self) -> ThreR {
        ThreR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter empty"]
    #[inline(always)]
    pub fn temt(&self) -> TemtR {
        TemtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error in receiver FIFO"]
    #[inline(always)]
    pub fn eirf(&self) -> EirfR {
        EirfR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Line Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LsrSpec;
impl crate::RegisterSpec for LsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lsr::R`](R) reader structure"]
impl crate::Readable for LsrSpec {}
#[doc = "`reset()` method sets LSR to value 0x60"]
impl crate::Resettable for LsrSpec {
    const RESET_VALUE: u8 = 0x60;
}
