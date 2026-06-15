#[doc = "Register `DATA` reader"]
pub type R = crate::R<DataSpec>;
#[doc = "Register `DATA` writer"]
pub type W = crate::W<DataSpec>;
#[doc = "Field `DATA` reader - Data byte (RX byte on read, TX/address byte on write)"]
pub type DataR = crate::FieldReader;
#[doc = "Field `DATA` writer - Data byte (RX byte on read, TX/address byte on write)"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CMD` reader - Command on write (0=START, 1=STOP, 2=RX, 3=TX)"]
pub type CmdR = crate::FieldReader;
#[doc = "Field `CMD` writer - Command on write (0=START, 1=STOP, 2=RX, 3=TX)"]
pub type CmdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:7 - Data byte (RX byte on read, TX/address byte on write)"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:9 - Command on write (0=START, 1=STOP, 2=RX, 3=TX)"]
    #[inline(always)]
    pub fn cmd(&self) -> CmdR {
        CmdR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data byte (RX byte on read, TX/address byte on write)"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, DataSpec> {
        DataW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Command on write (0=START, 1=STOP, 2=RX, 3=TX)"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CmdW<'_, DataSpec> {
        CmdW::new(self, 8)
    }
}
#[doc = "Data register: read pops RX FIFO; write issues a command\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DataSpec;
impl crate::RegisterSpec for DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data::R`](R) reader structure"]
impl crate::Readable for DataSpec {}
#[doc = "`write(|w| ..)` method takes [`data::W`](W) writer structure"]
impl crate::Writable for DataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA to value 0"]
impl crate::Resettable for DataSpec {}
