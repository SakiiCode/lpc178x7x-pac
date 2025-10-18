#[doc = "Register `LSR` reader"]
pub type R = crate::R<LsrSpec>;
#[doc = "Receiver Data Ready. UnLSR\\[0\\] is set when the UnRBR holds an unread character and is cleared when the UARTn RBR FIFO is empty.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rdr {
    #[doc = "0: The UARTn receiver FIFO is empty."]
    Empty = 0,
    #[doc = "1: The UARTn receiver FIFO is not empty."]
    Unread = 1,
}
impl From<Rdr> for bool {
    #[inline(always)]
    fn from(variant: Rdr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RDR` reader - Receiver Data Ready. UnLSR\\[0\\] is set when the UnRBR holds an unread character and is cleared when the UARTn RBR FIFO is empty."]
pub type RdrR = crate::BitReader<Rdr>;
impl RdrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rdr {
        match self.bits {
            false => Rdr::Empty,
            true => Rdr::Unread,
        }
    }
    #[doc = "The UARTn receiver FIFO is empty."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Rdr::Empty
    }
    #[doc = "The UARTn receiver FIFO is not empty."]
    #[inline(always)]
    pub fn is_unread(&self) -> bool {
        *self == Rdr::Unread
    }
}
#[doc = "Overrun Error. The overrun error condition is set as soon as it occurs. An UnLSR read clears UnLSR\\[1\\]. UnLSR\\[1\\] is set when UARTn RSR has a new character assembled and the UARTn RBR FIFO is full. In this case, the UARTn RBR FIFO will not be overwritten and the character in the UARTn RSR will be lost.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oe {
    #[doc = "0: Overrun error status is inactive."]
    Inactive = 0,
    #[doc = "1: Overrun error status is active."]
    Active = 1,
}
impl From<Oe> for bool {
    #[inline(always)]
    fn from(variant: Oe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OE` reader - Overrun Error. The overrun error condition is set as soon as it occurs. An UnLSR read clears UnLSR\\[1\\]. UnLSR\\[1\\] is set when UARTn RSR has a new character assembled and the UARTn RBR FIFO is full. In this case, the UARTn RBR FIFO will not be overwritten and the character in the UARTn RSR will be lost."]
pub type OeR = crate::BitReader<Oe>;
impl OeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oe {
        match self.bits {
            false => Oe::Inactive,
            true => Oe::Active,
        }
    }
    #[doc = "Overrun error status is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Oe::Inactive
    }
    #[doc = "Overrun error status is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Oe::Active
    }
}
#[doc = "Parity Error. When the parity bit of a received character is in the wrong state, a parity error occurs. An UnLSR read clears UnLSR\\[2\\]. Time of parity error detection is dependent on UnFCR\\[0\\]. Note: A parity error is associated with the character at the top of the UARTn RBR FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pe {
    #[doc = "0: Parity error status is inactive."]
    Inactive = 0,
    #[doc = "1: Parity error status is active."]
    Active = 1,
}
impl From<Pe> for bool {
    #[inline(always)]
    fn from(variant: Pe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - Parity Error. When the parity bit of a received character is in the wrong state, a parity error occurs. An UnLSR read clears UnLSR\\[2\\]. Time of parity error detection is dependent on UnFCR\\[0\\]. Note: A parity error is associated with the character at the top of the UARTn RBR FIFO."]
pub type PeR = crate::BitReader<Pe>;
impl PeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pe {
        match self.bits {
            false => Pe::Inactive,
            true => Pe::Active,
        }
    }
    #[doc = "Parity error status is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Pe::Inactive
    }
    #[doc = "Parity error status is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Pe::Active
    }
}
#[doc = "Framing Error. When the stop bit of a received character is a logic 0, a framing error occurs. An UnLSR read clears UnLSR\\[3\\]. The time of the framing error detection is dependent on UnFCR\\[0\\]. Upon detection of a framing error, the Rx will attempt to resynchronize to the data and assume that the bad stop bit is actually an early start bit. However, it cannot be assumed that the next received byte will be correct even if there is no Framing Error. Note: A framing error is associated with the character at the top of the UARTn RBR FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fe {
    #[doc = "0: Framing error status is inactive."]
    Inactive = 0,
    #[doc = "1: Framing error status is active."]
    Active = 1,
}
impl From<Fe> for bool {
    #[inline(always)]
    fn from(variant: Fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FE` reader - Framing Error. When the stop bit of a received character is a logic 0, a framing error occurs. An UnLSR read clears UnLSR\\[3\\]. The time of the framing error detection is dependent on UnFCR\\[0\\]. Upon detection of a framing error, the Rx will attempt to resynchronize to the data and assume that the bad stop bit is actually an early start bit. However, it cannot be assumed that the next received byte will be correct even if there is no Framing Error. Note: A framing error is associated with the character at the top of the UARTn RBR FIFO."]
pub type FeR = crate::BitReader<Fe>;
impl FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fe {
        match self.bits {
            false => Fe::Inactive,
            true => Fe::Active,
        }
    }
    #[doc = "Framing error status is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Fe::Inactive
    }
    #[doc = "Framing error status is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Fe::Active
    }
}
#[doc = "Break Interrupt. When RXDn is held in the spacing state (all zeroes) for one full character transmission (start, data, parity, stop), a break interrupt occurs. Once the break condition has been detected, the receiver goes idle until RXDn goes to marking state (all ones). An UnLSR read clears this status bit. The time of break detection is dependent on UnFCR\\[0\\]. Note: The break interrupt is associated with the character at the top of the UARTn RBR FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bi {
    #[doc = "0: Break interrupt status is inactive."]
    Inactive = 0,
    #[doc = "1: Break interrupt status is active."]
    Active = 1,
}
impl From<Bi> for bool {
    #[inline(always)]
    fn from(variant: Bi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BI` reader - Break Interrupt. When RXDn is held in the spacing state (all zeroes) for one full character transmission (start, data, parity, stop), a break interrupt occurs. Once the break condition has been detected, the receiver goes idle until RXDn goes to marking state (all ones). An UnLSR read clears this status bit. The time of break detection is dependent on UnFCR\\[0\\]. Note: The break interrupt is associated with the character at the top of the UARTn RBR FIFO."]
pub type BiR = crate::BitReader<Bi>;
impl BiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bi {
        match self.bits {
            false => Bi::Inactive,
            true => Bi::Active,
        }
    }
    #[doc = "Break interrupt status is inactive."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Bi::Inactive
    }
    #[doc = "Break interrupt status is active."]
    #[inline(always)]
    pub fn is_active(&self) -> bool {
        *self == Bi::Active
    }
}
#[doc = "Transmitter Holding Register Empty. THRE is set immediately upon detection of an empty UARTn THR and is cleared on a UnTHR write.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Thre {
    #[doc = "0: UnTHR contains valid data."]
    UnthrContainsValid = 0,
    #[doc = "1: UnTHR is empty."]
    UnthrIsEmpty_ = 1,
}
impl From<Thre> for bool {
    #[inline(always)]
    fn from(variant: Thre) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `THRE` reader - Transmitter Holding Register Empty. THRE is set immediately upon detection of an empty UARTn THR and is cleared on a UnTHR write."]
pub type ThreR = crate::BitReader<Thre>;
impl ThreR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Thre {
        match self.bits {
            false => Thre::UnthrContainsValid,
            true => Thre::UnthrIsEmpty_,
        }
    }
    #[doc = "UnTHR contains valid data."]
    #[inline(always)]
    pub fn is_unthr_contains_valid(&self) -> bool {
        *self == Thre::UnthrContainsValid
    }
    #[doc = "UnTHR is empty."]
    #[inline(always)]
    pub fn is_unthr_is_empty_(&self) -> bool {
        *self == Thre::UnthrIsEmpty_
    }
}
#[doc = "Transmitter Empty. TEMT is set when both UnTHR and UnTSR are empty; TEMT is cleared when either the UnTSR or the UnTHR contain valid data.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Temt {
    #[doc = "0: UnTHR and/or the UnTSR contains valid data."]
    ValidData = 0,
    #[doc = "1: UnTHR and the UnTSR are empty."]
    Empty = 1,
}
impl From<Temt> for bool {
    #[inline(always)]
    fn from(variant: Temt) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TEMT` reader - Transmitter Empty. TEMT is set when both UnTHR and UnTSR are empty; TEMT is cleared when either the UnTSR or the UnTHR contain valid data."]
pub type TemtR = crate::BitReader<Temt>;
impl TemtR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Temt {
        match self.bits {
            false => Temt::ValidData,
            true => Temt::Empty,
        }
    }
    #[doc = "UnTHR and/or the UnTSR contains valid data."]
    #[inline(always)]
    pub fn is_valid_data(&self) -> bool {
        *self == Temt::ValidData
    }
    #[doc = "UnTHR and the UnTSR are empty."]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Temt::Empty
    }
}
#[doc = "Error in RX FIFO . UnLSR\\[7\\] is set when a character with a Rx error such as framing error, parity error or break interrupt, is loaded into the UnRBR. This bit is cleared when the UnLSR register is read and there are no subsequent errors in the UARTn FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxfe {
    #[doc = "0: UnRBR contains no UARTn RX errors or UnFCR\\[0\\]=0."]
    UnrbrContainsNoUa = 0,
    #[doc = "1: UARTn RBR contains at least one UARTn RX error."]
    UartnRbrContainsA = 1,
}
impl From<Rxfe> for bool {
    #[inline(always)]
    fn from(variant: Rxfe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFE` reader - Error in RX FIFO . UnLSR\\[7\\] is set when a character with a Rx error such as framing error, parity error or break interrupt, is loaded into the UnRBR. This bit is cleared when the UnLSR register is read and there are no subsequent errors in the UARTn FIFO."]
pub type RxfeR = crate::BitReader<Rxfe>;
impl RxfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxfe {
        match self.bits {
            false => Rxfe::UnrbrContainsNoUa,
            true => Rxfe::UartnRbrContainsA,
        }
    }
    #[doc = "UnRBR contains no UARTn RX errors or UnFCR\\[0\\]=0."]
    #[inline(always)]
    pub fn is_unrbr_contains_no_ua(&self) -> bool {
        *self == Rxfe::UnrbrContainsNoUa
    }
    #[doc = "UARTn RBR contains at least one UARTn RX error."]
    #[inline(always)]
    pub fn is_uartn_rbr_contains_a(&self) -> bool {
        *self == Rxfe::UartnRbrContainsA
    }
}
impl R {
    #[doc = "Bit 0 - Receiver Data Ready. UnLSR\\[0\\] is set when the UnRBR holds an unread character and is cleared when the UARTn RBR FIFO is empty."]
    #[inline(always)]
    pub fn rdr(&self) -> RdrR {
        RdrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Overrun Error. The overrun error condition is set as soon as it occurs. An UnLSR read clears UnLSR\\[1\\]. UnLSR\\[1\\] is set when UARTn RSR has a new character assembled and the UARTn RBR FIFO is full. In this case, the UARTn RBR FIFO will not be overwritten and the character in the UARTn RSR will be lost."]
    #[inline(always)]
    pub fn oe(&self) -> OeR {
        OeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity Error. When the parity bit of a received character is in the wrong state, a parity error occurs. An UnLSR read clears UnLSR\\[2\\]. Time of parity error detection is dependent on UnFCR\\[0\\]. Note: A parity error is associated with the character at the top of the UARTn RBR FIFO."]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Framing Error. When the stop bit of a received character is a logic 0, a framing error occurs. An UnLSR read clears UnLSR\\[3\\]. The time of the framing error detection is dependent on UnFCR\\[0\\]. Upon detection of a framing error, the Rx will attempt to resynchronize to the data and assume that the bad stop bit is actually an early start bit. However, it cannot be assumed that the next received byte will be correct even if there is no Framing Error. Note: A framing error is associated with the character at the top of the UARTn RBR FIFO."]
    #[inline(always)]
    pub fn fe(&self) -> FeR {
        FeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Break Interrupt. When RXDn is held in the spacing state (all zeroes) for one full character transmission (start, data, parity, stop), a break interrupt occurs. Once the break condition has been detected, the receiver goes idle until RXDn goes to marking state (all ones). An UnLSR read clears this status bit. The time of break detection is dependent on UnFCR\\[0\\]. Note: The break interrupt is associated with the character at the top of the UARTn RBR FIFO."]
    #[inline(always)]
    pub fn bi(&self) -> BiR {
        BiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Transmitter Holding Register Empty. THRE is set immediately upon detection of an empty UARTn THR and is cleared on a UnTHR write."]
    #[inline(always)]
    pub fn thre(&self) -> ThreR {
        ThreR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmitter Empty. TEMT is set when both UnTHR and UnTSR are empty; TEMT is cleared when either the UnTSR or the UnTHR contain valid data."]
    #[inline(always)]
    pub fn temt(&self) -> TemtR {
        TemtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error in RX FIFO . UnLSR\\[7\\] is set when a character with a Rx error such as framing error, parity error or break interrupt, is loaded into the UnRBR. This bit is cleared when the UnLSR register is read and there are no subsequent errors in the UARTn FIFO."]
    #[inline(always)]
    pub fn rxfe(&self) -> RxfeR {
        RxfeR::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Line Status Register. Contains flags for transmit and receive status, including line errors.\n\nYou can [`read`](crate::Reg::read) this register and get [`lsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LsrSpec;
impl crate::RegisterSpec for LsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lsr::R`](R) reader structure"]
impl crate::Readable for LsrSpec {}
#[doc = "`reset()` method sets LSR to value 0x60"]
impl crate::Resettable for LsrSpec {
    const RESET_VALUE: u32 = 0x60;
}
