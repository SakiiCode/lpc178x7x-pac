#[doc = "Register `TXMODE` reader"]
pub type R = crate::R<TxmodeSpec>;
#[doc = "Register `TXMODE` writer"]
pub type W = crate::W<TxmodeSpec>;
#[doc = "Clock source selection for the transmit bit clock divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Txclksel {
    #[doc = "0: Select the TX fractional rate divider clock output as the source"]
    TxFrd = 0,
    #[doc = "2: Select the RX_MCLK signal as the TX_MCLK clock source"]
    RxMclk = 2,
}
impl From<Txclksel> for u8 {
    #[inline(always)]
    fn from(variant: Txclksel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Txclksel {
    type Ux = u8;
}
impl crate::IsEnum for Txclksel {}
#[doc = "Field `TXCLKSEL` reader - Clock source selection for the transmit bit clock divider."]
pub type TxclkselR = crate::FieldReader<Txclksel>;
impl TxclkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txclksel {
        match self.bits {
            0 => Txclksel::TxFrd,
            2 => Txclksel::RxMclk,
            _ => unreachable!(),
        }
    }
    #[doc = "Select the TX fractional rate divider clock output as the source"]
    #[inline(always)]
    pub fn is_tx_frd(&self) -> bool {
        *self == Txclksel::TxFrd
    }
    #[doc = "Select the RX_MCLK signal as the TX_MCLK clock source"]
    #[inline(always)]
    pub fn is_rx_mclk(&self) -> bool {
        *self == Txclksel::RxMclk
    }
}
#[doc = "Field `TXCLKSEL` writer - Clock source selection for the transmit bit clock divider."]
pub type TxclkselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Txclksel>;
impl<'a, REG> TxclkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Select the TX fractional rate divider clock output as the source"]
    #[inline(always)]
    pub fn tx_frd(self) -> &'a mut crate::W<REG> {
        self.variant(Txclksel::TxFrd)
    }
    #[doc = "Select the RX_MCLK signal as the TX_MCLK clock source"]
    #[inline(always)]
    pub fn rx_mclk(self) -> &'a mut crate::W<REG> {
        self.variant(Txclksel::RxMclk)
    }
}
#[doc = "Field `TX4PIN` reader - Transmit 4-pin mode selection. When 1, enables 4-pin mode."]
pub type Tx4pinR = crate::BitReader;
#[doc = "Field `TX4PIN` writer - Transmit 4-pin mode selection. When 1, enables 4-pin mode."]
pub type Tx4pinW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXMCENA` reader - Enable for the TX_MCLK output. When 0, output of TX_MCLK is not enabled. When 1, output of TX_MCLK is enabled."]
pub type TxmcenaR = crate::BitReader;
#[doc = "Field `TXMCENA` writer - Enable for the TX_MCLK output. When 0, output of TX_MCLK is not enabled. When 1, output of TX_MCLK is enabled."]
pub type TxmcenaW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Clock source selection for the transmit bit clock divider."]
    #[inline(always)]
    pub fn txclksel(&self) -> TxclkselR {
        TxclkselR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Transmit 4-pin mode selection. When 1, enables 4-pin mode."]
    #[inline(always)]
    pub fn tx4pin(&self) -> Tx4pinR {
        Tx4pinR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable for the TX_MCLK output. When 0, output of TX_MCLK is not enabled. When 1, output of TX_MCLK is enabled."]
    #[inline(always)]
    pub fn txmcena(&self) -> TxmcenaR {
        TxmcenaR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Clock source selection for the transmit bit clock divider."]
    #[inline(always)]
    pub fn txclksel(&mut self) -> TxclkselW<'_, TxmodeSpec> {
        TxclkselW::new(self, 0)
    }
    #[doc = "Bit 2 - Transmit 4-pin mode selection. When 1, enables 4-pin mode."]
    #[inline(always)]
    pub fn tx4pin(&mut self) -> Tx4pinW<'_, TxmodeSpec> {
        Tx4pinW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable for the TX_MCLK output. When 0, output of TX_MCLK is not enabled. When 1, output of TX_MCLK is enabled."]
    #[inline(always)]
    pub fn txmcena(&mut self) -> TxmcenaW<'_, TxmodeSpec> {
        TxmcenaW::new(self, 3)
    }
}
#[doc = "I2S Transmit mode control.\n\nYou can [`read`](crate::Reg::read) this register and get [`txmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxmodeSpec;
impl crate::RegisterSpec for TxmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txmode::R`](R) reader structure"]
impl crate::Readable for TxmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`txmode::W`](W) writer structure"]
impl crate::Writable for TxmodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXMODE to value 0"]
impl crate::Resettable for TxmodeSpec {}
