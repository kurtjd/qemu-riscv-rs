#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Register `STATUS` writer"]
pub type W = crate::W<StatusSpec>;
#[doc = "Field `RX_RDY` reader - RX FIFO not empty"]
pub type RxRdyR = crate::BitReader;
#[doc = "Field `NAK` reader - Target NAKed / link dropped (write-1-to-clear)"]
pub type NakR = crate::BitReader;
#[doc = "Field `NAK` writer - Target NAKed / link dropped (write-1-to-clear)"]
pub type NakW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PROTO_ERR` reader - Illegal command ordering (write-1-to-clear)"]
pub type ProtoErrR = crate::BitReader;
#[doc = "Field `PROTO_ERR` writer - Illegal command ordering (write-1-to-clear)"]
pub type ProtoErrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CMD_DONE` reader - Command completed (write-1-to-clear)"]
pub type CmdDoneR = crate::BitReader;
#[doc = "Field `CMD_DONE` writer - Command completed (write-1-to-clear)"]
pub type CmdDoneW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - RX FIFO not empty"]
    #[inline(always)]
    pub fn rx_rdy(&self) -> RxRdyR {
        RxRdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Target NAKed / link dropped (write-1-to-clear)"]
    #[inline(always)]
    pub fn nak(&self) -> NakR {
        NakR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Illegal command ordering (write-1-to-clear)"]
    #[inline(always)]
    pub fn proto_err(&self) -> ProtoErrR {
        ProtoErrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Command completed (write-1-to-clear)"]
    #[inline(always)]
    pub fn cmd_done(&self) -> CmdDoneR {
        CmdDoneR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Target NAKed / link dropped (write-1-to-clear)"]
    #[inline(always)]
    pub fn nak(&mut self) -> NakW<'_, StatusSpec> {
        NakW::new(self, 1)
    }
    #[doc = "Bit 2 - Illegal command ordering (write-1-to-clear)"]
    #[inline(always)]
    pub fn proto_err(&mut self) -> ProtoErrW<'_, StatusSpec> {
        ProtoErrW::new(self, 2)
    }
    #[doc = "Bit 3 - Command completed (write-1-to-clear)"]
    #[inline(always)]
    pub fn cmd_done(&mut self) -> CmdDoneW<'_, StatusSpec> {
        CmdDoneW::new(self, 3)
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0e;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for StatusSpec {}
