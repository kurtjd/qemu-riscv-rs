#[doc = "Register `VWIRE_OUT` reader"]
pub type R = crate::R<VwireOutSpec>;
#[doc = "Register `VWIRE_OUT` writer"]
pub type W = crate::W<VwireOutSpec>;
#[doc = "Field `DATA` reader - Virtual wire output data sent to peer"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `DATA` writer - Virtual wire output data sent to peer"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Virtual wire output data sent to peer"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Virtual wire output data sent to peer"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, VwireOutSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Virtual Wire Output Register\n\nYou can [`read`](crate::Reg::read) this register and get [`vwire_out::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vwire_out::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VwireOutSpec;
impl crate::RegisterSpec for VwireOutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vwire_out::R`](R) reader structure"]
impl crate::Readable for VwireOutSpec {}
#[doc = "`write(|w| ..)` method takes [`vwire_out::W`](W) writer structure"]
impl crate::Writable for VwireOutSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets VWIRE_OUT to value 0"]
impl crate::Resettable for VwireOutSpec {}
