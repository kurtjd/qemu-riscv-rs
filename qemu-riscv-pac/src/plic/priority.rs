#[doc = "Register `PRIORITY[%s]` reader"]
pub type R = crate::R<PrioritySpec>;
#[doc = "Register `PRIORITY[%s]` writer"]
pub type W = crate::W<PrioritySpec>;
#[doc = "Field `PRIORITY` reader - Priority level (0=disabled, 1=lowest, 7=highest)"]
pub type PriorityR = crate::FieldReader;
#[doc = "Field `PRIORITY` writer - Priority level (0=disabled, 1=lowest, 7=highest)"]
pub type PriorityW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Priority level (0=disabled, 1=lowest, 7=highest)"]
    #[inline(always)]
    pub fn priority(&self) -> PriorityR {
        PriorityR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Priority level (0=disabled, 1=lowest, 7=highest)"]
    #[inline(always)]
    pub fn priority(&mut self) -> PriorityW<'_, PrioritySpec> {
        PriorityW::new(self, 0)
    }
}
#[doc = "Priority for interrupt source %s (source 0 is reserved)\n\nYou can [`read`](crate::Reg::read) this register and get [`priority::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`priority::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PrioritySpec;
impl crate::RegisterSpec for PrioritySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`priority::R`](R) reader structure"]
impl crate::Readable for PrioritySpec {}
#[doc = "`write(|w| ..)` method takes [`priority::W`](W) writer structure"]
impl crate::Writable for PrioritySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PRIORITY[%s] to value 0"]
impl crate::Resettable for PrioritySpec {}
