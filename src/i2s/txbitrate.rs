#[doc = "Register `TXBITRATE` reader"]
pub type R = crate::R<TxbitrateSpec>;
#[doc = "Register `TXBITRATE` writer"]
pub type W = crate::W<TxbitrateSpec>;
#[doc = "Field `TX_BITRATE` reader - I2S transmit bit rate. This value plus one is used to divide TX_MCLK to produce the transmit bit clock."]
pub type TxBitrateR = crate::FieldReader;
#[doc = "Field `TX_BITRATE` writer - I2S transmit bit rate. This value plus one is used to divide TX_MCLK to produce the transmit bit clock."]
pub type TxBitrateW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - I2S transmit bit rate. This value plus one is used to divide TX_MCLK to produce the transmit bit clock."]
    #[inline(always)]
    pub fn tx_bitrate(&self) -> TxBitrateR {
        TxBitrateR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I2S transmit bit rate. This value plus one is used to divide TX_MCLK to produce the transmit bit clock."]
    #[inline(always)]
    pub fn tx_bitrate(&mut self) -> TxBitrateW<'_, TxbitrateSpec> {
        TxBitrateW::new(self, 0)
    }
}
#[doc = "I2S Transmit bit rate divider. This register determines the I2S transmit bit rate by specifying the value to divide TX_MCLK by in order to produce the transmit bit clock.\n\nYou can [`read`](crate::Reg::read) this register and get [`txbitrate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbitrate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxbitrateSpec;
impl crate::RegisterSpec for TxbitrateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txbitrate::R`](R) reader structure"]
impl crate::Readable for TxbitrateSpec {}
#[doc = "`write(|w| ..)` method takes [`txbitrate::W`](W) writer structure"]
impl crate::Writable for TxbitrateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXBITRATE to value 0"]
impl crate::Resettable for TxbitrateSpec {}
