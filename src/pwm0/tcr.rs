#[doc = "Register `TCR` reader"]
pub type R = crate::R<TcrSpec>;
#[doc = "Register `TCR` writer"]
pub type W = crate::W<TcrSpec>;
#[doc = "Counter Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "1: The PWM Timer Counter and PWM Prescale Counter are enabled for counting."]
    ThePwmTimerCounte = 1,
    #[doc = "0: The counters are disabled."]
    TheCountersAreDis = 0,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CE` reader - Counter Enable"]
pub type CeR = crate::BitReader<Enum>;
impl CeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            true => Enum::ThePwmTimerCounte,
            false => Enum::TheCountersAreDis,
        }
    }
    #[doc = "The PWM Timer Counter and PWM Prescale Counter are enabled for counting."]
    #[inline(always)]
    pub fn is_the_pwm_timer_counte(&self) -> bool {
        *self == Enum::ThePwmTimerCounte
    }
    #[doc = "The counters are disabled."]
    #[inline(always)]
    pub fn is_the_counters_are_dis(&self) -> bool {
        *self == Enum::TheCountersAreDis
    }
}
#[doc = "Field `CE` writer - Counter Enable"]
pub type CeW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> CeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The PWM Timer Counter and PWM Prescale Counter are enabled for counting."]
    #[inline(always)]
    pub fn the_pwm_timer_counte(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::ThePwmTimerCounte)
    }
    #[doc = "The counters are disabled."]
    #[inline(always)]
    pub fn the_counters_are_dis(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::TheCountersAreDis)
    }
}
#[doc = "Counter Reset\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "1: The PWM Timer Counter and the PWM Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until this bit is returned to zero."]
    ThePwmTimerCounte = 1,
    #[doc = "0: Clear reset."]
    ClearReset_ = 0,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CR` reader - Counter Reset"]
pub type CrR = crate::BitReader<Enum>;
impl CrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            true => Enum::ThePwmTimerCounte,
            false => Enum::ClearReset_,
        }
    }
    #[doc = "The PWM Timer Counter and the PWM Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until this bit is returned to zero."]
    #[inline(always)]
    pub fn is_the_pwm_timer_counte(&self) -> bool {
        *self == Enum::ThePwmTimerCounte
    }
    #[doc = "Clear reset."]
    #[inline(always)]
    pub fn is_clear_reset_(&self) -> bool {
        *self == Enum::ClearReset_
    }
}
#[doc = "Field `CR` writer - Counter Reset"]
pub type CrW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> CrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The PWM Timer Counter and the PWM Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until this bit is returned to zero."]
    #[inline(always)]
    pub fn the_pwm_timer_counte(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::ThePwmTimerCounte)
    }
    #[doc = "Clear reset."]
    #[inline(always)]
    pub fn clear_reset_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::ClearReset_)
    }
}
#[doc = "PWM Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "1: PWM mode is enabled (counter resets to 1). PWM mode causes the shadow registers to operate in connection with the Match registers. A program write to a Match register will not have an effect on the Match result until the corresponding bit in PWMLER has been set, followed by the occurrence of a PWM Match 0 event. Note that the PWM Match register that determines the PWM rate (PWM Match Register 0 - MR0) must be set up prior to the PWM being enabled. Otherwise a Match event will not occur to cause shadow register contents to become effective."]
    PwmModeIsEnabled_ = 1,
    #[doc = "0: Timer mode is enabled (counter resets to 0)."]
    TimerModeIsEnable = 0,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMEN` reader - PWM Enable"]
pub type PwmenR = crate::BitReader<Enum>;
impl PwmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            true => Enum::PwmModeIsEnabled_,
            false => Enum::TimerModeIsEnable,
        }
    }
    #[doc = "PWM mode is enabled (counter resets to 1). PWM mode causes the shadow registers to operate in connection with the Match registers. A program write to a Match register will not have an effect on the Match result until the corresponding bit in PWMLER has been set, followed by the occurrence of a PWM Match 0 event. Note that the PWM Match register that determines the PWM rate (PWM Match Register 0 - MR0) must be set up prior to the PWM being enabled. Otherwise a Match event will not occur to cause shadow register contents to become effective."]
    #[inline(always)]
    pub fn is_pwm_mode_is_enabled_(&self) -> bool {
        *self == Enum::PwmModeIsEnabled_
    }
    #[doc = "Timer mode is enabled (counter resets to 0)."]
    #[inline(always)]
    pub fn is_timer_mode_is_enable(&self) -> bool {
        *self == Enum::TimerModeIsEnable
    }
}
#[doc = "Field `PWMEN` writer - PWM Enable"]
pub type PwmenW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> PwmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "PWM mode is enabled (counter resets to 1). PWM mode causes the shadow registers to operate in connection with the Match registers. A program write to a Match register will not have an effect on the Match result until the corresponding bit in PWMLER has been set, followed by the occurrence of a PWM Match 0 event. Note that the PWM Match register that determines the PWM rate (PWM Match Register 0 - MR0) must be set up prior to the PWM being enabled. Otherwise a Match event will not occur to cause shadow register contents to become effective."]
    #[inline(always)]
    pub fn pwm_mode_is_enabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::PwmModeIsEnabled_)
    }
    #[doc = "Timer mode is enabled (counter resets to 0)."]
    #[inline(always)]
    pub fn timer_mode_is_enable(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::TimerModeIsEnable)
    }
}
#[doc = "Master Disable (PWM0 only). The two PWMs may be synchronized using the Master Disable control bit. The Master disable bit of the Master PWM (PWM0 module) controls a secondary enable input to both PWMs, as shown in Figure 141. This bit has no function in the Slave PWM (PWM1).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "1: Master use. PWM0 is the master, and both PWMs are enabled for counting."]
    MasterUsePwm0Is_ = 1,
    #[doc = "0: Individual use. The PWMs are used independently, and the individual Counter Enable bits are used to control the PWMs."]
    IndividualUseThe_ = 0,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MDIS` reader - Master Disable (PWM0 only). The two PWMs may be synchronized using the Master Disable control bit. The Master disable bit of the Master PWM (PWM0 module) controls a secondary enable input to both PWMs, as shown in Figure 141. This bit has no function in the Slave PWM (PWM1)."]
