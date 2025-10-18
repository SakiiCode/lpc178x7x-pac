#[doc = "Register `STATCLR` writer"]
pub type W = crate::W<StatclrSpec>;
#[doc = "Field `SIG_DONE_CLR` writer - Writing a 1 to this bits clears the signature generation completion flag (SIG_DONE) in the FMSTAT register."]
pub type SigDoneClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDWR_CLR_ST` writer - Clear read/write operation finished interrupt status bit (EEPROM). 0 leave corresponding bit unchanged. 1 clear corresponding bit."]
pub type RdwrClrStW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROG1_CLR_ST` writer - Clear program operation finished interrupt status bit for EEPROM device 1. 0 leave corresponding bit unchanged. 1 clear corresponding bit."]
pub type Prog1ClrStW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 2 - Writing a 1 to this bits clears the signature generation completion flag (SIG_DONE) in the FMSTAT register."]
    #[inline(always)]
    pub fn sig_done_clr(&mut self) -> SigDoneClrW<'_, StatclrSpec> {
        SigDoneClrW::new(self, 2)
    }
    #[doc = "Bit 26 - Clear read/write operation finished interrupt status bit (EEPROM). 0 leave corresponding bit unchanged. 1 clear corresponding bit."]
    #[inline(always)]
    pub fn rdwr_clr_st(&mut self) -> RdwrClrStW<'_, StatclrSpec> {
        RdwrClrStW::new(self, 26)
    }
    #[doc = "Bit 28 - Clear program operation finished interrupt status bit for EEPROM device 1. 0 leave corresponding bit unchanged. 1 clear corresponding bit."]
    #[inline(always)]
    pub fn prog1_clr_st(&mut self) -> Prog1ClrStW<'_, StatclrSpec> {
        Prog1ClrStW::new(self, 28)
    }
}
#[doc = "Signature generation status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatclrSpec;
impl crate::RegisterSpec for StatclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`statclr::W`](W) writer structure"]
impl crate::Writable for StatclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATCLR to value 0"]
impl crate::Resettable for StatclrSpec {}
