#[doc = "Register `PERIPH_ADDR` reader"]
pub type R = crate::R<PeriphAddrSpec>;
#[doc = "Field `OFFSET` reader - Byte offset within shared memory of last peer write"]
pub type OffsetR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Byte offset within shared memory of last peer write"]
    #[inline(always)]
    pub fn offset(&self) -> OffsetR {
        OffsetR::new(self.bits)
    }
}
#[doc = "Peripheral Address Register (offset of last peer shared memory write)\n\nYou can [`read`](crate::Reg::read) this register and get [`periph_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PeriphAddrSpec;
impl crate::RegisterSpec for PeriphAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`periph_addr::R`](R) reader structure"]
impl crate::Readable for PeriphAddrSpec {}
#[doc = "`reset()` method sets PERIPH_ADDR to value 0"]
impl crate::Resettable for PeriphAddrSpec {}
