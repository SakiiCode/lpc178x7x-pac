#[doc = "Register `TXSR` reader"]
pub type R = crate::R<TxsrSpec>;
#[doc = "Field `TS1` reader - When 1, the CAN controller 1 is sending a message (same as TS in the CAN1GSR)."]
pub type Ts1R = crate::BitReader;
#[doc = "Field `TS2` reader - When 1, the CAN controller 2 is sending a message (same as TS in the CAN2GSR)"]
pub type Ts2R = crate::BitReader;
#[doc = "Field `TBS1` reader - When 1, all 3 Tx Buffers of the CAN1 controller are available to the CPU (same as TBS in CAN1GSR)."]
pub type Tbs1R = crate::BitReader;
#[doc = "Field `TBS2` reader - When 1, all 3 Tx Buffers of the CAN2 controller are available to the CPU (same as TBS in CAN2GSR)."]
pub type Tbs2R = crate::BitReader;
#[doc = "Field `TCS1` reader - When 1, all requested transmissions have been completed successfully by the CAN1 controller (same as TCS in CAN1GSR)."]
pub type Tcs1R = crate::BitReader;
#[doc = "Field `TCS2` reader - When 1, all requested transmissions have been completed successfully by the CAN2 controller (same as TCS in CAN2GSR)."]
pub type Tcs2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - When 1, the CAN controller 1 is sending a message (same as TS in the CAN1GSR)."]
    #[inline(always)]
    pub fn ts1(&self) -> Ts1R {
        Ts1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, the CAN controller 2 is sending a message (same as TS in the CAN2GSR)"]
    #[inline(always)]
    pub fn ts2(&self) -> Ts2R {
        Ts2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - When 1, all 3 Tx Buffers of the CAN1 controller are available to the CPU (same as TBS in CAN1GSR)."]
    #[inline(always)]
    pub fn tbs1(&self) -> Tbs1R {
        Tbs1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When 1, all 3 Tx Buffers of the CAN2 controller are available to the CPU (same as TBS in CAN2GSR)."]
    #[inline(always)]
    pub fn tbs2(&self) -> Tbs2R {
        Tbs2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 16 - When 1, all requested transmissions have been completed successfully by the CAN1 controller (same as TCS in CAN1GSR)."]
    #[inline(always)]
    pub fn tcs1(&self) -> Tcs1R {
        Tcs1R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - When 1, all requested transmissions have been completed successfully by the CAN2 controller (same as TCS in CAN2GSR)."]
    #[inline(always)]
    pub fn tcs2(&self) -> Tcs2R {
        Tcs2R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[doc = "CAN Central Transmit Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxsrSpec;
impl crate::RegisterSpec for TxsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txsr::R`](R) reader structure"]
impl crate::Readable for TxsrSpec {}
#[doc = "`reset()` method sets TXSR to value 0x0003_0300"]
impl crate::Resettable for TxsrSpec {
    const RESET_VALUE: u32 = 0x0003_0300;
}
