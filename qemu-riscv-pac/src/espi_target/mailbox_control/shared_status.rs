#[doc = "Register `SHARED_STATUS` reader"]
pub type R = crate::R<SharedStatusSpec>;
#[doc = "Register `SHARED_STATUS` writer"]
pub type W = crate::W<SharedStatusSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Application-defined shared status, writable by EC and host\n\nYou can [`read`](crate::Reg::read) this register and get [`shared_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shared_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SharedStatusSpec;
impl crate::RegisterSpec for SharedStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shared_status::R`](R) reader structure"]
impl crate::Readable for SharedStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`shared_status::W`](W) writer structure"]
impl crate::Writable for SharedStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SHARED_STATUS to value 0"]
impl crate::Resettable for SharedStatusSpec {}
