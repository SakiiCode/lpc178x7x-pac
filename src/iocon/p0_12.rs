#[doc = "Register `P0_12` reader"]
pub type R = crate::R<P0_12Spec>;
#[doc = "Register `P0_12` writer"]
pub type W = crate::W<P0_12Spec>;
#[doc = "Selects pin function for pin P0\\[12\\]"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Func {
    #[doc = "0: General purpose digital input/output pin."]
    P0_12 = 0,
    #[doc = "1: Port Power enable signal for USB port 2."]
    UsbPpwr2 = 1,
    #[doc = "2: Master In Slave Out for SSP1."]
    Ssp1Miso = 2,
    #[doc = "3: A/D converter 0, input 6. When configured as an ADC input, the digital function of the pin must be disabled."]
    Adc0In6 = 3,
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
#[doc = "Field `FUNC` reader - Selects pin function for pin P0\\[12\\]"]
pub type FuncR = crate::FieldReader<Func>;
impl FuncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Func> {
        match self.bits {
            0 => Some(Func::P0_12),
            1 => Some(Func::UsbPpwr2),
            2 => Some(Func::Ssp1Miso),
            3 => Some(Func::Adc0In6),
            _ => None,
        }
    }
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn is_p0_12(&self) -> bool {
        *self == Func::P0_12
    }
    #[doc = "Port Power enable signal for USB port 2."]
    #[inline(always)]
    pub fn is_usb_ppwr2(&self) -> bool {
        *self == Func::UsbPpwr2
    }
    #[doc = "Master In Slave Out for SSP1."]
    #[inline(always)]
    pub fn is_ssp1_miso(&self) -> bool {
        *self == Func::Ssp1Miso
    }
    #[doc = "A/D converter 0, input 6. When configured as an ADC input, the digital function of the pin must be disabled."]
    #[inline(always)]
    pub fn is_adc0_in_6(&self) -> bool {
        *self == Func::Adc0In6
    }
}
#[doc = "Field `FUNC` writer - Selects pin function for pin P0\\[12\\]"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 3, Func>;
impl<'a, REG> FuncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn p0_12(self) -> &'a mut crate::W<REG> {
        self.variant(Func::P0_12)
    }
    #[doc = "Port Power enable signal for USB port 2."]
    #[inline(always)]
    pub fn usb_ppwr2(self) -> &'a mut crate::W<REG> {
        self.variant(Func::UsbPpwr2)
    }
    #[doc = "Master In Slave Out for SSP1."]
    #[inline(always)]
    pub fn ssp1_miso(self) -> &'a mut crate::W<REG> {
        self.variant(Func::Ssp1Miso)
    }
    #[doc = "A/D converter 0, input 6. When configured as an ADC input, the digital function of the pin must be disabled."]
    #[inline(always)]
    pub fn adc0_in_6(self) -> &'a mut crate::W<REG> {
        self.variant(Func::Adc0In6)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[12\\]"]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[12\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FuncW<'_, P0_12Spec> {
        FuncW::new(self, 0)
    }
}
#[doc = "I/O configuration register for pin P0\\[12\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_12::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P0_12Spec;
impl crate::RegisterSpec for P0_12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p0_12::R`](R) reader structure"]
impl crate::Readable for P0_12Spec {}
#[doc = "`write(|w| ..)` method takes [`p0_12::W`](W) writer structure"]
impl crate::Writable for P0_12Spec {
    type Safety = crate::Unsafe;
}
