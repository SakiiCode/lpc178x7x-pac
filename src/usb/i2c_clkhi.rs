#[doc = "Register `I2C_CLKHI` reader"]
pub type R = crate::R<I2cClkhiSpec>;
#[doc = "Register `I2C_CLKHI` writer"]
pub type W = crate::W<I2cClkhiSpec>;
#[doc = "Field `CDHI` reader - Clock divisor high. This value is the number of 48 MHz clocks the serial clock (SCL) will be high."]
pub type CdhiR = crate::FieldReader;
#[doc = "Field `CDHI` writer - Clock divisor high. This value is the number of 48 MHz clocks the serial clock (SCL) will be high."]
pub type CdhiW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Clock divisor high. This value is the number of 48 MHz clocks the serial clock (SCL) will be high."]
    #[inline(always)]
    pub fn cdhi(&self) -> CdhiR {
        CdhiR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Clock divisor high. This value is the number of 48 MHz clocks the serial clock (SCL) will be high."]
    #[inline(always)]
    pub fn cdhi(&mut self) -> CdhiW<'_, I2cClkhiSpec> {
        CdhiW::new(self, 0)
    }
}
#[doc = "I2C Clock High\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_clkhi::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_clkhi::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cClkhiSpec;
impl crate::RegisterSpec for I2cClkhiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_clkhi::R`](R) reader structure"]
impl crate::Readable for I2cClkhiSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_clkhi::W`](W) writer structure"]
impl crate::Writable for I2cClkhiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_CLKHI to value 0xb9"]
impl crate::Resettable for I2cClkhiSpec {
    const RESET_VALUE: u32 = 0xb9;
}
