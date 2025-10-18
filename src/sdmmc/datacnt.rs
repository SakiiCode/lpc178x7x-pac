#[doc = "Register `DATACNT` reader"]
pub type R = crate::R<DatacntSpec>;
#[doc = "Field `DATACOUNT` reader - Remaining data"]
pub type DatacountR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Remaining data"]
    #[inline(always)]
    pub fn datacount(&self) -> DatacountR {
        DatacountR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Data counter.\n\nYou can [`read`](crate::Reg::read) this register and get [`datacnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatacntSpec;
impl crate::RegisterSpec for DatacntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datacnt::R`](R) reader structure"]
impl crate::Readable for DatacntSpec {}
#[doc = "`reset()` method sets DATACNT to value 0"]
impl crate::Resettable for DatacntSpec {}
