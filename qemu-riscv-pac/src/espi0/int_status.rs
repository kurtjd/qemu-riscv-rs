#[doc = "Register `INT_STATUS` reader"]
pub type R = crate::R<IntStatusSpec>;
#[doc = "Register `INT_STATUS` writer"]
pub type W = crate::W<IntStatusSpec>;
#[doc = "Field `PERIPH_PENDING` reader - Peripheral channel interrupt pending (peer wrote to shared memory)"]
pub type PeriphPendingR = crate::BitReader;
#[doc = "Field `PERIPH_PENDING` writer - Peripheral channel interrupt pending (peer wrote to shared memory)"]
pub type PeriphPendingW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VWIRE_PENDING` reader - Virtual wire interrupt pending (peer updated virtual wires)"]
pub type VwirePendingR = crate::BitReader;
#[doc = "Field `VWIRE_PENDING` writer - Virtual wire interrupt pending (peer updated virtual wires)"]
pub type VwirePendingW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Peripheral channel interrupt pending (peer wrote to shared memory)"]
    #[inline(always)]
    pub fn periph_pending(&self) -> PeriphPendingR {
        PeriphPendingR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Virtual wire interrupt pending (peer updated virtual wires)"]
    #[inline(always)]
    pub fn vwire_pending(&self) -> VwirePendingR {
        VwirePendingR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral channel interrupt pending (peer wrote to shared memory)"]
    #[inline(always)]
    pub fn periph_pending(&mut self) -> PeriphPendingW<'_, IntStatusSpec> {
        PeriphPendingW::new(self, 0)
    }
    #[doc = "Bit 1 - Virtual wire interrupt pending (peer updated virtual wires)"]
    #[inline(always)]
    pub fn vwire_pending(&mut self) -> VwirePendingW<'_, IntStatusSpec> {
        VwirePendingW::new(self, 1)
    }
}
#[doc = "Interrupt Status Register (write-1-to-clear)\n\nYou can [`read`](crate::Reg::read) this register and get [`int_status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntStatusSpec;
impl crate::RegisterSpec for IntStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_status::R`](R) reader structure"]
impl crate::Readable for IntStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`int_status::W`](W) writer structure"]
impl crate::Writable for IntStatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_STATUS to value 0"]
impl crate::Resettable for IntStatusSpec {}
