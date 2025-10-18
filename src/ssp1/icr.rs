#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `RORIC` writer - Writing a 1 to this bit clears the frame was received when RxFIFO was full interrupt."]
pub type RoricW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTIC` writer - Writing a 1 to this bit clears the Rx FIFO was not empty and has not been read for a time-out period interrupt. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR / \\[SCR+1\\])."]
pub type RticW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Writing a 1 to this bit clears the frame was received when RxFIFO was full interrupt."]
    #[inline(always)]
    pub fn roric(&mut self) -> RoricW<'_, IcrSpec> {
        RoricW::new(self, 0)
    }
    #[doc = "Bit 1 - Writing a 1 to this bit clears the Rx FIFO was not empty and has not been read for a time-out period interrupt. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR / \\[SCR+1\\])."]
    #[inline(always)]
    pub fn rtic(&mut self) -> RticW<'_, IcrSpec> {
        RticW::new(self, 1)
    }
}
#[doc = "SSPICR Interrupt Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}
