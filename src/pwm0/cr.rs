#[doc = "Register `CR[%s]` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR[%s]` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "PWM\\[2\\] output single/double edge mode control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmsel2 {
    #[doc = "0: Single edge controlled mode is selected."]
    SingleEdgeControll = 0,
    #[doc = "1: Double edge controlled mode is selected."]
    DoubleEdgeControll = 1,
}
impl From<Pwmsel2> for bool {
    #[inline(always)]
    fn from(variant: Pwmsel2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMSEL2` reader - PWM\\[2\\] output single/double edge mode control."]
pub type Pwmsel2R = crate::BitReader<Pwmsel2>;
impl Pwmsel2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmsel2 {
        match self.bits {
            false => Pwmsel2::SingleEdgeControll,
            true => Pwmsel2::DoubleEdgeControll,
        }
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_single_edge_controll(&self) -> bool {
        *self == Pwmsel2::SingleEdgeControll
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_double_edge_controll(&self) -> bool {
        *self == Pwmsel2::DoubleEdgeControll
    }
}
#[doc = "Field `PWMSEL2` writer - PWM\\[2\\] output single/double edge mode control."]
pub type Pwmsel2W<'a, REG> = crate::BitWriter<'a, REG, Pwmsel2>;
impl<'a, REG> Pwmsel2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn single_edge_controll(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmsel2::SingleEdgeControll)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn double_edge_controll(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmsel2::DoubleEdgeControll)
    }
}
#[doc = "PWM\\[3\\] output edge control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmsel3 {
    #[doc = "0: Single edge controlled mode is selected."]
    SingleEdgeControll = 0,
    #[doc = "1: Double edge controlled mode is selected."]
    DoubleEdgeControll = 1,
}
impl From<Pwmsel3> for bool {
    #[inline(always)]
    fn from(variant: Pwmsel3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMSEL3` reader - PWM\\[3\\] output edge control."]
pub type Pwmsel3R = crate::BitReader<Pwmsel3>;
impl Pwmsel3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmsel3 {
        match self.bits {
            false => Pwmsel3::SingleEdgeControll,
            true => Pwmsel3::DoubleEdgeControll,
        }
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_single_edge_controll(&self) -> bool {
        *self == Pwmsel3::SingleEdgeControll
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_double_edge_controll(&self) -> bool {
        *self == Pwmsel3::DoubleEdgeControll
    }
}
#[doc = "Field `PWMSEL3` writer - PWM\\[3\\] output edge control."]
pub type Pwmsel3W<'a, REG> = crate::BitWriter<'a, REG, Pwmsel3>;
impl<'a, REG> Pwmsel3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn single_edge_controll(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmsel3::SingleEdgeControll)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn double_edge_controll(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmsel3::DoubleEdgeControll)
    }
}
#[doc = "PWM\\[4\\] output edge control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmsel4 {
    #[doc = "0: Single edge controlled mode is selected."]
    SingleEdgeControll = 0,
    #[doc = "1: Double edge controlled mode is selected."]
    DoubleEdgeControll = 1,
}
impl From<Pwmsel4> for bool {
    #[inline(always)]
    fn from(variant: Pwmsel4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMSEL4` reader - PWM\\[4\\] output edge control."]
pub type Pwmsel4R = crate::BitReader<Pwmsel4>;
impl Pwmsel4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmsel4 {
        match self.bits {
            false => Pwmsel4::SingleEdgeControll,
            true => Pwmsel4::DoubleEdgeControll,
        }
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_single_edge_controll(&self) -> bool {
        *self == Pwmsel4::SingleEdgeControll
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_double_edge_controll(&self) -> bool {
        *self == Pwmsel4::DoubleEdgeControll
    }
}
#[doc = "Field `PWMSEL4` writer - PWM\\[4\\] output edge control."]
pub type Pwmsel4W<'a, REG> = crate::BitWriter<'a, REG, Pwmsel4>;
impl<'a, REG> Pwmsel4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn single_edge_controll(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmsel4::SingleEdgeControll)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn double_edge_controll(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmsel4::DoubleEdgeControll)
    }
}
#[doc = "PWM\\[5\\] output edge control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmsel5 {
    #[doc = "0: Single edge controlled mode is selected."]
    SingleEdgeControll = 0,
    #[doc = "1: Double edge controlled mode is selected."]
    DoubleEdgeControll = 1,
}
impl From<Pwmsel5> for bool {
    #[inline(always)]
    fn from(variant: Pwmsel5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMSEL5` reader - PWM\\[5\\] output edge control."]
pub type Pwmsel5R = crate::BitReader<Pwmsel5>;
impl Pwmsel5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmsel5 {
        match self.bits {
            false => Pwmsel5::SingleEdgeControll,
            true => Pwmsel5::DoubleEdgeControll,
        }
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_single_edge_controll(&self) -> bool {
        *self == Pwmsel5::SingleEdgeControll
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_double_edge_controll(&self) -> bool {
        *self == Pwmsel5::DoubleEdgeControll
    }
}
#[doc = "Field `PWMSEL5` writer - PWM\\[5\\] output edge control."]
pub type Pwmsel5W<'a, REG> = crate::BitWriter<'a, REG, Pwmsel5>;
impl<'a, REG> Pwmsel5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn single_edge_controll(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmsel5::SingleEdgeControll)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn double_edge_controll(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmsel5::DoubleEdgeControll)
    }
}
#[doc = "PWM\\[6\\] output edge control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmsel6 {
    #[doc = "0: Single edge controlled mode is selected."]
    SingleEdgeControll = 0,
    #[doc = "1: Double edge controlled mode is selected."]
    DoubleEdgeControll = 1,
}
impl From<Pwmsel6> for bool {
    #[inline(always)]
    fn from(variant: Pwmsel6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMSEL6` reader - PWM\\[6\\] output edge control."]
pub type Pwmsel6R = crate::BitReader<Pwmsel6>;
impl Pwmsel6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmsel6 {
        match self.bits {
            false => Pwmsel6::SingleEdgeControll,
            true => Pwmsel6::DoubleEdgeControll,
        }
    }
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_single_edge_controll(&self) -> bool {
        *self == Pwmsel6::SingleEdgeControll
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn is_double_edge_controll(&self) -> bool {
        *self == Pwmsel6::DoubleEdgeControll
    }
}
#[doc = "Field `PWMSEL6` writer - PWM\\[6\\] output edge control."]
pub type Pwmsel6W<'a, REG> = crate::BitWriter<'a, REG, Pwmsel6>;
impl<'a, REG> Pwmsel6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Single edge controlled mode is selected."]
    #[inline(always)]
    pub fn single_edge_controll(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmsel6::SingleEdgeControll)
    }
    #[doc = "Double edge controlled mode is selected."]
    #[inline(always)]
    pub fn double_edge_controll(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmsel6::DoubleEdgeControll)
    }
}
#[doc = "PWM\\[1\\] output enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmena1 {
    #[doc = "0: The PWM output is disabled."]
    ThePwmOutputIsDi = 0,
    #[doc = "1: The PWM output is enabled."]
    ThePwmOutputIsEn = 1,
}
impl From<Pwmena1> for bool {
    #[inline(always)]
    fn from(variant: Pwmena1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMENA1` reader - PWM\\[1\\] output enable control."]
pub type Pwmena1R = crate::BitReader<Pwmena1>;
impl Pwmena1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmena1 {
        match self.bits {
            false => Pwmena1::ThePwmOutputIsDi,
            true => Pwmena1::ThePwmOutputIsEn,
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn is_the_pwm_output_is_di(&self) -> bool {
        *self == Pwmena1::ThePwmOutputIsDi
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn is_the_pwm_output_is_en(&self) -> bool {
        *self == Pwmena1::ThePwmOutputIsEn
    }
}
#[doc = "Field `PWMENA1` writer - PWM\\[1\\] output enable control."]
pub type Pwmena1W<'a, REG> = crate::BitWriter<'a, REG, Pwmena1>;
impl<'a, REG> Pwmena1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_di(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmena1::ThePwmOutputIsDi)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmena1::ThePwmOutputIsEn)
    }
}
#[doc = "PWM\\[2\\] output enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmena2 {
    #[doc = "0: The PWM output is disabled."]
    ThePwmOutputIsDi = 0,
    #[doc = "1: The PWM output is enabled."]
    ThePwmOutputIsEn = 1,
}
impl From<Pwmena2> for bool {
    #[inline(always)]
    fn from(variant: Pwmena2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMENA2` reader - PWM\\[2\\] output enable control."]
pub type Pwmena2R = crate::BitReader<Pwmena2>;
impl Pwmena2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmena2 {
        match self.bits {
            false => Pwmena2::ThePwmOutputIsDi,
            true => Pwmena2::ThePwmOutputIsEn,
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn is_the_pwm_output_is_di(&self) -> bool {
        *self == Pwmena2::ThePwmOutputIsDi
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn is_the_pwm_output_is_en(&self) -> bool {
        *self == Pwmena2::ThePwmOutputIsEn
    }
}
#[doc = "Field `PWMENA2` writer - PWM\\[2\\] output enable control."]
pub type Pwmena2W<'a, REG> = crate::BitWriter<'a, REG, Pwmena2>;
impl<'a, REG> Pwmena2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_di(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmena2::ThePwmOutputIsDi)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmena2::ThePwmOutputIsEn)
    }
}
#[doc = "PWM\\[3\\] output enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmena3 {
    #[doc = "0: The PWM output is disabled."]
    ThePwmOutputIsDi = 0,
    #[doc = "1: The PWM output is enabled."]
    ThePwmOutputIsEn = 1,
}
impl From<Pwmena3> for bool {
    #[inline(always)]
    fn from(variant: Pwmena3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMENA3` reader - PWM\\[3\\] output enable control."]
pub type Pwmena3R = crate::BitReader<Pwmena3>;
impl Pwmena3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmena3 {
        match self.bits {
            false => Pwmena3::ThePwmOutputIsDi,
            true => Pwmena3::ThePwmOutputIsEn,
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn is_the_pwm_output_is_di(&self) -> bool {
        *self == Pwmena3::ThePwmOutputIsDi
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn is_the_pwm_output_is_en(&self) -> bool {
        *self == Pwmena3::ThePwmOutputIsEn
    }
}
#[doc = "Field `PWMENA3` writer - PWM\\[3\\] output enable control."]
pub type Pwmena3W<'a, REG> = crate::BitWriter<'a, REG, Pwmena3>;
impl<'a, REG> Pwmena3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_di(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmena3::ThePwmOutputIsDi)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmena3::ThePwmOutputIsEn)
    }
}
#[doc = "PWM\\[4\\] output enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmena4 {
    #[doc = "0: The PWM output is disabled."]
    ThePwmOutputIsDi = 0,
    #[doc = "1: The PWM output is enabled."]
    ThePwmOutputIsEn = 1,
}
impl From<Pwmena4> for bool {
    #[inline(always)]
    fn from(variant: Pwmena4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMENA4` reader - PWM\\[4\\] output enable control."]
pub type Pwmena4R = crate::BitReader<Pwmena4>;
impl Pwmena4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmena4 {
        match self.bits {
            false => Pwmena4::ThePwmOutputIsDi,
            true => Pwmena4::ThePwmOutputIsEn,
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn is_the_pwm_output_is_di(&self) -> bool {
        *self == Pwmena4::ThePwmOutputIsDi
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn is_the_pwm_output_is_en(&self) -> bool {
        *self == Pwmena4::ThePwmOutputIsEn
    }
}
#[doc = "Field `PWMENA4` writer - PWM\\[4\\] output enable control."]
pub type Pwmena4W<'a, REG> = crate::BitWriter<'a, REG, Pwmena4>;
impl<'a, REG> Pwmena4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_di(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmena4::ThePwmOutputIsDi)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmena4::ThePwmOutputIsEn)
    }
}
#[doc = "PWM\\[5\\] output enable control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmena5 {
    #[doc = "0: The PWM output is disabled."]
    ThePwmOutputIsDi = 0,
    #[doc = "1: The PWM output is enabled."]
    ThePwmOutputIsEn = 1,
}
impl From<Pwmena5> for bool {
    #[inline(always)]
    fn from(variant: Pwmena5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMENA5` reader - PWM\\[5\\] output enable control."]
pub type Pwmena5R = crate::BitReader<Pwmena5>;
impl Pwmena5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmena5 {
        match self.bits {
            false => Pwmena5::ThePwmOutputIsDi,
            true => Pwmena5::ThePwmOutputIsEn,
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn is_the_pwm_output_is_di(&self) -> bool {
        *self == Pwmena5::ThePwmOutputIsDi
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn is_the_pwm_output_is_en(&self) -> bool {
        *self == Pwmena5::ThePwmOutputIsEn
    }
}
#[doc = "Field `PWMENA5` writer - PWM\\[5\\] output enable control."]
pub type Pwmena5W<'a, REG> = crate::BitWriter<'a, REG, Pwmena5>;
impl<'a, REG> Pwmena5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_di(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmena5::ThePwmOutputIsDi)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmena5::ThePwmOutputIsEn)
    }
}
#[doc = "PWM\\[6\\] output enable control. See PWMENA1 for details.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwmena6 {
    #[doc = "0: The PWM output is disabled."]
    ThePwmOutputIsDi = 0,
    #[doc = "1: The PWM output is enabled."]
    ThePwmOutputIsEn = 1,
}
impl From<Pwmena6> for bool {
    #[inline(always)]
    fn from(variant: Pwmena6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWMENA6` reader - PWM\\[6\\] output enable control. See PWMENA1 for details."]
pub type Pwmena6R = crate::BitReader<Pwmena6>;
impl Pwmena6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwmena6 {
        match self.bits {
            false => Pwmena6::ThePwmOutputIsDi,
            true => Pwmena6::ThePwmOutputIsEn,
        }
    }
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn is_the_pwm_output_is_di(&self) -> bool {
        *self == Pwmena6::ThePwmOutputIsDi
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn is_the_pwm_output_is_en(&self) -> bool {
        *self == Pwmena6::ThePwmOutputIsEn
    }
}
#[doc = "Field `PWMENA6` writer - PWM\\[6\\] output enable control. See PWMENA1 for details."]
pub type Pwmena6W<'a, REG> = crate::BitWriter<'a, REG, Pwmena6>;
impl<'a, REG> Pwmena6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The PWM output is disabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_di(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmena6::ThePwmOutputIsDi)
    }
    #[doc = "The PWM output is enabled."]
    #[inline(always)]
    pub fn the_pwm_output_is_en(self) -> &'a mut crate::W<REG> {
        self.variant(Pwmena6::ThePwmOutputIsEn)
    }
}
impl R {
    #[doc = "Bit 2 - PWM\\[2\\] output single/double edge mode control."]
    #[inline(always)]
    pub fn pwmsel2(&self) -> Pwmsel2R {
        Pwmsel2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PWM\\[3\\] output edge control."]
    #[inline(always)]
    pub fn pwmsel3(&self) -> Pwmsel3R {
        Pwmsel3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PWM\\[4\\] output edge control."]
    #[inline(always)]
    pub fn pwmsel4(&self) -> Pwmsel4R {
        Pwmsel4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PWM\\[5\\] output edge control."]
    #[inline(always)]
    pub fn pwmsel5(&self) -> Pwmsel5R {
        Pwmsel5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PWM\\[6\\] output edge control."]
    #[inline(always)]
    pub fn pwmsel6(&self) -> Pwmsel6R {
        Pwmsel6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - PWM\\[1\\] output enable control."]
    #[inline(always)]
    pub fn pwmena1(&self) -> Pwmena1R {
        Pwmena1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PWM\\[2\\] output enable control."]
    #[inline(always)]
    pub fn pwmena2(&self) -> Pwmena2R {
        Pwmena2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PWM\\[3\\] output enable control."]
    #[inline(always)]
    pub fn pwmena3(&self) -> Pwmena3R {
        Pwmena3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PWM\\[4\\] output enable control."]
    #[inline(always)]
    pub fn pwmena4(&self) -> Pwmena4R {
        Pwmena4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - PWM\\[5\\] output enable control."]
    #[inline(always)]
    pub fn pwmena5(&self) -> Pwmena5R {
        Pwmena5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PWM\\[6\\] output enable control. See PWMENA1 for details."]
    #[inline(always)]
    pub fn pwmena6(&self) -> Pwmena6R {
        Pwmena6R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - PWM\\[2\\] output single/double edge mode control."]
    #[inline(always)]
    pub fn pwmsel2(&mut self) -> Pwmsel2W<'_, CrSpec> {
        Pwmsel2W::new(self, 2)
    }
    #[doc = "Bit 3 - PWM\\[3\\] output edge control."]
    #[inline(always)]
    pub fn pwmsel3(&mut self) -> Pwmsel3W<'_, CrSpec> {
        Pwmsel3W::new(self, 3)
    }
    #[doc = "Bit 4 - PWM\\[4\\] output edge control."]
    #[inline(always)]
    pub fn pwmsel4(&mut self) -> Pwmsel4W<'_, CrSpec> {
        Pwmsel4W::new(self, 4)
    }
    #[doc = "Bit 5 - PWM\\[5\\] output edge control."]
    #[inline(always)]
    pub fn pwmsel5(&mut self) -> Pwmsel5W<'_, CrSpec> {
        Pwmsel5W::new(self, 5)
    }
    #[doc = "Bit 6 - PWM\\[6\\] output edge control."]
    #[inline(always)]
    pub fn pwmsel6(&mut self) -> Pwmsel6W<'_, CrSpec> {
        Pwmsel6W::new(self, 6)
    }
    #[doc = "Bit 9 - PWM\\[1\\] output enable control."]
    #[inline(always)]
    pub fn pwmena1(&mut self) -> Pwmena1W<'_, CrSpec> {
        Pwmena1W::new(self, 9)
    }
    #[doc = "Bit 10 - PWM\\[2\\] output enable control."]
    #[inline(always)]
    pub fn pwmena2(&mut self) -> Pwmena2W<'_, CrSpec> {
        Pwmena2W::new(self, 10)
    }
    #[doc = "Bit 11 - PWM\\[3\\] output enable control."]
    #[inline(always)]
    pub fn pwmena3(&mut self) -> Pwmena3W<'_, CrSpec> {
        Pwmena3W::new(self, 11)
    }
    #[doc = "Bit 12 - PWM\\[4\\] output enable control."]
    #[inline(always)]
    pub fn pwmena4(&mut self) -> Pwmena4W<'_, CrSpec> {
        Pwmena4W::new(self, 12)
    }
    #[doc = "Bit 13 - PWM\\[5\\] output enable control."]
    #[inline(always)]
    pub fn pwmena5(&mut self) -> Pwmena5W<'_, CrSpec> {
        Pwmena5W::new(self, 13)
    }
    #[doc = "Bit 14 - PWM\\[6\\] output enable control. See PWMENA1 for details."]
    #[inline(always)]
    pub fn pwmena6(&mut self) -> Pwmena6W<'_, CrSpec> {
        Pwmena6W::new(self, 14)
    }
}
#[doc = "PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs.\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR[%s] to value 0"]
impl crate::Resettable for CrSpec {}
