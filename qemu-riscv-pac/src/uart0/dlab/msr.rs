#[doc = "Register `MSR` reader"]
pub type R = crate::R<MsrSpec>;
#[doc = "Field `DCTS` reader - Delta clear to send"]
pub type DctsR = crate::BitReader;
#[doc = "Field `DDSR` reader - Delta data set ready"]
pub type DdsrR = crate::BitReader;
#[doc = "Field `TERI` reader - Trailing edge ring indicator"]
pub type TeriR = crate::BitReader;
#[doc = "Field `DDCD` reader - Delta data carrier detect"]
pub type DdcdR = crate::BitReader;
#[doc = "Field `CTS` reader - Clear to send"]
pub type CtsR = crate::BitReader;
#[doc = "Field `DSR` reader - Data set ready"]
pub type DsrR = crate::BitReader;
#[doc = "Field `RI` reader - Ring indicator"]
pub type RiR = crate::BitReader;
#[doc = "Field `DCD` reader - Data carrier detect"]
pub type DcdR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Delta clear to send"]
    #[inline(always)]
    pub fn dcts(&self) -> DctsR {
        DctsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Delta data set ready"]
    #[inline(always)]
    pub fn ddsr(&self) -> DdsrR {
        DdsrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Trailing edge ring indicator"]
    #[inline(always)]
    pub fn teri(&self) -> TeriR {
        TeriR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Delta data carrier detect"]
    #[inline(always)]
    pub fn ddcd(&self) -> DdcdR {
        DdcdR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear to send"]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Data set ready"]
    #[inline(always)]
    pub fn dsr(&self) -> DsrR {
        DsrR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Ring indicator"]
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Data carrier detect"]
    #[inline(always)]
    pub fn dcd(&self) -> DcdR {
        DcdR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Modem Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsrSpec;
impl crate::RegisterSpec for MsrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`msr::R`](R) reader structure"]
impl crate::Readable for MsrSpec {}
#[doc = "`reset()` method sets MSR to value 0"]
impl crate::Resettable for MsrSpec {}
