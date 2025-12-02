#[doc = "Register `MOD` reader"]
pub type R = crate::R<ModSpec>;
#[doc = "Register `MOD` writer"]
pub type W = crate::W<ModSpec>;
#[doc = "Reset Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rm {
    #[doc = "0: Normal.The CAN Controller is in the Operating Mode, and certain registers can not be written."]
    Operating = 0,
    #[doc = "1: Reset. CAN operation is disabled, writable registers can be written and the current transmission/reception of a message is aborted."]
    Reset = 1,
}
impl From<Rm> for bool {
    #[inline(always)]
    fn from(variant: Rm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RM` reader - Reset Mode."]
pub type RmR = crate::BitReader<Rm>;
impl RmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rm {
        match self.bits {
            false => Rm::Operating,
            true => Rm::Reset,
        }
    }
    #[doc = "Normal.The CAN Controller is in the Operating Mode, and certain registers can not be written."]
    #[inline(always)]
    pub fn is_operating(&self) -> bool {
        *self == Rm::Operating
    }
    #[doc = "Reset. CAN operation is disabled, writable registers can be written and the current transmission/reception of a message is aborted."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Rm::Reset
    }
}
#[doc = "Field `RM` writer - Reset Mode."]
pub type RmW<'a, REG> = crate::BitWriter<'a, REG, Rm>;
impl<'a, REG> RmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal.The CAN Controller is in the Operating Mode, and certain registers can not be written."]
    #[inline(always)]
    pub fn operating(self) -> &'a mut crate::W<REG> {
        self.variant(Rm::Operating)
    }
    #[doc = "Reset. CAN operation is disabled, writable registers can be written and the current transmission/reception of a message is aborted."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Rm::Reset)
    }
}
#[doc = "Listen Only Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lom {
    #[doc = "0: Normal. The CAN controller acknowledges a successfully received message on the CAN bus. The error counters are stopped at the current value."]
    Disabled = 0,
    #[doc = "1: Listen only. The controller gives no acknowledgment, even if a message is successfully received. Messages cannot be sent, and the controller operates in error passive mode. This mode is intended for software bit rate detection and hot plugging."]
    Enabled = 1,
}
impl From<Lom> for bool {
    #[inline(always)]
    fn from(variant: Lom) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `LOM` reader - Listen Only Mode."]
pub type LomR = crate::BitReader<Lom>;
impl LomR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Lom {
        match self.bits {
            false => Lom::Disabled,
            true => Lom::Enabled,
        }
    }
    #[doc = "Normal. The CAN controller acknowledges a successfully received message on the CAN bus. The error counters are stopped at the current value."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lom::Disabled
    }
    #[doc = "Listen only. The controller gives no acknowledgment, even if a message is successfully received. Messages cannot be sent, and the controller operates in error passive mode. This mode is intended for software bit rate detection and hot plugging."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Lom::Enabled
    }
}
#[doc = "Field `LOM` writer - Listen Only Mode."]
pub type LomW<'a, REG> = crate::BitWriter<'a, REG, Lom>;
impl<'a, REG> LomW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. The CAN controller acknowledges a successfully received message on the CAN bus. The error counters are stopped at the current value."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lom::Disabled)
    }
    #[doc = "Listen only. The controller gives no acknowledgment, even if a message is successfully received. Messages cannot be sent, and the controller operates in error passive mode. This mode is intended for software bit rate detection and hot plugging."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lom::Enabled)
    }
}
#[doc = "Self Test Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stm {
    #[doc = "0: Normal. A transmitted message must be acknowledged to be considered successful."]
    Disabled = 0,
    #[doc = "1: Self test. The controller will consider a Tx message successful even if there is no acknowledgment received. In this mode a full node test is possible without any other active node on the bus using the SRR bit in CANxCMR."]
    Enabled = 1,
}
impl From<Stm> for bool {
    #[inline(always)]
    fn from(variant: Stm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STM` reader - Self Test Mode."]
pub type StmR = crate::BitReader<Stm>;
impl StmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stm {
        match self.bits {
            false => Stm::Disabled,
            true => Stm::Enabled,
        }
    }
    #[doc = "Normal. A transmitted message must be acknowledged to be considered successful."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Stm::Disabled
    }
    #[doc = "Self test. The controller will consider a Tx message successful even if there is no acknowledgment received. In this mode a full node test is possible without any other active node on the bus using the SRR bit in CANxCMR."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Stm::Enabled
    }
}
#[doc = "Field `STM` writer - Self Test Mode."]
pub type StmW<'a, REG> = crate::BitWriter<'a, REG, Stm>;
impl<'a, REG> StmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal. A transmitted message must be acknowledged to be considered successful."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stm::Disabled)
    }
    #[doc = "Self test. The controller will consider a Tx message successful even if there is no acknowledgment received. In this mode a full node test is possible without any other active node on the bus using the SRR bit in CANxCMR."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Stm::Enabled)
    }
}
#[doc = "Transmit Priority Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tpm {
    #[doc = "0: CAN ID. The transmit priority for 3 Transmit Buffers depends on the CAN Identifier."]
    CanId = 0,
    #[doc = "1: Local priority. The transmit priority for 3 Transmit Buffers depends on the contents of the Tx Priority register within the Transmit Buffer."]
    Local = 1,
}
impl From<Tpm> for bool {
    #[inline(always)]
    fn from(variant: Tpm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TPM` reader - Transmit Priority Mode."]
pub type TpmR = crate::BitReader<Tpm>;
impl TpmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tpm {
        match self.bits {
            false => Tpm::CanId,
            true => Tpm::Local,
        }
    }
    #[doc = "CAN ID. The transmit priority for 3 Transmit Buffers depends on the CAN Identifier."]
    #[inline(always)]
    pub fn is_can_id(&self) -> bool {
        *self == Tpm::CanId
    }
    #[doc = "Local priority. The transmit priority for 3 Transmit Buffers depends on the contents of the Tx Priority register within the Transmit Buffer."]
    #[inline(always)]
    pub fn is_local(&self) -> bool {
        *self == Tpm::Local
    }
}
#[doc = "Field `TPM` writer - Transmit Priority Mode."]
pub type TpmW<'a, REG> = crate::BitWriter<'a, REG, Tpm>;
impl<'a, REG> TpmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CAN ID. The transmit priority for 3 Transmit Buffers depends on the CAN Identifier."]
    #[inline(always)]
    pub fn can_id(self) -> &'a mut crate::W<REG> {
        self.variant(Tpm::CanId)
    }
    #[doc = "Local priority. The transmit priority for 3 Transmit Buffers depends on the contents of the Tx Priority register within the Transmit Buffer."]
    #[inline(always)]
    pub fn local(self) -> &'a mut crate::W<REG> {
        self.variant(Tpm::Local)
    }
}
#[doc = "Sleep Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sm {
    #[doc = "0: Wake-up. Normal operation."]
    Disabled = 0,
    #[doc = "1: Sleep. The CAN controller enters Sleep Mode if no CAN interrupt is pending and there is no bus activity. See the Sleep Mode description Section 21.8.2 on page 565."]
    Enabled = 1,
}
impl From<Sm> for bool {
    #[inline(always)]
    fn from(variant: Sm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SM` reader - Sleep Mode."]
pub type SmR = crate::BitReader<Sm>;
impl SmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sm {
        match self.bits {
            false => Sm::Disabled,
            true => Sm::Enabled,
        }
    }
    #[doc = "Wake-up. Normal operation."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sm::Disabled
    }
    #[doc = "Sleep. The CAN controller enters Sleep Mode if no CAN interrupt is pending and there is no bus activity. See the Sleep Mode description Section 21.8.2 on page 565."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sm::Enabled
    }
}
#[doc = "Field `SM` writer - Sleep Mode."]
pub type SmW<'a, REG> = crate::BitWriter<'a, REG, Sm>;
impl<'a, REG> SmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Wake-up. Normal operation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sm::Disabled)
    }
    #[doc = "Sleep. The CAN controller enters Sleep Mode if no CAN interrupt is pending and there is no bus activity. See the Sleep Mode description Section 21.8.2 on page 565."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sm::Enabled)
    }
}
#[doc = "Receive Polarity Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rpm {
    #[doc = "0: Low active. RD input is active Low (dominant bit = 0)."]
    ActiveLow = 0,
    #[doc = "1: High active. RD input is active High (dominant bit = 1) -- reverse polarity."]
    ActiveHigh = 1,
}
impl From<Rpm> for bool {
    #[inline(always)]
    fn from(variant: Rpm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RPM` reader - Receive Polarity Mode."]
pub type RpmR = crate::BitReader<Rpm>;
impl RpmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rpm {
        match self.bits {
            false => Rpm::ActiveLow,
            true => Rpm::ActiveHigh,
        }
    }
    #[doc = "Low active. RD input is active Low (dominant bit = 0)."]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == Rpm::ActiveLow
    }
    #[doc = "High active. RD input is active High (dominant bit = 1) -- reverse polarity."]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == Rpm::ActiveHigh
    }
}
#[doc = "Field `RPM` writer - Receive Polarity Mode."]
pub type RpmW<'a, REG> = crate::BitWriter<'a, REG, Rpm>;
impl<'a, REG> RpmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low active. RD input is active Low (dominant bit = 0)."]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(Rpm::ActiveLow)
    }
    #[doc = "High active. RD input is active High (dominant bit = 1) -- reverse polarity."]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(Rpm::ActiveHigh)
    }
}
#[doc = "Test Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tm {
    #[doc = "0: Disabled. Normal operation."]
    Disabled = 0,
    #[doc = "1: Enabled. The TD pin will reflect the bit, detected on RD pin, with the next positive edge of the system clock."]
    Enabled = 1,
}
impl From<Tm> for bool {
    #[inline(always)]
    fn from(variant: Tm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TM` reader - Test Mode."]
pub type TmR = crate::BitReader<Tm>;
impl TmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tm {
        match self.bits {
            false => Tm::Disabled,
            true => Tm::Enabled,
        }
    }
    #[doc = "Disabled. Normal operation."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tm::Disabled
    }
    #[doc = "Enabled. The TD pin will reflect the bit, detected on RD pin, with the next positive edge of the system clock."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Tm::Enabled
    }
}
#[doc = "Field `TM` writer - Test Mode."]
pub type TmW<'a, REG> = crate::BitWriter<'a, REG, Tm>;
impl<'a, REG> TmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. Normal operation."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tm::Disabled)
    }
    #[doc = "Enabled. The TD pin will reflect the bit, detected on RD pin, with the next positive edge of the system clock."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Tm::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Reset Mode."]
    #[inline(always)]
    pub fn rm(&self) -> RmR {
        RmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Listen Only Mode."]
    #[inline(always)]
    pub fn lom(&self) -> LomR {
        LomR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Self Test Mode."]
    #[inline(always)]
    pub fn stm(&self) -> StmR {
        StmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit Priority Mode."]
    #[inline(always)]
    pub fn tpm(&self) -> TpmR {
        TpmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sleep Mode."]
    #[inline(always)]
    pub fn sm(&self) -> SmR {
        SmR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Receive Polarity Mode."]
    #[inline(always)]
    pub fn rpm(&self) -> RpmR {
        RpmR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Test Mode."]
    #[inline(always)]
    pub fn tm(&self) -> TmR {
        TmR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Reset Mode."]
    #[inline(always)]
    pub fn rm(&mut self) -> RmW<'_, ModSpec> {
        RmW::new(self, 0)
    }
    #[doc = "Bit 1 - Listen Only Mode."]
    #[inline(always)]
    pub fn lom(&mut self) -> LomW<'_, ModSpec> {
        LomW::new(self, 1)
    }
    #[doc = "Bit 2 - Self Test Mode."]
    #[inline(always)]
    pub fn stm(&mut self) -> StmW<'_, ModSpec> {
        StmW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit Priority Mode."]
    #[inline(always)]
    pub fn tpm(&mut self) -> TpmW<'_, ModSpec> {
        TpmW::new(self, 3)
    }
    #[doc = "Bit 4 - Sleep Mode."]
    #[inline(always)]
    pub fn sm(&mut self) -> SmW<'_, ModSpec> {
        SmW::new(self, 4)
    }
    #[doc = "Bit 5 - Receive Polarity Mode."]
    #[inline(always)]
    pub fn rpm(&mut self) -> RpmW<'_, ModSpec> {
        RpmW::new(self, 5)
    }
    #[doc = "Bit 7 - Test Mode."]
    #[inline(always)]
    pub fn tm(&mut self) -> TmW<'_, ModSpec> {
        TmW::new(self, 7)
    }
}
#[doc = "Controls the operating mode of the CAN Controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`mod_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mod_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModSpec;
impl crate::RegisterSpec for ModSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mod_::R`](R) reader structure"]
impl crate::Readable for ModSpec {}
#[doc = "`write(|w| ..)` method takes [`mod_::W`](W) writer structure"]
impl crate::Writable for ModSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MOD to value 0"]
impl crate::Resettable for ModSpec {}
