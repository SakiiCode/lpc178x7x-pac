#[doc = "Register `P0_22` reader"]
pub type R = crate::R<P0_22Spec>;
#[doc = "Register `P0_22` writer"]
pub type W = crate::W<P0_22Spec>;
#[doc = "Selects pin function for pin P0\\[22\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Func {
    #[doc = "0: General purpose digital input/output pin."]
    P0_22 = 0,
    #[doc = "1: Request to Send output for UART1. Can also be configured to be an RS-485/EIA-485 output enable signal for UART1."]
    U1Rts = 1,
    #[doc = "2: Data line 0 for SD card interface."]
    SdDat0 = 2,
    #[doc = "3: Transmitter output for USART4 (input/output in smart card mode)."]
    U4Txd = 3,
    #[doc = "4: CAN1 transmitter output."]
    CanTd1 = 4,
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
#[doc = "Field `FUNC` reader - Selects pin function for pin P0\\[22\\]"]
pub type FuncR = crate::FieldReader<Func>;
impl FuncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Func> {
        match self.bits {
            0 => Some(Func::P0_22),
            1 => Some(Func::U1Rts),
            2 => Some(Func::SdDat0),
            3 => Some(Func::U4Txd),
            4 => Some(Func::CanTd1),
            _ => None,
        }
    }
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn is_p0_22(&self) -> bool {
        *self == Func::P0_22
    }
    #[doc = "Request to Send output for UART1. Can also be configured to be an RS-485/EIA-485 output enable signal for UART1."]
    #[inline(always)]
    pub fn is_u1_rts(&self) -> bool {
        *self == Func::U1Rts
    }
    #[doc = "Data line 0 for SD card interface."]
    #[inline(always)]
    pub fn is_sd_dat_0(&self) -> bool {
        *self == Func::SdDat0
    }
    #[doc = "Transmitter output for USART4 (input/output in smart card mode)."]
    #[inline(always)]
    pub fn is_u4_txd(&self) -> bool {
        *self == Func::U4Txd
    }
    #[doc = "CAN1 transmitter output."]
    #[inline(always)]
    pub fn is_can_td1(&self) -> bool {
        *self == Func::CanTd1
    }
}
#[doc = "Field `FUNC` writer - Selects pin function for pin P0\\[22\\]"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 3, Func>;
impl<'a, REG> FuncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn p0_22(self) -> &'a mut crate::W<REG> {
        self.variant(Func::P0_22)
    }
    #[doc = "Request to Send output for UART1. Can also be configured to be an RS-485/EIA-485 output enable signal for UART1."]
    #[inline(always)]
    pub fn u1_rts(self) -> &'a mut crate::W<REG> {
        self.variant(Func::U1Rts)
    }
    #[doc = "Data line 0 for SD card interface."]
    #[inline(always)]
    pub fn sd_dat_0(self) -> &'a mut crate::W<REG> {
        self.variant(Func::SdDat0)
    }
    #[doc = "Transmitter output for USART4 (input/output in smart card mode)."]
    #[inline(always)]
    pub fn u4_txd(self) -> &'a mut crate::W<REG> {
        self.variant(Func::U4Txd)
    }
    #[doc = "CAN1 transmitter output."]
    #[inline(always)]
    pub fn can_td1(self) -> &'a mut crate::W<REG> {
        self.variant(Func::CanTd1)
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
#[doc = "Selects Analog/Digital mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Admode {
    #[doc = "0: Analog input mode."]
    Analog = 0,
    #[doc = "1: Digital functional mode."]
    Digital = 1,
}
impl From<Admode> for bool {
    #[inline(always)]
    fn from(variant: Admode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADMODE` reader - Selects Analog/Digital mode."]
pub type AdmodeR = crate::BitReader<Admode>;
impl AdmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Admode {
        match self.bits {
            false => Admode::Analog,
            true => Admode::Digital,
        }
    }
    #[doc = "Analog input mode."]
    #[inline(always)]
    pub fn is_analog(&self) -> bool {
        *self == Admode::Analog
    }
    #[doc = "Digital functional mode."]
    #[inline(always)]
    pub fn is_digital(&self) -> bool {
        *self == Admode::Digital
    }
}
#[doc = "Field `ADMODE` writer - Selects Analog/Digital mode."]
pub type AdmodeW<'a, REG> = crate::BitWriter<'a, REG, Admode>;
impl<'a, REG> AdmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Analog input mode."]
    #[inline(always)]
    pub fn analog(self) -> &'a mut crate::W<REG> {
        self.variant(Admode::Analog)
    }
    #[doc = "Digital functional mode."]
    #[inline(always)]
    pub fn digital(self) -> &'a mut crate::W<REG> {
        self.variant(Admode::Digital)
    }
}
#[doc = "Selects 10 ns input glitch filter.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Filtr {
    #[doc = "0: Filter disabled."]
    Disabled = 0,
    #[doc = "1: Filter enabled."]
    Enabled = 1,
}
impl From<Filtr> for bool {
    #[inline(always)]
    fn from(variant: Filtr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FILTR` reader - Selects 10 ns input glitch filter."]
pub type FiltrR = crate::BitReader<Filtr>;
impl FiltrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Filtr {
        match self.bits {
            false => Filtr::Disabled,
            true => Filtr::Enabled,
        }
    }
    #[doc = "Filter disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Filtr::Disabled
    }
    #[doc = "Filter enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Filtr::Enabled
    }
}
#[doc = "Field `FILTR` writer - Selects 10 ns input glitch filter."]
pub type FiltrW<'a, REG> = crate::BitWriter<'a, REG, Filtr>;
impl<'a, REG> FiltrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Filter disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Filtr::Disabled)
    }
    #[doc = "Filter enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Filtr::Enabled)
    }
}
#[doc = "Open-drain mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Od {
    #[doc = "0: Disable."]
    Disabled = 0,
    #[doc = "1: Open-drain mode enabled. This is not a true open-drain mode."]
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
    #[doc = "Open-drain mode enabled. This is not a true open-drain mode."]
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
    #[doc = "Open-drain mode enabled. This is not a true open-drain mode."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Od::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[22\\]"]
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
    #[doc = "Bit 7 - Selects Analog/Digital mode."]
    #[inline(always)]
    pub fn admode(&self) -> AdmodeR {
        AdmodeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Selects 10 ns input glitch filter."]
    #[inline(always)]
    pub fn filtr(&self) -> FiltrR {
        FiltrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Open-drain mode."]
    #[inline(always)]
    pub fn od(&self) -> OdR {
        OdR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[22\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FuncW<'_, P0_22Spec> {
        FuncW::new(self, 0)
    }
    #[doc = "Bits 3:4 - Selects function mode (on-chip pull-up/pull-down resistor control)."]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, P0_22Spec> {
        ModeW::new(self, 3)
    }
    #[doc = "Bit 5 - Hysteresis."]
    #[inline(always)]
    pub fn hys(&mut self) -> HysW<'_, P0_22Spec> {
        HysW::new(self, 5)
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline(always)]
    pub fn inv(&mut self) -> InvW<'_, P0_22Spec> {
        InvW::new(self, 6)
    }
    #[doc = "Bit 7 - Selects Analog/Digital mode."]
    #[inline(always)]
    pub fn admode(&mut self) -> AdmodeW<'_, P0_22Spec> {
        AdmodeW::new(self, 7)
    }
    #[doc = "Bit 8 - Selects 10 ns input glitch filter."]
    #[inline(always)]
    pub fn filtr(&mut self) -> FiltrW<'_, P0_22Spec> {
        FiltrW::new(self, 8)
    }
    #[doc = "Bit 10 - Open-drain mode."]
    #[inline(always)]
    pub fn od(&mut self) -> OdW<'_, P0_22Spec> {
        OdW::new(self, 10)
    }
}
#[doc = "I/O configuration register for pin P0\\[22\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P0_22Spec;
impl crate::RegisterSpec for P0_22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p0_22::R`](R) reader structure"]
impl crate::Readable for P0_22Spec {}
#[doc = "`write(|w| ..)` method takes [`p0_22::W`](W) writer structure"]
impl crate::Writable for P0_22Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P0_22 to value 0x01b0"]
impl crate::Resettable for P0_22Spec {
    const RESET_VALUE: u32 = 0x01b0;
}
