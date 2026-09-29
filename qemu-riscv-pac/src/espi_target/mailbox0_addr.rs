#[doc = "Register `MAILBOX0_ADDR` reader"]
pub type R = crate::R<Mailbox0AddrSpec>;
#[doc = "Register `MAILBOX0_ADDR` writer"]
pub type W = crate::W<Mailbox0AddrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Mailbox zero host decode base, not an EC pointer. Must be 4-KiB aligned and distinct from the other bases. Change only while CONTROL.ENABLE is zero.\n\nYou can [`read`](crate::Reg::read) this register and get [`mailbox0_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mailbox0_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mailbox0AddrSpec;
impl crate::RegisterSpec for Mailbox0AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mailbox0_addr::R`](R) reader structure"]
impl crate::Readable for Mailbox0AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`mailbox0_addr::W`](W) writer structure"]
impl crate::Writable for Mailbox0AddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAILBOX0_ADDR to value 0"]
impl crate::Resettable for Mailbox0AddrSpec {}
