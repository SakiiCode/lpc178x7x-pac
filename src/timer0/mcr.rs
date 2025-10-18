#[doc = "Register `MCR` reader"]
pub type R = crate::R<McrSpec>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<McrSpec>;
#[doc = "Interrupt on MR0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "1: Interrupt is generated when MR0 matches the value in the TC."]
    InterruptIsGenerat = 1,
    #[doc = "0: Interrupt is disabled"]
    InterruptIsDisable = 0,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR0I` reader - Interrupt on MR0"]
pub type Mr0iR = crate::BitReader<Enum>;
impl Mr0iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            true => Enum::InterruptIsGenerat,
            false => Enum::InterruptIsDisable,
        }
    }
    #[doc = "Interrupt is generated when MR0 matches the value in the TC."]
    #[inline(always)]
    pub fn is_interrupt_is_generat(&self) -> bool {
        *self == Enum::InterruptIsGenerat
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_interrupt_is_disable(&self) -> bool {
        *self == Enum::InterruptIsDisable
    }
}
#[doc = "Field `MR0I` writer - Interrupt on MR0"]
pub type Mr0iW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Mr0iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated when MR0 matches the value in the TC."]
    #[inline(always)]
    pub fn interrupt_is_generat(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::InterruptIsGenerat)
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn interrupt_is_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::InterruptIsDisable)
    }
}
#[doc = "Reset on MR0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "1: TC will be reset if MR0 matches it."]
    TcWillBeResetIf_ = 1,
    #[doc = "0: Feature disabled."]
    FeatureDisabled_ = 0,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR0R` reader - Reset on MR0"]
pub type Mr0rR = crate::BitReader<Enum>;
impl Mr0rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            true => Enum::TcWillBeResetIf_,
            false => Enum::FeatureDisabled_,
        }
    }
    #[doc = "TC will be reset if MR0 matches it."]
    #[inline(always)]
    pub fn is_tc_will_be_reset_if_(&self) -> bool {
        *self == Enum::TcWillBeResetIf_
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn is_feature_disabled_(&self) -> bool {
        *self == Enum::FeatureDisabled_
    }
}
#[doc = "Field `MR0R` writer - Reset on MR0"]
pub type Mr0rW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Mr0rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC will be reset if MR0 matches it."]
    #[inline(always)]
    pub fn tc_will_be_reset_if_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::TcWillBeResetIf_)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn feature_disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::FeatureDisabled_)
    }
}
#[doc = "Stop on MR0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "1: TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR0 matches the TC."]
    TcAndPcWillBeSt = 1,
    #[doc = "0: Feature disabled."]
    FeatureDisabled_ = 0,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR0S` reader - Stop on MR0"]
pub type Mr0sR = crate::BitReader<Enum>;
impl Mr0sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            true => Enum::TcAndPcWillBeSt,
            false => Enum::FeatureDisabled_,
        }
    }
    #[doc = "TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR0 matches the TC."]
    #[inline(always)]
    pub fn is_tc_and_pc_will_be_st(&self) -> bool {
        *self == Enum::TcAndPcWillBeSt
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn is_feature_disabled_(&self) -> bool {
        *self == Enum::FeatureDisabled_
    }
}
#[doc = "Field `MR0S` writer - Stop on MR0"]
pub type Mr0sW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Mr0sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR0 matches the TC."]
    #[inline(always)]
    pub fn tc_and_pc_will_be_st(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::TcAndPcWillBeSt)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn feature_disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::FeatureDisabled_)
    }
}
#[doc = "Interrupt on MR1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "1: Interrupt is generated when MR1 matches the value in the TC."]
    InterruptIsGenerat = 1,
    #[doc = "0: Interrupt is disabled."]
    InterruptIsDisable = 0,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR1I` reader - Interrupt on MR1"]
