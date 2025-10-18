#[doc = "Register `MOD` reader"]
pub type R = crate::R<ModSpec>;
#[doc = "Register `MOD` writer"]
pub type W = crate::W<ModSpec>;
#[doc = "Reset Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rm {
    #[doc = "0: Normal.The CAN Controller is in the Operating Mode, and certain registers can not be written."]
    NormalTheCanContr = 0,
    #[doc = "1: Reset. CAN operation is disabled, writable registers can be written and the current transmission/reception of a message is aborted."]
    ResetCanOperation = 1,
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
            false => Rm::NormalTheCanContr,
            true => Rm::ResetCanOperation,
        }
    }
    #[doc = "Normal.The CAN Controller is in the Operating Mode, and certain registers can not be written."]
    #[inline(always)]
    pub fn is_normal_the_can_contr(&self) -> bool {
        *self == Rm::NormalTheCanContr
    }
    #[doc = "Reset. CAN operation is disabled, writable registers can be written and the current transmission/reception of a message is aborted."]
    #[inline(always)]
    pub fn is_reset_can_operation(&self) -> bool {
        *self == Rm::ResetCanOperation
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
    pub fn normal_the_can_contr(self) -> &'a mut crate::W<REG> {
        self.variant(Rm::NormalTheCanContr)
    }
    #[doc = "Reset. CAN operation is disabled, writable registers can be written and the current transmission/reception of a message is aborted."]
    #[inline(always)]
    pub fn reset_can_operation(self) -> &'a mut crate::W<REG> {
        self.variant(Rm::ResetCanOperation)
    }
}
#[doc = "Listen Only Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Lom {
    #[doc = "0: Normal. The CAN controller acknowledges a successfully received message on the CAN bus. The error counters are stopped at the current value."]
    NormalTheCanCont = 0,
    #[doc = "1: Listen only. The controller gives no acknowledgment, even if a message is successfully received. Messages cannot be sent, and the controller operates in error passive mode. This mode is intended for software bit rate detection and hot plugging."]
    ListenOnlyTheCon = 1,
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
            false => Lom::NormalTheCanCont,
            true => Lom::ListenOnlyTheCon,
        }
    }
    #[doc = "Normal. The CAN controller acknowledges a successfully received message on the CAN bus. The error counters are stopped at the current value."]
    #[inline(always)]
    pub fn is_normal_the_can_cont(&self) -> bool {
        *self == Lom::NormalTheCanCont
    }
    #[doc = "Listen only. The controller gives no acknowledgment, even if a message is successfully received. Messages cannot be sent, and the controller operates in error passive mode. This mode is intended for software bit rate detection and hot plugging."]
    #[inline(always)]
    pub fn is_listen_only_the_con(&self) -> bool {
        *self == Lom::ListenOnlyTheCon
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
    pub fn normal_the_can_cont(self) -> &'a mut crate::W<REG> {
        self.variant(Lom::NormalTheCanCont)
    }
    #[doc = "Listen only. The controller gives no acknowledgment, even if a message is successfully received. Messages cannot be sent, and the controller operates in error passive mode. This mode is intended for software bit rate detection and hot plugging."]
    #[inline(always)]
    pub fn listen_only_the_con(self) -> &'a mut crate::W<REG> {
        self.variant(Lom::ListenOnlyTheCon)
    }
}
#[doc = "Self Test Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stm {
    #[doc = "0: Normal. A transmitted message must be acknowledged to be considered successful."]
    NormalATransmitte = 0,
    #[doc = "1: Self test. The controller will consider a Tx message successful even if there is no acknowledgment received. In this mode a full node test is possible without any other active node on the bus using the SRR bit in CANxCMR."]
    SelfTestTheContr = 1,
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
            false => Stm::NormalATransmitte,
            true => Stm::SelfTestTheContr,
        }
    }
    #[doc = "Normal. A transmitted message must be acknowledged to be considered successful."]
    #[inline(always)]
    pub fn is_normal_a_transmitte(&self) -> bool {
        *self == Stm::NormalATransmitte
    }
    #[doc = "Self test. The controller will consider a Tx message successful even if there is no acknowledgment received. In this mode a full node test is possible without any other active node on the bus using the SRR bit in CANxCMR."]
    #[inline(always)]
    pub fn is_self_test_the_contr(&self) -> bool {
        *self == Stm::SelfTestTheContr
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
    pub fn normal_a_transmitte(self) -> &'a mut crate::W<REG> {
        self.variant(Stm::NormalATransmitte)
    }
    #[doc = "Self test. The controller will consider a Tx message successful even if there is no acknowledgment received. In this mode a full node test is possible without any other active node on the bus using the SRR bit in CANxCMR."]
    #[inline(always)]
    pub fn self_test_the_contr(self) -> &'a mut crate::W<REG> {
        self.variant(Stm::SelfTestTheContr)
    }
}
#[doc = "Transmit Priority Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tpm {
    #[doc = "0: CAN ID. The transmit priority for 3 Transmit Buffers depends on the CAN Identifier."]
    CanIdTheTransmit = 0,
    #[doc = "1: Local priority. The transmit priority for 3 Transmit Buffers depends on the contents of the Tx Priority register within the Transmit Buffer."]
    LocalPriorityThe_ = 1,
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
            false => Tpm::CanIdTheTransmit,
            true => Tpm::LocalPriorityThe_,
        }
    }
    #[doc = "CAN ID. The transmit priority for 3 Transmit Buffers depends on the CAN Identifier."]
    #[inline(always)]
    pub fn is_can_id_the_transmit(&self) -> bool {
        *self == Tpm::CanIdTheTransmit
    }
    #[doc = "Local priority. The transmit priority for 3 Transmit Buffers depends on the contents of the Tx Priority register within the Transmit Buffer."]
    #[inline(always)]
    pub fn is_local_priority_the_(&self) -> bool {
        *self == Tpm::LocalPriorityThe_
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
    pub fn can_id_the_transmit(self) -> &'a mut crate::W<REG> {
        self.variant(Tpm::CanIdTheTransmit)
    }
    #[doc = "Local priority. The transmit priority for 3 Transmit Buffers depends on the contents of the Tx Priority register within the Transmit Buffer."]
    #[inline(always)]
    pub fn local_priority_the_(self) -> &'a mut crate::W<REG> {
        self.variant(Tpm::LocalPriorityThe_)
    }
}
#[doc = "Sleep Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sm {
    #[doc = "0: Wake-up. Normal operation."]
    WakeUpNormalOper = 0,
    #[doc = "1: Sleep. The CAN controller enters Sleep Mode if no CAN interrupt is pending and there is no bus activity. See the Sleep Mode description Section 21.8.2 on page 565."]
    SleepTheCanContr = 1,
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
            false => Sm::WakeUpNormalOper,
            true => Sm::SleepTheCanContr,
        }
    }
    #[doc = "Wake-up. Normal operation."]
    #[inline(always)]
    pub fn is_wake_up_normal_oper(&self) -> bool {
        *self == Sm::WakeUpNormalOper
    }
    #[doc = "Sleep. The CAN controller enters Sleep Mode if no CAN interrupt is pending and there is no bus activity. See the Sleep Mode description Section 21.8.2 on page 565."]
    #[inline(always)]
    pub fn is_sleep_the_can_contr(&self) -> bool {
        *self == Sm::SleepTheCanContr
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
    pub fn wake_up_normal_oper(self) -> &'a mut crate::W<REG> {
        self.variant(Sm::WakeUpNormalOper)
    }
    #[doc = "Sleep. The CAN controller enters Sleep Mode if no CAN interrupt is pending and there is no bus activity. See the Sleep Mode description Section 21.8.2 on page 565."]
    #[inline(always)]
    pub fn sleep_the_can_contr(self) -> &'a mut crate::W<REG> {
        self.variant(Sm::SleepTheCanContr)
    }
}
#[doc = "Receive Polarity Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rpm {
    #[doc = "0: Low active. RD input is active Low (dominant bit = 0)."]
    LowActiveRdInput = 0,
    #[doc = "1: High active. RD input is active High (dominant bit = 1) -- reverse polarity."]
    HighActiveRdInpu = 1,
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
            false => Rpm::LowActiveRdInput,
            true => Rpm::HighActiveRdInpu,
        }
    }
    #[doc = "Low active. RD input is active Low (dominant bit = 0)."]
    #[inline(always)]
    pub fn is_low_active_rd_input(&self) -> bool {
        *self == Rpm::LowActiveRdInput
    }
    #[doc = "High active. RD input is active High (dominant bit = 1) -- reverse polarity."]
    #[inline(always)]
    pub fn is_high_active_rd_inpu(&self) -> bool {
        *self == Rpm::HighActiveRdInpu
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
    pub fn low_active_rd_input(self) -> &'a mut crate::W<REG> {
        self.variant(Rpm::LowActiveRdInput)
    }
    #[doc = "High active. RD input is active High (dominant bit = 1) -- reverse polarity."]
    #[inline(always)]
    pub fn high_active_rd_inpu(self) -> &'a mut crate::W<REG> {
        self.variant(Rpm::HighActiveRdInpu)
    }
}
#[doc = "Test Mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tm {
    #[doc = "0: Disabled. Normal operation."]
    DisabledNormalOpe = 0,
    #[doc = "1: Enabled. The TD pin will reflect the bit, detected on RD pin, with the next positive edge of the system clock."]
    EnabledTheTdPin_ = 1,
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
            false => Tm::DisabledNormalOpe,
            true => Tm::EnabledTheTdPin_,
        }
    }
    #[doc = "Disabled. Normal operation."]
    #[inline(always)]
    pub fn is_disabled_normal_ope(&self) -> bool {
        *self == Tm::DisabledNormalOpe
    }
    #[doc = "Enabled. The TD pin will reflect the bit, detected on RD pin, with the next positive edge of the system clock."]
    #[inline(always)]
    pub fn is_enabled_the_td_pin_(&self) -> bool {
        *self == Tm::EnabledTheTdPin_
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
    pub fn disabled_normal_ope(self) -> &'a mut crate::W<REG> {
        self.variant(Tm::DisabledNormalOpe)
    }
    #[doc = "Enabled. The TD pin will reflect the bit, detected on RD pin, with the next positive edge of the system clock."]
    #[inline(always)]
    pub fn enabled_the_td_pin_(self) -> &'a mut crate::W<REG> {
        self.variant(Tm::EnabledTheTdPin_)
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
