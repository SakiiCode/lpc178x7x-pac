#[doc = "Register `CONSET` reader"]
pub type R = crate::R<ConsetSpec>;
#[doc = "Register `CONSET` writer"]
pub type W = crate::W<ConsetSpec>;
#[doc = "Field `AA` reader - Assert acknowledge flag."]
pub type AaR = crate::BitReader;
#[doc = "Field `AA` writer - Assert acknowledge flag."]
pub type AaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SI` reader - I2C interrupt flag."]
pub type SiR = crate::BitReader;
#[doc = "Field `SI` writer - I2C interrupt flag."]
pub type SiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STO` reader - STOP flag."]
pub type StoR = crate::BitReader;
#[doc = "Field `STO` writer - STOP flag."]
pub type StoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STA` reader - START flag."]
pub type StaR = crate::BitReader;
#[doc = "Field `STA` writer - START flag."]
pub type StaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2EN` reader - I2C interface enable."]
pub type I2enR = crate::BitReader;
#[doc = "Field `I2EN` writer - I2C interface enable."]
pub type I2enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 2 - Assert acknowledge flag."]
    #[inline(always)]
    pub fn aa(&self) -> AaR {
        AaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C interrupt flag."]
    #[inline(always)]
    pub fn si(&self) -> SiR {
        SiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - STOP flag."]
    #[inline(always)]
    pub fn sto(&self) -> StoR {
        StoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - START flag."]
    #[inline(always)]
    pub fn sta(&self) -> StaR {
        StaR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C interface enable."]
    #[inline(always)]
    pub fn i2en(&self) -> I2enR {
        I2enR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Assert acknowledge flag."]
    #[inline(always)]
    pub fn aa(&mut self) -> AaW<'_, ConsetSpec> {
        AaW::new(self, 2)
    }
    #[doc = "Bit 3 - I2C interrupt flag."]
    #[inline(always)]
    pub fn si(&mut self) -> SiW<'_, ConsetSpec> {
        SiW::new(self, 3)
    }
    #[doc = "Bit 4 - STOP flag."]
    #[inline(always)]
    pub fn sto(&mut self) -> StoW<'_, ConsetSpec> {
        StoW::new(self, 4)
    }
    #[doc = "Bit 5 - START flag."]
    #[inline(always)]
    pub fn sta(&mut self) -> StaW<'_, ConsetSpec> {
        StaW::new(self, 5)
    }
    #[doc = "Bit 6 - I2C interface enable."]
    #[inline(always)]
    pub fn i2en(&mut self) -> I2enW<'_, ConsetSpec> {
        I2enW::new(self, 6)
    }
}
#[doc = "I2C Control Set Register. When a one is written to a bit of this register, the corresponding bit in the I2C control register is set. Writing a zero has no effect on the corresponding bit in the I2C control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`conset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConsetSpec;
impl crate::RegisterSpec for ConsetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conset::R`](R) reader structure"]
impl crate::Readable for ConsetSpec {}
#[doc = "`write(|w| ..)` method takes [`conset::W`](W) writer structure"]
impl crate::Writable for ConsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONSET to value 0"]
impl crate::Resettable for ConsetSpec {}
