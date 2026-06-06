#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `RX_ACK` reader - ACK pending RX byte and continue (write-1 strobe)"]
pub type RxAckR = crate::BitReader;
#[doc = "Field `RX_ACK` writer - ACK pending RX byte and continue (write-1 strobe)"]
pub type RxAckW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `RX_NAK` reader - NAK pending RX byte / reject (write-1 strobe)"]
pub type RxNakR = crate::BitReader;
#[doc = "Field `RX_NAK` writer - NAK pending RX byte / reject (write-1 strobe)"]
pub type RxNakW<'a, REG> = crate::BitWriter1S<'a, REG>;
#[doc = "Field `INT_START` reader - Interrupt enable: addressed (START)"]
pub type IntStartR = crate::BitReader;
#[doc = "Field `INT_START` writer - Interrupt enable: addressed (START)"]
pub type IntStartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_STOP` reader - Interrupt enable: STOP"]
pub type IntStopR = crate::BitReader;
#[doc = "Field `INT_STOP` writer - Interrupt enable: STOP"]
pub type IntStopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_RX_RDY` reader - Interrupt enable: byte received"]
pub type IntRxRdyR = crate::BitReader;
#[doc = "Field `INT_RX_RDY` writer - Interrupt enable: byte received"]
pub type IntRxRdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INT_TX` reader - Interrupt enable: TX done"]
pub type IntTxR = crate::BitReader;
#[doc = "Field `INT_TX` writer - Interrupt enable: TX done"]
pub type IntTxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - Recover / soft reset (write-1, self-clearing)"]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - Recover / soft reset (write-1, self-clearing)"]
pub type ResetW<'a, REG> = crate::BitWriter1S<'a, REG>;
impl R {
    #[doc = "Bit 0 - ACK pending RX byte and continue (write-1 strobe)"]
    #[inline(always)]
    pub fn rx_ack(&self) -> RxAckR {
        RxAckR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NAK pending RX byte / reject (write-1 strobe)"]
    #[inline(always)]
    pub fn rx_nak(&self) -> RxNakR {
        RxNakR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable: addressed (START)"]
    #[inline(always)]
    pub fn int_start(&self) -> IntStartR {
        IntStartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable: STOP"]
    #[inline(always)]
    pub fn int_stop(&self) -> IntStopR {
        IntStopR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable: byte received"]
    #[inline(always)]
    pub fn int_rx_rdy(&self) -> IntRxRdyR {
        IntRxRdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt enable: TX done"]
    #[inline(always)]
    pub fn int_tx(&self) -> IntTxR {
        IntTxR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Recover / soft reset (write-1, self-clearing)"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ACK pending RX byte and continue (write-1 strobe)"]
    #[inline(always)]
    pub fn rx_ack(&mut self) -> RxAckW<'_, CtrlSpec> {
        RxAckW::new(self, 0)
    }
    #[doc = "Bit 1 - NAK pending RX byte / reject (write-1 strobe)"]
    #[inline(always)]
    pub fn rx_nak(&mut self) -> RxNakW<'_, CtrlSpec> {
        RxNakW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt enable: addressed (START)"]
    #[inline(always)]
    pub fn int_start(&mut self) -> IntStartW<'_, CtrlSpec> {
        IntStartW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt enable: STOP"]
    #[inline(always)]
    pub fn int_stop(&mut self) -> IntStopW<'_, CtrlSpec> {
        IntStopW::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt enable: byte received"]
    #[inline(always)]
    pub fn int_rx_rdy(&mut self) -> IntRxRdyW<'_, CtrlSpec> {
        IntRxRdyW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt enable: TX done"]
    #[inline(always)]
    pub fn int_tx(&mut self) -> IntTxW<'_, CtrlSpec> {
        IntTxW::new(self, 5)
    }
    #[doc = "Bit 6 - Recover / soft reset (write-1, self-clearing)"]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, CtrlSpec> {
        ResetW::new(self, 6)
    }
}
#[doc = "Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x43;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
