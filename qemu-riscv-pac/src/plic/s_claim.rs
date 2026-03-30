#[doc = "Register `S_CLAIM` reader"]
pub type R = crate::R<SClaimSpec>;
#[doc = "Register `S_CLAIM` writer"]
pub type W = crate::W<SClaimSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "S-mode interrupt claim/complete (context 1). Read to claim, write to complete.\n\nYou can [`read`](crate::Reg::read) this register and get [`s_claim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s_claim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SClaimSpec;
impl crate::RegisterSpec for SClaimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s_claim::R`](R) reader structure"]
impl crate::Readable for SClaimSpec {}
#[doc = "`write(|w| ..)` method takes [`s_claim::W`](W) writer structure"]
impl crate::Writable for SClaimSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S_CLAIM to value 0"]
impl crate::Resettable for SClaimSpec {}
