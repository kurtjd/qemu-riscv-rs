#[doc = "Register `IRQ_SET` reader"]
pub type R = crate::R<IrqSetSpec>;
#[doc = "Register `IRQ_SET` writer"]
pub type W = crate::W<IrqSetSpec>;
#[doc = "Field `PENDING` reader - "]
pub type PendingR = crate::BitReader;
#[doc = "Field `PENDING` writer - "]
pub type PendingW<'a, REG> = crate::BitWriter1S<'a, REG>;
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
    pub fn pending(&mut self) -> PendingW<'_, IrqSetSpec> {
        PendingW::new(self, 0)
    }
}
#[doc = "Read host IRQ pending; write one to assert host IRQ\n\nYou can [`read`](crate::Reg::read) this register and get [`irq_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqSetSpec;
impl crate::RegisterSpec for IrqSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq_set::R`](R) reader structure"]
impl crate::Readable for IrqSetSpec {}
#[doc = "`write(|w| ..)` method takes [`irq_set::W`](W) writer structure"]
impl crate::Writable for IrqSetSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets IRQ_SET to value 0"]
impl crate::Resettable for IrqSetSpec {}
