#[doc = "Register `DOORBELL` reader"]
pub type R = crate::R<DoorbellSpec>;
#[doc = "Register `DOORBELL` writer"]
pub type W = crate::W<DoorbellSpec>;
#[doc = "Field `PENDING` reader - "]
pub type PendingR = crate::BitReader;
#[doc = "Field `PENDING` writer - "]
pub type PendingW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pending(&self) -> PendingR {
        PendingR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pending(&mut self) -> PendingW<'_, DoorbellSpec> {
        PendingW::new(self, 0)
    }
}
#[doc = "Host-to-EC doorbell pending. EC writes bit zero as one to clear; pending doorbells assert PLIC source 5.\n\nYou can [`read`](crate::Reg::read) this register and get [`doorbell::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doorbell::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoorbellSpec;
impl crate::RegisterSpec for DoorbellSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doorbell::R`](R) reader structure"]
impl crate::Readable for DoorbellSpec {}
#[doc = "`write(|w| ..)` method takes [`doorbell::W`](W) writer structure"]
impl crate::Writable for DoorbellSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets DOORBELL to value 0"]
impl crate::Resettable for DoorbellSpec {}
