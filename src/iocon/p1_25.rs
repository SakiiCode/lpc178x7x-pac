#[doc = "Register `P1_25` reader"]
pub type R = crate::R<P1_25Spec>;
#[doc = "Register `P1_25` writer"]
pub type W = crate::W<P1_25Spec>;
#[doc = "Selects pin function for pin P1\\[25\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Func {
    #[doc = "0: General purpose digital input/output pin."]
    P1_25 = 0,
    #[doc = "1: Low Speed status for USB port 1 (OTG transceiver)."]
    UsbLs1 = 1,
    #[doc = "2: Host Enabled status for USB port 1."]
    UsbHsten1 = 2,
    #[doc = "3: Match output for Timer 1, channel 1."]
    T1Mat1 = 3,
    #[doc = "4: Motor control PWM channel 1, output A."]
    Mc1a = 4,
    #[doc = "5: Selectable clock output."]
    Clkout = 5,
    #[doc = "6: LCD data."]
    LcdVd11 = 6,
    #[doc = "7: LCD data."]
    LcdVd15 = 7,
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
#[doc = "Field `FUNC` reader - Selects pin function for pin P1\\[25\\]"]
pub type FuncR = crate::FieldReader<Func>;
impl FuncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Func {
        match self.bits {
            0 => Func::P1_25,
            1 => Func::UsbLs1,
            2 => Func::UsbHsten1,
            3 => Func::T1Mat1,
            4 => Func::Mc1a,
            5 => Func::Clkout,
            6 => Func::LcdVd11,
            7 => Func::LcdVd15,
            _ => unreachable!(),
        }
    }
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn is_p1_25(&self) -> bool {
        *self == Func::P1_25
    }
    #[doc = "Low Speed status for USB port 1 (OTG transceiver)."]
    #[inline(always)]
    pub fn is_usb_ls1(&self) -> bool {
        *self == Func::UsbLs1
    }
    #[doc = "Host Enabled status for USB port 1."]
    #[inline(always)]
    pub fn is_usb_hsten1(&self) -> bool {
        *self == Func::UsbHsten1
    }
    #[doc = "Match output for Timer 1, channel 1."]
    #[inline(always)]
    pub fn is_t1_mat1(&self) -> bool {
        *self == Func::T1Mat1
    }
    #[doc = "Motor control PWM channel 1, output A."]
    #[inline(always)]
    pub fn is_mc_1a(&self) -> bool {
        *self == Func::Mc1a
    }
    #[doc = "Selectable clock output."]
    #[inline(always)]
    pub fn is_clkout(&self) -> bool {
        *self == Func::Clkout
    }
    #[doc = "LCD data."]
    #[inline(always)]
    pub fn is_lcd_vd_11(&self) -> bool {
        *self == Func::LcdVd11
    }
    #[doc = "LCD data."]
    #[inline(always)]
    pub fn is_lcd_vd_15(&self) -> bool {
        *self == Func::LcdVd15
    }
}
#[doc = "Field `FUNC` writer - Selects pin function for pin P1\\[25\\]"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 3, Func, crate::Safe>;
impl<'a, REG> FuncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn p1_25(self) -> &'a mut crate::W<REG> {
        self.variant(Func::P1_25)
    }
    #[doc = "Low Speed status for USB port 1 (OTG transceiver)."]
    #[inline(always)]
    pub fn usb_ls1(self) -> &'a mut crate::W<REG> {
        self.variant(Func::UsbLs1)
    }
    #[doc = "Host Enabled status for USB port 1."]
    #[inline(always)]
    pub fn usb_hsten1(self) -> &'a mut crate::W<REG> {
        self.variant(Func::UsbHsten1)
    }
    #[doc = "Match output for Timer 1, channel 1."]
    #[inline(always)]
    pub fn t1_mat1(self) -> &'a mut crate::W<REG> {
        self.variant(Func::T1Mat1)
    }
    #[doc = "Motor control PWM channel 1, output A."]
    #[inline(always)]
    pub fn mc_1a(self) -> &'a mut crate::W<REG> {
        self.variant(Func::Mc1a)
    }
    #[doc = "Selectable clock output."]
    #[inline(always)]
    pub fn clkout(self) -> &'a mut crate::W<REG> {
        self.variant(Func::Clkout)
    }
    #[doc = "LCD data."]
    #[inline(always)]
    pub fn lcd_vd_11(self) -> &'a mut crate::W<REG> {
        self.variant(Func::LcdVd11)
    }
    #[doc = "LCD data."]
    #[inline(always)]
    pub fn lcd_vd_15(self) -> &'a mut crate::W<REG> {
        self.variant(Func::LcdVd15)
    }
}
#[doc = "Selects function mode (on-chip pull-up/pull-down resistor control).\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mode {
    #[doc = "0: Inactive (no pull-down/pull-up resistor enabled)."]
    Inactive = 0,
    #[doc = "1: Pull-down resistor enabled."]
    Pulldown = 1,
    #[doc = "2: Pull-up resistor enabled."]
    Pullup = 2,
    #[doc = "3: Repeater mode."]
    Repeater = 3,
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
            0 => Mode::Inactive,
            1 => Mode::Pulldown,
            2 => Mode::Pullup,
            3 => Mode::Repeater,
            _ => unreachable!(),
        }
    }
    #[doc = "Inactive (no pull-down/pull-up resistor enabled)."]
    #[inline(always)]
    pub fn is_inactive(&self) -> bool {
        *self == Mode::Inactive
    }
    #[doc = "Pull-down resistor enabled."]
    #[inline(always)]
    pub fn is_pulldown(&self) -> bool {
        *self == Mode::Pulldown
    }
    #[doc = "Pull-up resistor enabled."]
    #[inline(always)]
    pub fn is_pullup(&self) -> bool {
        *self == Mode::Pullup
    }
    #[doc = "Repeater mode."]
    #[inline(always)]
    pub fn is_repeater(&self) -> bool {
        *self == Mode::Repeater
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
    pub fn inactive(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Inactive)
    }
    #[doc = "Pull-down resistor enabled."]
    #[inline(always)]
    pub fn pulldown(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Pulldown)
    }
    #[doc = "Pull-up resistor enabled."]
    #[inline(always)]
    pub fn pullup(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Pullup)
    }
    #[doc = "Repeater mode."]
    #[inline(always)]
    pub fn repeater(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Repeater)
    }
}
#[doc = "Hysteresis.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hys {
    #[doc = "0: Disable."]
    Disabled = 0,
    #[doc = "1: Enable."]
    Enabled = 1,
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
            false => Hys::Disabled,
            true => Hys::Enabled,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Hys::Disabled
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Hys::Enabled
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
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hys::Disabled)
    }
    #[doc = "Enable."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hys::Enabled)
    }
}
#[doc = "Invert input\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Inv {
    #[doc = "0: Input not inverted (HIGH on pin reads as 1, LOW on pin reads as 0)."]
    ActiveHigh = 0,
    #[doc = "1: Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    ActiveLow = 1,
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
            false => Inv::ActiveHigh,
            true => Inv::ActiveLow,
        }
    }
    #[doc = "Input not inverted (HIGH on pin reads as 1, LOW on pin reads as 0)."]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == Inv::ActiveHigh
    }
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == Inv::ActiveLow
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
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(Inv::ActiveHigh)
    }
    #[doc = "Input inverted (HIGH on pin reads as 0, LOW on pin reads as 1)."]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(Inv::ActiveLow)
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
    Disabled = 0,
    #[doc = "1: Open-drain mode enabled. This is not a true open-drain mode. Input cannot be pulled up above VDD."]
    Enabled = 1,
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
            false => Od::Disabled,
            true => Od::Enabled,
        }
    }
    #[doc = "Disable."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Od::Disabled
    }
    #[doc = "Open-drain mode enabled. This is not a true open-drain mode. Input cannot be pulled up above VDD."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Od::Enabled
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
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Od::Disabled)
    }
    #[doc = "Open-drain mode enabled. This is not a true open-drain mode. Input cannot be pulled up above VDD."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Od::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P1\\[25\\]"]
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
    #[doc = "Bits 0:2 - Selects pin function for pin P1\\[25\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FuncW<'_, P1_25Spec> {
        FuncW::new(self, 0)
    }
    #[doc = "Bits 3:4 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, P1_25Spec> {
        ModeW::new(self, 3)
    }
    #[doc = "Bit 5 - Hysteresis."]
    #[inline(always)]
    pub fn hys(&mut self) -> HysW<'_, P1_25Spec> {
        HysW::new(self, 5)
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline(always)]
    pub fn inv(&mut self) -> InvW<'_, P1_25Spec> {
        InvW::new(self, 6)
    }
    #[doc = "Bit 9 - Driver slew rate"]
    #[inline(always)]
    pub fn slew(&mut self) -> SlewW<'_, P1_25Spec> {
        SlewW::new(self, 9)
    }
    #[doc = "Bit 10 - Open-drain mode."]
    #[inline(always)]
    pub fn od(&mut self) -> OdW<'_, P1_25Spec> {
        OdW::new(self, 10)
    }
}
#[doc = "I/O configuration register for pin P1\\[25\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_25::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1_25Spec;
impl crate::RegisterSpec for P1_25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p1_25::R`](R) reader structure"]
impl crate::Readable for P1_25Spec {}
#[doc = "`write(|w| ..)` method takes [`p1_25::W`](W) writer structure"]
impl crate::Writable for P1_25Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P1_25 to value 0x30"]
impl crate::Resettable for P1_25Spec {
    const RESET_VALUE: u32 = 0x30;
}
