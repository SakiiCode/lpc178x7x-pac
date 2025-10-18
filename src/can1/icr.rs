#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Receive Interrupt. This bit is set whenever the RBS bit in CANxSR and the RIE bit in CANxIER are both 1, indicating that a new message was received and stored in the Receive Buffer. The Receive Interrupt Bit is not cleared upon a read access to the Interrupt Register. Giving the Command Release Receive Buffer will clear RI temporarily. If there is another message available within the Receive Buffer after the release command, RI is set again. Otherwise RI remains cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Reset"]
    Reset = 0,
    #[doc = "1: Set"]
    Set = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RI` reader - Receive Interrupt. This bit is set whenever the RBS bit in CANxSR and the RIE bit in CANxIER are both 1, indicating that a new message was received and stored in the Receive Buffer. The Receive Interrupt Bit is not cleared upon a read access to the Interrupt Register. Giving the Command Release Receive Buffer will clear RI temporarily. If there is another message available within the Receive Buffer after the release command, RI is set again. Otherwise RI remains cleared."]
pub type RiR = crate::BitReader<Enum>;
impl RiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Reset,
            true => Enum::Set,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Enum::Reset
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Enum::Set
    }
}
#[doc = "Transmit Interrupt 1. This bit is set when the TBS1 bit in CANxSR goes from 0 to 1 (whenever a message out of TXB1 was successfully transmitted or aborted), indicating that Transmit buffer 1 is available, and the TIE1 bit in CANxIER is 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Reset"]
    Reset = 0,
    #[doc = "1: Set"]
    Set = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TI1` reader - Transmit Interrupt 1. This bit is set when the TBS1 bit in CANxSR goes from 0 to 1 (whenever a message out of TXB1 was successfully transmitted or aborted), indicating that Transmit buffer 1 is available, and the TIE1 bit in CANxIER is 1."]
pub type Ti1R = crate::BitReader<Enum>;
impl Ti1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Reset,
            true => Enum::Set,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Enum::Reset
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Enum::Set
    }
}
#[doc = "Error Warning Interrupt. This bit is set on every change (set or clear) of either the Error Status or Bus Status bit in CANxSR and the EIE bit bit is set within the Interrupt Enable Register at the time of the change.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Reset"]
    Reset = 0,
    #[doc = "1: Set"]
    Set = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EI` reader - Error Warning Interrupt. This bit is set on every change (set or clear) of either the Error Status or Bus Status bit in CANxSR and the EIE bit bit is set within the Interrupt Enable Register at the time of the change."]
pub type EiR = crate::BitReader<Enum>;
impl EiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Reset,
            true => Enum::Set,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Enum::Reset
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Enum::Set
    }
}
#[doc = "Data Overrun Interrupt. This bit is set when the DOS bit in CANxSR goes from 0 to 1 and the DOIE bit in CANxIER is 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Reset"]
    Reset = 0,
    #[doc = "1: Set"]
    Set = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOI` reader - Data Overrun Interrupt. This bit is set when the DOS bit in CANxSR goes from 0 to 1 and the DOIE bit in CANxIER is 1."]
