#[doc = "Register `I2C_CLKLO` writer"]
pub type W = crate::W<I2cClkloSpec>;
#[doc = "Field `CDLO` writer - Clock divisor low. This value is the number of 48 MHz clocks the serial clock (SCL) will be low."]
pub type CdloW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Clock divisor low. This value is the number of 48 MHz clocks the serial clock (SCL) will be low."]
    #[inline(always)]
    pub fn cdlo(&mut self) -> CdloW<'_, I2cClkloSpec> {
        CdloW::new(self, 0)
    }
}
#[doc = "I2C Clock Low\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_clklo::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cClkloSpec;
impl crate::RegisterSpec for I2cClkloSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`i2c_clklo::W`](W) writer structure"]
impl crate::Writable for I2cClkloSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_CLKLO to value 0xb9"]
impl crate::Resettable for I2cClkloSpec {
    const RESET_VALUE: u32 = 0xb9;
}
