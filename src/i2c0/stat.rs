#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Field `Status` reader - These bits give the actual status information about the I 2C interface."]
pub type StatusR = crate::FieldReader;
impl R {
    #[doc = "Bits 3:7 - These bits give the actual status information about the I 2C interface."]
    #[inline(always)]
    pub fn status(&self) -> StatusR {
        StatusR::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
#[doc = "I2C Status Register. During I2C operation, this register provides detailed status codes that allow software to determine the next action needed.\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`reset()` method sets STAT to value 0xf8"]
impl crate::Resettable for StatSpec {
    const RESET_VALUE: u32 = 0xf8;
}
