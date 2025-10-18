#[doc = "Register `CTIME1` reader"]
pub type R = crate::R<Ctime1Spec>;
#[doc = "Field `DOM` reader - Day of month value in the range of 1 to 28, 29, 30, or 31 (depending on the month and whether it is a leap year)."]
pub type DomR = crate::FieldReader;
#[doc = "Field `MONTH` reader - Month value in the range of 1 to 12."]
pub type MonthR = crate::FieldReader;
#[doc = "Field `YEAR` reader - Year value in the range of 0 to 4095."]
pub type YearR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:4 - Day of month value in the range of 1 to 28, 29, 30, or 31 (depending on the month and whether it is a leap year)."]
    #[inline(always)]
    pub fn dom(&self) -> DomR {
        DomR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Month value in the range of 1 to 12."]
    #[inline(always)]
    pub fn month(&self) -> MonthR {
        MonthR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:27 - Year value in the range of 0 to 4095."]
    #[inline(always)]
    pub fn year(&self) -> YearR {
        YearR::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "Consolidated Time Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ctime1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ctime1Spec;
impl crate::RegisterSpec for Ctime1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctime1::R`](R) reader structure"]
impl crate::Readable for Ctime1Spec {}
#[doc = "`reset()` method sets CTIME1 to value 0"]
impl crate::Resettable for Ctime1Spec {}
