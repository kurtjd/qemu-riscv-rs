#[doc = "Register `IRQ_EN` reader"]
pub type R = crate::R<IrqEnSpec>;
#[doc = "Register `IRQ_EN` writer"]
pub type W = crate::W<IrqEnSpec>;
#[doc = "Field `IRQ_EN` reader - Per-pin interrupt enable"]
pub type IrqEnR = crate::FieldReader<u32>;
#[doc = "Field `IRQ_EN` writer - Per-pin interrupt enable"]
pub type IrqEnW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Per-pin interrupt enable"]
    #[inline(always)]
    pub fn irq_en(&self) -> IrqEnR {
        IrqEnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Per-pin interrupt enable"]
    #[inline(always)]
    pub fn irq_en(&mut self) -> IrqEnW<'_, IrqEnSpec> {
        IrqEnW::new(self, 0)
    }
}
#[doc = "Per-pin interrupt enable\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqEnSpec;
impl crate::RegisterSpec for IrqEnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_en::R`](R) reader structure"]
impl crate::Readable for IrqEnSpec {}
#[doc = "`write(|w| ..)` method takes [`irq_en::W`](W) writer structure"]
impl crate::Writable for IrqEnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQ_EN to value 0"]
impl crate::Resettable for IrqEnSpec {}
