#[doc = "Register `IRQ_TRIG` reader"]
pub type R = crate::R<IrqTrigSpec>;
#[doc = "Register `IRQ_TRIG` writer"]
pub type W = crate::W<IrqTrigSpec>;
#[doc = "Field `IRQ_TRIG` reader - Per-pin trigger select (0 = level, 1 = edge)"]
pub type IrqTrigR = crate::FieldReader<u32>;
#[doc = "Field `IRQ_TRIG` writer - Per-pin trigger select (0 = level, 1 = edge)"]
pub type IrqTrigW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Per-pin trigger select (0 = level, 1 = edge)"]
    #[inline(always)]
    pub fn irq_trig(&self) -> IrqTrigR {
        IrqTrigR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Per-pin trigger select (0 = level, 1 = edge)"]
    #[inline(always)]
    pub fn irq_trig(&mut self) -> IrqTrigW<'_, IrqTrigSpec> {
        IrqTrigW::new(self, 0)
    }
}
#[doc = "Interrupt trigger type (0 = level, 1 = edge)\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_trig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_trig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqTrigSpec;
impl crate::RegisterSpec for IrqTrigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_trig::R`](R) reader structure"]
impl crate::Readable for IrqTrigSpec {}
#[doc = "`write(|w| ..)` method takes [`irq_trig::W`](W) writer structure"]
impl crate::Writable for IrqTrigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQ_TRIG to value 0"]
impl crate::Resettable for IrqTrigSpec {}
