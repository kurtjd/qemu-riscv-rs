#[doc = "Register `VWIRE_IN` reader"]
pub type R = crate::R<VwireInSpec>;
#[doc = "Field `DATA` reader - Virtual wire input data received from peer"]
pub type DataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Virtual wire input data received from peer"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
#[doc = "Virtual Wire Input Register (read-only)\n\nYou can [`read`](crate::Reg::read) this register and get [`vwire_in::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VwireInSpec;
impl crate::RegisterSpec for VwireInSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vwire_in::R`](R) reader structure"]
impl crate::Readable for VwireInSpec {}
#[doc = "`reset()` method sets VWIRE_IN to value 0"]
impl crate::Resettable for VwireInSpec {}
