#[doc = "Register `MAILBOX1_ADDR` reader"]
pub type R = crate::R<Mailbox1AddrSpec>;
#[doc = "Register `MAILBOX1_ADDR` writer"]
pub type W = crate::W<Mailbox1AddrSpec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "Mailbox one host decode base, not an EC pointer. Must be 4-KiB aligned and distinct from the other bases. Change only while CONTROL.ENABLE is zero.\n\nYou can [`read`](crate::Reg::read) this register and get [`mailbox1_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mailbox1_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mailbox1AddrSpec;
impl crate::RegisterSpec for Mailbox1AddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mailbox1_addr::R`](R) reader structure"]
impl crate::Readable for Mailbox1AddrSpec {}
#[doc = "`write(|w| ..)` method takes [`mailbox1_addr::W`](W) writer structure"]
impl crate::Writable for Mailbox1AddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAILBOX1_ADDR to value 0"]
impl crate::Resettable for Mailbox1AddrSpec {}
