#[doc = "Register `RXBITRATE` reader"]
pub type R = crate::R<RxbitrateSpec>;
#[doc = "Register `RXBITRATE` writer"]
pub type W = crate::W<RxbitrateSpec>;
#[doc = "Field `RX_BITRATE` reader - I2S receive bit rate. This value plus one is used to divide RX_MCLK to produce the receive bit clock."]
pub type RxBitrateR = crate::FieldReader;
#[doc = "Field `RX_BITRATE` writer - I2S receive bit rate. This value plus one is used to divide RX_MCLK to produce the receive bit clock."]
pub type RxBitrateW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - I2S receive bit rate. This value plus one is used to divide RX_MCLK to produce the receive bit clock."]
    #[inline(always)]
    pub fn rx_bitrate(&self) -> RxBitrateR {
        RxBitrateR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - I2S receive bit rate. This value plus one is used to divide RX_MCLK to produce the receive bit clock."]
    #[inline(always)]
    pub fn rx_bitrate(&mut self) -> RxBitrateW<'_, RxbitrateSpec> {
        RxBitrateW::new(self, 0)
    }
}
#[doc = "I2S Receive bit rate divider. This register determines the I2S receive bit rate by specifying the value to divide RX_MCLK by in order to produce the receive bit clock.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxbitrate::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxbitrate::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxbitrateSpec;
impl crate::RegisterSpec for RxbitrateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxbitrate::R`](R) reader structure"]
impl crate::Readable for RxbitrateSpec {}
#[doc = "`write(|w| ..)` method takes [`rxbitrate::W`](W) writer structure"]
impl crate::Writable for RxbitrateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXBITRATE to value 0"]
impl crate::Resettable for RxbitrateSpec {}
