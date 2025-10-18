#[doc = "Register `ENSET` writer"]
pub type W = crate::W<EnsetSpec>;
#[doc = "Field `RDWR_SET_EN` writer - Set read/write operation finished interrupt enable bit (EEPROM). 0: leave corresponding bit unchanged. 1: set corresponding bit."]
pub type RdwrSetEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PROG1_SET_EN` writer - Set program operation finished interrupt enable bit for EEPROM device 1. 0: leave corresponding bit unchanged. 1: set corresponding bit."]
pub type Prog1SetEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 26 - Set read/write operation finished interrupt enable bit (EEPROM). 0: leave corresponding bit unchanged. 1: set corresponding bit."]
    #[inline(always)]
    pub fn rdwr_set_en(&mut self) -> RdwrSetEnW<'_, EnsetSpec> {
        RdwrSetEnW::new(self, 26)
    }
    #[doc = "Bit 28 - Set program operation finished interrupt enable bit for EEPROM device 1. 0: leave corresponding bit unchanged. 1: set corresponding bit."]
    #[inline(always)]
    pub fn prog1_set_en(&mut self) -> Prog1SetEnW<'_, EnsetSpec> {
        Prog1SetEnW::new(self, 28)
    }
}
#[doc = "EEPROM interrupt enable set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnsetSpec;
impl crate::RegisterSpec for EnsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`enset::W`](W) writer structure"]
impl crate::Writable for EnsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ENSET to value 0"]
impl crate::Resettable for EnsetSpec {}
