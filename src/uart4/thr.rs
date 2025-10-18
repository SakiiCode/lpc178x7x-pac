#[doc = "Register `THR` writer"]
pub type W = crate::W<ThrSpec>;
#[doc = "Field `THR` writer - Writing to the UART4 Transmit Holding Register causes the data to be stored in the UART4 transmit FIFO. The byte will be sent when it reaches the bottom of the FIFO and the transmitter is available."]
pub type ThrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Writing to the UART4 Transmit Holding Register causes the data to be stored in the UART4 transmit FIFO. The byte will be sent when it reaches the bottom of the FIFO and the transmitter is available."]
    #[inline(always)]
    pub fn thr(&mut self) -> ThrW<'_, ThrSpec> {
        ThrW::new(self, 0)
    }
}
#[doc = "Transmit Holding Register. The next character to be transmitted is written here (DLAB =0).\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ThrSpec;
impl crate::RegisterSpec for ThrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`thr::W`](W) writer structure"]
impl crate::Writable for ThrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets THR to value 0"]
impl crate::Resettable for ThrSpec {}
