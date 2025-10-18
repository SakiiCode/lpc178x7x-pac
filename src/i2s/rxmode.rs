#[doc = "Register `RXMODE` reader"]
pub type R = crate::R<RxmodeSpec>;
#[doc = "Register `RXMODE` writer"]
pub type W = crate::W<RxmodeSpec>;
#[doc = "Clock source selection for the receive bit clock divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Enum {
    #[doc = "0: Select the RX fractional rate divider clock output as the source"]
    SelectTheRxFracti = 0,
    #[doc = "2: Select the TX_MCLK signal as the RX_MCLK clock source"]
    SelectTheTxMclkS = 2,
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
#[doc = "Field `RXCLKSEL` reader - Clock source selection for the receive bit clock divider."]
pub type RxclkselR = crate::FieldReader<Enum>;
impl RxclkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            0 => Enum::SelectTheRxFracti,
            2 => Enum::SelectTheTxMclkS,
            _ => unreachable!(),
        }
    }
    #[doc = "Select the RX fractional rate divider clock output as the source"]
    #[inline(always)]
    pub fn is_select_the_rx_fracti(&self) -> bool {
        *self == Enum::SelectTheRxFracti
    }
    #[doc = "Select the TX_MCLK signal as the RX_MCLK clock source"]
    #[inline(always)]
    pub fn is_select_the_tx_mclk_s(&self) -> bool {
        *self == Enum::SelectTheTxMclkS
    }
}
#[doc = "Field `RXCLKSEL` writer - Clock source selection for the receive bit clock divider."]
pub type RxclkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Enum>;
impl<'a, REG> RxclkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select the RX fractional rate divider clock output as the source"]
    #[inline(always)]
    pub fn select_the_rx_fracti(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::SelectTheRxFracti)
    }
    #[doc = "Select the TX_MCLK signal as the RX_MCLK clock source"]
    #[inline(always)]
    pub fn select_the_tx_mclk_s(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::SelectTheTxMclkS)
    }
}
#[doc = "Field `RX4PIN` reader - Receive 4-pin mode selection. When 1, enables 4-pin mode."]
pub type Rx4pinR = crate::BitReader;
#[doc = "Field `RX4PIN` writer - Receive 4-pin mode selection. When 1, enables 4-pin mode."]
pub type Rx4pinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXMCENA` reader - Enable for the RX_MCLK output. When 0, output of RX_MCLK is not enabled. When 1, output of RX_MCLK is enabled."]
pub type RxmcenaR = crate::BitReader;
#[doc = "Field `RXMCENA` writer - Enable for the RX_MCLK output. When 0, output of RX_MCLK is not enabled. When 1, output of RX_MCLK is enabled."]
pub type RxmcenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Clock source selection for the receive bit clock divider."]
    #[inline(always)]
    pub fn rxclksel(&self) -> RxclkselR {
        RxclkselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Receive 4-pin mode selection. When 1, enables 4-pin mode."]
    #[inline(always)]
    pub fn rx4pin(&self) -> Rx4pinR {
        Rx4pinR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable for the RX_MCLK output. When 0, output of RX_MCLK is not enabled. When 1, output of RX_MCLK is enabled."]
    #[inline(always)]
    pub fn rxmcena(&self) -> RxmcenaR {
        RxmcenaR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source selection for the receive bit clock divider."]
    #[inline(always)]
    pub fn rxclksel(&mut self) -> RxclkselW<'_, RxmodeSpec> {
        RxclkselW::new(self, 0)
    }
    #[doc = "Bit 2 - Receive 4-pin mode selection. When 1, enables 4-pin mode."]
    #[inline(always)]
    pub fn rx4pin(&mut self) -> Rx4pinW<'_, RxmodeSpec> {
        Rx4pinW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable for the RX_MCLK output. When 0, output of RX_MCLK is not enabled. When 1, output of RX_MCLK is enabled."]
    #[inline(always)]
    pub fn rxmcena(&mut self) -> RxmcenaW<'_, RxmodeSpec> {
        RxmcenaW::new(self, 3)
    }
}
#[doc = "I2S Receive mode control.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxmodeSpec;
impl crate::RegisterSpec for RxmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxmode::R`](R) reader structure"]
impl crate::Readable for RxmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`rxmode::W`](W) writer structure"]
impl crate::Writable for RxmodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXMODE to value 0"]
impl crate::Resettable for RxmodeSpec {}
