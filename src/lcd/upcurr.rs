#[doc = "Register `UPCURR` reader"]
pub type R = crate::R<UpcurrSpec>;
#[doc = "Field `LCDUPCURR` reader - LCD Upper Panel Current Address. Contains the current LCD upper panel data DMA address."]
pub type LcdupcurrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - LCD Upper Panel Current Address. Contains the current LCD upper panel data DMA address."]
    #[inline(always)]
    pub fn lcdupcurr(&self) -> LcdupcurrR {
        LcdupcurrR::new(self.bits)
    }
}
#[doc = "Upper Panel Current Address Value register\n\nYou can [`read`](crate::Reg::read) this register and get [`upcurr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UpcurrSpec;
impl crate::RegisterSpec for UpcurrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`upcurr::R`](R) reader structure"]
impl crate::Readable for UpcurrSpec {}
#[doc = "`reset()` method sets UPCURR to value 0"]
impl crate::Resettable for UpcurrSpec {}
