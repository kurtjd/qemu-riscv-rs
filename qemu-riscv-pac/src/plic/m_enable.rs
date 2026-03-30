#[doc = "Register `M_ENABLE%s` reader"]
pub type R = crate::R<MEnableSpec>;
#[doc = "Register `M_ENABLE%s` writer"]
pub type W = crate::W<MEnableSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "M-mode interrupt enable bits (context 0)\n\nYou can [`read`](crate::Reg::read) this register and get [`m_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEnableSpec;
impl crate::RegisterSpec for MEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m_enable::R`](R) reader structure"]
impl crate::Readable for MEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`m_enable::W`](W) writer structure"]
impl crate::Writable for MEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets M_ENABLE%s to value 0"]
impl crate::Resettable for MEnableSpec {}
