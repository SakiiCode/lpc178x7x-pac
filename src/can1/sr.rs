#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `RBS_1` reader - Receive Buffer Status. This bit is identical to the RBS bit in the CANxGSR."]
pub type Rbs1R = crate::BitReader;
#[doc = "Field `DOS_1` reader - Data Overrun Status. This bit is identical to the DOS bit in the CANxGSR."]
pub type Dos1R = crate::BitReader;
#[doc = "Transmit Buffer Status 1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Locked. Software cannot access the Tx Buffer 1 nor write to the corresponding CANxTFI, CANxTID, CANxTDA, and CANxTDB registers because a message is either waiting for transmission or is in transmitting process."]
    LockedSoftwareCan = 0,
    #[doc = "1: Released. Software may write a message into the Transmit Buffer 1 and its CANxTFI, CANxTID, CANxTDA, and CANxTDB registers."]
    ReleasedSoftwareM = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBS1_1` reader - Transmit Buffer Status 1."]
pub type Tbs1_1R = crate::BitReader<Enum>;
impl Tbs1_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::LockedSoftwareCan,
            true => Enum::ReleasedSoftwareM,
        }
    }
    #[doc = "Locked. Software cannot access the Tx Buffer 1 nor write to the corresponding CANxTFI, CANxTID, CANxTDA, and CANxTDB registers because a message is either waiting for transmission or is in transmitting process."]
    #[inline(always)]
    pub fn is_locked_software_can(&self) -> bool {
        *self == Enum::LockedSoftwareCan
    }
    #[doc = "Released. Software may write a message into the Transmit Buffer 1 and its CANxTFI, CANxTID, CANxTDA, and CANxTDB registers."]
    #[inline(always)]
    pub fn is_released_software_m(&self) -> bool {
        *self == Enum::ReleasedSoftwareM
    }
}
#[doc = "Transmission Complete Status.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Incomplete. The previously requested transmission for Tx Buffer 1 is not complete."]
    IncompleteThePrev = 0,
    #[doc = "1: Complete. The previously requested transmission for Tx Buffer 1 has been successfully completed."]
    CompleteThePrevio = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCS1_1` reader - Transmission Complete Status."]
pub type Tcs1_1R = crate::BitReader<Enum>;
impl Tcs1_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::IncompleteThePrev,
            true => Enum::CompleteThePrevio,
        }
    }
    #[doc = "Incomplete. The previously requested transmission for Tx Buffer 1 is not complete."]
    #[inline(always)]
    pub fn is_incomplete_the_prev(&self) -> bool {
        *self == Enum::IncompleteThePrev
    }
    #[doc = "Complete. The previously requested transmission for Tx Buffer 1 has been successfully completed."]
    #[inline(always)]
    pub fn is_complete_the_previo(&self) -> bool {
        *self == Enum::CompleteThePrevio
    }
}
#[doc = "Field `RS_1` reader - Receive Status. This bit is identical to the RS bit in the GSR."]
pub type Rs1R = crate::BitReader;
#[doc = "Transmit Status 1.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Idle. There is no transmission from Tx Buffer 1."]
    IdleThereIsNoTr = 0,
    #[doc = "1: Transmit. The CAN Controller is transmitting a message from Tx Buffer 1."]
    TransmitTheCanCo = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS1_1` reader - Transmit Status 1."]
pub type Ts1_1R = crate::BitReader<Enum>;
impl Ts1_1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::IdleThereIsNoTr,
            true => Enum::TransmitTheCanCo,
        }
    }
    #[doc = "Idle. There is no transmission from Tx Buffer 1."]
    #[inline(always)]
    pub fn is_idle_there_is_no_tr(&self) -> bool {
        *self == Enum::IdleThereIsNoTr
    }
    #[doc = "Transmit. The CAN Controller is transmitting a message from Tx Buffer 1."]
    #[inline(always)]
    pub fn is_transmit_the_can_co(&self) -> bool {
        *self == Enum::TransmitTheCanCo
    }
}
#[doc = "Field `ES_1` reader - Error Status. This bit is identical to the ES bit in the CANxGSR."]
pub type Es1R = crate::BitReader;
#[doc = "Field `BS_1` reader - Bus Status. This bit is identical to the BS bit in the CANxGSR."]
pub type Bs1R = crate::BitReader;
#[doc = "Field `RBS_2` reader - Receive Buffer Status. This bit is identical to the RBS bit in the CANxGSR."]
pub type Rbs2R = crate::BitReader;
#[doc = "Field `DOS_2` reader - Data Overrun Status. This bit is identical to the DOS bit in the CANxGSR."]
pub type Dos2R = crate::BitReader;
#[doc = "Transmit Buffer Status 2.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Locked. Software cannot access the Tx Buffer 2 nor write to the corresponding CANxTFI, CANxTID, CANxTDA, and CANxTDB registers because a message is either waiting for transmission or is in transmitting process."]
    LockedSoftwareCan = 0,
    #[doc = "1: Released. Software may write a message into the Transmit Buffer 2 and its CANxTFI, CANxTID, CANxTDA, and CANxTDB registers."]
    ReleasedSoftwareM = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBS2_2` reader - Transmit Buffer Status 2."]
