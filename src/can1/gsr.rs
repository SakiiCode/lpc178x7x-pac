#[doc = "Register `GSR` reader"]
pub type R = crate::R<GsrSpec>;
#[doc = "Receive Buffer Status. After reading all messages and releasing their memory space with the command 'Release Receive Buffer,' this bit is cleared.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rbs {
    #[doc = "0: Empty. No message is available."]
    EmptyNoMessageIs = 0,
    #[doc = "1: Full. At least one complete message is received by the Double Receive Buffer and available in the CANxRFS, CANxRID, and if applicable the CANxRDA and CANxRDB registers. This bit is cleared by the Release Receive Buffer command in CANxCMR, if no subsequent received message is available."]
    FullAtLeastOneC = 1,
}
impl From<Rbs> for bool {
    #[inline(always)]
    fn from(variant: Rbs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBS` reader - Receive Buffer Status. After reading all messages and releasing their memory space with the command 'Release Receive Buffer,' this bit is cleared."]
pub type RbsR = crate::BitReader<Rbs>;
impl RbsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rbs {
        match self.bits {
            false => Rbs::EmptyNoMessageIs,
            true => Rbs::FullAtLeastOneC,
        }
    }
    #[doc = "Empty. No message is available."]
    #[inline(always)]
    pub fn is_empty_no_message_is(&self) -> bool {
        *self == Rbs::EmptyNoMessageIs
    }
    #[doc = "Full. At least one complete message is received by the Double Receive Buffer and available in the CANxRFS, CANxRID, and if applicable the CANxRDA and CANxRDB registers. This bit is cleared by the Release Receive Buffer command in CANxCMR, if no subsequent received message is available."]
    #[inline(always)]
    pub fn is_full_at_least_one_c(&self) -> bool {
        *self == Rbs::FullAtLeastOneC
    }
}
#[doc = "Data Overrun Status. If there is not enough space to store the message within the Receive Buffer, that message is dropped and the Data Overrun condition is signalled to the CPU in the moment this message becomes valid. If this message is not completed successfully (e.g. because of an error), no overrun condition is signalled.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dos {
    #[doc = "0: Absent. No data overrun has occurred since the last Clear Data Overrun command was given/written to CANxCMR (or since Reset)."]
    AbsentNoDataOver = 0,
    #[doc = "1: Overrun. A message was lost because the preceding message to this CAN controller was not read and released quickly enough (there was not enough space for a new message in the Double Receive Buffer)."]
    OverrunAMessageW = 1,
}
impl From<Dos> for bool {
    #[inline(always)]
    fn from(variant: Dos) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DOS` reader - Data Overrun Status. If there is not enough space to store the message within the Receive Buffer, that message is dropped and the Data Overrun condition is signalled to the CPU in the moment this message becomes valid. If this message is not completed successfully (e.g. because of an error), no overrun condition is signalled."]
pub type DosR = crate::BitReader<Dos>;
impl DosR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dos {
        match self.bits {
            false => Dos::AbsentNoDataOver,
            true => Dos::OverrunAMessageW,
        }
    }
    #[doc = "Absent. No data overrun has occurred since the last Clear Data Overrun command was given/written to CANxCMR (or since Reset)."]
    #[inline(always)]
    pub fn is_absent_no_data_over(&self) -> bool {
        *self == Dos::AbsentNoDataOver
    }
    #[doc = "Overrun. A message was lost because the preceding message to this CAN controller was not read and released quickly enough (there was not enough space for a new message in the Double Receive Buffer)."]
    #[inline(always)]
    pub fn is_overrun_a_message_w(&self) -> bool {
        *self == Dos::OverrunAMessageW
    }
}
#[doc = "Transmit Buffer Status.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tbs {
    #[doc = "0: Locked. At least one of the Transmit Buffers is not available for the CPU, i.e. at least one previously queued message for this CAN controller has not yet been sent, and therefore software should not write to the CANxTFI, CANxTID, CANxTDA, nor CANxTDB registers of that (those) Tx buffer(s)."]
    LockedAtLeastOne = 0,
    #[doc = "1: Released. All three Transmit Buffers are available for the CPU. No transmit message is pending for this CAN controller (in any of the 3 Tx buffers), and software may write to any of the CANxTFI, CANxTID, CANxTDA, and CANxTDB registers."]
    ReleasedAllThree_ = 1,
}
impl From<Tbs> for bool {
    #[inline(always)]
    fn from(variant: Tbs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TBS` reader - Transmit Buffer Status."]
pub type TbsR = crate::BitReader<Tbs>;
impl TbsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tbs {
        match self.bits {
            false => Tbs::LockedAtLeastOne,
            true => Tbs::ReleasedAllThree_,
        }
    }
    #[doc = "Locked. At least one of the Transmit Buffers is not available for the CPU, i.e. at least one previously queued message for this CAN controller has not yet been sent, and therefore software should not write to the CANxTFI, CANxTID, CANxTDA, nor CANxTDB registers of that (those) Tx buffer(s)."]
    #[inline(always)]
    pub fn is_locked_at_least_one(&self) -> bool {
        *self == Tbs::LockedAtLeastOne
    }
    #[doc = "Released. All three Transmit Buffers are available for the CPU. No transmit message is pending for this CAN controller (in any of the 3 Tx buffers), and software may write to any of the CANxTFI, CANxTID, CANxTDA, and CANxTDB registers."]
    #[inline(always)]
    pub fn is_released_all_three_(&self) -> bool {
        *self == Tbs::ReleasedAllThree_
    }
}
#[doc = "Transmit Complete Status. The Transmission Complete Status bit is set '0' (incomplete) whenever the Transmission Request bit or the Self Reception Request bit is set '1' at least for one of the three Transmit Buffers. The Transmission Complete Status bit will remain '0' until all messages are transmitted successfully.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tcs {
    #[doc = "0: Incomplete. At least one requested transmission has not been successfully completed yet."]
    IncompleteAtLeast = 0,
    #[doc = "1: Complete. All requested transmission(s) has (have) been successfully completed."]
    CompleteAllReques = 1,
}
impl From<Tcs> for bool {
    #[inline(always)]
    fn from(variant: Tcs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TCS` reader - Transmit Complete Status. The Transmission Complete Status bit is set '0' (incomplete) whenever the Transmission Request bit or the Self Reception Request bit is set '1' at least for one of the three Transmit Buffers. The Transmission Complete Status bit will remain '0' until all messages are transmitted successfully."]
pub type TcsR = crate::BitReader<Tcs>;
impl TcsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tcs {
        match self.bits {
            false => Tcs::IncompleteAtLeast,
            true => Tcs::CompleteAllReques,
        }
    }
    #[doc = "Incomplete. At least one requested transmission has not been successfully completed yet."]
    #[inline(always)]
    pub fn is_incomplete_at_least(&self) -> bool {
        *self == Tcs::IncompleteAtLeast
    }
    #[doc = "Complete. All requested transmission(s) has (have) been successfully completed."]
    #[inline(always)]
    pub fn is_complete_all_reques(&self) -> bool {
        *self == Tcs::CompleteAllReques
    }
}
#[doc = "Receive Status. If both the Receive Status and the Transmit Status bits are '0' (idle), the CAN-Bus is idle. If both bits are set, the controller is waiting to become idle again. After hardware reset 11 consecutive recessive bits have to be detected until idle status is reached. After Bus-off this will take 128 times of 11 consecutive recessive bits.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rs {
    #[doc = "0: Idle. The CAN controller is idle."]
    IdleTheCanContro = 0,
    #[doc = "1: Receive. The CAN controller is receiving a message."]
    ReceiveTheCanCon = 1,
}
impl From<Rs> for bool {
    #[inline(always)]
    fn from(variant: Rs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RS` reader - Receive Status. If both the Receive Status and the Transmit Status bits are '0' (idle), the CAN-Bus is idle. If both bits are set, the controller is waiting to become idle again. After hardware reset 11 consecutive recessive bits have to be detected until idle status is reached. After Bus-off this will take 128 times of 11 consecutive recessive bits."]
pub type RsR = crate::BitReader<Rs>;
impl RsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rs {
        match self.bits {
            false => Rs::IdleTheCanContro,
            true => Rs::ReceiveTheCanCon,
        }
    }
    #[doc = "Idle. The CAN controller is idle."]
    #[inline(always)]
    pub fn is_idle_the_can_contro(&self) -> bool {
        *self == Rs::IdleTheCanContro
    }
    #[doc = "Receive. The CAN controller is receiving a message."]
    #[inline(always)]
    pub fn is_receive_the_can_con(&self) -> bool {
        *self == Rs::ReceiveTheCanCon
    }
}
#[doc = "Transmit Status. If both the Receive Status and the Transmit Status bits are '0' (idle), the CAN-Bus is idle. If both bits are set, the controller is waiting to become idle again. After hardware reset 11 consecutive recessive bits have to be detected until idle status is reached. After Bus-off this will take 128 times of 11 consecutive recessive bits.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ts {
    #[doc = "0: Idle. The CAN controller is idle."]
    IdleTheCanContro = 0,
    #[doc = "1: Transmit. The CAN controller is sending a message."]
    TransmitTheCanCo = 1,
}
impl From<Ts> for bool {
    #[inline(always)]
    fn from(variant: Ts) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TS` reader - Transmit Status. If both the Receive Status and the Transmit Status bits are '0' (idle), the CAN-Bus is idle. If both bits are set, the controller is waiting to become idle again. After hardware reset 11 consecutive recessive bits have to be detected until idle status is reached. After Bus-off this will take 128 times of 11 consecutive recessive bits."]
pub type TsR = crate::BitReader<Ts>;
impl TsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ts {
        match self.bits {
            false => Ts::IdleTheCanContro,
            true => Ts::TransmitTheCanCo,
        }
    }
    #[doc = "Idle. The CAN controller is idle."]
    #[inline(always)]
    pub fn is_idle_the_can_contro(&self) -> bool {
        *self == Ts::IdleTheCanContro
    }
    #[doc = "Transmit. The CAN controller is sending a message."]
    #[inline(always)]
    pub fn is_transmit_the_can_co(&self) -> bool {
        *self == Ts::TransmitTheCanCo
    }
}
#[doc = "Error Status. Errors detected during reception or transmission will effect the error counters according to the CAN specification. The Error Status bit is set when at least one of the error counters has reached or exceeded the Error Warning Limit. An Error Warning Interrupt is generated, if enabled. The default value of the Error Warning Limit after hardware reset is 96 decimal, see also Section 21.7.7 CAN Error Warning Limit register (CAN1EWL - 0x4004 4018, CAN2EWL - 0x4004 8018).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Es {
    #[doc = "0: OK. Both error counters are below the Error Warning Limit."]
    OkBothErrorCount = 0,
    #[doc = "1: Error. One or both of the Transmit and Receive Error Counters has reached the limit set in the Error Warning Limit register."]
    ErrorOneOrBothO = 1,
}
impl From<Es> for bool {
    #[inline(always)]
    fn from(variant: Es) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ES` reader - Error Status. Errors detected during reception or transmission will effect the error counters according to the CAN specification. The Error Status bit is set when at least one of the error counters has reached or exceeded the Error Warning Limit. An Error Warning Interrupt is generated, if enabled. The default value of the Error Warning Limit after hardware reset is 96 decimal, see also Section 21.7.7 CAN Error Warning Limit register (CAN1EWL - 0x4004 4018, CAN2EWL - 0x4004 8018)."]
pub type EsR = crate::BitReader<Es>;
impl EsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Es {
        match self.bits {
            false => Es::OkBothErrorCount,
            true => Es::ErrorOneOrBothO,
        }
    }
    #[doc = "OK. Both error counters are below the Error Warning Limit."]
    #[inline(always)]
    pub fn is_ok_both_error_count(&self) -> bool {
        *self == Es::OkBothErrorCount
    }
    #[doc = "Error. One or both of the Transmit and Receive Error Counters has reached the limit set in the Error Warning Limit register."]
    #[inline(always)]
    pub fn is_error_one_or_both_o(&self) -> bool {
        *self == Es::ErrorOneOrBothO
    }
}
#[doc = "Bus Status. Mode bit '1' (present) and an Error Warning Interrupt is generated, if enabled. Afterwards the Transmit Error Counter is set to '127', and the Receive Error Counter is cleared. It will stay in this mode until the CPU clears the Reset Mode bit. Once this is completed the CAN Controller will wait the minimum protocol-defined time (128 occurrences of the Bus-Free signal) counting down the Transmit Error Counter. After that, the Bus Status bit is cleared (Bus-On), the Error Status bit is set '0' (ok), the Error Counters are reset, and an Error Warning Interrupt is generated, if enabled. Reading the TX Error Counter during this time gives information about the status of the Bus-Off recovery.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bs {
    #[doc = "0: Bus-on. The CAN Controller is involved in bus activities"]
    BusOnTheCanCont = 0,
    #[doc = "1: Bus-off. The CAN controller is currently not involved/prohibited from bus activity because the Transmit Error Counter reached its limiting value of 255."]
    BusOffTheCanCon = 1,
}
impl From<Bs> for bool {
    #[inline(always)]
    fn from(variant: Bs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BS` reader - Bus Status. Mode bit '1' (present) and an Error Warning Interrupt is generated, if enabled. Afterwards the Transmit Error Counter is set to '127', and the Receive Error Counter is cleared. It will stay in this mode until the CPU clears the Reset Mode bit. Once this is completed the CAN Controller will wait the minimum protocol-defined time (128 occurrences of the Bus-Free signal) counting down the Transmit Error Counter. After that, the Bus Status bit is cleared (Bus-On), the Error Status bit is set '0' (ok), the Error Counters are reset, and an Error Warning Interrupt is generated, if enabled. Reading the TX Error Counter during this time gives information about the status of the Bus-Off recovery."]
pub type BsR = crate::BitReader<Bs>;
impl BsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bs {
        match self.bits {
            false => Bs::BusOnTheCanCont,
            true => Bs::BusOffTheCanCon,
        }
    }
    #[doc = "Bus-on. The CAN Controller is involved in bus activities"]
    #[inline(always)]
    pub fn is_bus_on_the_can_cont(&self) -> bool {
        *self == Bs::BusOnTheCanCont
    }
    #[doc = "Bus-off. The CAN controller is currently not involved/prohibited from bus activity because the Transmit Error Counter reached its limiting value of 255."]
    #[inline(always)]
    pub fn is_bus_off_the_can_con(&self) -> bool {
        *self == Bs::BusOffTheCanCon
    }
}
#[doc = "Field `RXERR` reader - The current value of the Rx Error Counter (an 8-bit value)."]
pub type RxerrR = crate::FieldReader;
#[doc = "Field `TXERR` reader - The current value of the Tx Error Counter (an 8-bit value)."]
pub type TxerrR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Receive Buffer Status. After reading all messages and releasing their memory space with the command 'Release Receive Buffer,' this bit is cleared."]
    #[inline(always)]
    pub fn rbs(&self) -> RbsR {
        RbsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data Overrun Status. If there is not enough space to store the message within the Receive Buffer, that message is dropped and the Data Overrun condition is signalled to the CPU in the moment this message becomes valid. If this message is not completed successfully (e.g. because of an error), no overrun condition is signalled."]
    #[inline(always)]
    pub fn dos(&self) -> DosR {
        DosR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Status."]
    #[inline(always)]
    pub fn tbs(&self) -> TbsR {
        TbsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Complete Status. The Transmission Complete Status bit is set '0' (incomplete) whenever the Transmission Request bit or the Self Reception Request bit is set '1' at least for one of the three Transmit Buffers. The Transmission Complete Status bit will remain '0' until all messages are transmitted successfully."]
    #[inline(always)]
    pub fn tcs(&self) -> TcsR {
        TcsR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Receive Status. If both the Receive Status and the Transmit Status bits are '0' (idle), the CAN-Bus is idle. If both bits are set, the controller is waiting to become idle again. After hardware reset 11 consecutive recessive bits have to be detected until idle status is reached. After Bus-off this will take 128 times of 11 consecutive recessive bits."]
    #[inline(always)]
    pub fn rs(&self) -> RsR {
        RsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmit Status. If both the Receive Status and the Transmit Status bits are '0' (idle), the CAN-Bus is idle. If both bits are set, the controller is waiting to become idle again. After hardware reset 11 consecutive recessive bits have to be detected until idle status is reached. After Bus-off this will take 128 times of 11 consecutive recessive bits."]
    #[inline(always)]
    pub fn ts(&self) -> TsR {
        TsR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Error Status. Errors detected during reception or transmission will effect the error counters according to the CAN specification. The Error Status bit is set when at least one of the error counters has reached or exceeded the Error Warning Limit. An Error Warning Interrupt is generated, if enabled. The default value of the Error Warning Limit after hardware reset is 96 decimal, see also Section 21.7.7 CAN Error Warning Limit register (CAN1EWL - 0x4004 4018, CAN2EWL - 0x4004 8018)."]
    #[inline(always)]
    pub fn es(&self) -> EsR {
        EsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus Status. Mode bit '1' (present) and an Error Warning Interrupt is generated, if enabled. Afterwards the Transmit Error Counter is set to '127', and the Receive Error Counter is cleared. It will stay in this mode until the CPU clears the Reset Mode bit. Once this is completed the CAN Controller will wait the minimum protocol-defined time (128 occurrences of the Bus-Free signal) counting down the Transmit Error Counter. After that, the Bus Status bit is cleared (Bus-On), the Error Status bit is set '0' (ok), the Error Counters are reset, and an Error Warning Interrupt is generated, if enabled. Reading the TX Error Counter during this time gives information about the status of the Bus-Off recovery."]
    #[inline(always)]
    pub fn bs(&self) -> BsR {
        BsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 16:23 - The current value of the Rx Error Counter (an 8-bit value)."]
    #[inline(always)]
    pub fn rxerr(&self) -> RxerrR {
        RxerrR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - The current value of the Tx Error Counter (an 8-bit value)."]
    #[inline(always)]
    pub fn txerr(&self) -> TxerrR {
        TxerrR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "Global Controller Status and Error Counters. The error counters can only be written when RM in CANMOD is 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`gsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GsrSpec;
impl crate::RegisterSpec for GsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gsr::R`](R) reader structure"]
impl crate::Readable for GsrSpec {}
#[doc = "`reset()` method sets GSR to value 0x3c"]
impl crate::Resettable for GsrSpec {
    const RESET_VALUE: u32 = 0x3c;
}