pub type Mr1iR = crate::BitReader<Enum>;
impl Mr1iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            true => Enum::InterruptIsGenerat,
            false => Enum::InterruptIsDisable,
        }
    }
    #[doc = "Interrupt is generated when MR1 matches the value in the TC."]
    #[inline(always)]
    pub fn is_interrupt_is_generat(&self) -> bool {
        *self == Enum::InterruptIsGenerat
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn is_interrupt_is_disable(&self) -> bool {
        *self == Enum::InterruptIsDisable
    }
}
#[doc = "Field `MR1I` writer - Interrupt on MR1"]
pub type Mr1iW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Mr1iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated when MR1 matches the value in the TC."]
    #[inline(always)]
    pub fn interrupt_is_generat(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::InterruptIsGenerat)
    }
    #[doc = "Interrupt is disabled."]
    #[inline(always)]
    pub fn interrupt_is_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::InterruptIsDisable)
    }
}
#[doc = "Reset on MR1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "1: TC will be reset if MR1 matches it."]
    TcWillBeResetIf_ = 1,
    #[doc = "0: Feature disabled."]
    FeatureDisabled_ = 0,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR1R` reader - Reset on MR1"]
pub type Mr1rR = crate::BitReader<Enum>;
impl Mr1rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            true => Enum::TcWillBeResetIf_,
            false => Enum::FeatureDisabled_,
        }
    }
    #[doc = "TC will be reset if MR1 matches it."]
    #[inline(always)]
    pub fn is_tc_will_be_reset_if_(&self) -> bool {
        *self == Enum::TcWillBeResetIf_
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn is_feature_disabled_(&self) -> bool {
        *self == Enum::FeatureDisabled_
    }
}
#[doc = "Field `MR1R` writer - Reset on MR1"]
pub type Mr1rW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Mr1rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC will be reset if MR1 matches it."]
    #[inline(always)]
    pub fn tc_will_be_reset_if_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::TcWillBeResetIf_)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn feature_disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::FeatureDisabled_)
    }
}
#[doc = "Stop on MR1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "1: TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR1 matches the TC."]
    TcAndPcWillBeSt = 1,
    #[doc = "0: Feature disabled."]
    FeatureDisabled_ = 0,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR1S` reader - Stop on MR1"]
pub type Mr1sR = crate::BitReader<Enum>;
impl Mr1sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            true => Enum::TcAndPcWillBeSt,
            false => Enum::FeatureDisabled_,
        }
    }
    #[doc = "TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR1 matches the TC."]
    #[inline(always)]
    pub fn is_tc_and_pc_will_be_st(&self) -> bool {
        *self == Enum::TcAndPcWillBeSt
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn is_feature_disabled_(&self) -> bool {
        *self == Enum::FeatureDisabled_
    }
}
#[doc = "Field `MR1S` writer - Stop on MR1"]
pub type Mr1sW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Mr1sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR1 matches the TC."]
    #[inline(always)]
    pub fn tc_and_pc_will_be_st(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::TcAndPcWillBeSt)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn feature_disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::FeatureDisabled_)
    }
}
#[doc = "Interrupt on MR2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "1: Interrupt is generated when MR2 matches the value in the TC."]
    InterruptIsGenerat = 1,
    #[doc = "0: Interrupt is disabled"]
    InterruptIsDisable = 0,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR2I` reader - Interrupt on MR2"]
pub type Mr2iR = crate::BitReader<Enum>;
impl Mr2iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            true => Enum::InterruptIsGenerat,
            false => Enum::InterruptIsDisable,
        }
    }
    #[doc = "Interrupt is generated when MR2 matches the value in the TC."]
    #[inline(always)]
    pub fn is_interrupt_is_generat(&self) -> bool {
        *self == Enum::InterruptIsGenerat
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn is_interrupt_is_disable(&self) -> bool {
        *self == Enum::InterruptIsDisable
    }
}
#[doc = "Field `MR2I` writer - Interrupt on MR2"]
pub type Mr2iW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Mr2iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated when MR2 matches the value in the TC."]
    #[inline(always)]
    pub fn interrupt_is_generat(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::InterruptIsGenerat)
    }
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn interrupt_is_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::InterruptIsDisable)
    }
}
#[doc = "Reset on MR2\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "1: TC will be reset if MR2 matches it."]
    TcWillBeResetIf_ = 1,
    #[doc = "0: Feature disabled."]
    FeatureDisabled_ = 0,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR2R` reader - Reset on MR2"]
