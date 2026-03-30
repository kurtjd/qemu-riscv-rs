#[doc = "Register `M_CLAIM` reader"]
pub type R = crate::R<MClaimSpec>;
#[doc = "Register `M_CLAIM` writer"]
pub type W = crate::W<MClaimSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "M-mode interrupt claim/complete (context 0). Read to claim, write to complete.\n\nYou can [`read`](crate::Reg::read) this register and get [`m_claim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m_claim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MClaimSpec;
impl crate::RegisterSpec for MClaimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m_claim::R`](R) reader structure"]
impl crate::Readable for MClaimSpec {}
#[doc = "`write(|w| ..)` method takes [`m_claim::W`](W) writer structure"]
impl crate::Writable for MClaimSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets M_CLAIM to value 0"]
impl crate::Resettable for MClaimSpec {}
