#[doc = "Register `MCR` reader"]
pub type R = crate::R<McrSpec>;
#[doc = "Register `MCR` writer"]
pub type W = crate::W<McrSpec>;
#[doc = "Interrupt PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmmr0i {
    #[doc = "0: Disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt on PWMMR0: an interrupt is generated when PWMMR0 matches the value in the PWMTC."]
    Enabled = 1,
}
impl From<Pwmmr0i> for bool {
    #[inline(always)]
    fn from(variant: Pwmmr0i) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR0I` reader - Interrupt PWM0"]
pub type Pwmmr0iR = crate::BitReader<Pwmmr0i>;
impl Pwmmr0iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmmr0i {
        match self.bits {
            false => Pwmmr0i::Disabled,
            true => Pwmmr0i::Enabled,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pwmmr0i::Disabled
    }
    #[doc = "Interrupt on PWMMR0: an interrupt is generated when PWMMR0 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pwmmr0i::Enabled
    }
}
#[doc = "Field `PWMMR0I` writer - Interrupt PWM0"]
pub type Pwmmr0iW<'a, REG> = crate::BitWriter<'a, REG, Pwmmr0i>;
impl<'a, REG> Pwmmr0iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr0i::Disabled)
    }
    #[doc = "Interrupt on PWMMR0: an interrupt is generated when PWMMR0 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr0i::Enabled)
    }
}
#[doc = "Reset PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmmr0r {
    #[doc = "0: Disabled."]
    Disabled = 0,
    #[doc = "1: Reset on PWMMR0: the PWMTC will be reset if PWMMR0 matches it."]
    Enabled = 1,
}
impl From<Pwmmr0r> for bool {
    #[inline(always)]
    fn from(variant: Pwmmr0r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR0R` reader - Reset PWM0"]
pub type Pwmmr0rR = crate::BitReader<Pwmmr0r>;
impl Pwmmr0rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmmr0r {
        match self.bits {
            false => Pwmmr0r::Disabled,
            true => Pwmmr0r::Enabled,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pwmmr0r::Disabled
    }
    #[doc = "Reset on PWMMR0: the PWMTC will be reset if PWMMR0 matches it."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pwmmr0r::Enabled
    }
}
#[doc = "Field `PWMMR0R` writer - Reset PWM0"]
pub type Pwmmr0rW<'a, REG> = crate::BitWriter<'a, REG, Pwmmr0r>;
impl<'a, REG> Pwmmr0rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr0r::Disabled)
    }
    #[doc = "Reset on PWMMR0: the PWMTC will be reset if PWMMR0 matches it."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr0r::Enabled)
    }
}
#[doc = "Stop PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmmr0s {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Stop on PWMMR0: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    Enabled = 1,
}
impl From<Pwmmr0s> for bool {
    #[inline(always)]
    fn from(variant: Pwmmr0s) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR0S` reader - Stop PWM0"]
pub type Pwmmr0sR = crate::BitReader<Pwmmr0s>;
impl Pwmmr0sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmmr0s {
        match self.bits {
            false => Pwmmr0s::Disabled,
            true => Pwmmr0s::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pwmmr0s::Disabled
    }
    #[doc = "Stop on PWMMR0: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pwmmr0s::Enabled
    }
}
#[doc = "Field `PWMMR0S` writer - Stop PWM0"]
pub type Pwmmr0sW<'a, REG> = crate::BitWriter<'a, REG, Pwmmr0s>;
impl<'a, REG> Pwmmr0sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr0s::Disabled)
    }
    #[doc = "Stop on PWMMR0: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr0s::Enabled)
    }
}
#[doc = "Interrupt PWM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmmr1i {
    #[doc = "0: Disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt on PWMMR1: an interrupt is generated when PWMMR1 matches the value in the PWMTC."]
    Enabled = 1,
}
impl From<Pwmmr1i> for bool {
    #[inline(always)]
    fn from(variant: Pwmmr1i) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR1I` reader - Interrupt PWM1"]
pub type Pwmmr1iR = crate::BitReader<Pwmmr1i>;
impl Pwmmr1iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmmr1i {
        match self.bits {
            false => Pwmmr1i::Disabled,
            true => Pwmmr1i::Enabled,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pwmmr1i::Disabled
    }
    #[doc = "Interrupt on PWMMR1: an interrupt is generated when PWMMR1 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pwmmr1i::Enabled
    }
}
#[doc = "Field `PWMMR1I` writer - Interrupt PWM1"]
pub type Pwmmr1iW<'a, REG> = crate::BitWriter<'a, REG, Pwmmr1i>;
impl<'a, REG> Pwmmr1iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr1i::Disabled)
    }
    #[doc = "Interrupt on PWMMR1: an interrupt is generated when PWMMR1 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr1i::Enabled)
    }
}
#[doc = "Reset PWM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmmr1r {
    #[doc = "0: Disabled."]
    Disabled = 0,
    #[doc = "1: Reset on PWMMR1: the PWMTC will be reset if PWMMR1 matches it."]
    Enabled = 1,
}
impl From<Pwmmr1r> for bool {
    #[inline(always)]
    fn from(variant: Pwmmr1r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR1R` reader - Reset PWM1"]
pub type Pwmmr1rR = crate::BitReader<Pwmmr1r>;
impl Pwmmr1rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmmr1r {
        match self.bits {
            false => Pwmmr1r::Disabled,
            true => Pwmmr1r::Enabled,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pwmmr1r::Disabled
    }
    #[doc = "Reset on PWMMR1: the PWMTC will be reset if PWMMR1 matches it."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pwmmr1r::Enabled
    }
}
#[doc = "Field `PWMMR1R` writer - Reset PWM1"]
pub type Pwmmr1rW<'a, REG> = crate::BitWriter<'a, REG, Pwmmr1r>;
impl<'a, REG> Pwmmr1rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr1r::Disabled)
    }
    #[doc = "Reset on PWMMR1: the PWMTC will be reset if PWMMR1 matches it."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr1r::Enabled)
    }
}
#[doc = "Stop PWM1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmmr1s {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Stop on PWMMR1: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR1 matches the PWMTC."]
    Enabled = 1,
}
impl From<Pwmmr1s> for bool {
    #[inline(always)]
    fn from(variant: Pwmmr1s) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR1S` reader - Stop PWM1"]
pub type Pwmmr1sR = crate::BitReader<Pwmmr1s>;
impl Pwmmr1sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmmr1s {
        match self.bits {
            false => Pwmmr1s::Disabled,
            true => Pwmmr1s::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pwmmr1s::Disabled
    }
    #[doc = "Stop on PWMMR1: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR1 matches the PWMTC."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pwmmr1s::Enabled
    }
}
#[doc = "Field `PWMMR1S` writer - Stop PWM1"]
pub type Pwmmr1sW<'a, REG> = crate::BitWriter<'a, REG, Pwmmr1s>;
impl<'a, REG> Pwmmr1sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr1s::Disabled)
    }
    #[doc = "Stop on PWMMR1: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR1 matches the PWMTC."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr1s::Enabled)
    }
}
#[doc = "Interrupt PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmmr2i {
    #[doc = "0: Disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt on PWMMR2: an interrupt is generated when PWMMR2 matches the value in the PWMTC."]
    Enabled = 1,
}
impl From<Pwmmr2i> for bool {
    #[inline(always)]
    fn from(variant: Pwmmr2i) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR2I` reader - Interrupt PWM0"]
pub type Pwmmr2iR = crate::BitReader<Pwmmr2i>;
impl Pwmmr2iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmmr2i {
        match self.bits {
            false => Pwmmr2i::Disabled,
            true => Pwmmr2i::Enabled,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pwmmr2i::Disabled
    }
    #[doc = "Interrupt on PWMMR2: an interrupt is generated when PWMMR2 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pwmmr2i::Enabled
    }
}
#[doc = "Field `PWMMR2I` writer - Interrupt PWM0"]
pub type Pwmmr2iW<'a, REG> = crate::BitWriter<'a, REG, Pwmmr2i>;
impl<'a, REG> Pwmmr2iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr2i::Disabled)
    }
    #[doc = "Interrupt on PWMMR2: an interrupt is generated when PWMMR2 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr2i::Enabled)
    }
}
#[doc = "Reset PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmmr2r {
    #[doc = "0: Disabled."]
    Disabled = 0,
    #[doc = "1: Reset on PWMMR2: the PWMTC will be reset if PWMMR2 matches it."]
    Enabled = 1,
}
impl From<Pwmmr2r> for bool {
    #[inline(always)]
    fn from(variant: Pwmmr2r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR2R` reader - Reset PWM0"]
pub type Pwmmr2rR = crate::BitReader<Pwmmr2r>;
impl Pwmmr2rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmmr2r {
        match self.bits {
            false => Pwmmr2r::Disabled,
            true => Pwmmr2r::Enabled,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pwmmr2r::Disabled
    }
    #[doc = "Reset on PWMMR2: the PWMTC will be reset if PWMMR2 matches it."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pwmmr2r::Enabled
    }
}
#[doc = "Field `PWMMR2R` writer - Reset PWM0"]
pub type Pwmmr2rW<'a, REG> = crate::BitWriter<'a, REG, Pwmmr2r>;
impl<'a, REG> Pwmmr2rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr2r::Disabled)
    }
    #[doc = "Reset on PWMMR2: the PWMTC will be reset if PWMMR2 matches it."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr2r::Enabled)
    }
}
#[doc = "Stop PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmmr2s {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Stop on PWMMR2: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    Enabled = 1,
}
impl From<Pwmmr2s> for bool {
    #[inline(always)]
    fn from(variant: Pwmmr2s) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR2S` reader - Stop PWM0"]
pub type Pwmmr2sR = crate::BitReader<Pwmmr2s>;
impl Pwmmr2sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmmr2s {
        match self.bits {
            false => Pwmmr2s::Disabled,
            true => Pwmmr2s::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pwmmr2s::Disabled
    }
    #[doc = "Stop on PWMMR2: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pwmmr2s::Enabled
    }
}
#[doc = "Field `PWMMR2S` writer - Stop PWM0"]
pub type Pwmmr2sW<'a, REG> = crate::BitWriter<'a, REG, Pwmmr2s>;
impl<'a, REG> Pwmmr2sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr2s::Disabled)
    }
    #[doc = "Stop on PWMMR2: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr2s::Enabled)
    }
}
#[doc = "Interrupt PWM3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmmr3i {
    #[doc = "0: Disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt on PWMMR3: an interrupt is generated when PWMMR3 matches the value in the PWMTC."]
    Enabled = 1,
}
impl From<Pwmmr3i> for bool {
    #[inline(always)]
    fn from(variant: Pwmmr3i) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR3I` reader - Interrupt PWM3"]
pub type Pwmmr3iR = crate::BitReader<Pwmmr3i>;
impl Pwmmr3iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmmr3i {
        match self.bits {
            false => Pwmmr3i::Disabled,
            true => Pwmmr3i::Enabled,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pwmmr3i::Disabled
    }
    #[doc = "Interrupt on PWMMR3: an interrupt is generated when PWMMR3 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pwmmr3i::Enabled
    }
}
#[doc = "Field `PWMMR3I` writer - Interrupt PWM3"]
pub type Pwmmr3iW<'a, REG> = crate::BitWriter<'a, REG, Pwmmr3i>;
impl<'a, REG> Pwmmr3iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr3i::Disabled)
    }
    #[doc = "Interrupt on PWMMR3: an interrupt is generated when PWMMR3 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr3i::Enabled)
    }
}
#[doc = "Reset PWM3\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmmr3r {
    #[doc = "0: Disabled."]
    Disabled = 0,
    #[doc = "1: Reset on PWMMR3: the PWMTC will be reset if PWMMR3 matches it."]
    Enabled = 1,
}
impl From<Pwmmr3r> for bool {
    #[inline(always)]
    fn from(variant: Pwmmr3r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR3R` reader - Reset PWM3"]
pub type Pwmmr3rR = crate::BitReader<Pwmmr3r>;
impl Pwmmr3rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmmr3r {
        match self.bits {
            false => Pwmmr3r::Disabled,
            true => Pwmmr3r::Enabled,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pwmmr3r::Disabled
    }
    #[doc = "Reset on PWMMR3: the PWMTC will be reset if PWMMR3 matches it."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pwmmr3r::Enabled
    }
}
#[doc = "Field `PWMMR3R` writer - Reset PWM3"]
pub type Pwmmr3rW<'a, REG> = crate::BitWriter<'a, REG, Pwmmr3r>;
impl<'a, REG> Pwmmr3rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr3r::Disabled)
    }
    #[doc = "Reset on PWMMR3: the PWMTC will be reset if PWMMR3 matches it."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr3r::Enabled)
    }
}
#[doc = "Stop PWM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmmr3s {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Stop on PWMMR3: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    Enabled = 1,
}
impl From<Pwmmr3s> for bool {
    #[inline(always)]
    fn from(variant: Pwmmr3s) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR3S` reader - Stop PWM0"]
pub type Pwmmr3sR = crate::BitReader<Pwmmr3s>;
impl Pwmmr3sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmmr3s {
        match self.bits {
            false => Pwmmr3s::Disabled,
            true => Pwmmr3s::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pwmmr3s::Disabled
    }
    #[doc = "Stop on PWMMR3: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pwmmr3s::Enabled
    }
}
#[doc = "Field `PWMMR3S` writer - Stop PWM0"]
pub type Pwmmr3sW<'a, REG> = crate::BitWriter<'a, REG, Pwmmr3s>;
impl<'a, REG> Pwmmr3sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr3s::Disabled)
    }
    #[doc = "Stop on PWMMR3: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR0 matches the PWMTC."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr3s::Enabled)
    }
}
#[doc = "Interrupt PWM4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmmr4i {
    #[doc = "0: Disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt on PWMMR4: an interrupt is generated when PWMMR4 matches the value in the PWMTC."]
    Enabled = 1,
}
impl From<Pwmmr4i> for bool {
    #[inline(always)]
    fn from(variant: Pwmmr4i) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR4I` reader - Interrupt PWM4"]
pub type Pwmmr4iR = crate::BitReader<Pwmmr4i>;
impl Pwmmr4iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmmr4i {
        match self.bits {
            false => Pwmmr4i::Disabled,
            true => Pwmmr4i::Enabled,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pwmmr4i::Disabled
    }
    #[doc = "Interrupt on PWMMR4: an interrupt is generated when PWMMR4 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pwmmr4i::Enabled
    }
}
#[doc = "Field `PWMMR4I` writer - Interrupt PWM4"]
pub type Pwmmr4iW<'a, REG> = crate::BitWriter<'a, REG, Pwmmr4i>;
impl<'a, REG> Pwmmr4iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr4i::Disabled)
    }
    #[doc = "Interrupt on PWMMR4: an interrupt is generated when PWMMR4 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr4i::Enabled)
    }
}
#[doc = "Reset PWM4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmmr4r {
    #[doc = "0: Disabled."]
    Disabled = 0,
    #[doc = "1: Reset on PWMMR4: the PWMTC will be reset if PWMMR4 matches it."]
    Enabled = 1,
}
impl From<Pwmmr4r> for bool {
    #[inline(always)]
    fn from(variant: Pwmmr4r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR4R` reader - Reset PWM4"]
pub type Pwmmr4rR = crate::BitReader<Pwmmr4r>;
impl Pwmmr4rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmmr4r {
        match self.bits {
            false => Pwmmr4r::Disabled,
            true => Pwmmr4r::Enabled,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pwmmr4r::Disabled
    }
    #[doc = "Reset on PWMMR4: the PWMTC will be reset if PWMMR4 matches it."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pwmmr4r::Enabled
    }
}
#[doc = "Field `PWMMR4R` writer - Reset PWM4"]
pub type Pwmmr4rW<'a, REG> = crate::BitWriter<'a, REG, Pwmmr4r>;
impl<'a, REG> Pwmmr4rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr4r::Disabled)
    }
    #[doc = "Reset on PWMMR4: the PWMTC will be reset if PWMMR4 matches it."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr4r::Enabled)
    }
}
#[doc = "Stop PWM4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmmr4s {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Stop on PWMMR4: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR4 matches the PWMTC."]
    Enabled = 1,
}
impl From<Pwmmr4s> for bool {
    #[inline(always)]
    fn from(variant: Pwmmr4s) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR4S` reader - Stop PWM4"]
pub type Pwmmr4sR = crate::BitReader<Pwmmr4s>;
impl Pwmmr4sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmmr4s {
        match self.bits {
            false => Pwmmr4s::Disabled,
            true => Pwmmr4s::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pwmmr4s::Disabled
    }
    #[doc = "Stop on PWMMR4: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR4 matches the PWMTC."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pwmmr4s::Enabled
    }
}
#[doc = "Field `PWMMR4S` writer - Stop PWM4"]
pub type Pwmmr4sW<'a, REG> = crate::BitWriter<'a, REG, Pwmmr4s>;
impl<'a, REG> Pwmmr4sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr4s::Disabled)
    }
    #[doc = "Stop on PWMMR4: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR4 matches the PWMTC."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr4s::Enabled)
    }
}
#[doc = "Interrupt PWM5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmmr5i {
    #[doc = "0: Disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt on PWMMR5: an interrupt is generated when PWMMR5 matches the value in the PWMTC."]
    Enabled = 1,
}
impl From<Pwmmr5i> for bool {
    #[inline(always)]
    fn from(variant: Pwmmr5i) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR5I` reader - Interrupt PWM5"]
pub type Pwmmr5iR = crate::BitReader<Pwmmr5i>;
impl Pwmmr5iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmmr5i {
        match self.bits {
            false => Pwmmr5i::Disabled,
            true => Pwmmr5i::Enabled,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pwmmr5i::Disabled
    }
    #[doc = "Interrupt on PWMMR5: an interrupt is generated when PWMMR5 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pwmmr5i::Enabled
    }
}
#[doc = "Field `PWMMR5I` writer - Interrupt PWM5"]
pub type Pwmmr5iW<'a, REG> = crate::BitWriter<'a, REG, Pwmmr5i>;
impl<'a, REG> Pwmmr5iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr5i::Disabled)
    }
    #[doc = "Interrupt on PWMMR5: an interrupt is generated when PWMMR5 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr5i::Enabled)
    }
}
#[doc = "Reset PWM5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmmr5r {
    #[doc = "0: Disabled."]
    Disabled = 0,
    #[doc = "1: Reset on PWMMR5: the PWMTC will be reset if PWMMR5 matches it."]
    Enabled = 1,
}
impl From<Pwmmr5r> for bool {
    #[inline(always)]
    fn from(variant: Pwmmr5r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR5R` reader - Reset PWM5"]
pub type Pwmmr5rR = crate::BitReader<Pwmmr5r>;
impl Pwmmr5rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmmr5r {
        match self.bits {
            false => Pwmmr5r::Disabled,
            true => Pwmmr5r::Enabled,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pwmmr5r::Disabled
    }
    #[doc = "Reset on PWMMR5: the PWMTC will be reset if PWMMR5 matches it."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pwmmr5r::Enabled
    }
}
#[doc = "Field `PWMMR5R` writer - Reset PWM5"]
pub type Pwmmr5rW<'a, REG> = crate::BitWriter<'a, REG, Pwmmr5r>;
impl<'a, REG> Pwmmr5rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr5r::Disabled)
    }
    #[doc = "Reset on PWMMR5: the PWMTC will be reset if PWMMR5 matches it."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr5r::Enabled)
    }
}
#[doc = "Stop PWM5\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmmr5s {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Stop on PWMMR5: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR5 matches the PWMTC."]
    Enabled = 1,
}
impl From<Pwmmr5s> for bool {
    #[inline(always)]
    fn from(variant: Pwmmr5s) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR5S` reader - Stop PWM5"]
pub type Pwmmr5sR = crate::BitReader<Pwmmr5s>;
impl Pwmmr5sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmmr5s {
        match self.bits {
            false => Pwmmr5s::Disabled,
            true => Pwmmr5s::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pwmmr5s::Disabled
    }
    #[doc = "Stop on PWMMR5: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR5 matches the PWMTC."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pwmmr5s::Enabled
    }
}
#[doc = "Field `PWMMR5S` writer - Stop PWM5"]
pub type Pwmmr5sW<'a, REG> = crate::BitWriter<'a, REG, Pwmmr5s>;
impl<'a, REG> Pwmmr5sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr5s::Disabled)
    }
    #[doc = "Stop on PWMMR5: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR5 matches the PWMTC."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr5s::Enabled)
    }
}
#[doc = "Interrupt PWM6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmmr6i {
    #[doc = "0: Disabled."]
    Disabled = 0,
    #[doc = "1: Interrupt on PWMMR6: an interrupt is generated when PWMMR6 matches the value in the PWMTC."]
    Enabled = 1,
}
impl From<Pwmmr6i> for bool {
    #[inline(always)]
    fn from(variant: Pwmmr6i) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR6I` reader - Interrupt PWM6"]
pub type Pwmmr6iR = crate::BitReader<Pwmmr6i>;
impl Pwmmr6iR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmmr6i {
        match self.bits {
            false => Pwmmr6i::Disabled,
            true => Pwmmr6i::Enabled,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pwmmr6i::Disabled
    }
    #[doc = "Interrupt on PWMMR6: an interrupt is generated when PWMMR6 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pwmmr6i::Enabled
    }
}
#[doc = "Field `PWMMR6I` writer - Interrupt PWM6"]
pub type Pwmmr6iW<'a, REG> = crate::BitWriter<'a, REG, Pwmmr6i>;
impl<'a, REG> Pwmmr6iW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr6i::Disabled)
    }
    #[doc = "Interrupt on PWMMR6: an interrupt is generated when PWMMR6 matches the value in the PWMTC."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr6i::Enabled)
    }
}
#[doc = "Reset PWM6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmmr6r {
    #[doc = "0: Disabled."]
    Disabled = 0,
    #[doc = "1: Reset on PWMMR6: the PWMTC will be reset if PWMMR6 matches it."]
    Enabled = 1,
}
impl From<Pwmmr6r> for bool {
    #[inline(always)]
    fn from(variant: Pwmmr6r) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR6R` reader - Reset PWM6"]
pub type Pwmmr6rR = crate::BitReader<Pwmmr6r>;
impl Pwmmr6rR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmmr6r {
        match self.bits {
            false => Pwmmr6r::Disabled,
            true => Pwmmr6r::Enabled,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pwmmr6r::Disabled
    }
    #[doc = "Reset on PWMMR6: the PWMTC will be reset if PWMMR6 matches it."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pwmmr6r::Enabled
    }
}
#[doc = "Field `PWMMR6R` writer - Reset PWM6"]
pub type Pwmmr6rW<'a, REG> = crate::BitWriter<'a, REG, Pwmmr6r>;
impl<'a, REG> Pwmmr6rW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr6r::Disabled)
    }
    #[doc = "Reset on PWMMR6: the PWMTC will be reset if PWMMR6 matches it."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr6r::Enabled)
    }
}
#[doc = "Stop PWM6\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmmr6s {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Stop on PWMMR6: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR6 matches the PWMTC."]
    Enabled = 1,
}
impl From<Pwmmr6s> for bool {
    #[inline(always)]
    fn from(variant: Pwmmr6s) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMMR6S` reader - Stop PWM6"]
pub type Pwmmr6sR = crate::BitReader<Pwmmr6s>;
impl Pwmmr6sR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmmr6s {
        match self.bits {
            false => Pwmmr6s::Disabled,
            true => Pwmmr6s::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pwmmr6s::Disabled
    }
    #[doc = "Stop on PWMMR6: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR6 matches the PWMTC."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pwmmr6s::Enabled
    }
}
#[doc = "Field `PWMMR6S` writer - Stop PWM6"]
pub type Pwmmr6sW<'a, REG> = crate::BitWriter<'a, REG, Pwmmr6s>;
impl<'a, REG> Pwmmr6sW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr6s::Disabled)
    }
    #[doc = "Stop on PWMMR6: the PWMTC and PWMPC will be stopped and PWMTCR bit 0 will be set to 0 if PWMMR6 matches the PWMTC."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmmr6s::Enabled)
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
