#[doc = "Register `PENDING%s` reader"]
pub type R = crate::R<PendingSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Interrupt pending bits\n\nYou can [`read`](crate::Reg::read) this register and get [`pending::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PendingSpec;
impl crate::RegisterSpec for PendingSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pending::R`](R) reader structure"]
impl crate::Readable for PendingSpec {}
#[doc = "`reset()` method sets PENDING%s to value 0"]
impl crate::Resettable for PendingSpec {}
