#[doc = "Register `P5_3` reader"]
pub type R = crate::R<P5_3Spec>;
#[doc = "Register `P5_3` writer"]
pub type W = crate::W<P5_3Spec>;
#[doc = "Selects pin function for pin P5\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Func {
    #[doc = "0: General purpose digital input/output pin."]
    P5_3 = 0,
    #[doc = "4: Receiver input for USART4."]
    U4Rxd = 4,
    #[doc = "5: I2C0 clock input/output (this pin uses a specialized I2C pad that supports I2C Fast Mode Plus."]
    I2c0Scl = 5,
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
#[doc = "Field `FUNC` reader - Selects pin function for pin P5\\[3\\]"]
pub type FuncR = crate::FieldReader<Func>;
impl FuncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Func> {
        match self.bits {
            0 => Some(Func::P5_3),
            4 => Some(Func::U4Rxd),
            5 => Some(Func::I2c0Scl),
            _ => None,
        }
    }
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn is_p5_3(&self) -> bool {
        *self == Func::P5_3
    }
    #[doc = "Receiver input for USART4."]
    #[inline(always)]
    pub fn is_u4_rxd(&self) -> bool {
        *self == Func::U4Rxd
    }
    #[doc = "I2C0 clock input/output (this pin uses a specialized I2C pad that supports I2C Fast Mode Plus."]
    #[inline(always)]
    pub fn is_i2c0_scl(&self) -> bool {
        *self == Func::I2c0Scl
    }
}
#[doc = "Field `FUNC` writer - Selects pin function for pin P5\\[3\\]"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 3, Func>;
impl<'a, REG> FuncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn p5_3(self) -> &'a mut crate::W<REG> {
        self.variant(Func::P5_3)
    }
    #[doc = "Receiver input for USART4."]
    #[inline(always)]
    pub fn u4_rxd(self) -> &'a mut crate::W<REG> {
        self.variant(Func::U4Rxd)
    }
    #[doc = "I2C0 clock input/output (this pin uses a specialized I2C pad that supports I2C Fast Mode Plus."]
    #[inline(always)]
    pub fn i2c0_scl(self) -> &'a mut crate::W<REG> {
        self.variant(Func::I2c0Scl)
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
    #[doc = "Bits 0:2 - Selects pin function for pin P5\\[3\\]"]
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
    #[doc = "Bits 0:2 - Selects pin function for pin P5\\[3\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FuncW<'_, P5_3Spec> {
        FuncW::new(self, 0)
    }
    #[doc = "Bit 6 - Invert input"]
    #[inline(always)]
    pub fn inv(&mut self) -> InvW<'_, P5_3Spec> {
        InvW::new(self, 6)
    }
    #[doc = "Bit 8 - Configures I2C features for standard mode, fast mode, and Fast Mode Plus operation."]
    #[inline(always)]
    pub fn hs(&mut self) -> HsW<'_, P5_3Spec> {
        HsW::new(self, 8)
    }
    #[doc = "Bit 9 - Controls sink current capability of the pin, only for P5\\[2\\] and P5\\[3\\]."]
    #[inline(always)]
    pub fn hidrive(&mut self) -> HidriveW<'_, P5_3Spec> {
        HidriveW::new(self, 9)
    }
}
#[doc = "I/O configuration register for pin P5\\[3\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p5_3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5_3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P5_3Spec;
impl crate::RegisterSpec for P5_3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p5_3::R`](R) reader structure"]
impl crate::Readable for P5_3Spec {}
#[doc = "`write(|w| ..)` method takes [`p5_3::W`](W) writer structure"]
impl crate::Writable for P5_3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P5_3 to value 0x80"]
impl crate::Resettable for P5_3Spec {
    const RESET_VALUE: u32 = 0x80;
}
