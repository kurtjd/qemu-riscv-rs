#[doc = "Register `S_THRESHOLD` reader"]
pub type R = crate::R<SThresholdSpec>;
#[doc = "Register `S_THRESHOLD` writer"]
pub type W = crate::W<SThresholdSpec>;
#[doc = "Field `THRESHOLD` reader - Priority threshold (interrupts with priority <= threshold are masked)"]
pub type ThresholdR = crate::FieldReader;
#[doc = "Field `THRESHOLD` writer - Priority threshold (interrupts with priority <= threshold are masked)"]
pub type ThresholdW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Priority threshold (interrupts with priority <= threshold are masked)"]
    #[inline(always)]
    pub fn threshold(&self) -> ThresholdR {
        ThresholdR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Priority threshold (interrupts with priority <= threshold are masked)"]
    #[inline(always)]
    pub fn threshold(&mut self) -> ThresholdW<'_, SThresholdSpec> {
        ThresholdW::new(self, 0)
    }
}
#[doc = "S-mode priority threshold (context 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`s_threshold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s_threshold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SThresholdSpec;
impl crate::RegisterSpec for SThresholdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s_threshold::R`](R) reader structure"]
impl crate::Readable for SThresholdSpec {}
#[doc = "`write(|w| ..)` method takes [`s_threshold::W`](W) writer structure"]
impl crate::Writable for SThresholdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S_THRESHOLD to value 0"]
impl crate::Resettable for SThresholdSpec {}
