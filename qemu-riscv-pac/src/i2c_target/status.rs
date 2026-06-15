#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `RW` reader - Transfer direction (1=controller read, 0=controller write)"]
pub type RwR = crate::BitReader;
#[doc = "Field `START` reader - Addressed: address matched (write-1-to-clear)"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - Addressed: address matched (write-1-to-clear)"]
pub type StartW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `STOP` reader - Controller sent STOP (write-1-to-clear)"]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - Controller sent STOP (write-1-to-clear)"]
pub type StopW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RESTART` reader - Repeated START, also sets START (write-1-to-clear)"]
pub type RestartR = crate::BitReader;
#[doc = "Field `RESTART` writer - Repeated START, also sets START (write-1-to-clear)"]
pub type RestartW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RX_RDY` reader - Byte in DATA awaiting strobe"]
pub type RxRdyR = crate::BitReader;
#[doc = "Field `TX_DONE` reader - TX byte clocked and controller replied (write-1-to-clear)"]
pub type TxDoneR = crate::BitReader;
#[doc = "Field `TX_DONE` writer - TX byte clocked and controller replied (write-1-to-clear)"]
pub type TxDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `TX_NAK` reader - Last TX byte: 1=controller NAKed (done)"]
pub type TxNakR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transfer direction (1=controller read, 0=controller write)"]
    #[inline(always)]
    pub fn rw(&self) -> RwR {
        RwR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Addressed: address matched (write-1-to-clear)"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Controller sent STOP (write-1-to-clear)"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Repeated START, also sets START (write-1-to-clear)"]
    #[inline(always)]
    pub fn restart(&self) -> RestartR {
        RestartR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Byte in DATA awaiting strobe"]
    #[inline(always)]
    pub fn rx_rdy(&self) -> RxRdyR {
        RxRdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TX byte clocked and controller replied (write-1-to-clear)"]
    #[inline(always)]
    pub fn tx_done(&self) -> TxDoneR {
        TxDoneR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Last TX byte: 1=controller NAKed (done)"]
    #[inline(always)]
    pub fn tx_nak(&self) -> TxNakR {
        TxNakR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Addressed: address matched (write-1-to-clear)"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, StatusSpec> {
        StartW::new(self, 1)
    }
    #[doc = "Bit 2 - Controller sent STOP (write-1-to-clear)"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, StatusSpec> {
        StopW::new(self, 2)
    }
    #[doc = "Bit 3 - Repeated START, also sets START (write-1-to-clear)"]
    #[inline(always)]
    pub fn restart(&mut self) -> RestartW<'_, StatusSpec> {
        RestartW::new(self, 3)
    }
    #[doc = "Bit 5 - TX byte clocked and controller replied (write-1-to-clear)"]
    #[inline(always)]
    pub fn tx_done(&mut self) -> TxDoneW<'_, StatusSpec> {
        TxDoneW::new(self, 5)
    }
}
#[doc = "Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`status::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`write(|w| ..)` method takes [`status::W`](W) writer structure"]
impl crate::Writable for StatusSpec {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x2e;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
