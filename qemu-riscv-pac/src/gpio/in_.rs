#[doc = "Register `IN` reader"]
pub type R = crate::R<InSpec>;
#[doc = "Field `IN` reader - Per-pin input level"]
pub type InR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Per-pin input level"]
    #[inline(always)]
    pub fn in_(&self) -> InR {
        InR::new(self.bits)
    }
}
#[doc = "Input level (one bit per pin, driven by each pin's peer)\n\nYou can [`read`](crate::Reg::read) this register and get [`in_::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InSpec;
impl crate::RegisterSpec for InSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_::R`](R) reader structure"]
impl crate::Readable for InSpec {}
#[doc = "`reset()` method sets IN to value 0"]
impl crate::Resettable for InSpec {}
