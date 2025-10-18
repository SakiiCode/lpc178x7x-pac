#[doc = "Register `FMSW3` reader"]
pub type R = crate::R<Fmsw3Spec>;
#[doc = "Field `SW3_127_96` reader - Word 3 of 128-bit signature (bits 127 to 96)."]
pub type Sw3_127_96R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Word 3 of 128-bit signature (bits 127 to 96)."]
    #[inline(always)]
    pub fn sw3_127_96(&self) -> Sw3_127_96R {
        Sw3_127_96R::new(self.bits)
    }
}
#[doc = "128-bit signature Word 3\n\nYou can [`read`](crate::Reg::read) this register and get [`fmsw3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fmsw3Spec;
impl crate::RegisterSpec for Fmsw3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fmsw3::R`](R) reader structure"]
impl crate::Readable for Fmsw3Spec {}
#[doc = "`reset()` method sets FMSW3 to value 0"]
impl crate::Resettable for Fmsw3Spec {}
