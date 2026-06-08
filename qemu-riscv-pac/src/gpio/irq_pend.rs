#[doc = "Register `IRQ_PEND` reader"]
pub type R = crate::R<IrqPendSpec>;
#[doc = "Register `IRQ_PEND` writer"]
pub type W = crate::W<IrqPendSpec>;
#[doc = "Field `IRQ_PEND` reader - Per-pin interrupt pending (write-1-to-clear)"]
pub type IrqPendR = crate::FieldReader<u32>;
#[doc = "Field `IRQ_PEND` writer - Per-pin interrupt pending (write-1-to-clear)"]
pub type IrqPendW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Per-pin interrupt pending (write-1-to-clear)"]
    #[inline(always)]
    pub fn irq_pend(&self) -> IrqPendR {
        IrqPendR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Per-pin interrupt pending (write-1-to-clear)"]
    #[inline(always)]
    pub fn irq_pend(&mut self) -> IrqPendW<'_, IrqPendSpec> {
        IrqPendW::new(self, 0)
    }
}
#[doc = "Per-pin interrupt pending (write-1-to-clear)\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_pend::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_pend::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqPendSpec;
impl crate::RegisterSpec for IrqPendSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_pend::R`](R) reader structure"]
impl crate::Readable for IrqPendSpec {}
#[doc = "`write(|w| ..)` method takes [`irq_pend::W`](W) writer structure"]
impl crate::Writable for IrqPendSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xffff_ffff;
}
#[doc = "`reset()` method sets IRQ_PEND to value 0"]
impl crate::Resettable for IrqPendSpec {}
