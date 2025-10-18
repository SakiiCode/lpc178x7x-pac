#[doc = "Register `P0_23` reader"]
pub type R = crate::R<P0_23Spec>;
#[doc = "Register `P0_23` writer"]
pub type W = crate::W<P0_23Spec>;
#[doc = "Selects pin function for pin P0\\[23\\]"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Func {
    #[doc = "0: General purpose digital input/output pin."]
    P0_23 = 0,
    #[doc = "1: A/D converter 0, input 0. When configured as an ADC input, the digital function of the pin must be disabled."]
    Adc0In0 = 1,
    #[doc = "2: Receive Clock. It is driven by the master and received by the slave. Corresponds to the signal SCK in the I2S-bus specification."]
    I2sRxSck = 2,
    #[doc = "3: Capture input for Timer 3, channel 0."]
    T3Cap0 = 3,
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
#[doc = "Field `FUNC` reader - Selects pin function for pin P0\\[23\\]"]
pub type FuncR = crate::FieldReader<Func>;
impl FuncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Func> {
        match self.bits {
            0 => Some(Func::P0_23),
            1 => Some(Func::Adc0In0),
            2 => Some(Func::I2sRxSck),
            3 => Some(Func::T3Cap0),
            _ => None,
        }
    }
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn is_p0_23(&self) -> bool {
        *self == Func::P0_23
    }
    #[doc = "A/D converter 0, input 0. When configured as an ADC input, the digital function of the pin must be disabled."]
    #[inline(always)]
    pub fn is_adc0_in_0(&self) -> bool {
        *self == Func::Adc0In0
    }
    #[doc = "Receive Clock. It is driven by the master and received by the slave. Corresponds to the signal SCK in the I2S-bus specification."]
    #[inline(always)]
    pub fn is_i2s_rx_sck(&self) -> bool {
        *self == Func::I2sRxSck
    }
    #[doc = "Capture input for Timer 3, channel 0."]
    #[inline(always)]
    pub fn is_t3_cap0(&self) -> bool {
        *self == Func::T3Cap0
    }
}
#[doc = "Field `FUNC` writer - Selects pin function for pin P0\\[23\\]"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 3, Func>;
impl<'a, REG> FuncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn p0_23(self) -> &'a mut crate::W<REG> {
        self.variant(Func::P0_23)
    }
    #[doc = "A/D converter 0, input 0. When configured as an ADC input, the digital function of the pin must be disabled."]
    #[inline(always)]
    pub fn adc0_in_0(self) -> &'a mut crate::W<REG> {
        self.variant(Func::Adc0In0)
    }
    #[doc = "Receive Clock. It is driven by the master and received by the slave. Corresponds to the signal SCK in the I2S-bus specification."]
    #[inline(always)]
    pub fn i2s_rx_sck(self) -> &'a mut crate::W<REG> {
        self.variant(Func::I2sRxSck)
    }
    #[doc = "Capture input for Timer 3, channel 0."]
    #[inline(always)]
    pub fn t3_cap0(self) -> &'a mut crate::W<REG> {
        self.variant(Func::T3Cap0)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[23\\]"]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[23\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FuncW<'_, P0_23Spec> {
        FuncW::new(self, 0)
    }
}
#[doc = "I/O configuration register for pin P0\\[23\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_23::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P0_23Spec;
impl crate::RegisterSpec for P0_23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p0_23::R`](R) reader structure"]
impl crate::Readable for P0_23Spec {}
#[doc = "`write(|w| ..)` method takes [`p0_23::W`](W) writer structure"]
impl crate::Writable for P0_23Spec {
    type Safety = crate::Unsafe;
}
