#[doc = "Register `MSR` reader"]
pub type R = crate::R<MsrSpec>;
#[doc = "Field `E1` reader - When 1, one or both of the CAN1 Tx and Rx Error Counters has reached the limit set in the CAN1EWL register (same as ES in CAN1GSR)"]
pub type E1R = crate::BitReader;
#[doc = "Field `E2` reader - When 1, one or both of the CAN2 Tx and Rx Error Counters has reached the limit set in the CAN2EWL register (same as ES in CAN2GSR)"]
pub type E2R = crate::BitReader;
#[doc = "Field `BS1` reader - When 1, the CAN1 controller is currently involved in bus activities (same as BS in CAN1GSR)."]
pub type Bs1R = crate::BitReader;
#[doc = "Field `BS2` reader - When 1, the CAN2 controller is currently involved in bus activities (same as BS in CAN2GSR)."]
pub type Bs2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - When 1, one or both of the CAN1 Tx and Rx Error Counters has reached the limit set in the CAN1EWL register (same as ES in CAN1GSR)"]
    #[inline(always)]
    pub fn e1(&self) -> E1R {
        E1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, one or both of the CAN2 Tx and Rx Error Counters has reached the limit set in the CAN2EWL register (same as ES in CAN2GSR)"]
    #[inline(always)]
    pub fn e2(&self) -> E2R {
        E2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - When 1, the CAN1 controller is currently involved in bus activities (same as BS in CAN1GSR)."]
    #[inline(always)]
    pub fn bs1(&self) -> Bs1R {
        Bs1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When 1, the CAN2 controller is currently involved in bus activities (same as BS in CAN2GSR)."]
    #[inline(always)]
    pub fn bs2(&self) -> Bs2R {
        Bs2R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "CAN Central Miscellaneous Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MsrSpec;
impl crate::RegisterSpec for MsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`msr::R`](R) reader structure"]
impl crate::Readable for MsrSpec {}
#[doc = "`reset()` method sets MSR to value 0"]
impl crate::Resettable for MsrSpec {}
