#[doc = "Register `ENCLR` writer"]
pub type W = crate::W<EnclrSpec>;
#[doc = "Field `RDWR_CLR_EN` writer - Clear read/write operation finished interrupt enable bit (EEPROM). 0: leave corresponding bit unchanged. 1: clear corresponding bit."]
pub type RdwrClrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROG1_CLR_EN` writer - Clear program operation finished interrupt enable bit for EEPROM device 1. 0: leave corresponding bit unchanged. 1: clear corresponding bit."]
pub type Prog1ClrEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 26 - Clear read/write operation finished interrupt enable bit (EEPROM). 0: leave corresponding bit unchanged. 1: clear corresponding bit."]
    #[inline(always)]
    pub fn rdwr_clr_en(&mut self) -> RdwrClrEnW<'_, EnclrSpec> {
        RdwrClrEnW::new(self, 26)
    }
    #[doc = "Bit 28 - Clear program operation finished interrupt enable bit for EEPROM device 1. 0: leave corresponding bit unchanged. 1: clear corresponding bit."]
    #[inline(always)]
    pub fn prog1_clr_en(&mut self) -> Prog1ClrEnW<'_, EnclrSpec> {
        Prog1ClrEnW::new(self, 28)
    }
}
#[doc = "EEPROM interrupt enable clear\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnclrSpec;
impl crate::RegisterSpec for EnclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`enclr::W`](W) writer structure"]
impl crate::Writable for EnclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENCLR to value 0"]
impl crate::Resettable for EnclrSpec {}
