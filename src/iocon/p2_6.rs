#[doc = "Register `P2_6` reader"]
pub type R = crate::R<P2_6Spec>;
#[doc = "Register `P2_6` writer"]
pub type W = crate::W<P2_6Spec>;
#[doc = "Selects pin function for pin P2\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Enum {
    #[doc = "0: General purpose digital input/output pin."]
    P2_6 = 0,
    #[doc = "1: Capture input for PWM1, channel 0."]
    Pwm1Cap0 = 1,
    #[doc = "2: Ring Indicator input for UART1."]
    U1Ri = 2,
    #[doc = "3: Capture input for Timer 2, channel 0."]
    T2Cap0 = 3,
    #[doc = "4: RS-485/EIA-485 output enable signal for UART2."]
    U2Oe = 4,
    #[doc = "5: Trace clock."]
    Traceclk = 5,
    #[doc = "6: LCD data."]
    LcdVd0 = 6,
    #[doc = "7: LCD data."]
    LcdVd4 = 7,
}
impl From<Enum> for u8 {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Enum {
    type Ux = u8;
}
impl crate::IsEnum for Enum {}
#[doc = "Field `FUNC` reader - Selects pin function for pin P2\\[6\\]"]
pub type FuncR = crate::FieldReader<Enum>;
impl FuncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            0 => Enum::P2_6,
            1 => Enum::Pwm1Cap0,
            2 => Enum::U1Ri,
            3 => Enum::T2Cap0,
            4 => Enum::U2Oe,
            5 => Enum::Traceclk,
            6 => Enum::LcdVd0,
            7 => Enum::LcdVd4,
            _ => unreachable!(),
        }
    }
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn is_p2_6(&self) -> bool {
        *self == Enum::P2_6
    }
    #[doc = "Capture input for PWM1, channel 0."]
    #[inline(always)]
    pub fn is_pwm1_cap0(&self) -> bool {
        *self == Enum::Pwm1Cap0
    }
    #[doc = "Ring Indicator input for UART1."]
    #[inline(always)]
    pub fn is_u1_ri(&self) -> bool {
        *self == Enum::U1Ri
    }
    #[doc = "Capture input for Timer 2, channel 0."]
    #[inline(always)]
    pub fn is_t2_cap0(&self) -> bool {
        *self == Enum::T2Cap0
    }
    #[doc = "RS-485/EIA-485 output enable signal for UART2."]
    #[inline(always)]
    pub fn is_u2_oe(&self) -> bool {
        *self == Enum::U2Oe
    }
    #[doc = "Trace clock."]
    #[inline(always)]
    pub fn is_traceclk(&self) -> bool {
        *self == Enum::Traceclk
    }
    #[doc = "LCD data."]
    #[inline(always)]
    pub fn is_lcd_vd_0(&self) -> bool {
        *self == Enum::LcdVd0
    }
    #[doc = "LCD data."]
    #[inline(always)]
    pub fn is_lcd_vd_4(&self) -> bool {
        *self == Enum::LcdVd4
    }
}
#[doc = "Field `FUNC` writer - Selects pin function for pin P2\\[6\\]"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 3, Enum, crate::Safe>;
impl<'a, REG> FuncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn p2_6(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::P2_6)
    }
    #[doc = "Capture input for PWM1, channel 0."]
    #[inline(always)]
    pub fn pwm1_cap0(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Pwm1Cap0)
    }
    #[doc = "Ring Indicator input for UART1."]
    #[inline(always)]
    pub fn u1_ri(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::U1Ri)
    }
    #[doc = "Capture input for Timer 2, channel 0."]
    #[inline(always)]
    pub fn t2_cap0(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::T2Cap0)
    }
    #[doc = "RS-485/EIA-485 output enable signal for UART2."]
    #[inline(always)]
    pub fn u2_oe(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::U2Oe)
    }
    #[doc = "Trace clock."]
    #[inline(always)]
    pub fn traceclk(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Traceclk)
    }
    #[doc = "LCD data."]
    #[inline(always)]
    pub fn lcd_vd_0(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::LcdVd0)
    }
    #[doc = "LCD data."]
    #[inline(always)]
    pub fn lcd_vd_4(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::LcdVd4)
    }
}
#[doc = "Selects function mode (on-chip pull-up/pull-down resistor control).\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Enum {
    #[doc = "0: Inactive (no pull-down/pull-up resistor enabled)."]
    InactiveNoPullDo = 0,
    #[doc = "1: Pull-down resistor enabled."]
    PullDownResistorE = 1,
    #[doc = "2: Pull-up resistor enabled."]
    PullUpResistorEna = 2,
    #[doc = "3: Repeater mode."]
    RepeaterMode_ = 3,
}
impl From<Enum> for u8 {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Enum {
    type Ux = u8;
}
impl crate::IsEnum for Enum {}
#[doc = "Field `MODE` reader - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type ModeR = crate::FieldReader<Enum>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            0 => Enum::InactiveNoPullDo,
            1 => Enum::PullDownResistorE,
            2 => Enum::PullUpResistorEna,
            3 => Enum::RepeaterMode_,
            _ => unreachable!(),
        }
    }
    #[doc = "Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn is_inactive_no_pull_do(&self) -> bool {
        *self == Enum::InactiveNoPullDo
    }
    #[doc = "Pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down_resistor_e(&self) -> bool {
        *self == Enum::PullDownResistorE
    }
    #[doc = "Pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up_resistor_ena(&self) -> bool {
        *self == Enum::PullUpResistorEna
    }
    #[doc = "Repeater mode."]
    #[inline(always)]
    pub fn is_repeater_mode_(&self) -> bool {
        *self == Enum::RepeaterMode_
    }
}
#[doc = "Field `MODE` writer - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Enum, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn inactive_no_pull_do(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::InactiveNoPullDo)
    }
    #[doc = "Pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down_resistor_e(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::PullDownResistorE)
    }
    #[doc = "Pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up_resistor_ena(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::PullUpResistorEna)
    }
    #[doc = "Repeater mode."]
    #[inline(always)]
    pub fn repeater_mode_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::RepeaterMode_)
    }
}
#[doc = "Hysteresis.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disable."]
    Disable_ = 0,
    #[doc = "1: Enable."]
    Enable_ = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HYS` reader - Hysteresis."]
pub type HysR = crate::BitReader<Enum>;
impl HysR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disable_,
            true => Enum::Enable_,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable_(&self) -> bool {
        *self == Enum::Disable_
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_enable_(&self) -> bool {
        *self == Enum::Enable_
    }
}
#[doc = "Field `HYS` writer - Hysteresis."]
pub type HysW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> HysW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disable_)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn enable_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Enable_)
    }
}
#[doc = "Invert input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Input not inverted (HIGH on pin reads as 1, LOW on pin reads as 0)."]
    InputNotInverted_ = 0,
    #[doc = "1: Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    InputInvertedHigh = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INV` reader - Invert input"]
