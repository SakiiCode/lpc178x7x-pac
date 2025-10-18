#[doc = "Register `MCR` reader"]
pub type R = crate::R<McrSpec>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<McrSpec>;
#[doc = "Interrupt PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disabled."]
    Disabled_ = 0,
    #[doc = "1: Interrupt on PWMMR0: an interrupt is generated when PWMMR0 matches the value in the PWMTC."]
    InterruptOnPwmmr0 = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR0I` reader - Interrupt PWM0"]
pub type Pwmmr0iR = crate::BitReader<Enum>;
impl Pwmmr0iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disabled_,
            true => Enum::InterruptOnPwmmr0,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == Enum::Disabled_
    }
    #[doc = "Interrupt on PWMMR0: an interrupt is generated when PWMMR0 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr0(&self) -> bool {
        *self == Enum::InterruptOnPwmmr0
    }
}
#[doc = "Field `PWMMR0I` writer - Interrupt PWM0"]
pub type Pwmmr0iW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Pwmmr0iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled_)
    }
    #[doc = "Interrupt on PWMMR0: an interrupt is generated when PWMMR0 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr0(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::InterruptOnPwmmr0)
    }
}
#[doc = "Reset PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disabled."]
    Disabled_ = 0,
    #[doc = "1: Reset on PWMMR0: the PWMTC will be reset if PWMMR0 matches it."]
    ResetOnPwmmr0The = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR0R` reader - Reset PWM0"]
pub type Pwmmr0rR = crate::BitReader<Enum>;
impl Pwmmr0rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disabled_,
            true => Enum::ResetOnPwmmr0The,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == Enum::Disabled_
    }
    #[doc = "Reset on PWMMR0: the PWMTC will be reset if PWMMR0 matches it."]
    #[inline(always)]
    pub fn is_reset_on_pwmmr0_the(&self) -> bool {
        *self == Enum::ResetOnPwmmr0The
    }
}
#[doc = "Field `PWMMR0R` writer - Reset PWM0"]
pub type Pwmmr0rW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Pwmmr0rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled_)
    }
    #[doc = "Reset on PWMMR0: the PWMTC will be reset if PWMMR0 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr0_the(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::ResetOnPwmmr0The)
    }
}
#[doc = "Stop PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Stop on PWMMR0: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    StopOnPwmmr0The_ = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR0S` reader - Stop PWM0"]
pub type Pwmmr0sR = crate::BitReader<Enum>;
impl Pwmmr0sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disabled,
            true => Enum::StopOnPwmmr0The_,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enum::Disabled
    }
    #[doc = "Stop on PWMMR0: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    #[inline(always)]
    pub fn is_stop_on_pwmmr0_the_(&self) -> bool {
        *self == Enum::StopOnPwmmr0The_
    }
}
#[doc = "Field `PWMMR0S` writer - Stop PWM0"]
pub type Pwmmr0sW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Pwmmr0sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled)
    }
    #[doc = "Stop on PWMMR0: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr0_the_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::StopOnPwmmr0The_)
    }
}
#[doc = "Interrupt PWM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disabled."]
    Disabled_ = 0,
    #[doc = "1: Interrupt on PWMMR1: an interrupt is generated when PWMMR1 matches the value in the PWMTC."]
    InterruptOnPwmmr1 = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR1I` reader - Interrupt PWM1"]
pub type Pwmmr1iR = crate::BitReader<Enum>;
impl Pwmmr1iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disabled_,
            true => Enum::InterruptOnPwmmr1,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == Enum::Disabled_
    }
    #[doc = "Interrupt on PWMMR1: an interrupt is generated when PWMMR1 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr1(&self) -> bool {
        *self == Enum::InterruptOnPwmmr1
    }
}
#[doc = "Field `PWMMR1I` writer - Interrupt PWM1"]
pub type Pwmmr1iW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Pwmmr1iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled_)
    }
    #[doc = "Interrupt on PWMMR1: an interrupt is generated when PWMMR1 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr1(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::InterruptOnPwmmr1)
    }
}
#[doc = "Reset PWM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disabled."]
    Disabled_ = 0,
    #[doc = "1: Reset on PWMMR1: the PWMTC will be reset if PWMMR1 matches it."]
    ResetOnPwmmr1The = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR1R` reader - Reset PWM1"]
