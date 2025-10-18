#[doc = "Register `P0_28` reader"]
pub type R = crate::R<P0_28Spec>;
#[doc = "Register `P0_28` writer"]
pub type W = crate::W<P0_28Spec>;
#[doc = "Selects pin function for pin P0\\[28\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Enum {
    #[doc = "0: General purpose digital input/output pin."]
    P0_28 = 0,
    #[doc = "1: I2C0 clock input/output (this pin uses a specialized I2C pad."]
    I2c0Scl = 1,
    #[doc = "2: I2C serial clock for communication with an external USB transceiver."]
    UsbScl1 = 2,
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
#[doc = "Field `FUNC` reader - Selects pin function for pin P0\\[28\\]"]
pub type FuncR = crate::FieldReader<Enum>;
impl FuncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Enum> {
        match self.bits {
            0 => Some(Enum::P0_28),
            1 => Some(Enum::I2c0Scl),
            2 => Some(Enum::UsbScl1),
            _ => None,
        }
    }
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn is_p0_28(&self) -> bool {
        *self == Enum::P0_28
    }
    #[doc = "I2C0 clock input/output (this pin uses a specialized I2C pad."]
    #[inline(always)]
    pub fn is_i2c0_scl(&self) -> bool {
        *self == Enum::I2c0Scl
    }
    #[doc = "I2C serial clock for communication with an external USB transceiver."]
    #[inline(always)]
    pub fn is_usb_scl1(&self) -> bool {
        *self == Enum::UsbScl1
    }
}
#[doc = "Field `FUNC` writer - Selects pin function for pin P0\\[28\\]"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 3, Enum>;
impl<'a, REG> FuncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn p0_28(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::P0_28)
    }
    #[doc = "I2C0 clock input/output (this pin uses a specialized I2C pad."]
    #[inline(always)]
    pub fn i2c0_scl(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::I2c0Scl)
    }
    #[doc = "I2C serial clock for communication with an external USB transceiver."]
    #[inline(always)]
    pub fn usb_scl1(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::UsbScl1)
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
#[doc = "Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: I2C 50ns glitch filter and slew rate control enabled."]
    Enabled = 0,
    #[doc = "1: I2C 50ns glitch filter and slew rate control disabled."]
    Disabled = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HS` reader - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation."]
pub type HsR = crate::BitReader<Enum>;
impl HsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Enabled,
            true => Enum::Disabled,
        }
    }
    #[doc = "I2C 50ns glitch filter and slew rate control enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enum::Enabled
    }
    #[doc = "I2C 50ns glitch filter and slew rate control disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enum::Disabled
    }
}
#[doc = "Field `HS` writer - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation."]
pub type HsW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> HsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "I2C 50ns glitch filter and slew rate control enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Enabled)
    }
    #[doc = "I2C 50ns glitch filter and slew rate control disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled)
    }
}
#[doc = "Controls sink current capability of the pin, only for P5\\[2\\] and P5\\[3\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Output drive sink is 4 mA. This is sufficient for standard and fast mode I2C."]
    Lowdrive = 0,
    #[doc = "1: Output drive sink is 20 mA. This is needed for Fast Mode Plus I2C. Refer to the appropriate specific device data sheet for details."]
    Highdrive = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HIDRIVE` reader - Controls sink current capability of the pin, only for P5\\[2\\] and P5\\[3\\]."]
pub type HidriveR = crate::BitReader<Enum>;
impl HidriveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Lowdrive,
            true => Enum::Highdrive,
        }
    }
    #[doc = "Output drive sink is 4 mA. This is sufficient for standard and fast mode I2C."]
    #[inline(always)]
    pub fn is_lowdrive(&self) -> bool {
        *self == Enum::Lowdrive
    }
    #[doc = "Output drive sink is 20 mA. This is needed for Fast Mode Plus I2C. Refer to the appropriate specific device data sheet for details."]
    #[inline(always)]
    pub fn is_highdrive(&self) -> bool {
        *self == Enum::Highdrive
    }
}
#[doc = "Field `HIDRIVE` writer - Controls sink current capability of the pin, only for P5\\[2\\] and P5\\[3\\]."]
pub type HidriveW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> HidriveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Output drive sink is 4 mA. This is sufficient for standard and fast mode I2C."]
    #[inline(always)]
    pub fn lowdrive(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Lowdrive)
    }
    #[doc = "Output drive sink is 20 mA. This is needed for Fast Mode Plus I2C. Refer to the appropriate specific device data sheet for details."]
    #[inline(always)]
    pub fn highdrive(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Highdrive)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[28\\]"]
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
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[28\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FuncW<'_, P0_28Spec> {
        FuncW::new(self, 0)
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline(always)]
    pub fn inv(&mut self) -> InvW<'_, P0_28Spec> {
        InvW::new(self, 6)
    }
    #[doc = "Bit 8 - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation."]
    #[inline(always)]
    pub fn hs(&mut self) -> HsW<'_, P0_28Spec> {
        HsW::new(self, 8)
    }
    #[doc = "Bit 9 - Controls sink current capability of the pin, only for P5\\[2\\] and P5\\[3\\]."]
    #[inline(always)]
    pub fn hidrive(&mut self) -> HidriveW<'_, P0_28Spec> {
        HidriveW::new(self, 9)
    }
}
#[doc = "I/O configuration register for pin P0\\[28\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P0_28Spec;
impl crate::RegisterSpec for P0_28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p0_28::R`](R) reader structure"]
impl crate::Readable for P0_28Spec {}
#[doc = "`write(|w| ..)` method takes [`p0_28::W`](W) writer structure"]
impl crate::Writable for P0_28Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P0_28 to value 0x80"]
impl crate::Resettable for P0_28Spec {
    const RESET_VALUE: u32 = 0x80;
}
