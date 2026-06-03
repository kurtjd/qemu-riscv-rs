#[doc = "Register `INT_ENABLE` reader"]
pub type R = crate::R<IntEnableSpec>;
#[doc = "Register `INT_ENABLE` writer"]
pub type W = crate::W<IntEnableSpec>;
#[doc = "Field `PERIPH_EN` reader - Enable peripheral channel interrupt"]
pub type PeriphEnR = crate::BitReader;
#[doc = "Field `PERIPH_EN` writer - Enable peripheral channel interrupt"]
pub type PeriphEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VWIRE_EN` reader - Enable virtual wire interrupt"]
pub type VwireEnR = crate::BitReader;
#[doc = "Field `VWIRE_EN` writer - Enable virtual wire interrupt"]
pub type VwireEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable peripheral channel interrupt"]
    #[inline(always)]
    pub fn periph_en(&self) -> PeriphEnR {
        PeriphEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable virtual wire interrupt"]
    #[inline(always)]
    pub fn vwire_en(&self) -> VwireEnR {
        VwireEnR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable peripheral channel interrupt"]
    #[inline(always)]
    pub fn periph_en(&mut self) -> PeriphEnW<'_, IntEnableSpec> {
        PeriphEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable virtual wire interrupt"]
    #[inline(always)]
    pub fn vwire_en(&mut self) -> VwireEnW<'_, IntEnableSpec> {
        VwireEnW::new(self, 1)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_enable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_enable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntEnableSpec;
impl crate::RegisterSpec for IntEnableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_enable::R`](R) reader structure"]
impl crate::Readable for IntEnableSpec {}
#[doc = "`write(|w| ..)` method takes [`int_enable::W`](W) writer structure"]
impl crate::Writable for IntEnableSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENABLE to value 0"]
impl crate::Resettable for IntEnableSpec {}