pub type Pwmmr1rR = crate::BitReader<Enum>;
impl Pwmmr1rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disabled_,
            true => Enum::ResetOnPwmmr1The,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == Enum::Disabled_
    }
    #[doc = "Reset on PWMMR1: the PWMTC will be reset if PWMMR1 matches it."]
    #[inline(always)]
    pub fn is_reset_on_pwmmr1_the(&self) -> bool {
        *self == Enum::ResetOnPwmmr1The
    }
}
#[doc = "Field `PWMMR1R` writer - Reset PWM1"]
pub type Pwmmr1rW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Pwmmr1rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled_)
    }
    #[doc = "Reset on PWMMR1: the PWMTC will be reset if PWMMR1 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr1_the(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::ResetOnPwmmr1The)
    }
}
#[doc = "Stop PWM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Stop on PWMMR1: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR1 matches the PWMTC."]
    StopOnPwmmr1The_ = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR1S` reader - Stop PWM1"]
pub type Pwmmr1sR = crate::BitReader<Enum>;
impl Pwmmr1sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disabled,
            true => Enum::StopOnPwmmr1The_,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enum::Disabled
    }
    #[doc = "Stop on PWMMR1: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR1 matches the PWMTC."]
    #[inline(always)]
    pub fn is_stop_on_pwmmr1_the_(&self) -> bool {
        *self == Enum::StopOnPwmmr1The_
    }
}
#[doc = "Field `PWMMR1S` writer - Stop PWM1"]
pub type Pwmmr1sW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Pwmmr1sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled)
    }
    #[doc = "Stop on PWMMR1: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR1 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr1_the_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::StopOnPwmmr1The_)
    }
}
#[doc = "Interrupt PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disabled."]
    Disabled_ = 0,
    #[doc = "1: Interrupt on PWMMR2: an interrupt is generated when PWMMR2 matches the value in the PWMTC."]
    InterruptOnPwmmr2 = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR2I` reader - Interrupt PWM0"]
pub type Pwmmr2iR = crate::BitReader<Enum>;
impl Pwmmr2iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disabled_,
            true => Enum::InterruptOnPwmmr2,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == Enum::Disabled_
    }
    #[doc = "Interrupt on PWMMR2: an interrupt is generated when PWMMR2 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr2(&self) -> bool {
        *self == Enum::InterruptOnPwmmr2
    }
}
#[doc = "Field `PWMMR2I` writer - Interrupt PWM0"]
pub type Pwmmr2iW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Pwmmr2iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled_)
    }
    #[doc = "Interrupt on PWMMR2: an interrupt is generated when PWMMR2 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr2(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::InterruptOnPwmmr2)
    }
}
#[doc = "Reset PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disabled."]
    Disabled_ = 0,
    #[doc = "1: Reset on PWMMR2: the PWMTC will be reset if PWMMR2 matches it."]
    ResetOnPwmmr2The = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR2R` reader - Reset PWM0"]
pub type Pwmmr2rR = crate::BitReader<Enum>;
impl Pwmmr2rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disabled_,
            true => Enum::ResetOnPwmmr2The,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == Enum::Disabled_
    }
    #[doc = "Reset on PWMMR2: the PWMTC will be reset if PWMMR2 matches it."]
    #[inline(always)]
    pub fn is_reset_on_pwmmr2_the(&self) -> bool {
        *self == Enum::ResetOnPwmmr2The
    }
}
#[doc = "Field `PWMMR2R` writer - Reset PWM0"]
pub type Pwmmr2rW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Pwmmr2rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled_)
    }
    #[doc = "Reset on PWMMR2: the PWMTC will be reset if PWMMR2 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr2_the(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::ResetOnPwmmr2The)
    }
}
#[doc = "Stop PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Stop on PWMMR2: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    StopOnPwmmr2The_ = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR2S` reader - Stop PWM0"]
pub type Pwmmr2sR = crate::BitReader<Enum>;
impl Pwmmr2sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disabled,
            true => Enum::StopOnPwmmr2The_,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enum::Disabled
    }
    #[doc = "Stop on PWMMR2: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    #[inline(always)]
    pub fn is_stop_on_pwmmr2_the_(&self) -> bool {
        *self == Enum::StopOnPwmmr2The_
    }
}
#[doc = "Field `PWMMR2S` writer - Stop PWM0"]
pub type Pwmmr2sW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Pwmmr2sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled)
    }
    #[doc = "Stop on PWMMR2: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr2_the_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::StopOnPwmmr2The_)
    }
}
#[doc = "Interrupt PWM3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disabled."]
    Disabled_ = 0,
    #[doc = "1: Interrupt on PWMMR3: an interrupt is generated when PWMMR3 matches the value in the PWMTC."]
    InterruptOnPwmmr3 = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR3I` reader - Interrupt PWM3"]
pub type Pwmmr3iR = crate::BitReader<Enum>;
impl Pwmmr3iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disabled_,
            true => Enum::InterruptOnPwmmr3,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == Enum::Disabled_
    }
    #[doc = "Interrupt on PWMMR3: an interrupt is generated when PWMMR3 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr3(&self) -> bool {
        *self == Enum::InterruptOnPwmmr3
    }
}
#[doc = "Field `PWMMR3I` writer - Interrupt PWM3"]
pub type Pwmmr3iW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Pwmmr3iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled_)
    }
    #[doc = "Interrupt on PWMMR3: an interrupt is generated when PWMMR3 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr3(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::InterruptOnPwmmr3)
    }
}
#[doc = "Reset PWM3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disabled."]
    Disabled_ = 0,
    #[doc = "1: Reset on PWMMR3: the PWMTC will be reset if PWMMR3 matches it."]
    ResetOnPwmmr3The = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR3R` reader - Reset PWM3"]