pub type Tbs2_2R = crate::BitReader<Enum>;
impl Tbs2_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::LockedSoftwareCan,
            true => Enum::ReleasedSoftwareM,
        }
    }
    #[doc = "Locked. Software cannot access the Tx Buffer 2 nor write to the corresponding CANxTFI, CANxTID, CANxTDA, and CANxTDB registers because a message is either waiting for transmission or is in transmitting process."]
    #[inline(always)]
    pub fn is_locked_software_can(&self) -> bool {
        *self == Enum::LockedSoftwareCan
    }
    #[doc = "Released. Software may write a message into the Transmit Buffer 2 and its CANxTFI, CANxTID, CANxTDA, and CANxTDB registers."]
    #[inline(always)]
    pub fn is_released_software_m(&self) -> bool {
        *self == Enum::ReleasedSoftwareM
    }
}
#[doc = "Transmission Complete Status.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Incomplete. The previously requested transmission for Tx Buffer 2 is not complete."]
    IncompleteThePrev = 0,
    #[doc = "1: Complete. The previously requested transmission for Tx Buffer 2 has been successfully completed."]
    CompleteThePrevio = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCS2_2` reader - Transmission Complete Status."]
pub type Tcs2_2R = crate::BitReader<Enum>;
impl Tcs2_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::IncompleteThePrev,
            true => Enum::CompleteThePrevio,
        }
    }
    #[doc = "Incomplete. The previously requested transmission for Tx Buffer 2 is not complete."]
    #[inline(always)]
    pub fn is_incomplete_the_prev(&self) -> bool {
        *self == Enum::IncompleteThePrev
    }
    #[doc = "Complete. The previously requested transmission for Tx Buffer 2 has been successfully completed."]
    #[inline(always)]
    pub fn is_complete_the_previo(&self) -> bool {
        *self == Enum::CompleteThePrevio
    }
}
#[doc = "Field `RS_2` reader - Receive Status. This bit is identical to the RS bit in the GSR."]
pub type Rs2R = crate::BitReader;
#[doc = "Transmit Status 2.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Idle. There is no transmission from Tx Buffer 2."]
    IdleThereIsNoTr = 0,
    #[doc = "1: Transmit. The CAN Controller is transmitting a message from Tx Buffer 2."]
    TransmitTheCanCo = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS2_2` reader - Transmit Status 2."]
