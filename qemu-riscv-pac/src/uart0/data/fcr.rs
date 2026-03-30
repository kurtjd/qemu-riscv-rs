#[doc = "Register `FCR` writer"]
pub type W = crate::W<FcrSpec>;
#[doc = "Field `FIFOE` writer - FIFO enable"]
pub type FifoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFIFOR` writer - Receiver FIFO reset (self-clearing)"]
pub type RfiforW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XFIFOR` writer - Transmitter FIFO reset (self-clearing)"]
pub type XfiforW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAM` writer - DMA mode select"]
pub type DmamW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RFTL` writer - Receiver FIFO trigger level"]
pub type RftlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl W {
    #[doc = "Bit 0 - FIFO enable"]
    #[inline(always)]
    pub fn fifoe(&mut self) -> FifoeW<'_, FcrSpec> {
        FifoeW::new(self, 0)
    }
    #[doc = "Bit 1 - Receiver FIFO reset (self-clearing)"]
    #[inline(always)]
    pub fn rfifor(&mut self) -> RfiforW<'_, FcrSpec> {
        RfiforW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmitter FIFO reset (self-clearing)"]
    #[inline(always)]
    pub fn xfifor(&mut self) -> XfiforW<'_, FcrSpec> {
        XfiforW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA mode select"]
    #[inline(always)]
    pub fn dmam(&mut self) -> DmamW<'_, FcrSpec> {
        DmamW::new(self, 3)
    }
    #[doc = "Bits 6:7 - Receiver FIFO trigger level"]
    #[inline(always)]
    pub fn rftl(&mut self) -> RftlW<'_, FcrSpec> {
        RftlW::new(self, 6)
    }
}
#[doc = "FIFO Control Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcrSpec;
impl crate::RegisterSpec for FcrSpec {
    type Ux = u8;
}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FcrSpec {}