pub type Pwmmr3rR = crate::BitReader<Enum>;
impl Pwmmr3rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disabled_,
            true => Enum::ResetOnPwmmr3The,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == Enum::Disabled_
    }
    #[doc = "Reset on PWMMR3: the PWMTC will be reset if PWMMR3 matches it."]
    #[inline(always)]
    pub fn is_reset_on_pwmmr3_the(&self) -> bool {
        *self == Enum::ResetOnPwmmr3The
    }
}
#[doc = "Field `PWMMR3R` writer - Reset PWM3"]
pub type Pwmmr3rW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Pwmmr3rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled_)
    }
    #[doc = "Reset on PWMMR3: the PWMTC will be reset if PWMMR3 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr3_the(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::ResetOnPwmmr3The)
    }
}
#[doc = "Stop PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Stop on PWMMR3: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    StopOnPwmmr3The_ = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR3S` reader - Stop PWM0"]
pub type Pwmmr3sR = crate::BitReader<Enum>;
impl Pwmmr3sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disabled,
            true => Enum::StopOnPwmmr3The_,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enum::Disabled
    }
    #[doc = "Stop on PWMMR3: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    #[inline(always)]
    pub fn is_stop_on_pwmmr3_the_(&self) -> bool {
        *self == Enum::StopOnPwmmr3The_
    }
}
#[doc = "Field `PWMMR3S` writer - Stop PWM0"]
pub type Pwmmr3sW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Pwmmr3sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled)
    }
    #[doc = "Stop on PWMMR3: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr3_the_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::StopOnPwmmr3The_)
    }
}
#[doc = "Interrupt PWM4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disabled."]
    Disabled_ = 0,
    #[doc = "1: Interrupt on PWMMR4: an interrupt is generated when PWMMR4 matches the value in the PWMTC."]
    InterruptOnPwmmr4 = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR4I` reader - Interrupt PWM4"]
pub type Pwmmr4iR = crate::BitReader<Enum>;
impl Pwmmr4iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disabled_,
            true => Enum::InterruptOnPwmmr4,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == Enum::Disabled_
    }
    #[doc = "Interrupt on PWMMR4: an interrupt is generated when PWMMR4 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr4(&self) -> bool {
        *self == Enum::InterruptOnPwmmr4
    }
}
#[doc = "Field `PWMMR4I` writer - Interrupt PWM4"]
pub type Pwmmr4iW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Pwmmr4iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled_)
    }
    #[doc = "Interrupt on PWMMR4: an interrupt is generated when PWMMR4 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr4(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::InterruptOnPwmmr4)
    }
}
#[doc = "Reset PWM4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disabled."]
    Disabled_ = 0,
    #[doc = "1: Reset on PWMMR4: the PWMTC will be reset if PWMMR4 matches it."]
    ResetOnPwmmr4The = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR4R` reader - Reset PWM4"]
