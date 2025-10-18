#[doc = "Register `RXSR` reader"]
pub type R = crate::R<RxsrSpec>;
#[doc = "Field `RS1` reader - When 1, CAN1 is receiving a message (same as RS in CAN1GSR)."]
pub type Rs1R = crate::BitReader;
#[doc = "Field `RS2` reader - When 1, CAN2 is receiving a message (same as RS in CAN2GSR)."]
pub type Rs2R = crate::BitReader;
#[doc = "Field `RB1` reader - When 1, a received message is available in the CAN1 controller (same as RBS in CAN1GSR)."]
pub type Rb1R = crate::BitReader;
#[doc = "Field `RB2` reader - When 1, a received message is available in the CAN2 controller (same as RBS in CAN2GSR)."]
pub type Rb2R = crate::BitReader;
#[doc = "Field `DOS1` reader - When 1, a message was lost because the preceding message to CAN1 controller was not read out quickly enough (same as DOS in CAN1GSR)."]
pub type Dos1R = crate::BitReader;
#[doc = "Field `DOS2` reader - When 1, a message was lost because the preceding message to CAN2 controller was not read out quickly enough (same as DOS in CAN2GSR)."]
pub type Dos2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - When 1, CAN1 is receiving a message (same as RS in CAN1GSR)."]
    #[inline(always)]
    pub fn rs1(&self) -> Rs1R {
        Rs1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, CAN2 is receiving a message (same as RS in CAN2GSR)."]
    #[inline(always)]
    pub fn rs2(&self) -> Rs2R {
        Rs2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - When 1, a received message is available in the CAN1 controller (same as RBS in CAN1GSR)."]
    #[inline(always)]
    pub fn rb1(&self) -> Rb1R {
        Rb1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When 1, a received message is available in the CAN2 controller (same as RBS in CAN2GSR)."]
    #[inline(always)]
    pub fn rb2(&self) -> Rb2R {
        Rb2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - When 1, a message was lost because the preceding message to CAN1 controller was not read out quickly enough (same as DOS in CAN1GSR)."]
    #[inline(always)]
    pub fn dos1(&self) -> Dos1R {
        Dos1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When 1, a message was lost because the preceding message to CAN2 controller was not read out quickly enough (same as DOS in CAN2GSR)."]
    #[inline(always)]
    pub fn dos2(&self) -> Dos2R {
        Dos2R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "CAN Central Receive Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxsrSpec;
impl crate::RegisterSpec for RxsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxsr::R`](R) reader structure"]
impl crate::Readable for RxsrSpec {}
#[doc = "`reset()` method sets RXSR to value 0"]
impl crate::Resettable for RxsrSpec {}