pub type DoiR = crate::BitReader<Enum>;
impl DoiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Reset,
            true => Enum::Set,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Enum::Reset
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Enum::Set
    }
}
#[doc = "Wake-Up Interrupt. This bit is set if the CAN controller is sleeping and bus activity is detected and the WUIE bit in CANxIER is 1. A Wake-Up Interrupt is also generated if the CPU tries to set the Sleep bit while the CAN controller is involved in bus activities or a CAN Interrupt is pending. The WUI flag can also get asserted when the according enable bit WUIE is not set. In this case a Wake-Up Interrupt does not get asserted.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Reset"]
    Reset = 0,
    #[doc = "1: Set"]
    Set = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WUI` reader - Wake-Up Interrupt. This bit is set if the CAN controller is sleeping and bus activity is detected and the WUIE bit in CANxIER is 1. A Wake-Up Interrupt is also generated if the CPU tries to set the Sleep bit while the CAN controller is involved in bus activities or a CAN Interrupt is pending. The WUI flag can also get asserted when the according enable bit WUIE is not set. In this case a Wake-Up Interrupt does not get asserted."]
pub type WuiR = crate::BitReader<Enum>;
impl WuiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Reset,
            true => Enum::Set,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Enum::Reset
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Enum::Set
    }
}
#[doc = "Error Passive Interrupt. This bit is set if the EPIE bit in CANxIER is 1, and the CAN controller switches between Error Passive and Error Active mode in either direction. This is the case when the CAN Controller has reached the Error Passive Status (at least one error counter exceeds the CAN protocol defined level of 127) or if the CAN Controller is in Error Passive Status and enters the Error Active Status again.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Reset"]
    Reset = 0,
    #[doc = "1: Set"]
    Set = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EPI` reader - Error Passive Interrupt. This bit is set if the EPIE bit in CANxIER is 1, and the CAN controller switches between Error Passive and Error Active mode in either direction. This is the case when the CAN Controller has reached the Error Passive Status (at least one error counter exceeds the CAN protocol defined level of 127) or if the CAN Controller is in Error Passive Status and enters the Error Active Status again."]
pub type EpiR = crate::BitReader<Enum>;
impl EpiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Reset,
            true => Enum::Set,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Enum::Reset
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Enum::Set
    }
}
#[doc = "Arbitration Lost Interrupt. This bit is set if the ALIE bit in CANxIER is 1, and the CAN controller loses arbitration while attempting to transmit. In this case the CAN node becomes a receiver.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Reset"]
    Reset = 0,
    #[doc = "1: Set"]
    Set = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ALI` reader - Arbitration Lost Interrupt. This bit is set if the ALIE bit in CANxIER is 1, and the CAN controller loses arbitration while attempting to transmit. In this case the CAN node becomes a receiver."]
pub type AliR = crate::BitReader<Enum>;
impl AliR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Reset,
            true => Enum::Set,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Enum::Reset
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Enum::Set
    }
}
#[doc = "Bus Error Interrupt -- this bit is set if the BEIE bit in CANxIER is 1, and the CAN controller detects an error on the bus.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Reset"]
    Reset = 0,
    #[doc = "1: Set"]
    Set = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BEI` reader - Bus Error Interrupt -- this bit is set if the BEIE bit in CANxIER is 1, and the CAN controller detects an error on the bus."]
pub type BeiR = crate::BitReader<Enum>;
impl BeiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Reset,
            true => Enum::Set,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Enum::Reset
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Enum::Set
    }
}
#[doc = "ID Ready Interrupt -- this bit is set if the IDIE bit in CANxIER is 1, and a CAN Identifier has been received (a message was successfully transmitted or aborted). This bit is set whenever a message was successfully transmitted or aborted and the IDIE bit is set in the IER register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Reset"]
    Reset = 0,
    #[doc = "1: Set"]
    Set = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IDI` reader - ID Ready Interrupt -- this bit is set if the IDIE bit in CANxIER is 1, and a CAN Identifier has been received (a message was successfully transmitted or aborted). This bit is set whenever a message was successfully transmitted or aborted and the IDIE bit is set in the IER register."]
pub type IdiR = crate::BitReader<Enum>;
impl IdiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Reset,
            true => Enum::Set,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Enum::Reset
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Enum::Set
    }
}
#[doc = "Transmit Interrupt 2. This bit is set when the TBS2 bit in CANxSR goes from 0 to 1 (whenever a message out of TXB2 was successfully transmitted or aborted), indicating that Transmit buffer 2 is available, and the TIE2 bit in CANxIER is 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Reset"]
    Reset = 0,
    #[doc = "1: Set"]
    Set = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TI2` reader - Transmit Interrupt 2. This bit is set when the TBS2 bit in CANxSR goes from 0 to 1 (whenever a message out of TXB2 was successfully transmitted or aborted), indicating that Transmit buffer 2 is available, and the TIE2 bit in CANxIER is 1."]
