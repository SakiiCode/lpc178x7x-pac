#[doc = "Register `P1_31` reader"]
pub type R = crate::R<P1_31Spec>;
#[doc = "Register `P1_31` writer"]
pub type W = crate::W<P1_31Spec>;
#[doc = "Selects pin function for pin P1\\[31\\]"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Enum {
    #[doc = "0: General purpose digital input/output pin."]
    P1_31 = 0,
    #[doc = "1: Over-Current status for USB port 2."]
    UsbOvrcr2 = 1,
    #[doc = "2: Serial Clock for SSP1."]
    Ssp1Sck = 2,
    #[doc = "3: A/D converter 0, input 5. When configured as an ADC input, the digital function of the pin must be disabled."]
    Adc0In5 = 3,
    #[doc = "4: I2C0 clock input/output (this pin does not use a specialized I2C pad."]
    I2c0Scl = 4,
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
#[doc = "Field `FUNC` reader - Selects pin function for pin P1\\[31\\]"]
pub type FuncR = crate::FieldReader<Enum>;
impl FuncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Enum> {
        match self.bits {
            0 => Some(Enum::P1_31),
            1 => Some(Enum::UsbOvrcr2),
            2 => Some(Enum::Ssp1Sck),
            3 => Some(Enum::Adc0In5),
            4 => Some(Enum::I2c0Scl),
            _ => None,
        }
    }
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn is_p1_31(&self) -> bool {
        *self == Enum::P1_31
    }
    #[doc = "Over-Current status for USB port 2."]
    #[inline(always)]
    pub fn is_usb_ovrcr2(&self) -> bool {
        *self == Enum::UsbOvrcr2
    }
    #[doc = "Serial Clock for SSP1."]
    #[inline(always)]
    pub fn is_ssp1_sck(&self) -> bool {
        *self == Enum::Ssp1Sck
    }
    #[doc = "A/D converter 0, input 5. When configured as an ADC input, the digital function of the pin must be disabled."]
    #[inline(always)]
    pub fn is_adc0_in_5(&self) -> bool {
        *self == Enum::Adc0In5
    }
    #[doc = "I2C0 clock input/output (this pin does not use a specialized I2C pad."]
    #[inline(always)]
    pub fn is_i2c0_scl(&self) -> bool {
        *self == Enum::I2c0Scl
    }
}
#[doc = "Field `FUNC` writer - Selects pin function for pin P1\\[31\\]"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 3, Enum>;
impl<'a, REG> FuncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn p1_31(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::P1_31)
    }
    #[doc = "Over-Current status for USB port 2."]
    #[inline(always)]
    pub fn usb_ovrcr2(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::UsbOvrcr2)
    }
    #[doc = "Serial Clock for SSP1."]
    #[inline(always)]
    pub fn ssp1_sck(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Ssp1Sck)
    }
    #[doc = "A/D converter 0, input 5. When configured as an ADC input, the digital function of the pin must be disabled."]
    #[inline(always)]
    pub fn adc0_in_5(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Adc0In5)
    }
    #[doc = "I2C0 clock input/output (this pin does not use a specialized I2C pad."]
    #[inline(always)]
    pub fn i2c0_scl(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::I2c0Scl)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P1\\[31\\]"]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P1\\[31\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FuncW<'_, P1_31Spec> {
        FuncW::new(self, 0)
    }
}
#[doc = "I/O configuration register for pin P1\\[31\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_31::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1_31Spec;
impl crate::RegisterSpec for P1_31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p1_31::R`](R) reader structure"]
impl crate::Readable for P1_31Spec {}
#[doc = "`write(|w| ..)` method takes [`p1_31::W`](W) writer structure"]
impl crate::Writable for P1_31Spec {
    type Safety = crate::Unsafe;
}
