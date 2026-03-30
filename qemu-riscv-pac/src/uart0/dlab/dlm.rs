#[doc = "Register `DLM` reader"]
pub type R = crate::R<DlmSpec>;
#[doc = "Register `DLM` writer"]
pub type W = crate::W<DlmSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Divisor Latch MSB (DLAB=1)\n\nYou can [`read`](crate::Reg::read) this register and get [`dlm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DlmSpec;
impl crate::RegisterSpec for DlmSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`dlm::R`](R) reader structure"]
impl crate::Readable for DlmSpec {}
#[doc = "`write(|w| ..)` method takes [`dlm::W`](W) writer structure"]
impl crate::Writable for DlmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DLM to value 0"]
impl crate::Resettable for DlmSpec {}