pub type Mr2rR = crate::BitReader<Enum>;
impl Mr2rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            true => Enum::TcWillBeResetIf_,
            false => Enum::FeatureDisabled_,
        }
    }
    #[doc = "TC will be reset if MR2 matches it."]
    #[inline(always)]
    pub fn is_tc_will_be_reset_if_(&self) -> bool {
        *self == Enum::TcWillBeResetIf_
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn is_feature_disabled_(&self) -> bool {
        *self == Enum::FeatureDisabled_
    }
}
#[doc = "Field `MR2R` writer - Reset on MR2"]
pub type Mr2rW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Mr2rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC will be reset if MR2 matches it."]
    #[inline(always)]
    pub fn tc_will_be_reset_if_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::TcWillBeResetIf_)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn feature_disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::FeatureDisabled_)
    }
}
#[doc = "Stop on MR2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "1: TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR2 matches the TC"]
    TcAndPcWillBeSt = 1,
    #[doc = "0: Feature disabled."]
    FeatureDisabled_ = 0,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR2S` reader - Stop on MR2."]
pub type Mr2sR = crate::BitReader<Enum>;
impl Mr2sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            true => Enum::TcAndPcWillBeSt,
            false => Enum::FeatureDisabled_,
        }
    }
    #[doc = "TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR2 matches the TC"]
    #[inline(always)]
    pub fn is_tc_and_pc_will_be_st(&self) -> bool {
        *self == Enum::TcAndPcWillBeSt
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn is_feature_disabled_(&self) -> bool {
        *self == Enum::FeatureDisabled_
    }
}
#[doc = "Field `MR2S` writer - Stop on MR2."]
pub type Mr2sW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Mr2sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR2 matches the TC"]
    #[inline(always)]
    pub fn tc_and_pc_will_be_st(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::TcAndPcWillBeSt)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn feature_disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::FeatureDisabled_)
    }
}
#[doc = "Interrupt on MR3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "1: Interrupt is generated when MR3 matches the value in the TC."]
    InterruptIsGenerat = 1,
    #[doc = "0: This interrupt is disabled"]
    ThisInterruptIsDi = 0,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR3I` reader - Interrupt on MR3"]
pub type Mr3iR = crate::BitReader<Enum>;
impl Mr3iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            true => Enum::InterruptIsGenerat,
            false => Enum::ThisInterruptIsDi,
        }
    }
    #[doc = "Interrupt is generated when MR3 matches the value in the TC."]
    #[inline(always)]
    pub fn is_interrupt_is_generat(&self) -> bool {
        *self == Enum::InterruptIsGenerat
    }
    #[doc = "This interrupt is disabled"]
    #[inline(always)]
    pub fn is_this_interrupt_is_di(&self) -> bool {
        *self == Enum::ThisInterruptIsDi
    }
}
#[doc = "Field `MR3I` writer - Interrupt on MR3"]
pub type Mr3iW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Mr3iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Interrupt is generated when MR3 matches the value in the TC."]
    #[inline(always)]
    pub fn interrupt_is_generat(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::InterruptIsGenerat)
    }
    #[doc = "This interrupt is disabled"]
    #[inline(always)]
    pub fn this_interrupt_is_di(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::ThisInterruptIsDi)
    }
}
#[doc = "Reset on MR3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "1: TC will be reset if MR3 matches it."]
    TcWillBeResetIf_ = 1,
    #[doc = "0: Feature disabled."]
    FeatureDisabled_ = 0,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR3R` reader - Reset on MR3"]
pub type Mr3rR = crate::BitReader<Enum>;
impl Mr3rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            true => Enum::TcWillBeResetIf_,
            false => Enum::FeatureDisabled_,
        }
    }
    #[doc = "TC will be reset if MR3 matches it."]
    #[inline(always)]
    pub fn is_tc_will_be_reset_if_(&self) -> bool {
        *self == Enum::TcWillBeResetIf_
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn is_feature_disabled_(&self) -> bool {
        *self == Enum::FeatureDisabled_
    }
}
#[doc = "Field `MR3R` writer - Reset on MR3"]
pub type Mr3rW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Mr3rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC will be reset if MR3 matches it."]
    #[inline(always)]
    pub fn tc_will_be_reset_if_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::TcWillBeResetIf_)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn feature_disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::FeatureDisabled_)
    }
}
#[doc = "Stop on MR3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "1: TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR3 matches the TC."]
    TcAndPcWillBeSt = 1,
    #[doc = "0: Feature disabled."]
    FeatureDisabled_ = 0,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MR3S` reader - Stop on MR3"]