pub type MdisR = crate::BitReader<Enum>;
impl MdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            true => Enum::MasterUsePwm0Is_,
            false => Enum::IndividualUseThe_,
        }
    }
    #[doc = "Master use. PWM0 is the master, and both PWMs are enabled for counting."]
    #[inline(always)]
    pub fn is_master_use_pwm0_is_(&self) -> bool {
        *self == Enum::MasterUsePwm0Is_
    }
    #[doc = "Individual use. The PWMs are used independently, and the individual Counter Enable bits are used to control the PWMs."]
    #[inline(always)]
    pub fn is_individual_use_the_(&self) -> bool {
        *self == Enum::IndividualUseThe_
    }
}
#[doc = "Field `MDIS` writer - Master Disable (PWM0 only). The two PWMs may be synchronized using the Master Disable control bit. The Master disable bit of the Master PWM (PWM0 module) controls a secondary enable input to both PWMs, as shown in Figure 141. This bit has no function in the Slave PWM (PWM1)."]
pub type MdisW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> MdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Master use. PWM0 is the master, and both PWMs are enabled for counting."]
    #[inline(always)]
    pub fn master_use_pwm0_is_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::MasterUsePwm0Is_)
    }
    #[doc = "Individual use. The PWMs are used independently, and the individual Counter Enable bits are used to control the PWMs."]
    #[inline(always)]
    pub fn individual_use_the_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::IndividualUseThe_)
    }
}
impl R {
    #[doc = "Bit 0 - Counter Enable"]
    #[inline(always)]
    pub fn ce(&self) -> CeR {
        CeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter Reset"]
    #[inline(always)]
    pub fn cr(&self) -> CrR {
        CrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM Enable"]
    #[inline(always)]
    pub fn pwmen(&self) -> PwmenR {
        PwmenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Master Disable (PWM0 only). The two PWMs may be synchronized using the Master Disable control bit. The Master disable bit of the Master PWM (PWM0 module) controls a secondary enable input to both PWMs, as shown in Figure 141. This bit has no function in the Slave PWM (PWM1)."]
    #[inline(always)]
    pub fn mdis(&self) -> MdisR {
        MdisR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter Enable"]
    #[inline(always)]
    pub fn ce(&mut self) -> CeW<'_, TcrSpec> {
        CeW::new(self, 0)
    }
    #[doc = "Bit 1 - Counter Reset"]
    #[inline(always)]
    pub fn cr(&mut self) -> CrW<'_, TcrSpec> {
        CrW::new(self, 1)
    }
    #[doc = "Bit 3 - PWM Enable"]
    #[inline(always)]
    pub fn pwmen(&mut self) -> PwmenW<'_, TcrSpec> {
        PwmenW::new(self, 3)
    }
    #[doc = "Bit 4 - Master Disable (PWM0 only). The two PWMs may be synchronized using the Master Disable control bit. The Master disable bit of the Master PWM (PWM0 module) controls a secondary enable input to both PWMs, as shown in Figure 141. This bit has no function in the Slave PWM (PWM1)."]
    #[inline(always)]
    pub fn mdis(&mut self) -> MdisW<'_, TcrSpec> {
        MdisW::new(self, 4)
    }
}
#[doc = "Timer Control Register. The TCR is used to control the Timer Counter functions.\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcrSpec;
impl crate::RegisterSpec for TcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcr::R`](R) reader structure"]
impl crate::Readable for TcrSpec {}
#[doc = "`write(|w| ..)` method takes [`tcr::W`](W) writer structure"]
impl crate::Writable for TcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCR to value 0"]
impl crate::Resettable for TcrSpec {}
