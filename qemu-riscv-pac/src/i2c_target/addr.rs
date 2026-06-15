#[doc = "Register `ADDR` reader"]
pub type R = crate::R<AddrSpec>;
#[doc = "Register `ADDR` writer"]
pub type W = crate::W<AddrSpec>;
#[doc = "Field `ADDR` reader - 7-bit address this target answers to"]
pub type AddrR = crate::FieldReader;
#[doc = "Field `ADDR` writer - 7-bit address this target answers to"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:6 - 7-bit address this target answers to"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit address this target answers to"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, AddrSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "Target match address\n\nYou can [`read`](crate::Reg::read) this register and get [`addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AddrSpec;
impl crate::RegisterSpec for AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr::R`](R) reader structure"]
impl crate::Readable for AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`addr::W`](W) writer structure"]
impl crate::Writable for AddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADDR to value 0"]
impl crate::Resettable for AddrSpec {}