pub type Pwmmr4rR = crate::BitReader<Enum>;
impl Pwmmr4rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disabled_,
            true => Enum::ResetOnPwmmr4The,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == Enum::Disabled_
    }
    #[doc = "Reset on PWMMR4: the PWMTC will be reset if PWMMR4 matches it."]
    #[inline(always)]
    pub fn is_reset_on_pwmmr4_the(&self) -> bool {
        *self == Enum::ResetOnPwmmr4The
    }
}
#[doc = "Field `PWMMR4R` writer - Reset PWM4"]
pub type Pwmmr4rW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Pwmmr4rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled_)
    }
    #[doc = "Reset on PWMMR4: the PWMTC will be reset if PWMMR4 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr4_the(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::ResetOnPwmmr4The)
    }
}
#[doc = "Stop PWM4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Stop on PWMMR4: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR4 matches the PWMTC."]
    StopOnPwmmr4The_ = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR4S` reader - Stop PWM4"]
pub type Pwmmr4sR = crate::BitReader<Enum>;
impl Pwmmr4sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disabled,
            true => Enum::StopOnPwmmr4The_,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enum::Disabled
    }
    #[doc = "Stop on PWMMR4: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR4 matches the PWMTC."]
    #[inline(always)]
    pub fn is_stop_on_pwmmr4_the_(&self) -> bool {
        *self == Enum::StopOnPwmmr4The_
    }
}
#[doc = "Field `PWMMR4S` writer - Stop PWM4"]
pub type Pwmmr4sW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Pwmmr4sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled)
    }
    #[doc = "Stop on PWMMR4: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR4 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr4_the_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::StopOnPwmmr4The_)
    }
}
#[doc = "Interrupt PWM5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disabled."]
    Disabled_ = 0,
    #[doc = "1: Interrupt on PWMMR5: an interrupt is generated when PWMMR5 matches the value in the PWMTC."]
    InterruptOnPwmmr5 = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR5I` reader - Interrupt PWM5"]
pub type Pwmmr5iR = crate::BitReader<Enum>;
impl Pwmmr5iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disabled_,
            true => Enum::InterruptOnPwmmr5,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == Enum::Disabled_
    }
    #[doc = "Interrupt on PWMMR5: an interrupt is generated when PWMMR5 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr5(&self) -> bool {
        *self == Enum::InterruptOnPwmmr5
    }
}
#[doc = "Field `PWMMR5I` writer - Interrupt PWM5"]
pub type Pwmmr5iW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Pwmmr5iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled_)
    }
    #[doc = "Interrupt on PWMMR5: an interrupt is generated when PWMMR5 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr5(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::InterruptOnPwmmr5)
    }
}
#[doc = "Reset PWM5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disabled."]
    Disabled_ = 0,
    #[doc = "1: Reset on PWMMR5: the PWMTC will be reset if PWMMR5 matches it."]
    ResetOnPwmmr5The = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR5R` reader - Reset PWM5"]
pub type Pwmmr5rR = crate::BitReader<Enum>;
impl Pwmmr5rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disabled_,
            true => Enum::ResetOnPwmmr5The,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == Enum::Disabled_
    }
    #[doc = "Reset on PWMMR5: the PWMTC will be reset if PWMMR5 matches it."]
    #[inline(always)]
    pub fn is_reset_on_pwmmr5_the(&self) -> bool {
        *self == Enum::ResetOnPwmmr5The
    }
}
#[doc = "Field `PWMMR5R` writer - Reset PWM5"]
pub type Pwmmr5rW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Pwmmr5rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled_)
    }
    #[doc = "Reset on PWMMR5: the PWMTC will be reset if PWMMR5 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr5_the(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::ResetOnPwmmr5The)
    }
}
#[doc = "Stop PWM5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Stop on PWMMR5: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR5 matches the PWMTC."]
    StopOnPwmmr5The_ = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR5S` reader - Stop PWM5"]