pub type InvR = crate::BitReader<Enum>;
impl InvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::InputNotInverted_,
            true => Enum::InputInvertedHigh,
        }
    }
    #[doc = "Input not inverted (HIGH on pin reads as 1, LOW on pin reads as 0)."]
    #[inline(always)]
    pub fn is_input_not_inverted_(&self) -> bool {
        *self == Enum::InputNotInverted_
    }
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    #[inline(always)]
    pub fn is_input_inverted_high(&self) -> bool {
        *self == Enum::InputInvertedHigh
    }
}
#[doc = "Field `INV` writer - Invert input"]
pub type InvW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> InvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input not inverted (HIGH on pin reads as 1, LOW on pin reads as 0)."]
    #[inline(always)]
    pub fn input_not_inverted_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::InputNotInverted_)
    }
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    #[inline(always)]
    pub fn input_inverted_high(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::InputInvertedHigh)
    }
}
#[doc = "Driver slew rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Standard mode, output slew rate control is enabled. More outputs can be switched simultaneously."]
    Standard = 0,
    #[doc = "1: Fast mode, slew rate control is disabled. Refer to the appropriate specific device data sheet for details."]
    Fast = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEW` reader - Driver slew rate"]
pub type SlewR = crate::BitReader<Enum>;
impl SlewR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Standard,
            true => Enum::Fast,
        }
    }
    #[doc = "Standard mode, output slew rate control is enabled. More outputs can be switched simultaneously."]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == Enum::Standard
    }
    #[doc = "Fast mode, slew rate control is disabled. Refer to the appropriate specific device data sheet for details."]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == Enum::Fast
    }
}
#[doc = "Field `SLEW` writer - Driver slew rate"]
pub type SlewW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> SlewW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard mode, output slew rate control is enabled. More outputs can be switched simultaneously."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Standard)
    }
    #[doc = "Fast mode, slew rate control is disabled. Refer to the appropriate specific device data sheet for details."]
    #[inline(always)]
    pub fn fast(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Fast)
    }
}
#[doc = "Open-drain mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disable."]
    Disable_ = 0,
    #[doc = "1: Open-drain mode enabled. This is not a true open-drain mode. Input cannot be pulled up above VDD."]
    OpenDrainModeEnab = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OD` reader - Open-drain mode."]
