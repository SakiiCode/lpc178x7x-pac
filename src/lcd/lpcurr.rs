#[doc = "Register `LPCURR` reader"]
pub type R = crate::R<LpcurrSpec>;
#[doc = "Field `LCDLPCURR` reader - LCD Lower Panel Current Address. Contains the current LCD lower panel data DMA address."]
pub type LcdlpcurrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - LCD Lower Panel Current Address. Contains the current LCD lower panel data DMA address."]
    #[inline(always)]
    pub fn lcdlpcurr(&self) -> LcdlpcurrR {
        LcdlpcurrR::new(self.bits)
    }
}
#[doc = "Lower Panel Current Address Value register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpcurr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpcurrSpec;
impl crate::RegisterSpec for LpcurrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpcurr::R`](R) reader structure"]
impl crate::Readable for LpcurrSpec {}
#[doc = "`reset()` method sets LPCURR to value 0"]
impl crate::Resettable for LpcurrSpec {}
