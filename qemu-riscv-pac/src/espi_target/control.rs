#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `ENABLE` reader - Enable host access to mailbox data and control windows"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable host access to mailbox data and control windows"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable host access to mailbox data and control windows"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable host access to mailbox data and control windows"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, ControlSpec> {
        EnableW::new(self, 0)
    }
}
#[doc = "Enable peripheral-channel mailbox access. Program all three decode bases while disabled; enabling requires distinct, 4-KiB-aligned bases.\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for ControlSpec {}