pub type Ti2R = crate::BitReader<Enum>;
impl Ti2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Reset,
            true => Enum::Set,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Enum::Reset
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Enum::Set
    }
}
#[doc = "Transmit Interrupt 3. This bit is set when the TBS3 bit in CANxSR goes from 0 to 1 (whenever a message out of TXB3 was successfully transmitted or aborted), indicating that Transmit buffer 3 is available, and the TIE3 bit in CANxIER is 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Reset"]
    Reset = 0,
    #[doc = "1: Set"]
    Set = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TI3` reader - Transmit Interrupt 3. This bit is set when the TBS3 bit in CANxSR goes from 0 to 1 (whenever a message out of TXB3 was successfully transmitted or aborted), indicating that Transmit buffer 3 is available, and the TIE3 bit in CANxIER is 1."]
pub type Ti3R = crate::BitReader<Enum>;
impl Ti3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Reset,
            true => Enum::Set,
        }
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Enum::Reset
    }
    #[doc = "Set"]
    #[inline(always)]
    pub fn is_set(&self) -> bool {
        *self == Enum::Set
    }
}
#[doc = "Field `ERRBIT4_0` reader - Error Code Capture: when the CAN controller detects a bus error, the location of the error within the frame is captured in this field. The value reflects an internal state variable, and as a result is not very linear: 00011 = Start of Frame 00010 = ID28 ... ID21 00110 = ID20 ... ID18 00100 = SRTR Bit 00101 = IDE bit 00111 = ID17 ... 13 01111 = ID12 ... ID5 01110 = ID4 ... ID0 01100 = RTR Bit 01101 = Reserved Bit 1 01001 = Reserved Bit 0 01011 = Data Length Code 01010 = Data Field 01000 = CRC Sequence 11000 = CRC Delimiter 11001 = Acknowledge Slot 11011 = Acknowledge Delimiter 11010 = End of Frame 10010 = Intermission Whenever a bus error occurs, the corresponding bus error interrupt is forced, if enabled. At the same time, the current position of the Bit Stream Processor is captured into the Error Code Capture Register. The content within this register is fixed until the user software has read out its content once. From now on, the capture mechanism is activated again, i.e. reading the CANxICR enables another Bus Error Interrupt."]
pub type Errbit4_0R = crate::FieldReader;
#[doc = "When the CAN controller detects a bus error, the direction of the current bit is captured in this bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Error occurred during transmitting."]
    ErrorOccurredDurin = 0,
    #[doc = "1: Error occurred during receiving."]
    ErrorOccurredDurin = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERRDIR` reader - When the CAN controller detects a bus error, the direction of the current bit is captured in this bit."]
