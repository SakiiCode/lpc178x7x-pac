#[doc = "Register `I2C_CTL` reader"]
pub type R = crate::R<I2cCtlSpec>;
#[doc = "Register `I2C_CTL` writer"]
pub type W = crate::W<I2cCtlSpec>;
#[doc = "Transmit Done Interrupt Enable. This enables the TDI interrupt signalling that this I2C issued a STOP condition.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tdie {
    #[doc = "0: Disable the TDI interrupt."]
    DisableTheTdiInte = 0,
    #[doc = "1: Enable the TDI interrupt."]
    EnableTheTdiInter = 1,
}
impl From<Tdie> for bool {
    #[inline(always)]
    fn from(variant: Tdie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDIE` reader - Transmit Done Interrupt Enable. This enables the TDI interrupt signalling that this I2C issued a STOP condition."]
pub type TdieR = crate::BitReader<Tdie>;
impl TdieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tdie {
        match self.bits {
            false => Tdie::DisableTheTdiInte,
            true => Tdie::EnableTheTdiInter,
        }
    }
    #[doc = "Disable the TDI interrupt."]
    #[inline(always)]
    pub fn is_disable_the_tdi_inte(&self) -> bool {
        *self == Tdie::DisableTheTdiInte
    }
    #[doc = "Enable the TDI interrupt."]
    #[inline(always)]
    pub fn is_enable_the_tdi_inter(&self) -> bool {
        *self == Tdie::EnableTheTdiInter
    }
}
#[doc = "Field `TDIE` writer - Transmit Done Interrupt Enable. This enables the TDI interrupt signalling that this I2C issued a STOP condition."]
pub type TdieW<'a, REG> = crate::BitWriter<'a, REG, Tdie>;
impl<'a, REG> TdieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the TDI interrupt."]
    #[inline(always)]
    pub fn disable_the_tdi_inte(self) -> &'a mut crate::W<REG> {
        self.variant(Tdie::DisableTheTdiInte)
    }
    #[doc = "Enable the TDI interrupt."]
    #[inline(always)]
    pub fn enable_the_tdi_inter(self) -> &'a mut crate::W<REG> {
        self.variant(Tdie::EnableTheTdiInter)
    }
}
#[doc = "Transmitter Arbitration Failure Interrupt Enable. This enables the AFI interrupt which is asserted during transmission when trying to set SDA high, but the bus is driven low by another device.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Afie {
    #[doc = "0: Disable the AFI."]
    DisableTheAfi_ = 0,
    #[doc = "1: Enable the AFI."]
    EnableTheAfi_ = 1,
}
impl From<Afie> for bool {
    #[inline(always)]
    fn from(variant: Afie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AFIE` reader - Transmitter Arbitration Failure Interrupt Enable. This enables the AFI interrupt which is asserted during transmission when trying to set SDA high, but the bus is driven low by another device."]
pub type AfieR = crate::BitReader<Afie>;
impl AfieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Afie {
        match self.bits {
            false => Afie::DisableTheAfi_,
            true => Afie::EnableTheAfi_,
        }
    }
    #[doc = "Disable the AFI."]
    #[inline(always)]
    pub fn is_disable_the_afi_(&self) -> bool {
        *self == Afie::DisableTheAfi_
    }
    #[doc = "Enable the AFI."]
    #[inline(always)]
    pub fn is_enable_the_afi_(&self) -> bool {
        *self == Afie::EnableTheAfi_
    }
}
#[doc = "Field `AFIE` writer - Transmitter Arbitration Failure Interrupt Enable. This enables the AFI interrupt which is asserted during transmission when trying to set SDA high, but the bus is driven low by another device."]
pub type AfieW<'a, REG> = crate::BitWriter<'a, REG, Afie>;
impl<'a, REG> AfieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the AFI."]
    #[inline(always)]
    pub fn disable_the_afi_(self) -> &'a mut crate::W<REG> {
        self.variant(Afie::DisableTheAfi_)
    }
    #[doc = "Enable the AFI."]
    #[inline(always)]
    pub fn enable_the_afi_(self) -> &'a mut crate::W<REG> {
        self.variant(Afie::EnableTheAfi_)
    }
}
#[doc = "Transmitter No Acknowledge Interrupt Enable. This enables the NAI interrupt signalling that transmitted byte was not acknowledged.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Naie {
    #[doc = "0: Disable the NAI."]
    DisableTheNai_ = 0,
    #[doc = "1: Enable the NAI."]
    EnableTheNai_ = 1,
}
impl From<Naie> for bool {
    #[inline(always)]
    fn from(variant: Naie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NAIE` reader - Transmitter No Acknowledge Interrupt Enable. This enables the NAI interrupt signalling that transmitted byte was not acknowledged."]
pub type NaieR = crate::BitReader<Naie>;
impl NaieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Naie {
        match self.bits {
            false => Naie::DisableTheNai_,
            true => Naie::EnableTheNai_,
        }
    }
    #[doc = "Disable the NAI."]
    #[inline(always)]
    pub fn is_disable_the_nai_(&self) -> bool {
        *self == Naie::DisableTheNai_
    }
    #[doc = "Enable the NAI."]
    #[inline(always)]
    pub fn is_enable_the_nai_(&self) -> bool {
        *self == Naie::EnableTheNai_
    }
}
#[doc = "Field `NAIE` writer - Transmitter No Acknowledge Interrupt Enable. This enables the NAI interrupt signalling that transmitted byte was not acknowledged."]
pub type NaieW<'a, REG> = crate::BitWriter<'a, REG, Naie>;
impl<'a, REG> NaieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the NAI."]
    #[inline(always)]
    pub fn disable_the_nai_(self) -> &'a mut crate::W<REG> {
        self.variant(Naie::DisableTheNai_)
    }
    #[doc = "Enable the NAI."]
    #[inline(always)]
    pub fn enable_the_nai_(self) -> &'a mut crate::W<REG> {
        self.variant(Naie::EnableTheNai_)
    }
}
#[doc = "Master Transmitter Data Request Interrupt Enable. This enables the DRMI interrupt which signals that the master transmitter has run out of data, has not issued a STOP, and is holding the SCL line low.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drmie {
    #[doc = "0: Disable the DRMI interrupt."]
    DisableTheDrmiInt = 0,
    #[doc = "1: Enable the DRMI interrupt."]
    EnableTheDrmiInte = 1,
}
impl From<Drmie> for bool {
    #[inline(always)]
    fn from(variant: Drmie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRMIE` reader - Master Transmitter Data Request Interrupt Enable. This enables the DRMI interrupt which signals that the master transmitter has run out of data, has not issued a STOP, and is holding the SCL line low."]
pub type DrmieR = crate::BitReader<Drmie>;
impl DrmieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drmie {
        match self.bits {
            false => Drmie::DisableTheDrmiInt,
            true => Drmie::EnableTheDrmiInte,
        }
    }
    #[doc = "Disable the DRMI interrupt."]
    #[inline(always)]
    pub fn is_disable_the_drmi_int(&self) -> bool {
        *self == Drmie::DisableTheDrmiInt
    }
    #[doc = "Enable the DRMI interrupt."]
    #[inline(always)]
    pub fn is_enable_the_drmi_inte(&self) -> bool {
        *self == Drmie::EnableTheDrmiInte
    }
}
#[doc = "Field `DRMIE` writer - Master Transmitter Data Request Interrupt Enable. This enables the DRMI interrupt which signals that the master transmitter has run out of data, has not issued a STOP, and is holding the SCL line low."]
pub type DrmieW<'a, REG> = crate::BitWriter<'a, REG, Drmie>;
impl<'a, REG> DrmieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the DRMI interrupt."]
    #[inline(always)]
    pub fn disable_the_drmi_int(self) -> &'a mut crate::W<REG> {
        self.variant(Drmie::DisableTheDrmiInt)
    }
    #[doc = "Enable the DRMI interrupt."]
    #[inline(always)]
    pub fn enable_the_drmi_inte(self) -> &'a mut crate::W<REG> {
        self.variant(Drmie::EnableTheDrmiInte)
    }
}
#[doc = "Slave Transmitter Data Request Interrupt Enable. This enables the DRSI interrupt which signals that the slave transmitter has run out of data and the last byte was acknowledged, so the SCL line is being held low.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Drsie {
    #[doc = "0: Disable the DRSI interrupt."]
    DisableTheDrsiInt = 0,
    #[doc = "1: Enable the DRSI interrupt."]
    EnableTheDrsiInte = 1,
}
impl From<Drsie> for bool {
    #[inline(always)]
    fn from(variant: Drsie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRSIE` reader - Slave Transmitter Data Request Interrupt Enable. This enables the DRSI interrupt which signals that the slave transmitter has run out of data and the last byte was acknowledged, so the SCL line is being held low."]
pub type DrsieR = crate::BitReader<Drsie>;
impl DrsieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Drsie {
        match self.bits {
            false => Drsie::DisableTheDrsiInt,
            true => Drsie::EnableTheDrsiInte,
        }
    }
    #[doc = "Disable the DRSI interrupt."]
    #[inline(always)]
    pub fn is_disable_the_drsi_int(&self) -> bool {
        *self == Drsie::DisableTheDrsiInt
    }
    #[doc = "Enable the DRSI interrupt."]
    #[inline(always)]
    pub fn is_enable_the_drsi_inte(&self) -> bool {
        *self == Drsie::EnableTheDrsiInte
    }
}
#[doc = "Field `DRSIE` writer - Slave Transmitter Data Request Interrupt Enable. This enables the DRSI interrupt which signals that the slave transmitter has run out of data and the last byte was acknowledged, so the SCL line is being held low."]
pub type DrsieW<'a, REG> = crate::BitWriter<'a, REG, Drsie>;
impl<'a, REG> DrsieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the DRSI interrupt."]
    #[inline(always)]
    pub fn disable_the_drsi_int(self) -> &'a mut crate::W<REG> {
        self.variant(Drsie::DisableTheDrsiInt)
    }
    #[doc = "Enable the DRSI interrupt."]
    #[inline(always)]
    pub fn enable_the_drsi_inte(self) -> &'a mut crate::W<REG> {
        self.variant(Drsie::EnableTheDrsiInte)
    }
}
#[doc = "Receive FIFO Full Interrupt Enable. This enables the Receive FIFO Full interrupt to indicate that the receive FIFO cannot accept any more data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Refie {
    #[doc = "0: Disable the RFFI."]
    DisableTheRffi_ = 0,
    #[doc = "1: Enable the RFFI."]
    EnableTheRffi_ = 1,
}
impl From<Refie> for bool {
    #[inline(always)]
    fn from(variant: Refie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REFIE` reader - Receive FIFO Full Interrupt Enable. This enables the Receive FIFO Full interrupt to indicate that the receive FIFO cannot accept any more data."]
pub type RefieR = crate::BitReader<Refie>;
impl RefieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Refie {
        match self.bits {
            false => Refie::DisableTheRffi_,
            true => Refie::EnableTheRffi_,
        }
    }
    #[doc = "Disable the RFFI."]
    #[inline(always)]
    pub fn is_disable_the_rffi_(&self) -> bool {
        *self == Refie::DisableTheRffi_
    }
    #[doc = "Enable the RFFI."]
    #[inline(always)]
    pub fn is_enable_the_rffi_(&self) -> bool {
        *self == Refie::EnableTheRffi_
    }
}
#[doc = "Field `REFIE` writer - Receive FIFO Full Interrupt Enable. This enables the Receive FIFO Full interrupt to indicate that the receive FIFO cannot accept any more data."]
pub type RefieW<'a, REG> = crate::BitWriter<'a, REG, Refie>;
impl<'a, REG> RefieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the RFFI."]
    #[inline(always)]
    pub fn disable_the_rffi_(self) -> &'a mut crate::W<REG> {
        self.variant(Refie::DisableTheRffi_)
    }
    #[doc = "Enable the RFFI."]
    #[inline(always)]
    pub fn enable_the_rffi_(self) -> &'a mut crate::W<REG> {
        self.variant(Refie::EnableTheRffi_)
    }
}
#[doc = "Receive Data Available Interrupt Enable. This enables the DAI interrupt to indicate that data is available in the receive FIFO (i.e. not empty).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rfdaie {
    #[doc = "0: Disable the DAI."]
    DisableTheDai_ = 0,
    #[doc = "1: Enable the DAI."]
    EnableTheDai_ = 1,
}
impl From<Rfdaie> for bool {
    #[inline(always)]
    fn from(variant: Rfdaie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFDAIE` reader - Receive Data Available Interrupt Enable. This enables the DAI interrupt to indicate that data is available in the receive FIFO (i.e. not empty)."]
pub type RfdaieR = crate::BitReader<Rfdaie>;
impl RfdaieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rfdaie {
        match self.bits {
            false => Rfdaie::DisableTheDai_,
            true => Rfdaie::EnableTheDai_,
        }
    }
    #[doc = "Disable the DAI."]
    #[inline(always)]
    pub fn is_disable_the_dai_(&self) -> bool {
        *self == Rfdaie::DisableTheDai_
    }
    #[doc = "Enable the DAI."]
    #[inline(always)]
    pub fn is_enable_the_dai_(&self) -> bool {
        *self == Rfdaie::EnableTheDai_
    }
}
#[doc = "Field `RFDAIE` writer - Receive Data Available Interrupt Enable. This enables the DAI interrupt to indicate that data is available in the receive FIFO (i.e. not empty)."]
pub type RfdaieW<'a, REG> = crate::BitWriter<'a, REG, Rfdaie>;
impl<'a, REG> RfdaieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the DAI."]
    #[inline(always)]
    pub fn disable_the_dai_(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdaie::DisableTheDai_)
    }
    #[doc = "Enable the DAI."]
    #[inline(always)]
    pub fn enable_the_dai_(self) -> &'a mut crate::W<REG> {
        self.variant(Rfdaie::EnableTheDai_)
    }
}
#[doc = "Transmit FIFO Not Full Interrupt Enable. This enables the Transmit FIFO Not Full interrupt to indicate that the more data can be written to the transmit FIFO. Note that this is not full. It is intended help the CPU to write to the I2C block only when there is room in the FIFO and do this without polling the status register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tffie {
    #[doc = "0: Disable the TFFI."]
    DisableTheTffi_ = 0,
    #[doc = "1: Enable the TFFI."]
    EnableTheTffi_ = 1,
}
impl From<Tffie> for bool {
    #[inline(always)]
    fn from(variant: Tffie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFFIE` reader - Transmit FIFO Not Full Interrupt Enable. This enables the Transmit FIFO Not Full interrupt to indicate that the more data can be written to the transmit FIFO. Note that this is not full. It is intended help the CPU to write to the I2C block only when there is room in the FIFO and do this without polling the status register."]
pub type TffieR = crate::BitReader<Tffie>;
impl TffieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tffie {
        match self.bits {
            false => Tffie::DisableTheTffi_,
            true => Tffie::EnableTheTffi_,
        }
    }
    #[doc = "Disable the TFFI."]
    #[inline(always)]
    pub fn is_disable_the_tffi_(&self) -> bool {
        *self == Tffie::DisableTheTffi_
    }
    #[doc = "Enable the TFFI."]
    #[inline(always)]
    pub fn is_enable_the_tffi_(&self) -> bool {
        *self == Tffie::EnableTheTffi_
    }
}
#[doc = "Field `TFFIE` writer - Transmit FIFO Not Full Interrupt Enable. This enables the Transmit FIFO Not Full interrupt to indicate that the more data can be written to the transmit FIFO. Note that this is not full. It is intended help the CPU to write to the I2C block only when there is room in the FIFO and do this without polling the status register."]
pub type TffieW<'a, REG> = crate::BitWriter<'a, REG, Tffie>;
impl<'a, REG> TffieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the TFFI."]
    #[inline(always)]
    pub fn disable_the_tffi_(self) -> &'a mut crate::W<REG> {
        self.variant(Tffie::DisableTheTffi_)
    }
    #[doc = "Enable the TFFI."]
    #[inline(always)]
    pub fn enable_the_tffi_(self) -> &'a mut crate::W<REG> {
        self.variant(Tffie::EnableTheTffi_)
    }
}
#[doc = "Soft reset. This is only needed in unusual circumstances. If a device issues a start condition without issuing a stop condition. A system timer may be used to reset the I2C if the bus remains busy longer than the time-out period. On a soft reset, the Tx and Rx FIFOs are flushed, I2C_STS register is cleared, and all internal state machines are reset to appear idle. The I2C_CLKHI, I2C_CLKLO and I2C_CTL (except Soft Reset Bit) are NOT modified by a soft reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srst {
    #[doc = "0: No reset."]
    NoReset = 0,
    #[doc = "1: Reset the I2C to idle state. Self clearing."]
    Reset = 1,
}
impl From<Srst> for bool {
    #[inline(always)]
    fn from(variant: Srst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRST` reader - Soft reset. This is only needed in unusual circumstances. If a device issues a start condition without issuing a stop condition. A system timer may be used to reset the I2C if the bus remains busy longer than the time-out period. On a soft reset, the Tx and Rx FIFOs are flushed, I2C_STS register is cleared, and all internal state machines are reset to appear idle. The I2C_CLKHI, I2C_CLKLO and I2C_CTL (except Soft Reset Bit) are NOT modified by a soft reset."]
pub type SrstR = crate::BitReader<Srst>;
impl SrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Srst {
        match self.bits {
            false => Srst::NoReset,
            true => Srst::Reset,
        }
    }
    #[doc = "No reset."]
    #[inline(always)]
    pub fn is_no_reset(&self) -> bool {
        *self == Srst::NoReset
    }
    #[doc = "Reset the I2C to idle state. Self clearing."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Srst::Reset
    }
}
#[doc = "Field `SRST` writer - Soft reset. This is only needed in unusual circumstances. If a device issues a start condition without issuing a stop condition. A system timer may be used to reset the I2C if the bus remains busy longer than the time-out period. On a soft reset, the Tx and Rx FIFOs are flushed, I2C_STS register is cleared, and all internal state machines are reset to appear idle. The I2C_CLKHI, I2C_CLKLO and I2C_CTL (except Soft Reset Bit) are NOT modified by a soft reset."]
pub type SrstW<'a, REG> = crate::BitWriter<'a, REG, Srst>;
impl<'a, REG> SrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No reset."]
    #[inline(always)]
    pub fn no_reset(self) -> &'a mut crate::W<REG> {
        self.variant(Srst::NoReset)
    }
    #[doc = "Reset the I2C to idle state. Self clearing."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Srst::Reset)
    }
}
impl R {
    #[doc = "Bit 0 - Transmit Done Interrupt Enable. This enables the TDI interrupt signalling that this I2C issued a STOP condition."]
    #[inline(always)]
    pub fn tdie(&self) -> TdieR {
        TdieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmitter Arbitration Failure Interrupt Enable. This enables the AFI interrupt which is asserted during transmission when trying to set SDA high, but the bus is driven low by another device."]
    #[inline(always)]
    pub fn afie(&self) -> AfieR {
        AfieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmitter No Acknowledge Interrupt Enable. This enables the NAI interrupt signalling that transmitted byte was not acknowledged."]
    #[inline(always)]
    pub fn naie(&self) -> NaieR {
        NaieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master Transmitter Data Request Interrupt Enable. This enables the DRMI interrupt which signals that the master transmitter has run out of data, has not issued a STOP, and is holding the SCL line low."]
    #[inline(always)]
    pub fn drmie(&self) -> DrmieR {
        DrmieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave Transmitter Data Request Interrupt Enable. This enables the DRSI interrupt which signals that the slave transmitter has run out of data and the last byte was acknowledged, so the SCL line is being held low."]
    #[inline(always)]
    pub fn drsie(&self) -> DrsieR {
        DrsieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive FIFO Full Interrupt Enable. This enables the Receive FIFO Full interrupt to indicate that the receive FIFO cannot accept any more data."]
    #[inline(always)]
    pub fn refie(&self) -> RefieR {
        RefieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Data Available Interrupt Enable. This enables the DAI interrupt to indicate that data is available in the receive FIFO (i.e. not empty)."]
    #[inline(always)]
    pub fn rfdaie(&self) -> RfdaieR {
        RfdaieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transmit FIFO Not Full Interrupt Enable. This enables the Transmit FIFO Not Full interrupt to indicate that the more data can be written to the transmit FIFO. Note that this is not full. It is intended help the CPU to write to the I2C block only when there is room in the FIFO and do this without polling the status register."]
    #[inline(always)]
    pub fn tffie(&self) -> TffieR {
        TffieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Soft reset. This is only needed in unusual circumstances. If a device issues a start condition without issuing a stop condition. A system timer may be used to reset the I2C if the bus remains busy longer than the time-out period. On a soft reset, the Tx and Rx FIFOs are flushed, I2C_STS register is cleared, and all internal state machines are reset to appear idle. The I2C_CLKHI, I2C_CLKLO and I2C_CTL (except Soft Reset Bit) are NOT modified by a soft reset."]
    #[inline(always)]
    pub fn srst(&self) -> SrstR {
        SrstR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Done Interrupt Enable. This enables the TDI interrupt signalling that this I2C issued a STOP condition."]
    #[inline(always)]
    pub fn tdie(&mut self) -> TdieW<'_, I2cCtlSpec> {
        TdieW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmitter Arbitration Failure Interrupt Enable. This enables the AFI interrupt which is asserted during transmission when trying to set SDA high, but the bus is driven low by another device."]
    #[inline(always)]
    pub fn afie(&mut self) -> AfieW<'_, I2cCtlSpec> {
        AfieW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmitter No Acknowledge Interrupt Enable. This enables the NAI interrupt signalling that transmitted byte was not acknowledged."]
    #[inline(always)]
    pub fn naie(&mut self) -> NaieW<'_, I2cCtlSpec> {
        NaieW::new(self, 2)
    }
    #[doc = "Bit 3 - Master Transmitter Data Request Interrupt Enable. This enables the DRMI interrupt which signals that the master transmitter has run out of data, has not issued a STOP, and is holding the SCL line low."]
    #[inline(always)]
    pub fn drmie(&mut self) -> DrmieW<'_, I2cCtlSpec> {
        DrmieW::new(self, 3)
    }
    #[doc = "Bit 4 - Slave Transmitter Data Request Interrupt Enable. This enables the DRSI interrupt which signals that the slave transmitter has run out of data and the last byte was acknowledged, so the SCL line is being held low."]
    #[inline(always)]
    pub fn drsie(&mut self) -> DrsieW<'_, I2cCtlSpec> {
        DrsieW::new(self, 4)
    }
    #[doc = "Bit 5 - Receive FIFO Full Interrupt Enable. This enables the Receive FIFO Full interrupt to indicate that the receive FIFO cannot accept any more data."]
    #[inline(always)]
    pub fn refie(&mut self) -> RefieW<'_, I2cCtlSpec> {
        RefieW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive Data Available Interrupt Enable. This enables the DAI interrupt to indicate that data is available in the receive FIFO (i.e. not empty)."]
    #[inline(always)]
    pub fn rfdaie(&mut self) -> RfdaieW<'_, I2cCtlSpec> {
        RfdaieW::new(self, 6)
    }
    #[doc = "Bit 7 - Transmit FIFO Not Full Interrupt Enable. This enables the Transmit FIFO Not Full interrupt to indicate that the more data can be written to the transmit FIFO. Note that this is not full. It is intended help the CPU to write to the I2C block only when there is room in the FIFO and do this without polling the status register."]
    #[inline(always)]
    pub fn tffie(&mut self) -> TffieW<'_, I2cCtlSpec> {
        TffieW::new(self, 7)
    }
    #[doc = "Bit 8 - Soft reset. This is only needed in unusual circumstances. If a device issues a start condition without issuing a stop condition. A system timer may be used to reset the I2C if the bus remains busy longer than the time-out period. On a soft reset, the Tx and Rx FIFOs are flushed, I2C_STS register is cleared, and all internal state machines are reset to appear idle. The I2C_CLKHI, I2C_CLKLO and I2C_CTL (except Soft Reset Bit) are NOT modified by a soft reset."]
    #[inline(always)]
    pub fn srst(&mut self) -> SrstW<'_, I2cCtlSpec> {
        SrstW::new(self, 8)
    }
}
#[doc = "I2C Control\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_ctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i2c_ctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cCtlSpec;
impl crate::RegisterSpec for I2cCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_ctl::R`](R) reader structure"]
impl crate::Readable for I2cCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`i2c_ctl::W`](W) writer structure"]
impl crate::Writable for I2cCtlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I2C_CTL to value 0"]
impl crate::Resettable for I2cCtlSpec {}
