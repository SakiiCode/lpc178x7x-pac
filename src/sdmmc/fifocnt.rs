#[doc = "Register `FIFOCNT` reader"]
pub type R = crate::R<FifocntSpec>;
#[doc = "Field `DATACOUNT` reader - Remaining data"]
pub type DatacountR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - Remaining data"]
    #[inline(always)]
    pub fn datacount(&self) -> DatacountR {
        DatacountR::new((self.bits & 0x7fff) as u16)
    }
}
#[doc = "FIFO Counter.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifocnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FifocntSpec;
impl crate::RegisterSpec for FifocntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifocnt::R`](R) reader structure"]
impl crate::Readable for FifocntSpec {}
#[doc = "`reset()` method sets FIFOCNT to value 0"]
impl crate::Resettable for FifocntSpec {}
