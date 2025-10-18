#[doc = "Register `TSV1` reader"]
pub type R = crate::R<Tsv1Spec>;
#[doc = "Field `TBC` reader - Transmit byte count. The total number of bytes in the frame, not counting the collided bytes."]
pub type TbcR = crate::FieldReader<u16>;
#[doc = "Field `TCC` reader - Transmit collision count. Number of collisions the current packet incurred during transmission attempts. The maximum number of collisions (16) cannot be represented."]
pub type TccR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Transmit byte count. The total number of bytes in the frame, not counting the collided bytes."]
    #[inline(always)]
    pub fn tbc(&self) -> TbcR {
        TbcR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Transmit collision count. Number of collisions the current packet incurred during transmission attempts. The maximum number of collisions (16) cannot be represented."]
    #[inline(always)]
    pub fn tcc(&self) -> TccR {
        TccR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "Transmit status vector 1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`tsv1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tsv1Spec;
impl crate::RegisterSpec for Tsv1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsv1::R`](R) reader structure"]
impl crate::Readable for Tsv1Spec {}
#[doc = "`reset()` method sets TSV1 to value 0"]
impl crate::Resettable for Tsv1Spec {}
