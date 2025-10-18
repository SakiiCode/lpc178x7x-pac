#[doc = "Register `EWL` reader"]
pub type R = crate::R<EwlSpec>;
#[doc = "Register `EWL` writer"]
pub type W = crate::W<EwlSpec>;
#[doc = "Field `EWL` reader - During CAN operation, this value is compared to both the Tx and Rx Error Counters. If either of these counter matches this value, the Error Status (ES) bit in CANSR is set."]
pub type EwlR = crate::FieldReader;
#[doc = "Field `EWL` writer - During CAN operation, this value is compared to both the Tx and Rx Error Counters. If either of these counter matches this value, the Error Status (ES) bit in CANSR is set."]
pub type EwlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - During CAN operation, this value is compared to both the Tx and Rx Error Counters. If either of these counter matches this value, the Error Status (ES) bit in CANSR is set."]
    #[inline(always)]
    pub fn ewl(&self) -> EwlR {
        EwlR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - During CAN operation, this value is compared to both the Tx and Rx Error Counters. If either of these counter matches this value, the Error Status (ES) bit in CANSR is set."]
    #[inline(always)]
    pub fn ewl(&mut self) -> EwlW<'_, EwlSpec> {
        EwlW::new(self, 0)
    }
}
#[doc = "Error Warning Limit. Can only be written when RM in CANMOD is 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`ewl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ewl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EwlSpec;
impl crate::RegisterSpec for EwlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ewl::R`](R) reader structure"]
impl crate::Readable for EwlSpec {}
#[doc = "`write(|w| ..)` method takes [`ewl::W`](W) writer structure"]
impl crate::Writable for EwlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EWL to value 0x60"]
impl crate::Resettable for EwlSpec {
    const RESET_VALUE: u32 = 0x60;
}
