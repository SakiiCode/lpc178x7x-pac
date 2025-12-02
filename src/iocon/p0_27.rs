#[doc = "Register `P0_27` reader"]
pub type R = crate::R<P0_27Spec>;
#[doc = "Register `P0_27` writer"]
pub type W = crate::W<P0_27Spec>;
#[doc = "Selects pin function for pin P0\\[27\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Func {
    #[doc = "0: General purpose digital input/output pin."]
    P0_27 = 0,
    #[doc = "1: I2C0 data input/output. (This pin uses a specialized I2C pad)."]
    I2c0Sda = 1,
    #[doc = "2: I2C serial data for communication with an external USB transceiver."]
    UsbSda1 = 2,
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
#[doc = "Field `FUNC` reader - Selects pin function for pin P0\\[27\\]"]
pub type FuncR = crate::FieldReader<Func>;
impl FuncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Func> {
        match self.bits {
            0 => Some(Func::P0_27),
            1 => Some(Func::I2c0Sda),
            2 => Some(Func::UsbSda1),
            _ => None,
        }
    }
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn is_p0_27(&self) -> bool {
        *self == Func::P0_27
    }
    #[doc = "I2C0 data input/output. (This pin uses a specialized I2C pad)."]
    #[inline(always)]
    pub fn is_i2c0_sda(&self) -> bool {
        *self == Func::I2c0Sda
    }
    #[doc = "I2C serial data for communication with an external USB transceiver."]
    #[inline(always)]
    pub fn is_usb_sda1(&self) -> bool {
        *self == Func::UsbSda1
    }
}
#[doc = "Field `FUNC` writer - Selects pin function for pin P0\\[27\\]"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 3, Func>;
impl<'a, REG> FuncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn p0_27(self) -> &'a mut crate::W<REG> {
        self.variant(Func::P0_27)
    }
    #[doc = "I2C0 data input/output. (This pin uses a specialized I2C pad)."]
    #[inline(always)]
    pub fn i2c0_sda(self) -> &'a mut crate::W<REG> {
        self.variant(Func::I2c0Sda)
    }
    #[doc = "I2C serial data for communication with an external USB transceiver."]
    #[inline(always)]
    pub fn usb_sda1(self) -> &'a mut crate::W<REG> {
        self.variant(Func::UsbSda1)
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
#[doc = "Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hs {
    #[doc = "0: I2C 50ns glitch filter and slew rate control enabled."]
    Enabled = 0,
    #[doc = "1: I2C 50ns glitch filter and slew rate control disabled."]
    Disabled = 1,
}
impl From<Hs> for bool {
    #[inline(always)]
    fn from(variant: Hs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HS` reader - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation."]
pub type HsR = crate::BitReader<Hs>;
impl HsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hs {
        match self.bits {
            false => Hs::Enabled,
            true => Hs::Disabled,
        }
    }
    #[doc = "I2C 50ns glitch filter and slew rate control enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Hs::Enabled
    }
    #[doc = "I2C 50ns glitch filter and slew rate control disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Hs::Disabled
    }
}
#[doc = "Field `HS` writer - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation."]
pub type HsW<'a, REG> = crate::BitWriter<'a, REG, Hs>;
impl<'a, REG> HsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C 50ns glitch filter and slew rate control enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hs::Enabled)
    }
    #[doc = "I2C 50ns glitch filter and slew rate control disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hs::Disabled)
    }
}
#[doc = "Controls sink current capability of the pin, only for P5\\[2\\] and P5\\[3\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hidrive {
    #[doc = "0: Output drive sink is 4 mA. This is sufficient for standard and fast mode I2C."]
    Lowdrive = 0,
    #[doc = "1: Output drive sink is 20 mA. This is needed for Fast Mode Plus I2C. Refer to the appropriate specific device data sheet for details."]
    Highdrive = 1,
}
impl From<Hidrive> for bool {
    #[inline(always)]
    fn from(variant: Hidrive) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIDRIVE` reader - Controls sink current capability of the pin, only for P5\\[2\\] and P5\\[3\\]."]
pub type HidriveR = crate::BitReader<Hidrive>;
impl HidriveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hidrive {
        match self.bits {
            false => Hidrive::Lowdrive,
            true => Hidrive::Highdrive,
        }
    }
    #[doc = "Output drive sink is 4 mA. This is sufficient for standard and fast mode I2C."]
    #[inline(always)]
    pub fn is_lowdrive(&self) -> bool {
        *self == Hidrive::Lowdrive
    }
    #[doc = "Output drive sink is 20 mA. This is needed for Fast Mode Plus I2C. Refer to the appropriate specific device data sheet for details."]
    #[inline(always)]
    pub fn is_highdrive(&self) -> bool {
        *self == Hidrive::Highdrive
    }
}
#[doc = "Field `HIDRIVE` writer - Controls sink current capability of the pin, only for P5\\[2\\] and P5\\[3\\]."]
pub type HidriveW<'a, REG> = crate::BitWriter<'a, REG, Hidrive>;
impl<'a, REG> HidriveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output drive sink is 4 mA. This is sufficient for standard and fast mode I2C."]
    #[inline(always)]
    pub fn lowdrive(self) -> &'a mut crate::W<REG> {
        self.variant(Hidrive::Lowdrive)
    }
    #[doc = "Output drive sink is 20 mA. This is needed for Fast Mode Plus I2C. Refer to the appropriate specific device data sheet for details."]
    #[inline(always)]
    pub fn highdrive(self) -> &'a mut crate::W<REG> {
        self.variant(Hidrive::Highdrive)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[27\\]"]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation."]
    #[inline(always)]
    pub fn hs(&self) -> HsR {
        HsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Controls sink current capability of the pin, only for P5\\[2\\] and P5\\[3\\]."]
    #[inline(always)]
    pub fn hidrive(&self) -> HidriveR {
        HidriveR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[27\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FuncW<'_, P0_27Spec> {
        FuncW::new(self, 0)
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline(always)]
    pub fn inv(&mut self) -> InvW<'_, P0_27Spec> {
        InvW::new(self, 6)
    }
    #[doc = "Bit 8 - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation."]
    #[inline(always)]
    pub fn hs(&mut self) -> HsW<'_, P0_27Spec> {
        HsW::new(self, 8)
    }
    #[doc = "Bit 9 - Controls sink current capability of the pin, only for P5\\[2\\] and P5\\[3\\]."]
    #[inline(always)]
    pub fn hidrive(&mut self) -> HidriveW<'_, P0_27Spec> {
        HidriveW::new(self, 9)
    }
}
#[doc = "I/O configuration register for pin P0\\[27\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_27::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_27::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P0_27Spec;
impl crate::RegisterSpec for P0_27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p0_27::R`](R) reader structure"]
impl crate::Readable for P0_27Spec {}
#[doc = "`write(|w| ..)` method takes [`p0_27::W`](W) writer structure"]
impl crate::Writable for P0_27Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P0_27 to value 0x80"]
impl crate::Resettable for P0_27Spec {
    const RESET_VALUE: u32 = 0x80;
}