pub type Pwmmr5sR = crate::BitReader<Enum>;
impl Pwmmr5sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disabled,
            true => Enum::StopOnPwmmr5The_,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enum::Disabled
    }
    #[doc = "Stop on PWMMR5: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR5 matches the PWMTC."]
    #[inline(always)]
    pub fn is_stop_on_pwmmr5_the_(&self) -> bool {
        *self == Enum::StopOnPwmmr5The_
    }
}
#[doc = "Field `PWMMR5S` writer - Stop PWM5"]
pub type Pwmmr5sW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Pwmmr5sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled)
    }
    #[doc = "Stop on PWMMR5: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR5 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr5_the_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::StopOnPwmmr5The_)
    }
}
#[doc = "Interrupt PWM6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disabled."]
    Disabled_ = 0,
    #[doc = "1: Interrupt on PWMMR6: an interrupt is generated when PWMMR6 matches the value in the PWMTC."]
    InterruptOnPwmmr6 = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR6I` reader - Interrupt PWM6"]
pub type Pwmmr6iR = crate::BitReader<Enum>;
impl Pwmmr6iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disabled_,
            true => Enum::InterruptOnPwmmr6,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == Enum::Disabled_
    }
    #[doc = "Interrupt on PWMMR6: an interrupt is generated when PWMMR6 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn is_interrupt_on_pwmmr6(&self) -> bool {
        *self == Enum::InterruptOnPwmmr6
    }
}
#[doc = "Field `PWMMR6I` writer - Interrupt PWM6"]
pub type Pwmmr6iW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Pwmmr6iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled_)
    }
    #[doc = "Interrupt on PWMMR6: an interrupt is generated when PWMMR6 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn interrupt_on_pwmmr6(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::InterruptOnPwmmr6)
    }
}
#[doc = "Reset PWM6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disabled."]
    Disabled_ = 0,
    #[doc = "1: Reset on PWMMR6: the PWMTC will be reset if PWMMR6 matches it."]
    ResetOnPwmmr6The = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR6R` reader - Reset PWM6"]