pub type Mr3sR = crate::BitReader<Enum>;
impl Mr3sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            true => Enum::TcAndPcWillBeSt,
            false => Enum::FeatureDisabled_,
        }
    }
    #[doc = "TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR3 matches the TC."]
    #[inline(always)]
    pub fn is_tc_and_pc_will_be_st(&self) -> bool {
        *self == Enum::TcAndPcWillBeSt
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn is_feature_disabled_(&self) -> bool {
        *self == Enum::FeatureDisabled_
    }
}
#[doc = "Field `MR3S` writer - Stop on MR3"]
pub type Mr3sW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Mr3sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TC and PC will be stopped and TCR\\[0\\] will be set to 0 if MR3 matches the TC."]
    #[inline(always)]
    pub fn tc_and_pc_will_be_st(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::TcAndPcWillBeSt)
    }
    #[doc = "Feature disabled."]
    #[inline(always)]
    pub fn feature_disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::FeatureDisabled_)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt on MR0"]
    #[inline(always)]
    pub fn mr0i(&self) -> Mr0iR {
        Mr0iR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset on MR0"]
    #[inline(always)]
    pub fn mr0r(&self) -> Mr0rR {
        Mr0rR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop on MR0"]
    #[inline(always)]
    pub fn mr0s(&self) -> Mr0sR {
        Mr0sR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt on MR1"]
    #[inline(always)]
    pub fn mr1i(&self) -> Mr1iR {
        Mr1iR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset on MR1"]
    #[inline(always)]
    pub fn mr1r(&self) -> Mr1rR {
        Mr1rR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop on MR1"]
    #[inline(always)]
    pub fn mr1s(&self) -> Mr1sR {
        Mr1sR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt on MR2"]
    #[inline(always)]
    pub fn mr2i(&self) -> Mr2iR {
        Mr2iR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset on MR2"]
    #[inline(always)]
    pub fn mr2r(&self) -> Mr2rR {
        Mr2rR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Stop on MR2."]
    #[inline(always)]
    pub fn mr2s(&self) -> Mr2sR {
        Mr2sR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt on MR3"]
    #[inline(always)]
    pub fn mr3i(&self) -> Mr3iR {
        Mr3iR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reset on MR3"]
    #[inline(always)]
    pub fn mr3r(&self) -> Mr3rR {
        Mr3rR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Stop on MR3"]
    #[inline(always)]
    pub fn mr3s(&self) -> Mr3sR {
        Mr3sR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt on MR0"]
    #[inline(always)]
    pub fn mr0i(&mut self) -> Mr0iW<'_, McrSpec> {
        Mr0iW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset on MR0"]
    #[inline(always)]
    pub fn mr0r(&mut self) -> Mr0rW<'_, McrSpec> {
        Mr0rW::new(self, 1)
    }
    #[doc = "Bit 2 - Stop on MR0"]
    #[inline(always)]
    pub fn mr0s(&mut self) -> Mr0sW<'_, McrSpec> {
        Mr0sW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt on MR1"]
    #[inline(always)]
    pub fn mr1i(&mut self) -> Mr1iW<'_, McrSpec> {
        Mr1iW::new(self, 3)
    }
    #[doc = "Bit 4 - Reset on MR1"]
    #[inline(always)]
    pub fn mr1r(&mut self) -> Mr1rW<'_, McrSpec> {
        Mr1rW::new(self, 4)
    }
    #[doc = "Bit 5 - Stop on MR1"]
    #[inline(always)]
    pub fn mr1s(&mut self) -> Mr1sW<'_, McrSpec> {
        Mr1sW::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt on MR2"]
    #[inline(always)]
    pub fn mr2i(&mut self) -> Mr2iW<'_, McrSpec> {
        Mr2iW::new(self, 6)
    }
    #[doc = "Bit 7 - Reset on MR2"]
    #[inline(always)]
    pub fn mr2r(&mut self) -> Mr2rW<'_, McrSpec> {
        Mr2rW::new(self, 7)
    }
    #[doc = "Bit 8 - Stop on MR2."]
    #[inline(always)]
    pub fn mr2s(&mut self) -> Mr2sW<'_, McrSpec> {
        Mr2sW::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt on MR3"]
    #[inline(always)]
    pub fn mr3i(&mut self) -> Mr3iW<'_, McrSpec> {
        Mr3iW::new(self, 9)
    }
    #[doc = "Bit 10 - Reset on MR3"]
    #[inline(always)]
    pub fn mr3r(&mut self) -> Mr3rW<'_, McrSpec> {
        Mr3rW::new(self, 10)
    }
    #[doc = "Bit 11 - Stop on MR3"]
    #[inline(always)]
    pub fn mr3s(&mut self) -> Mr3sW<'_, McrSpec> {
        Mr3sW::new(self, 11)
    }
}
#[doc = "Match Control Register. The MCR is used to control if an interrupt is generated and if the TC is reset when a Match occurs.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct McrSpec;
impl crate::RegisterSpec for McrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mcr::R`](R) reader structure"]
impl crate::Readable for McrSpec {}
#[doc = "`write(|w| ..)` method takes [`mcr::W`](W) writer structure"]
impl crate::Writable for McrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MCR to value 0"]
impl crate::Resettable for McrSpec {}
