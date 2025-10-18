#[doc = "Register `P0_25` reader"]
pub type R = crate::R<P0_25Spec>;
#[doc = "Register `P0_25` writer"]
pub type W = crate::W<P0_25Spec>;
#[doc = "Selects pin function for pin P0\\[25\\]"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Enum {
    #[doc = "0: General purpose digital input/output pin."]
    P0_25 = 0,
    #[doc = "1: A/D converter 0, input 2. When configured as an ADC input, the digital function of the pin must be disabled."]
    Adc0In2 = 1,
    #[doc = "2: Receive data. It is driven by the transmitter and read by the receiver. Corresponds to the signal SD in the I2S-bus specification."]
    I2sRxSda = 2,
    #[doc = "3: Transmitter output for UART3."]
    U3Txd = 3,
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
#[doc = "Field `FUNC` reader - Selects pin function for pin P0\\[25\\]"]
pub type FuncR = crate::FieldReader<Enum>;
impl FuncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Enum> {
        match self.bits {
            0 => Some(Enum::P0_25),
            1 => Some(Enum::Adc0In2),
            2 => Some(Enum::I2sRxSda),
            3 => Some(Enum::U3Txd),
            _ => None,
        }
    }
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn is_p0_25(&self) -> bool {
        *self == Enum::P0_25
    }
    #[doc = "A/D converter 0, input 2. When configured as an ADC input, the digital function of the pin must be disabled."]
    #[inline(always)]
    pub fn is_adc0_in_2(&self) -> bool {
        *self == Enum::Adc0In2
    }
    #[doc = "Receive data. It is driven by the transmitter and read by the receiver. Corresponds to the signal SD in the I2S-bus specification."]
    #[inline(always)]
    pub fn is_i2s_rx_sda(&self) -> bool {
        *self == Enum::I2sRxSda
    }
    #[doc = "Transmitter output for UART3."]
    #[inline(always)]
    pub fn is_u3_txd(&self) -> bool {
        *self == Enum::U3Txd
    }
}
#[doc = "Field `FUNC` writer - Selects pin function for pin P0\\[25\\]"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 3, Enum>;
impl<'a, REG> FuncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn p0_25(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::P0_25)
    }
    #[doc = "A/D converter 0, input 2. When configured as an ADC input, the digital function of the pin must be disabled."]
    #[inline(always)]
    pub fn adc0_in_2(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Adc0In2)
    }
    #[doc = "Receive data. It is driven by the transmitter and read by the receiver. Corresponds to the signal SD in the I2S-bus specification."]
    #[inline(always)]
    pub fn i2s_rx_sda(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::I2sRxSda)
    }
    #[doc = "Transmitter output for UART3."]
    #[inline(always)]
    pub fn u3_txd(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::U3Txd)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[25\\]"]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[25\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FuncW<'_, P0_25Spec> {
        FuncW::new(self, 0)
    }
}
#[doc = "I/O configuration register for pin P0\\[25\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_25::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P0_25Spec;
impl crate::RegisterSpec for P0_25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p0_25::R`](R) reader structure"]
impl crate::Readable for P0_25Spec {}
#[doc = "`write(|w| ..)` method takes [`p0_25::W`](W) writer structure"]
impl crate::Writable for P0_25Spec {
    type Safety = crate::Unsafe;
}
