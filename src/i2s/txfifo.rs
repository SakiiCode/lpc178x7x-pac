#[doc = "Register `TXFIFO` writer"]
pub type W = crate::W<TxfifoSpec>;
#[doc = "Field `I2STXFIFO` writer - 8 x 32-bit transmit FIFO."]
pub type I2stxfifoW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - 8 x 32-bit transmit FIFO."]
    #[inline(always)]
    pub fn i2stxfifo(&mut self) -> I2stxfifoW<'_, TxfifoSpec> {
        I2stxfifoW::new(self, 0)
    }
}
#[doc = "I2S Transmit FIFO. Access register for the 8 x 32-bit transmitter FIFO.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txfifo::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxfifoSpec;
impl crate::RegisterSpec for TxfifoSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`txfifo::W`](W) writer structure"]
impl crate::Writable for TxfifoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXFIFO to value 0"]
impl crate::Resettable for TxfifoSpec {}
