#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `RX_RDY_IE` reader - RX-ready interrupt enable"]
pub type RxRdyIeR = crate::BitReader;
#[doc = "Field `RX_RDY_IE` writer - RX-ready interrupt enable"]
pub type RxRdyIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMD_DONE_IE` reader - Command-complete interrupt enable"]
pub type CmdDoneIeR = crate::BitReader;
#[doc = "Field `CMD_DONE_IE` writer - Command-complete interrupt enable"]
pub type CmdDoneIeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLEAR_FIFO` reader - Clear RX FIFO (write-1, self-clearing)"]
pub type ClearFifoR = crate::BitReader;
#[doc = "Field `CLEAR_FIFO` writer - Clear RX FIFO (write-1, self-clearing)"]
pub type ClearFifoW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RESET` reader - Soft reset (write-1, self-clearing)"]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - Soft reset (write-1, self-clearing)"]
pub type ResetW<'a, REG> = crate::BitWriter1S<'a, REG>;
impl R {
    #[doc = "Bit 0 - RX-ready interrupt enable"]
    #[inline(always)]
    pub fn rx_rdy_ie(&self) -> RxRdyIeR {
        RxRdyIeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Command-complete interrupt enable"]
    #[inline(always)]
    pub fn cmd_done_ie(&self) -> CmdDoneIeR {
        CmdDoneIeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear RX FIFO (write-1, self-clearing)"]
    #[inline(always)]
    pub fn clear_fifo(&self) -> ClearFifoR {
        ClearFifoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Soft reset (write-1, self-clearing)"]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RX-ready interrupt enable"]
    #[inline(always)]
    pub fn rx_rdy_ie(&mut self) -> RxRdyIeW<'_, CtrlSpec> {
        RxRdyIeW::new(self, 0)
    }
    #[doc = "Bit 1 - Command-complete interrupt enable"]
    #[inline(always)]
    pub fn cmd_done_ie(&mut self) -> CmdDoneIeW<'_, CtrlSpec> {
        CmdDoneIeW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear RX FIFO (write-1, self-clearing)"]
    #[inline(always)]
    pub fn clear_fifo(&mut self) -> ClearFifoW<'_, CtrlSpec> {
        ClearFifoW::new(self, 2)
    }
    #[doc = "Bit 3 - Soft reset (write-1, self-clearing)"]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, CtrlSpec> {
        ResetW::new(self, 3)
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0c;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