pub type Ts2_2R = crate::BitReader<Enum>;
impl Ts2_2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::IdleThereIsNoTr,
            true => Enum::TransmitTheCanCo,
        }
    }
    #[doc = "Idle. There is no transmission from Tx Buffer 2."]
    #[inline(always)]
    pub fn is_idle_there_is_no_tr(&self) -> bool {
        *self == Enum::IdleThereIsNoTr
    }
    #[doc = "Transmit. The CAN Controller is transmitting a message from Tx Buffer 2."]
    #[inline(always)]
    pub fn is_transmit_the_can_co(&self) -> bool {
        *self == Enum::TransmitTheCanCo
    }
}
#[doc = "Field `ES_2` reader - Error Status. This bit is identical to the ES bit in the CANxGSR."]
pub type Es2R = crate::BitReader;
#[doc = "Field `BS_2` reader - Bus Status. This bit is identical to the BS bit in the CANxGSR."]
pub type Bs2R = crate::BitReader;
#[doc = "Field `RBS_3` reader - Receive Buffer Status. This bit is identical to the RBS bit in the CANxGSR."]
pub type Rbs3R = crate::BitReader;
#[doc = "Field `DOS_3` reader - Data Overrun Status. This bit is identical to the DOS bit in the CANxGSR."]
pub type Dos3R = crate::BitReader;
#[doc = "Transmit Buffer Status 3.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Locked. Software cannot access the Tx Buffer 3 nor write to the corresponding CANxTFI, CANxTID, CANxTDA, and CANxTDB registers because a message is either waiting for transmission or is in transmitting process."]
    LockedSoftwareCan = 0,
    #[doc = "1: Released. Software may write a message into the Transmit Buffer 3 and its CANxTFI, CANxTID, CANxTDA, and CANxTDB registers."]
    ReleasedSoftwareM = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBS3_3` reader - Transmit Buffer Status 3."]
pub type Tbs3_3R = crate::BitReader<Enum>;
impl Tbs3_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::LockedSoftwareCan,
            true => Enum::ReleasedSoftwareM,
        }
    }
    #[doc = "Locked. Software cannot access the Tx Buffer 3 nor write to the corresponding CANxTFI, CANxTID, CANxTDA, and CANxTDB registers because a message is either waiting for transmission or is in transmitting process."]
    #[inline(always)]
    pub fn is_locked_software_can(&self) -> bool {
        *self == Enum::LockedSoftwareCan
    }
    #[doc = "Released. Software may write a message into the Transmit Buffer 3 and its CANxTFI, CANxTID, CANxTDA, and CANxTDB registers."]
    #[inline(always)]
    pub fn is_released_software_m(&self) -> bool {
        *self == Enum::ReleasedSoftwareM
    }
}
#[doc = "Transmission Complete Status.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Incomplete. The previously requested transmission for Tx Buffer 3 is not complete."]
    IncompleteThePrev = 0,
    #[doc = "1: Complete. The previously requested transmission for Tx Buffer 3 has been successfully completed."]
    CompleteThePrevio = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCS3_3` reader - Transmission Complete Status."]
