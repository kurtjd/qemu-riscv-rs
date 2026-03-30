#[doc = "Register `MCR` reader"]
pub type R = crate::R<McrSpec>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<McrSpec>;
#[doc = "Field `DTR` reader - Data terminal ready"]
pub type DtrR = crate::BitReader;
#[doc = "Field `DTR` writer - Data terminal ready"]
pub type DtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTS` reader - Request to send"]
pub type RtsR = crate::BitReader;
#[doc = "Field `RTS` writer - Request to send"]
pub type RtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT1` reader - Output 1"]
pub type Out1R = crate::BitReader;
#[doc = "Field `OUT1` writer - Output 1"]
pub type Out1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT2` reader - Output 2"]
pub type Out2R = crate::BitReader;
#[doc = "Field `OUT2` writer - Output 2"]
pub type Out2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOP` reader - Loopback mode"]
pub type LoopR = crate::BitReader;
#[doc = "Field `LOOP` writer - Loopback mode"]
pub type LoopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data terminal ready"]
    #[inline(always)]
    pub fn dtr(&self) -> DtrR {
        DtrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Request to send"]
    #[inline(always)]
    pub fn rts(&self) -> RtsR {
        RtsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Output 1"]
    #[inline(always)]
    pub fn out1(&self) -> Out1R {
        Out1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Output 2"]
    #[inline(always)]
    pub fn out2(&self) -> Out2R {
        Out2R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Loopback mode"]
    #[inline(always)]
    pub fn loop_(&self) -> LoopR {
        LoopR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data terminal ready"]
    #[inline(always)]
    pub fn dtr(&mut self) -> DtrW<'_, McrSpec> {
        DtrW::new(self, 0)
    }
    #[doc = "Bit 1 - Request to send"]
    #[inline(always)]
    pub fn rts(&mut self) -> RtsW<'_, McrSpec> {
        RtsW::new(self, 1)
    }
    #[doc = "Bit 2 - Output 1"]
    #[inline(always)]
    pub fn out1(&mut self) -> Out1W<'_, McrSpec> {
        Out1W::new(self, 2)
    }
    #[doc = "Bit 3 - Output 2"]
    #[inline(always)]
    pub fn out2(&mut self) -> Out2W<'_, McrSpec> {
        Out2W::new(self, 3)
    }
    #[doc = "Bit 4 - Loopback mode"]
    #[inline(always)]
    pub fn loop_(&mut self) -> LoopW<'_, McrSpec> {
        LoopW::new(self, 4)
    }
}
#[doc = "Modem Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McrSpec;
impl crate::RegisterSpec for McrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for McrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for McrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for McrSpec {}
