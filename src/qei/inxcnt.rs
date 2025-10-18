#[doc = "Register `INXCNT` reader"]
pub type R = crate::R<InxcntSpec>;
#[doc = "Field `ENCPOS` reader - Current index counter value."]
pub type EncposR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current index counter value."]
    #[inline(always)]
    pub fn encpos(&self) -> EncposR {
        EncposR::new(self.bits)
    }
}
#[doc = "Index count register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`inxcnt::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InxcntSpec;
impl crate::RegisterSpec for InxcntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inxcnt::R`](R) reader structure"]
impl crate::Readable for InxcntSpec {}
#[doc = "`reset()` method sets INXCNT to value 0"]
impl crate::Resettable for InxcntSpec {}