pub type Tcs3_3R = crate::BitReader<Enum>;
impl Tcs3_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::IncompleteThePrev,
            true => Enum::CompleteThePrevio,
        }
    }
    #[doc = "Incomplete. The previously requested transmission for Tx Buffer 3 is not complete."]
    #[inline(always)]
    pub fn is_incomplete_the_prev(&self) -> bool {
        *self == Enum::IncompleteThePrev
    }
    #[doc = "Complete. The previously requested transmission for Tx Buffer 3 has been successfully completed."]
    #[inline(always)]
    pub fn is_complete_the_previo(&self) -> bool {
        *self == Enum::CompleteThePrevio
    }
}
#[doc = "Field `RS_3` reader - Receive Status. This bit is identical to the RS bit in the GSR."]
pub type Rs3R = crate::BitReader;
#[doc = "Transmit Status 3.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Idle. There is no transmission from Tx Buffer 3."]
    IdleThereIsNoTr = 0,
    #[doc = "1: Transmit. The CAN Controller is transmitting a message from Tx Buffer 3."]
    TransmitTheCanCo = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS3_3` reader - Transmit Status 3."]
pub type Ts3_3R = crate::BitReader<Enum>;
impl Ts3_3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::IdleThereIsNoTr,
            true => Enum::TransmitTheCanCo,
        }
    }
    #[doc = "Idle. There is no transmission from Tx Buffer 3."]
    #[inline(always)]
    pub fn is_idle_there_is_no_tr(&self) -> bool {
        *self == Enum::IdleThereIsNoTr
    }
    #[doc = "Transmit. The CAN Controller is transmitting a message from Tx Buffer 3."]
    #[inline(always)]
    pub fn is_transmit_the_can_co(&self) -> bool {
        *self == Enum::TransmitTheCanCo
    }
}
#[doc = "Field `ES_3` reader - Error Status. This bit is identical to the ES bit in the CANxGSR."]
pub type Es3R = crate::BitReader;
#[doc = "Field `BS_3` reader - Bus Status. This bit is identical to the BS bit in the CANxGSR."]
pub type Bs3R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Receive Buffer Status. This bit is identical to the RBS bit in the CANxGSR."]
    #[inline(always)]
    pub fn rbs_1(&self) -> Rbs1R {
        Rbs1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Overrun Status. This bit is identical to the DOS bit in the CANxGSR."]
    #[inline(always)]
    pub fn dos_1(&self) -> Dos1R {
        Dos1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Status 1."]
    #[inline(always)]
    pub fn tbs1_1(&self) -> Tbs1_1R {
        Tbs1_1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmission Complete Status."]
    #[inline(always)]
    pub fn tcs1_1(&self) -> Tcs1_1R {
        Tcs1_1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Status. This bit is identical to the RS bit in the GSR."]
    #[inline(always)]
    pub fn rs_1(&self) -> Rs1R {
        Rs1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Status 1."]
    #[inline(always)]
    pub fn ts1_1(&self) -> Ts1_1R {
        Ts1_1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Error Status. This bit is identical to the ES bit in the CANxGSR."]
    #[inline(always)]
    pub fn es_1(&self) -> Es1R {
        Es1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus Status. This bit is identical to the BS bit in the CANxGSR."]
    #[inline(always)]
    pub fn bs_1(&self) -> Bs1R {
        Bs1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Buffer Status. This bit is identical to the RBS bit in the CANxGSR."]
    #[inline(always)]
    pub fn rbs_2(&self) -> Rbs2R {
        Rbs2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data Overrun Status. This bit is identical to the DOS bit in the CANxGSR."]
    #[inline(always)]
    pub fn dos_2(&self) -> Dos2R {
        Dos2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit Buffer Status 2."]
    #[inline(always)]
    pub fn tbs2_2(&self) -> Tbs2_2R {
        Tbs2_2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmission Complete Status."]
    #[inline(always)]
    pub fn tcs2_2(&self) -> Tcs2_2R {
        Tcs2_2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Receive Status. This bit is identical to the RS bit in the GSR."]
    #[inline(always)]
    pub fn rs_2(&self) -> Rs2R {
        Rs2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Transmit Status 2."]
    #[inline(always)]
    pub fn ts2_2(&self) -> Ts2_2R {
        Ts2_2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Error Status. This bit is identical to the ES bit in the CANxGSR."]
    #[inline(always)]
    pub fn es_2(&self) -> Es2R {
        Es2R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Bus Status. This bit is identical to the BS bit in the CANxGSR."]
    #[inline(always)]
    pub fn bs_2(&self) -> Bs2R {
        Bs2R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Receive Buffer Status. This bit is identical to the RBS bit in the CANxGSR."]
    #[inline(always)]
    pub fn rbs_3(&self) -> Rbs3R {
        Rbs3R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Data Overrun Status. This bit is identical to the DOS bit in the CANxGSR."]
    #[inline(always)]
    pub fn dos_3(&self) -> Dos3R {
        Dos3R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Transmit Buffer Status 3."]
    #[inline(always)]
    pub fn tbs3_3(&self) -> Tbs3_3R {
        Tbs3_3R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Transmission Complete Status."]
    #[inline(always)]
    pub fn tcs3_3(&self) -> Tcs3_3R {
        Tcs3_3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Receive Status. This bit is identical to the RS bit in the GSR."]
    #[inline(always)]
    pub fn rs_3(&self) -> Rs3R {
        Rs3R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Transmit Status 3."]
    #[inline(always)]
    pub fn ts3_3(&self) -> Ts3_3R {
        Ts3_3R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Error Status. This bit is identical to the ES bit in the CANxGSR."]
    #[inline(always)]
    pub fn es_3(&self) -> Es3R {
        Es3R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Bus Status. This bit is identical to the BS bit in the CANxGSR."]
    #[inline(always)]
    pub fn bs_3(&self) -> Bs3R {
        Bs3R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0x003c_3c3c"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0x003c_3c3c;
}
