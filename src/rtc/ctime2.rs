#[doc = "Register `CTIME2` reader"]
pub type R = crate::R<Ctime2Spec>;
#[doc = "Field `DOY` reader - Day of year value in the range of 1 to 365 (366 for leap years)."]
pub type DoyR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Day of year value in the range of 1 to 365 (366 for leap years)."]
    #[inline(always)]
    pub fn doy(&self) -> DoyR {
        DoyR::new((self.bits & 0x0fff) as u16)
    }
}
#[doc = "Consolidated Time Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ctime2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctime2Spec;
impl crate::RegisterSpec for Ctime2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctime2::R`](R) reader structure"]
impl crate::Readable for Ctime2Spec {}
#[doc = "`reset()` method sets CTIME2 to value 0"]
impl crate::Resettable for Ctime2Spec {}