pub type OdR = crate::BitReader<Enum>;
impl OdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disable_,
            true => Enum::OpenDrainModeEnab,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable_(&self) -> bool {
        *self == Enum::Disable_
    }
    #[doc = "Open-drain mode enabled. This is not a true open-drain mode. Input cannot be pulled up above VDD."]
    #[inline(always)]
    pub fn is_open_drain_mode_enab(&self) -> bool {
        *self == Enum::OpenDrainModeEnab
    }
}
#[doc = "Field `OD` writer - Open-drain mode."]
pub type OdW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> OdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disable_)
    }
    #[doc = "Open-drain mode enabled. This is not a true open-drain mode. Input cannot be pulled up above VDD."]
    #[inline(always)]
    pub fn open_drain_mode_enab(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::OpenDrainModeEnab)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P2\\[6\\]"]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Hysteresis."]
    #[inline(always)]
    pub fn hys(&self) -> HysR {
        HysR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Driver slew rate"]
    #[inline(always)]
    pub fn slew(&self) -> SlewR {
        SlewR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Open-drain mode."]
    #[inline(always)]
    pub fn od(&self) -> OdR {
        OdR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P2\\[6\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FuncW<'_, P2_6Spec> {
        FuncW::new(self, 0)
    }
    #[doc = "Bits 3:4 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, P2_6Spec> {
        ModeW::new(self, 3)
    }
    #[doc = "Bit 5 - Hysteresis."]
    #[inline(always)]
    pub fn hys(&mut self) -> HysW<'_, P2_6Spec> {
        HysW::new(self, 5)
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline(always)]
    pub fn inv(&mut self) -> InvW<'_, P2_6Spec> {
        InvW::new(self, 6)
    }
    #[doc = "Bit 9 - Driver slew rate"]
    #[inline(always)]
    pub fn slew(&mut self) -> SlewW<'_, P2_6Spec> {
        SlewW::new(self, 9)
    }
    #[doc = "Bit 10 - Open-drain mode."]
    #[inline(always)]
    pub fn od(&mut self) -> OdW<'_, P2_6Spec> {
        OdW::new(self, 10)
    }
}
#[doc = "I/O configuration register for pin P2\\[6\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2_6Spec;
impl crate::RegisterSpec for P2_6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p2_6::R`](R) reader structure"]
impl crate::Readable for P2_6Spec {}
#[doc = "`write(|w| ..)` method takes [`p2_6::W`](W) writer structure"]
impl crate::Writable for P2_6Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P2_6 to value 0x30"]
impl crate::Resettable for P2_6Spec {
    const RESET_VALUE: u32 = 0x30;
}