pub type Pwmmr6rR = crate::BitReader<Enum>;
impl Pwmmr6rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disabled_,
            true => Enum::ResetOnPwmmr6The,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == Enum::Disabled_
    }
    #[doc = "Reset on PWMMR6: the PWMTC will be reset if PWMMR6 matches it."]
    #[inline(always)]
    pub fn is_reset_on_pwmmr6_the(&self) -> bool {
        *self == Enum::ResetOnPwmmr6The
    }
}
#[doc = "Field `PWMMR6R` writer - Reset PWM6"]
pub type Pwmmr6rW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Pwmmr6rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled_)
    }
    #[doc = "Reset on PWMMR6: the PWMTC will be reset if PWMMR6 matches it."]
    #[inline(always)]
    pub fn reset_on_pwmmr6_the(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::ResetOnPwmmr6The)
    }
}
#[doc = "Stop PWM6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Stop on PWMMR6: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR6 matches the PWMTC."]
    StopOnPwmmr6The_ = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR6S` reader - Stop PWM6"]
pub type Pwmmr6sR = crate::BitReader<Enum>;
impl Pwmmr6sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disabled,
            true => Enum::StopOnPwmmr6The_,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enum::Disabled
    }
    #[doc = "Stop on PWMMR6: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR6 matches the PWMTC."]
    #[inline(always)]
    pub fn is_stop_on_pwmmr6_the_(&self) -> bool {
        *self == Enum::StopOnPwmmr6The_
    }
}
#[doc = "Field `PWMMR6S` writer - Stop PWM6"]
pub type Pwmmr6sW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Pwmmr6sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled)
    }
    #[doc = "Stop on PWMMR6: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR6 matches the PWMTC."]
    #[inline(always)]
    pub fn stop_on_pwmmr6_the_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::StopOnPwmmr6The_)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt PWM0"]
    #[inline(always)]
    pub fn pwmmr0i(&self) -> Pwmmr0iR {
        Pwmmr0iR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reset PWM0"]
    #[inline(always)]
    pub fn pwmmr0r(&self) -> Pwmmr0rR {
        Pwmmr0rR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop PWM0"]
    #[inline(always)]
    pub fn pwmmr0s(&self) -> Pwmmr0sR {
        Pwmmr0sR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt PWM1"]
    #[inline(always)]
    pub fn pwmmr1i(&self) -> Pwmmr1iR {
        Pwmmr1iR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset PWM1"]
    #[inline(always)]
    pub fn pwmmr1r(&self) -> Pwmmr1rR {
        Pwmmr1rR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop PWM1"]
    #[inline(always)]
    pub fn pwmmr1s(&self) -> Pwmmr1sR {
        Pwmmr1sR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt PWM0"]
    #[inline(always)]
    pub fn pwmmr2i(&self) -> Pwmmr2iR {
        Pwmmr2iR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reset PWM0"]
    #[inline(always)]
    pub fn pwmmr2r(&self) -> Pwmmr2rR {
        Pwmmr2rR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Stop PWM0"]
    #[inline(always)]
    pub fn pwmmr2s(&self) -> Pwmmr2sR {
        Pwmmr2sR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Interrupt PWM3"]
    #[inline(always)]
    pub fn pwmmr3i(&self) -> Pwmmr3iR {
        Pwmmr3iR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reset PWM3"]
    #[inline(always)]
    pub fn pwmmr3r(&self) -> Pwmmr3rR {
        Pwmmr3rR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Stop PWM0"]
    #[inline(always)]
    pub fn pwmmr3s(&self) -> Pwmmr3sR {
        Pwmmr3sR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt PWM4"]
    #[inline(always)]
    pub fn pwmmr4i(&self) -> Pwmmr4iR {
        Pwmmr4iR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reset PWM4"]
    #[inline(always)]
    pub fn pwmmr4r(&self) -> Pwmmr4rR {
        Pwmmr4rR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Stop PWM4"]
    #[inline(always)]
    pub fn pwmmr4s(&self) -> Pwmmr4sR {
        Pwmmr4sR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Interrupt PWM5"]
    #[inline(always)]
    pub fn pwmmr5i(&self) -> Pwmmr5iR {
        Pwmmr5iR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reset PWM5"]
    #[inline(always)]
    pub fn pwmmr5r(&self) -> Pwmmr5rR {
        Pwmmr5rR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Stop PWM5"]
    #[inline(always)]
    pub fn pwmmr5s(&self) -> Pwmmr5sR {
        Pwmmr5sR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Interrupt PWM6"]
    #[inline(always)]
    pub fn pwmmr6i(&self) -> Pwmmr6iR {
        Pwmmr6iR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reset PWM6"]
    #[inline(always)]
    pub fn pwmmr6r(&self) -> Pwmmr6rR {
        Pwmmr6rR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Stop PWM6"]
    #[inline(always)]
    pub fn pwmmr6s(&self) -> Pwmmr6sR {
        Pwmmr6sR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt PWM0"]
    #[inline(always)]
    pub fn pwmmr0i(&mut self) -> Pwmmr0iW<'_, McrSpec> {
        Pwmmr0iW::new(self, 0)
    }
    #[doc = "Bit 1 - Reset PWM0"]
    #[inline(always)]
    pub fn pwmmr0r(&mut self) -> Pwmmr0rW<'_, McrSpec> {
        Pwmmr0rW::new(self, 1)
    }
    #[doc = "Bit 2 - Stop PWM0"]
    #[inline(always)]
    pub fn pwmmr0s(&mut self) -> Pwmmr0sW<'_, McrSpec> {
        Pwmmr0sW::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt PWM1"]
    #[inline(always)]
    pub fn pwmmr1i(&mut self) -> Pwmmr1iW<'_, McrSpec> {
        Pwmmr1iW::new(self, 3)
    }
    #[doc = "Bit 4 - Reset PWM1"]
    #[inline(always)]
    pub fn pwmmr1r(&mut self) -> Pwmmr1rW<'_, McrSpec> {
        Pwmmr1rW::new(self, 4)
    }
    #[doc = "Bit 5 - Stop PWM1"]
    #[inline(always)]
    pub fn pwmmr1s(&mut self) -> Pwmmr1sW<'_, McrSpec> {
        Pwmmr1sW::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt PWM0"]
    #[inline(always)]
    pub fn pwmmr2i(&mut self) -> Pwmmr2iW<'_, McrSpec> {
        Pwmmr2iW::new(self, 6)
    }
    #[doc = "Bit 7 - Reset PWM0"]
    #[inline(always)]
    pub fn pwmmr2r(&mut self) -> Pwmmr2rW<'_, McrSpec> {
        Pwmmr2rW::new(self, 7)
    }
    #[doc = "Bit 8 - Stop PWM0"]
    #[inline(always)]
    pub fn pwmmr2s(&mut self) -> Pwmmr2sW<'_, McrSpec> {
        Pwmmr2sW::new(self, 8)
    }
    #[doc = "Bit 9 - Interrupt PWM3"]
    #[inline(always)]
    pub fn pwmmr3i(&mut self) -> Pwmmr3iW<'_, McrSpec> {
        Pwmmr3iW::new(self, 9)
    }
    #[doc = "Bit 10 - Reset PWM3"]
    #[inline(always)]
    pub fn pwmmr3r(&mut self) -> Pwmmr3rW<'_, McrSpec> {
        Pwmmr3rW::new(self, 10)
    }
    #[doc = "Bit 11 - Stop PWM0"]
    #[inline(always)]
    pub fn pwmmr3s(&mut self) -> Pwmmr3sW<'_, McrSpec> {
        Pwmmr3sW::new(self, 11)
    }
    #[doc = "Bit 12 - Interrupt PWM4"]
    #[inline(always)]
    pub fn pwmmr4i(&mut self) -> Pwmmr4iW<'_, McrSpec> {
        Pwmmr4iW::new(self, 12)
    }
    #[doc = "Bit 13 - Reset PWM4"]
    #[inline(always)]
    pub fn pwmmr4r(&mut self) -> Pwmmr4rW<'_, McrSpec> {
        Pwmmr4rW::new(self, 13)
    }
    #[doc = "Bit 14 - Stop PWM4"]
    #[inline(always)]
    pub fn pwmmr4s(&mut self) -> Pwmmr4sW<'_, McrSpec> {
        Pwmmr4sW::new(self, 14)
    }
    #[doc = "Bit 15 - Interrupt PWM5"]
    #[inline(always)]
    pub fn pwmmr5i(&mut self) -> Pwmmr5iW<'_, McrSpec> {
        Pwmmr5iW::new(self, 15)
    }
    #[doc = "Bit 16 - Reset PWM5"]
    #[inline(always)]
    pub fn pwmmr5r(&mut self) -> Pwmmr5rW<'_, McrSpec> {
        Pwmmr5rW::new(self, 16)
    }
    #[doc = "Bit 17 - Stop PWM5"]
    #[inline(always)]
    pub fn pwmmr5s(&mut self) -> Pwmmr5sW<'_, McrSpec> {
        Pwmmr5sW::new(self, 17)
    }
    #[doc = "Bit 18 - Interrupt PWM6"]
    #[inline(always)]
    pub fn pwmmr6i(&mut self) -> Pwmmr6iW<'_, McrSpec> {
        Pwmmr6iW::new(self, 18)
    }
    #[doc = "Bit 19 - Reset PWM6"]
    #[inline(always)]
    pub fn pwmmr6r(&mut self) -> Pwmmr6rW<'_, McrSpec> {
        Pwmmr6rW::new(self, 19)
    }
    #[doc = "Bit 20 - Stop PWM6"]
    #[inline(always)]
    pub fn pwmmr6s(&mut self) -> Pwmmr6sW<'_, McrSpec> {
        Pwmmr6sW::new(self, 20)
    }
}
#[doc = "Match Control Register. The MCR is used to control whether an interrupt is generated and if the PWM counter is reset when a Match occurs.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
