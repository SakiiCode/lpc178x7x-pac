#[doc = "Register `IIR` reader"]
pub type R = crate::R<IirSpec>;
#[doc = "Interrupt status. Note that UnIIR\\[0\\] is active low. The pending interrupt can be determined by evaluating UnIIR\\[3:1\\].\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: At least one interrupt is pending."]
    AtLeastOneInterru = 0,
    #[doc = "1: No interrupt is pending."]
    NoInterruptIsPend = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INTSTATUS` reader - Interrupt status. Note that UnIIR\\[0\\] is active low. The pending interrupt can be determined by evaluating UnIIR\\[3:1\\]."]
pub type IntstatusR = crate::BitReader<Enum>;
impl IntstatusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::AtLeastOneInterru,
            true => Enum::NoInterruptIsPend,
        }
    }
    #[doc = "At least one interrupt is pending."]
    #[inline(always)]
    pub fn is_at_least_one_interru(&self) -> bool {
        *self == Enum::AtLeastOneInterru
    }
    #[doc = "No interrupt is pending."]
    #[inline(always)]
    pub fn is_no_interrupt_is_pend(&self) -> bool {
        *self == Enum::NoInterruptIsPend
    }
}
#[doc = "Interrupt identification. UnIER\\[3:1\\] identifies an interrupt corresponding to the UARTn Rx or TX FIFO. All other combinations of UnIER\\[3:1\\] not listed below are reserved (000,100,101,111).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Enum {
    #[doc = "3: 1 - Receive Line Status (RLS)."]
    _1ReceiveLineS = 3,
    #[doc = "2: 2a - Receive Data Available (RDA)."]
    _2a_ReceiveDataAv = 2,
    #[doc = "6: 2b - Character Time-out Indicator (CTI)."]
    _2b_CharacterTime_ = 6,
    #[doc = "1: 3 - THRE Interrupt"]
    _3ThreInterrupt = 1,
}
impl From<Enum> for u8 {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Enum {
    type Ux = u8;
}
impl crate::IsEnum for Enum {}
#[doc = "Field `INTID` reader - Interrupt identification. UnIER\\[3:1\\] identifies an interrupt corresponding to the UARTn Rx or TX FIFO. All other combinations of UnIER\\[3:1\\] not listed below are reserved (000,100,101,111)."]
pub type IntidR = crate::FieldReader<Enum>;
impl IntidR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Enum> {
        match self.bits {
            3 => Some(Enum::_1ReceiveLineS),
            2 => Some(Enum::_2a_ReceiveDataAv),
            6 => Some(Enum::_2b_CharacterTime_),
            1 => Some(Enum::_3ThreInterrupt),
            _ => None,
        }
    }
    #[doc = "1 - Receive Line Status (RLS)."]
    #[inline(always)]
    pub fn is_1_receive_line_s(&self) -> bool {
        *self == Enum::_1ReceiveLineS
    }
    #[doc = "2a - Receive Data Available (RDA)."]
    #[inline(always)]
    pub fn is_2a__receive_data_av(&self) -> bool {
        *self == Enum::_2a_ReceiveDataAv
    }
    #[doc = "2b - Character Time-out Indicator (CTI)."]
    #[inline(always)]
    pub fn is_2b__character_time_(&self) -> bool {
        *self == Enum::_2b_CharacterTime_
    }
    #[doc = "3 - THRE Interrupt"]
    #[inline(always)]
    pub fn is_3_thre_interrupt(&self) -> bool {
        *self == Enum::_3ThreInterrupt
    }
}
#[doc = "Field `FIFOENABLE` reader - Copies of UnFCR\\[0\\]."]
pub type FifoenableR = crate::FieldReader;
#[doc = "Field `ABEOINT` reader - End of auto-baud interrupt. True if auto-baud has finished successfully and interrupt is enabled."]
pub type AbeointR = crate::BitReader;
#[doc = "Field `ABTOINT` reader - Auto-baud time-out interrupt. True if auto-baud has timed out and interrupt is enabled."]
pub type AbtointR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt status. Note that UnIIR\\[0\\] is active low. The pending interrupt can be determined by evaluating UnIIR\\[3:1\\]."]
    #[inline(always)]
    pub fn intstatus(&self) -> IntstatusR {
        IntstatusR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Interrupt identification. UnIER\\[3:1\\] identifies an interrupt corresponding to the UARTn Rx or TX FIFO. All other combinations of UnIER\\[3:1\\] not listed below are reserved (000,100,101,111)."]
    #[inline(always)]
    pub fn intid(&self) -> IntidR {
        IntidR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 6:7 - Copies of UnFCR\\[0\\]."]
    #[inline(always)]
    pub fn fifoenable(&self) -> FifoenableR {
        FifoenableR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - End of auto-baud interrupt. True if auto-baud has finished successfully and interrupt is enabled."]
    #[inline(always)]
    pub fn abeoint(&self) -> AbeointR {
        AbeointR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Auto-baud time-out interrupt. True if auto-baud has timed out and interrupt is enabled."]
    #[inline(always)]
    pub fn abtoint(&self) -> AbtointR {
        AbtointR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "Interrupt ID Register. Identifies which interrupt(s) are pending.\n\nYou can [`read`](crate::Reg::read) this register and get [`iir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IirSpec;
impl crate::RegisterSpec for IirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`iir::R`](R) reader structure"]
impl crate::Readable for IirSpec {}
#[doc = "`reset()` method sets IIR to value 0x01"]
impl crate::Resettable for IirSpec {
    const RESET_VALUE: u32 = 0x01;
}
