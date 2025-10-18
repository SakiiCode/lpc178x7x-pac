#[doc = "Register `POS` reader"]
pub type R = crate::R<PosSpec>;
#[doc = "Field `POS` reader - Current position value."]
pub type PosR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Current position value."]
    #[inline(always)]
    pub fn pos(&self) -> PosR {
        PosR::new(self.bits)
    }
}
#[doc = "Position register\n\nYou can [`read`](crate::Reg::read) this register and get [`pos::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PosSpec;
impl crate::RegisterSpec for PosSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pos::R`](R) reader structure"]
impl crate::Readable for PosSpec {}
#[doc = "`reset()` method sets POS to value 0"]
impl crate::Resettable for PosSpec {}
