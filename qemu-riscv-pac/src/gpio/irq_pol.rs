#[doc = "Register `IRQ_POL` reader"]
pub type R = crate::R<IrqPolSpec>;
#[doc = "Register `IRQ_POL` writer"]
pub type W = crate::W<IrqPolSpec>;
#[doc = "Field `IRQ_POL` reader - Per-pin polarity select (0 = low/falling, 1 = high/rising)"]
pub type IrqPolR = crate::FieldReader<u32>;
#[doc = "Field `IRQ_POL` writer - Per-pin polarity select (0 = low/falling, 1 = high/rising)"]
pub type IrqPolW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Per-pin polarity select (0 = low/falling, 1 = high/rising)"]
    #[inline(always)]
    pub fn irq_pol(&self) -> IrqPolR {
        IrqPolR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Per-pin polarity select (0 = low/falling, 1 = high/rising)"]
    #[inline(always)]
    pub fn irq_pol(&mut self) -> IrqPolW<'_, IrqPolSpec> {
        IrqPolW::new(self, 0)
    }
}
#[doc = "Interrupt polarity (0 = low/falling, 1 = high/rising)\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_pol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_pol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqPolSpec;
impl crate::RegisterSpec for IrqPolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_pol::R`](R) reader structure"]
impl crate::Readable for IrqPolSpec {}
#[doc = "`write(|w| ..)` method takes [`irq_pol::W`](W) writer structure"]
impl crate::Writable for IrqPolSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQ_POL to value 0"]
impl crate::Resettable for IrqPolSpec {}
