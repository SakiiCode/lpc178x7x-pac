#[doc = "Register `CTIME0` reader"]
pub type R = crate::R<Ctime0Spec>;
#[doc = "Field `SECONDS` reader - Seconds value in the range of 0 to 59"]
pub type SecondsR = crate::FieldReader;
#[doc = "Field `MINUTES` reader - Minutes value in the range of 0 to 59"]
pub type MinutesR = crate::FieldReader;
#[doc = "Field `HOURS` reader - Hours value in the range of 0 to 23"]
pub type HoursR = crate::FieldReader;
#[doc = "Field `DOW` reader - Day of week value in the range of 0 to 6"]
pub type DowR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:5 - Seconds value in the range of 0 to 59"]
    #[inline(always)]
    pub fn seconds(&self) -> SecondsR {
        SecondsR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Minutes value in the range of 0 to 59"]
    #[inline(always)]
    pub fn minutes(&self) -> MinutesR {
        MinutesR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - Hours value in the range of 0 to 23"]
    #[inline(always)]
    pub fn hours(&self) -> HoursR {
        HoursR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 24:26 - Day of week value in the range of 0 to 6"]
    #[inline(always)]
    pub fn dow(&self) -> DowR {
        DowR::new(((self.bits >> 24) & 7) as u8)
    }
}
#[doc = "Consolidated Time Register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ctime0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctime0Spec;
impl crate::RegisterSpec for Ctime0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctime0::R`](R) reader structure"]
impl crate::Readable for Ctime0Spec {}
#[doc = "`reset()` method sets CTIME0 to value 0"]
impl crate::Resettable for Ctime0Spec {}