pub type ErrdirR = crate::BitReader<Enum>;
impl ErrdirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::ErrorOccurredDurin,
            true => Enum::ErrorOccurredDurin,
        }
    }
    #[doc = "Error occurred during transmitting."]
    #[inline(always)]
    pub fn is_error_occurred_durin(&self) -> bool {
        *self == Enum::ErrorOccurredDurin
    }
    #[doc = "Error occurred during receiving."]
    #[inline(always)]
    pub fn is_error_occurred_durin(&self) -> bool {
        *self == Enum::ErrorOccurredDurin
    }
}
#[doc = "When the CAN controller detects a bus error, the type of error is captured in this field:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Enum {
    #[doc = "0: Bit error"]
    BitError = 0,
    #[doc = "1: Form error"]
    FormError = 1,
    #[doc = "2: Stuff error"]
    StuffError = 2,
    #[doc = "3: Other error"]
    OtherError = 3,
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
#[doc = "Field `ERRC1_0` reader - When the CAN controller detects a bus error, the type of error is captured in this field:"]
pub type Errc1_0R = crate::FieldReader<Enum>;
impl Errc1_0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            0 => Enum::BitError,
            1 => Enum::FormError,
            2 => Enum::StuffError,
            3 => Enum::OtherError,
            _ => unreachable!(),
        }
    }
    #[doc = "Bit error"]
    #[inline(always)]
    pub fn is_bit_error(&self) -> bool {
        *self == Enum::BitError
    }
    #[doc = "Form error"]
    #[inline(always)]
    pub fn is_form_error(&self) -> bool {
        *self == Enum::FormError
    }
    #[doc = "Stuff error"]
    #[inline(always)]
    pub fn is_stuff_error(&self) -> bool {
        *self == Enum::StuffError
    }
    #[doc = "Other error"]
    #[inline(always)]
    pub fn is_other_error(&self) -> bool {
        *self == Enum::OtherError
    }
}
#[doc = "Field `ALCBIT` reader - Each time arbitration is lost while trying to send on the CAN, the bit number within the frame is captured into this field. After the content of ALCBIT is read, the ALI bit is cleared and a new Arbitration Lost interrupt can occur. 00 = arbitration lost in the first bit (MS) of identifier ... 11 = arbitration lost in SRTS bit (RTR bit for standard frame messages) 12 = arbitration lost in IDE bit 13 = arbitration lost in 12th bit of identifier (extended frame only) ... 30 = arbitration lost in last bit of identifier (extended frame only) 31 = arbitration lost in RTR bit (extended frame only) On arbitration lost, the corresponding arbitration lost interrupt is forced, if enabled. At that time, the current bit position of the Bit Stream Processor is captured into the Arbitration Lost Capture Register. The content within this register is fixed until the user application has read out its contents once. From now on, the capture mechanism is activated again."]
pub type AlcbitR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Receive Interrupt. This bit is set whenever the RBS bit in CANxSR and the RIE bit in CANxIER are both 1, indicating that a new message was received and stored in the Receive Buffer. The Receive Interrupt Bit is not cleared upon a read access to the Interrupt Register. Giving the Command Release Receive Buffer will clear RI temporarily. If there is another message available within the Receive Buffer after the release command, RI is set again. Otherwise RI remains cleared."]
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Interrupt 1. This bit is set when the TBS1 bit in CANxSR goes from 0 to 1 (whenever a message out of TXB1 was successfully transmitted or aborted), indicating that Transmit buffer 1 is available, and the TIE1 bit in CANxIER is 1."]
    #[inline(always)]
    pub fn ti1(&self) -> Ti1R {
        Ti1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error Warning Interrupt. This bit is set on every change (set or clear) of either the Error Status or Bus Status bit in CANxSR and the EIE bit bit is set within the Interrupt Enable Register at the time of the change."]
    #[inline(always)]
    pub fn ei(&self) -> EiR {
        EiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Overrun Interrupt. This bit is set when the DOS bit in CANxSR goes from 0 to 1 and the DOIE bit in CANxIER is 1."]
    #[inline(always)]
    pub fn doi(&self) -> DoiR {
        DoiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake-Up Interrupt. This bit is set if the CAN controller is sleeping and bus activity is detected and the WUIE bit in CANxIER is 1. A Wake-Up Interrupt is also generated if the CPU tries to set the Sleep bit while the CAN controller is involved in bus activities or a CAN Interrupt is pending. The WUI flag can also get asserted when the according enable bit WUIE is not set. In this case a Wake-Up Interrupt does not get asserted."]
    #[inline(always)]
    pub fn wui(&self) -> WuiR {
        WuiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error Passive Interrupt. This bit is set if the EPIE bit in CANxIER is 1, and the CAN controller switches between Error Passive and Error Active mode in either direction. This is the case when the CAN Controller has reached the Error Passive Status (at least one error counter exceeds the CAN protocol defined level of 127) or if the CAN Controller is in Error Passive Status and enters the Error Active Status again."]
    #[inline(always)]
    pub fn epi(&self) -> EpiR {
        EpiR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Arbitration Lost Interrupt. This bit is set if the ALIE bit in CANxIER is 1, and the CAN controller loses arbitration while attempting to transmit. In this case the CAN node becomes a receiver."]
    #[inline(always)]
    pub fn ali(&self) -> AliR {
        AliR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus Error Interrupt -- this bit is set if the BEIE bit in CANxIER is 1, and the CAN controller detects an error on the bus."]
    #[inline(always)]
    pub fn bei(&self) -> BeiR {
        BeiR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ID Ready Interrupt -- this bit is set if the IDIE bit in CANxIER is 1, and a CAN Identifier has been received (a message was successfully transmitted or aborted). This bit is set whenever a message was successfully transmitted or aborted and the IDIE bit is set in the IER register."]
    #[inline(always)]
    pub fn idi(&self) -> IdiR {
        IdiR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit Interrupt 2. This bit is set when the TBS2 bit in CANxSR goes from 0 to 1 (whenever a message out of TXB2 was successfully transmitted or aborted), indicating that Transmit buffer 2 is available, and the TIE2 bit in CANxIER is 1."]
    #[inline(always)]
    pub fn ti2(&self) -> Ti2R {
        Ti2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit Interrupt 3. This bit is set when the TBS3 bit in CANxSR goes from 0 to 1 (whenever a message out of TXB3 was successfully transmitted or aborted), indicating that Transmit buffer 3 is available, and the TIE3 bit in CANxIER is 1."]
    #[inline(always)]
    pub fn ti3(&self) -> Ti3R {
        Ti3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Error Code Capture: when the CAN controller detects a bus error, the location of the error within the frame is captured in this field. The value reflects an internal state variable, and as a result is not very linear: 00011 = Start of Frame 00010 = ID28 ... ID21 00110 = ID20 ... ID18 00100 = SRTR Bit 00101 = IDE bit 00111 = ID17 ... 13 01111 = ID12 ... ID5 01110 = ID4 ... ID0 01100 = RTR Bit 01101 = Reserved Bit 1 01001 = Reserved Bit 0 01011 = Data Length Code 01010 = Data Field 01000 = CRC Sequence 11000 = CRC Delimiter 11001 = Acknowledge Slot 11011 = Acknowledge Delimiter 11010 = End of Frame 10010 = Intermission Whenever a bus error occurs, the corresponding bus error interrupt is forced, if enabled. At the same time, the current position of the Bit Stream Processor is captured into the Error Code Capture Register. The content within this register is fixed until the user software has read out its content once. From now on, the capture mechanism is activated again, i.e. reading the CANxICR enables another Bus Error Interrupt."]
    #[inline(always)]
    pub fn errbit4_0(&self) -> Errbit4_0R {
        Errbit4_0R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - When the CAN controller detects a bus error, the direction of the current bit is captured in this bit."]
    #[inline(always)]
    pub fn errdir(&self) -> ErrdirR {
        ErrdirR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - When the CAN controller detects a bus error, the type of error is captured in this field:"]
    #[inline(always)]
    pub fn errc1_0(&self) -> Errc1_0R {
        Errc1_0R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:31 - Each time arbitration is lost while trying to send on the CAN, the bit number within the frame is captured into this field. After the content of ALCBIT is read, the ALI bit is cleared and a new Arbitration Lost interrupt can occur. 00 = arbitration lost in the first bit (MS) of identifier ... 11 = arbitration lost in SRTS bit (RTR bit for standard frame messages) 12 = arbitration lost in IDE bit 13 = arbitration lost in 12th bit of identifier (extended frame only) ... 30 = arbitration lost in last bit of identifier (extended frame only) 31 = arbitration lost in RTR bit (extended frame only) On arbitration lost, the corresponding arbitration lost interrupt is forced, if enabled. At that time, the current bit position of the Bit Stream Processor is captured into the Arbitration Lost Capture Register. The content within this register is fixed until the user application has read out its contents once. From now on, the capture mechanism is activated again."]
    #[inline(always)]
    pub fn alcbit(&self) -> AlcbitR {
        AlcbitR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Interrupt status, Arbitration Lost Capture, Error Code Capture\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for IcrSpec {}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}
