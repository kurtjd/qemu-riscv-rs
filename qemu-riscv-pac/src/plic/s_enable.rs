#[doc = "Register `S_ENABLE%s` reader"]
pub type R = crate::R<SEnableSpec>;
#[doc = "Register `S_ENABLE%s` writer"]
pub type W = crate::W<SEnableSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "S-mode interrupt enable bits (context 1)\n\nYou can [`read`](crate::Reg::read) this register and get [`s_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEnableSpec;
impl crate::RegisterSpec for SEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s_enable::R`](R) reader structure"]
impl crate::Readable for SEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`s_enable::W`](W) writer structure"]
impl crate::Writable for SEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S_ENABLE%s to value 0"]
impl crate::Resettable for SEnableSpec {}
