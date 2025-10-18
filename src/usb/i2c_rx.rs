#[doc = "Register `I2C_RX` reader"]
pub type R = crate::R<I2cRxSpec>;
#[doc = "Field `RXDATA` reader - Receive data."]
pub type RxdataR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Receive data."]
    #[inline(always)]
    pub fn rxdata(&self) -> RxdataR {
        RxdataR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "I2C Receive\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_rx::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cRxSpec;
impl crate::RegisterSpec for I2cRxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_rx::R`](R) reader structure"]
impl crate::Readable for I2cRxSpec {}
#[doc = "`reset()` method sets I2C_RX to value 0"]
impl crate::Resettable for I2cRxSpec {}
