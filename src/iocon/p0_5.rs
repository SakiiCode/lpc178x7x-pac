#[doc = "Register `P0_5` reader"]
pub type R = crate::R<P0_5Spec>;
#[doc = "Register `P0_5` writer"]
pub type W = crate::W<P0_5Spec>;
#[doc = "Selects pin function for pin P0\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Func {
    #[doc = "0: General purpose digital input/output pin."]
    P0_5 = 0,
    #[doc = "1: I2S Receive word select. It is driven by the master and received by the slave. Corresponds to the signal WS in the I2S-bus specification."]
    I2sRxWs = 1,
    #[doc = "2: CAN2 transmitter output."]
    CanTd2 = 2,
    #[doc = "3: Capture input for Timer 2, channel 1."]
    T2Cap1 = 3,
    #[doc = "7: LCD data."]
    LcdVd1 = 7,
}
impl From<Func> for u8 {
    #[inline(always)]
    fn from(variant: Func) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Func {
    type Ux = u8;
}
impl crate::IsEnum for Func {}
#[doc = "Field `FUNC` reader - Selects pin function for pin P0\\[5\\]"]
pub type FuncR = crate::FieldReader<Func>;
impl FuncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Func> {
        match self.bits {
            0 => Some(Func::P0_5),
            1 => Some(Func::I2sRxWs),
            2 => Some(Func::CanTd2),
            3 => Some(Func::T2Cap1),
            7 => Some(Func::LcdVd1),
            _ => None,
        }
    }
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn is_p0_5(&self) -> bool {
        *self == Func::P0_5
    }
    #[doc = "I2S Receive word select. It is driven by the master and received by the slave. Corresponds to the signal WS in the I2S-bus specification."]
    #[inline(always)]
    pub fn is_i2s_rx_ws(&self) -> bool {
        *self == Func::I2sRxWs
    }
    #[doc = "CAN2 transmitter output."]
    #[inline(always)]
    pub fn is_can_td2(&self) -> bool {
        *self == Func::CanTd2
    }
    #[doc = "Capture input for Timer 2, channel 1."]
    #[inline(always)]
    pub fn is_t2_cap1(&self) -> bool {
        *self == Func::T2Cap1
    }
    #[doc = "LCD data."]
    #[inline(always)]
    pub fn is_lcd_vd_1(&self) -> bool {
        *self == Func::LcdVd1
    }
}
#[doc = "Field `FUNC` writer - Selects pin function for pin P0\\[5\\]"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 3, Func>;
impl<'a, REG> FuncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn p0_5(self) -> &'a mut crate::W<REG> {
        self.variant(Func::P0_5)
    }
    #[doc = "I2S Receive word select. It is driven by the master and received by the slave. Corresponds to the signal WS in the I2S-bus specification."]
    #[inline(always)]
    pub fn i2s_rx_ws(self) -> &'a mut crate::W<REG> {
        self.variant(Func::I2sRxWs)
    }
    #[doc = "CAN2 transmitter output."]
    #[inline(always)]
    pub fn can_td2(self) -> &'a mut crate::W<REG> {
        self.variant(Func::CanTd2)
    }
    #[doc = "Capture input for Timer 2, channel 1."]
    #[inline(always)]
    pub fn t2_cap1(self) -> &'a mut crate::W<REG> {
        self.variant(Func::T2Cap1)
    }
    #[doc = "LCD data."]
    #[inline(always)]
    pub fn lcd_vd_1(self) -> &'a mut crate::W<REG> {
        self.variant(Func::LcdVd1)
    }
}
#[doc = "Selects function mode (on-chip pull-up/pull-down resistor control).\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Inactive (no pull-down/pull-up resistor enabled)."]
    InactiveNoPullDo = 0,
    #[doc = "1: Pull-down resistor enabled."]
    PullDownResistorE = 1,
    #[doc = "2: Pull-up resistor enabled."]
    PullUpResistorEna = 2,
    #[doc = "3: Repeater mode."]
    RepeaterMode_ = 3,
}
impl From<Mode> for u8 {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mode {
    type Ux = u8;
}
impl crate::IsEnum for Mode {}
#[doc = "Field `MODE` reader - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type ModeR = crate::FieldReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            0 => Mode::InactiveNoPullDo,
            1 => Mode::PullDownResistorE,
            2 => Mode::PullUpResistorEna,
            3 => Mode::RepeaterMode_,
            _ => unreachable!(),
        }
    }
    #[doc = "Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn is_inactive_no_pull_do(&self) -> bool {
        *self == Mode::InactiveNoPullDo
    }
    #[doc = "Pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pull_down_resistor_e(&self) -> bool {
        *self == Mode::PullDownResistorE
    }
    #[doc = "Pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pull_up_resistor_ena(&self) -> bool {
        *self == Mode::PullUpResistorEna
    }
    #[doc = "Repeater mode."]
    #[inline(always)]
    pub fn is_repeater_mode_(&self) -> bool {
        *self == Mode::RepeaterMode_
    }
}
#[doc = "Field `MODE` writer - Selects function mode (on-chip pull-up/pull-down resistor control)."]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mode, crate::Safe>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn inactive_no_pull_do(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::InactiveNoPullDo)
    }
    #[doc = "Pull-down resistor enabled."]
    #[inline(always)]
    pub fn pull_down_resistor_e(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::PullDownResistorE)
    }
    #[doc = "Pull-up resistor enabled."]
    #[inline(always)]
    pub fn pull_up_resistor_ena(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::PullUpResistorEna)
    }
    #[doc = "Repeater mode."]
    #[inline(always)]
    pub fn repeater_mode_(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::RepeaterMode_)
    }
}
#[doc = "Hysteresis.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hys {
    #[doc = "0: Disable."]
    Disable_ = 0,
    #[doc = "1: Enable."]
    Enable_ = 1,
}
impl From<Hys> for bool {
    #[inline(always)]
    fn from(variant: Hys) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HYS` reader - Hysteresis."]
pub type HysR = crate::BitReader<Hys>;
impl HysR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hys {
        match self.bits {
            false => Hys::Disable_,
            true => Hys::Enable_,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable_(&self) -> bool {
        *self == Hys::Disable_
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_enable_(&self) -> bool {
        *self == Hys::Enable_
    }
}
#[doc = "Field `HYS` writer - Hysteresis."]
pub type HysW<'a, REG> = crate::BitWriter<'a, REG, Hys>;
impl<'a, REG> HysW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable_(self) -> &'a mut crate::W<REG> {
        self.variant(Hys::Disable_)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn enable_(self) -> &'a mut crate::W<REG> {
        self.variant(Hys::Enable_)
    }
}
#[doc = "Invert input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inv {
    #[doc = "0: Input not inverted (HIGH on pin reads as 1, LOW on pin reads as 0)."]
    InputNotInverted_ = 0,
    #[doc = "1: Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    InputInvertedHigh = 1,
}
impl From<Inv> for bool {
    #[inline(always)]
    fn from(variant: Inv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INV` reader - Invert input"]
pub type InvR = crate::BitReader<Inv>;
impl InvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Inv {
        match self.bits {
            false => Inv::InputNotInverted_,
            true => Inv::InputInvertedHigh,
        }
    }
    #[doc = "Input not inverted (HIGH on pin reads as 1, LOW on pin reads as 0)."]
    #[inline(always)]
    pub fn is_input_not_inverted_(&self) -> bool {
        *self == Inv::InputNotInverted_
    }
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    #[inline(always)]
    pub fn is_input_inverted_high(&self) -> bool {
        *self == Inv::InputInvertedHigh
    }
}
#[doc = "Field `INV` writer - Invert input"]
pub type InvW<'a, REG> = crate::BitWriter<'a, REG, Inv>;
impl<'a, REG> InvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Input not inverted (HIGH on pin reads as 1, LOW on pin reads as 0)."]
    #[inline(always)]
    pub fn input_not_inverted_(self) -> &'a mut crate::W<REG> {
        self.variant(Inv::InputNotInverted_)
    }
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    #[inline(always)]
    pub fn input_inverted_high(self) -> &'a mut crate::W<REG> {
        self.variant(Inv::InputInvertedHigh)
    }
}
#[doc = "Driver slew rate\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Slew {
    #[doc = "0: Standard mode, output slew rate control is enabled. More outputs can be switched simultaneously."]
    Standard = 0,
    #[doc = "1: Fast mode, slew rate control is disabled. Refer to the appropriate specific device data sheet for details."]
    Fast = 1,
}
impl From<Slew> for bool {
    #[inline(always)]
    fn from(variant: Slew) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SLEW` reader - Driver slew rate"]
pub type SlewR = crate::BitReader<Slew>;
impl SlewR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Slew {
        match self.bits {
            false => Slew::Standard,
            true => Slew::Fast,
        }
    }
    #[doc = "Standard mode, output slew rate control is enabled. More outputs can be switched simultaneously."]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == Slew::Standard
    }
    #[doc = "Fast mode, slew rate control is disabled. Refer to the appropriate specific device data sheet for details."]
    #[inline(always)]
    pub fn is_fast(&self) -> bool {
        *self == Slew::Fast
    }
}
#[doc = "Field `SLEW` writer - Driver slew rate"]
pub type SlewW<'a, REG> = crate::BitWriter<'a, REG, Slew>;
impl<'a, REG> SlewW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard mode, output slew rate control is enabled. More outputs can be switched simultaneously."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(Slew::Standard)
    }
    #[doc = "Fast mode, slew rate control is disabled. Refer to the appropriate specific device data sheet for details."]
    #[inline(always)]
    pub fn fast(self) -> &'a mut crate::W<REG> {
        self.variant(Slew::Fast)
    }
}
#[doc = "Open-drain mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Od {
    #[doc = "0: Disable."]
    Disable_ = 0,
    #[doc = "1: Open-drain mode enabled. This is not a true open-drain mode. Input cannot be pulled up above VDD."]
    OpenDrainModeEnab = 1,
}
impl From<Od> for bool {
    #[inline(always)]
    fn from(variant: Od) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OD` reader - Open-drain mode."]
pub type OdR = crate::BitReader<Od>;
impl OdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Od {
        match self.bits {
            false => Od::Disable_,
            true => Od::OpenDrainModeEnab,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disable_(&self) -> bool {
        *self == Od::Disable_
    }
    #[doc = "Open-drain mode enabled. This is not a true open-drain mode. Input cannot be pulled up above VDD."]
    #[inline(always)]
    pub fn is_open_drain_mode_enab(&self) -> bool {
        *self == Od::OpenDrainModeEnab
    }
}
#[doc = "Field `OD` writer - Open-drain mode."]
pub type OdW<'a, REG> = crate::BitWriter<'a, REG, Od>;
impl<'a, REG> OdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable."]
    #[inline(always)]
    pub fn disable_(self) -> &'a mut crate::W<REG> {
        self.variant(Od::Disable_)
    }
    #[doc = "Open-drain mode enabled. This is not a true open-drain mode. Input cannot be pulled up above VDD."]
    #[inline(always)]
    pub fn open_drain_mode_enab(self) -> &'a mut crate::W<REG> {
        self.variant(Od::OpenDrainModeEnab)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[5\\]"]
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
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[5\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FuncW<'_, P0_5Spec> {
        FuncW::new(self, 0)
    }
    #[doc = "Bits 3:4 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, P0_5Spec> {
        ModeW::new(self, 3)
    }
    #[doc = "Bit 5 - Hysteresis."]
    #[inline(always)]
    pub fn hys(&mut self) -> HysW<'_, P0_5Spec> {
        HysW::new(self, 5)
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline(always)]
    pub fn inv(&mut self) -> InvW<'_, P0_5Spec> {
        InvW::new(self, 6)
    }
    #[doc = "Bit 9 - Driver slew rate"]
    #[inline(always)]
    pub fn slew(&mut self) -> SlewW<'_, P0_5Spec> {
        SlewW::new(self, 9)
    }
    #[doc = "Bit 10 - Open-drain mode."]
    #[inline(always)]
    pub fn od(&mut self) -> OdW<'_, P0_5Spec> {
        OdW::new(self, 10)
    }
}
#[doc = "I/O configuration register for pin P0\\[5\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P0_5Spec;
impl crate::RegisterSpec for P0_5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p0_5::R`](R) reader structure"]
impl crate::Readable for P0_5Spec {}
#[doc = "`write(|w| ..)` method takes [`p0_5::W`](W) writer structure"]
impl crate::Writable for P0_5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P0_5 to value 0x30"]
impl crate::Resettable for P0_5Spec {
    const RESET_VALUE: u32 = 0x30;
}
