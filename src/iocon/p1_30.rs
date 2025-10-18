#[doc = "Register `P1_30` reader"]
pub type R = crate::R<P1_30Spec>;
#[doc = "Register `P1_30` writer"]
pub type W = crate::W<P1_30Spec>;
#[doc = "Selects pin function for pin P1\\[30\\]"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Func {
    #[doc = "0: General purpose digital input/output pin."]
    P1_30 = 0,
    #[doc = "1: Power Status for USB port 2."]
    UsbPwrd2 = 1,
    #[doc = "2: Monitors the presence of USB bus power.This signal must be HIGH for USB reset to occur."]
    UsbVbus = 2,
    #[doc = "3: A/D converter 0, input 4. When configured as an ADC input, the digital function of the pin must be disabled."]
    Adc0In4 = 3,
    #[doc = "4: I2C0 data input/output (this pin does not use a specialized I2C pad."]
    I2c0Sda = 4,
    #[doc = "5: RS-485/EIA-485 output enable signal for UART3."]
    U3Oe = 5,
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
#[doc = "Field `FUNC` reader - Selects pin function for pin P1\\[30\\]"]
pub type FuncR = crate::FieldReader<Func>;
impl FuncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Func> {
        match self.bits {
            0 => Some(Func::P1_30),
            1 => Some(Func::UsbPwrd2),
            2 => Some(Func::UsbVbus),
            3 => Some(Func::Adc0In4),
            4 => Some(Func::I2c0Sda),
            5 => Some(Func::U3Oe),
            _ => None,
        }
    }
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn is_p1_30(&self) -> bool {
        *self == Func::P1_30
    }
    #[doc = "Power Status for USB port 2."]
    #[inline(always)]
    pub fn is_usb_pwrd2(&self) -> bool {
        *self == Func::UsbPwrd2
    }
    #[doc = "Monitors the presence of USB bus power.This signal must be HIGH for USB reset to occur."]
    #[inline(always)]
    pub fn is_usb_vbus(&self) -> bool {
        *self == Func::UsbVbus
    }
    #[doc = "A/D converter 0, input 4. When configured as an ADC input, the digital function of the pin must be disabled."]
    #[inline(always)]
    pub fn is_adc0_in_4(&self) -> bool {
        *self == Func::Adc0In4
    }
    #[doc = "I2C0 data input/output (this pin does not use a specialized I2C pad."]
    #[inline(always)]
    pub fn is_i2c0_sda(&self) -> bool {
        *self == Func::I2c0Sda
    }
    #[doc = "RS-485/EIA-485 output enable signal for UART3."]
    #[inline(always)]
    pub fn is_u3_oe(&self) -> bool {
        *self == Func::U3Oe
    }
}
#[doc = "Field `FUNC` writer - Selects pin function for pin P1\\[30\\]"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 3, Func>;
impl<'a, REG> FuncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn p1_30(self) -> &'a mut crate::W<REG> {
        self.variant(Func::P1_30)
    }
    #[doc = "Power Status for USB port 2."]
    #[inline(always)]
    pub fn usb_pwrd2(self) -> &'a mut crate::W<REG> {
        self.variant(Func::UsbPwrd2)
    }
    #[doc = "Monitors the presence of USB bus power.This signal must be HIGH for USB reset to occur."]
    #[inline(always)]
    pub fn usb_vbus(self) -> &'a mut crate::W<REG> {
        self.variant(Func::UsbVbus)
    }
    #[doc = "A/D converter 0, input 4. When configured as an ADC input, the digital function of the pin must be disabled."]
    #[inline(always)]
    pub fn adc0_in_4(self) -> &'a mut crate::W<REG> {
        self.variant(Func::Adc0In4)
    }
    #[doc = "I2C0 data input/output (this pin does not use a specialized I2C pad."]
    #[inline(always)]
    pub fn i2c0_sda(self) -> &'a mut crate::W<REG> {
        self.variant(Func::I2c0Sda)
    }
    #[doc = "RS-485/EIA-485 output enable signal for UART3."]
    #[inline(always)]
    pub fn u3_oe(self) -> &'a mut crate::W<REG> {
        self.variant(Func::U3Oe)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P1\\[30\\]"]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P1\\[30\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FuncW<'_, P1_30Spec> {
        FuncW::new(self, 0)
    }
}
#[doc = "I/O configuration register for pin P1\\[30\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p1_30::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1_30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P1_30Spec;
impl crate::RegisterSpec for P1_30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p1_30::R`](R) reader structure"]
impl crate::Readable for P1_30Spec {}
#[doc = "`write(|w| ..)` method takes [`p1_30::W`](W) writer structure"]
impl crate::Writable for P1_30Spec {
    type Safety = crate::Unsafe;
}
