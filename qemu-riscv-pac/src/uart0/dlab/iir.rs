#[doc = "Register `IIR` reader"]
pub type R = crate::R<IirSpec>;
#[doc = "Field `IPEND` reader - Interrupt NOT pending (0 = interrupt pending)"]
pub type IpendR = crate::BitReader;
#[doc = "Field `IID` reader - Interrupt ID"]
pub type IidR = crate::FieldReader;
#[doc = "Field `FIFOEN` reader - FIFOs enabled"]
pub type FifoenR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Interrupt NOT pending (0 = interrupt pending)"]
    #[inline(always)]
    pub fn ipend(&self) -> IpendR {
        IpendR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Interrupt ID"]
    #[inline(always)]
    pub fn iid(&self) -> IidR {
        IidR::new((self.bits >> 1) & 7)
    }
    #[doc = "Bits 6:7 - FIFOs enabled"]
    #[inline(always)]
    pub fn fifoen(&self) -> FifoenR {
        FifoenR::new((self.bits >> 6) & 3)
    }
}
#[doc = "Interrupt Identification Register\n\nYou can [`read`](crate::Reg::read) this register and get [`iir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IirSpec;
impl crate::RegisterSpec for IirSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`iir::R`](R) reader structure"]
impl crate::Readable for IirSpec {}
#[doc = "`reset()` method sets IIR to value 0x01"]
impl crate::Resettable for IirSpec {
    const RESET_VALUE: u8 = 0x01;
}
