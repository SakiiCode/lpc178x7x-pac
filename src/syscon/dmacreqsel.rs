#[doc = "Register `DMACREQSEL` reader"]
pub type R = crate::R<DmacreqselSpec>;
#[doc = "Register `DMACREQSEL` writer"]
pub type W = crate::W<DmacreqselSpec>;
#[doc = "Field `DMASEL00` reader - Selects the DMA request for GPDMA input 0: 0 - (unused) 1 - Timer 0 match 0 is selected."]
pub type Dmasel00R = crate::BitReader;
#[doc = "Field `DMASEL00` writer - Selects the DMA request for GPDMA input 0: 0 - (unused) 1 - Timer 0 match 0 is selected."]
pub type Dmasel00W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMASEL01` reader - Selects the DMA request for GPDMA input 1: 0 - SD card interface is selected. 1 - Timer 0 match 1 is selected."]
pub type Dmasel01R = crate::BitReader;
#[doc = "Field `DMASEL01` writer - Selects the DMA request for GPDMA input 1: 0 - SD card interface is selected. 1 - Timer 0 match 1 is selected."]
pub type Dmasel01W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMASEL02` reader - Selects the DMA request for GPDMA input 2: 0 - SSP0 transmit is selected. 1 - Timer 1 match 0 is selected."]
pub type Dmasel02R = crate::BitReader;
#[doc = "Field `DMASEL02` writer - Selects the DMA request for GPDMA input 2: 0 - SSP0 transmit is selected. 1 - Timer 1 match 0 is selected."]
pub type Dmasel02W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMASEL03` reader - Selects the DMA request for GPDMA input 3: 0 - SSP0 receive is selected. 1 - Timer 1 match 1 is selected."]
pub type Dmasel03R = crate::BitReader;
#[doc = "Field `DMASEL03` writer - Selects the DMA request for GPDMA input 3: 0 - SSP0 receive is selected. 1 - Timer 1 match 1 is selected."]
pub type Dmasel03W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMASEL04` reader - Selects the DMA request for GPDMA input 4: 0 - SSP1 transmit is selected. 1 - Timer 2 match 0 is selected."]
pub type Dmasel04R = crate::BitReader;
#[doc = "Field `DMASEL04` writer - Selects the DMA request for GPDMA input 4: 0 - SSP1 transmit is selected. 1 - Timer 2 match 0 is selected."]
pub type Dmasel04W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMASEL05` reader - Selects the DMA request for GPDMA input 5: 0 - SSP1 receive is selected. 1 - Timer 2 match 1 is selected."]
pub type Dmasel05R = crate::BitReader;
#[doc = "Field `DMASEL05` writer - Selects the DMA request for GPDMA input 5: 0 - SSP1 receive is selected. 1 - Timer 2 match 1 is selected."]
pub type Dmasel05W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMASEL06` reader - Selects the DMA request for GPDMA input 6: 0 - SSP2 transmit is selected. 1 - I2S channel 0 is selected."]
pub type Dmasel06R = crate::BitReader;
#[doc = "Field `DMASEL06` writer - Selects the DMA request for GPDMA input 6: 0 - SSP2 transmit is selected. 1 - I2S channel 0 is selected."]
pub type Dmasel06W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMASEL07` reader - Selects the DMA request for GPDMA input 7: 0 - SSP2 receive is selected. 1 - I2S channel 1 is selected."]
pub type Dmasel07R = crate::BitReader;
#[doc = "Field `DMASEL07` writer - Selects the DMA request for GPDMA input 7: 0 - SSP2 receive is selected. 1 - I2S channel 1 is selected."]
pub type Dmasel07W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMASEL10` reader - Selects the DMA request for GPDMA input 10: 0 - UART0 transmit is selected. 1 - UART3 transmit is selected."]
pub type Dmasel10R = crate::BitReader;
#[doc = "Field `DMASEL10` writer - Selects the DMA request for GPDMA input 10: 0 - UART0 transmit is selected. 1 - UART3 transmit is selected."]
pub type Dmasel10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMASEL11` reader - Selects the DMA request for GPDMA input 11: 0 - UART0 receive is selected. 1 - UART3 receive is selected."]
pub type Dmasel11R = crate::BitReader;
#[doc = "Field `DMASEL11` writer - Selects the DMA request for GPDMA input 11: 0 - UART0 receive is selected. 1 - UART3 receive is selected."]
pub type Dmasel11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMASEL12` reader - Selects the DMA request for GPDMA input 12: 0 - UART1 transmit is selected. 1 - UART4 transmit is selected."]
pub type Dmasel12R = crate::BitReader;
#[doc = "Field `DMASEL12` writer - Selects the DMA request for GPDMA input 12: 0 - UART1 transmit is selected. 1 - UART4 transmit is selected."]
pub type Dmasel12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMASEL13` reader - Selects the DMA request for GPDMA input 13: 0 - UART1 receive is selected. 1 - UART4 receive is selected."]
pub type Dmasel13R = crate::BitReader;
#[doc = "Field `DMASEL13` writer - Selects the DMA request for GPDMA input 13: 0 - UART1 receive is selected. 1 - UART4 receive is selected."]
pub type Dmasel13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMASEL14` reader - Selects the DMA request for GPDMA input 14: 0 - UART2 transmit is selected. 1 - Timer 3 match 0 is selected."]
pub type Dmasel14R = crate::BitReader;
#[doc = "Field `DMASEL14` writer - Selects the DMA request for GPDMA input 14: 0 - UART2 transmit is selected. 1 - Timer 3 match 0 is selected."]
pub type Dmasel14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMASEL15` reader - Selects the DMA request for GPDMA input 15: 0 - UART2 receive is selected. 1 - Timer 3 match 1 is selected."]
pub type Dmasel15R = crate::BitReader;
#[doc = "Field `DMASEL15` writer - Selects the DMA request for GPDMA input 15: 0 - UART2 receive is selected. 1 - Timer 3 match 1 is selected."]
pub type Dmasel15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Selects the DMA request for GPDMA input 0: 0 - (unused) 1 - Timer 0 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel00(&self) -> Dmasel00R {
        Dmasel00R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Selects the DMA request for GPDMA input 1: 0 - SD card interface is selected. 1 - Timer 0 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel01(&self) -> Dmasel01R {
        Dmasel01R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects the DMA request for GPDMA input 2: 0 - SSP0 transmit is selected. 1 - Timer 1 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel02(&self) -> Dmasel02R {
        Dmasel02R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Selects the DMA request for GPDMA input 3: 0 - SSP0 receive is selected. 1 - Timer 1 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel03(&self) -> Dmasel03R {
        Dmasel03R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Selects the DMA request for GPDMA input 4: 0 - SSP1 transmit is selected. 1 - Timer 2 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel04(&self) -> Dmasel04R {
        Dmasel04R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Selects the DMA request for GPDMA input 5: 0 - SSP1 receive is selected. 1 - Timer 2 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel05(&self) -> Dmasel05R {
        Dmasel05R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Selects the DMA request for GPDMA input 6: 0 - SSP2 transmit is selected. 1 - I2S channel 0 is selected."]
    #[inline(always)]
    pub fn dmasel06(&self) -> Dmasel06R {
        Dmasel06R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Selects the DMA request for GPDMA input 7: 0 - SSP2 receive is selected. 1 - I2S channel 1 is selected."]
    #[inline(always)]
    pub fn dmasel07(&self) -> Dmasel07R {
        Dmasel07R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects the DMA request for GPDMA input 10: 0 - UART0 transmit is selected. 1 - UART3 transmit is selected."]
    #[inline(always)]
    pub fn dmasel10(&self) -> Dmasel10R {
        Dmasel10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Selects the DMA request for GPDMA input 11: 0 - UART0 receive is selected. 1 - UART3 receive is selected."]
    #[inline(always)]
    pub fn dmasel11(&self) -> Dmasel11R {
        Dmasel11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Selects the DMA request for GPDMA input 12: 0 - UART1 transmit is selected. 1 - UART4 transmit is selected."]
    #[inline(always)]
    pub fn dmasel12(&self) -> Dmasel12R {
        Dmasel12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Selects the DMA request for GPDMA input 13: 0 - UART1 receive is selected. 1 - UART4 receive is selected."]
    #[inline(always)]
    pub fn dmasel13(&self) -> Dmasel13R {
        Dmasel13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Selects the DMA request for GPDMA input 14: 0 - UART2 transmit is selected. 1 - Timer 3 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel14(&self) -> Dmasel14R {
        Dmasel14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Selects the DMA request for GPDMA input 15: 0 - UART2 receive is selected. 1 - Timer 3 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel15(&self) -> Dmasel15R {
        Dmasel15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects the DMA request for GPDMA input 0: 0 - (unused) 1 - Timer 0 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel00(&mut self) -> Dmasel00W<'_, DmacreqselSpec> {
        Dmasel00W::new(self, 0)
    }
    #[doc = "Bit 1 - Selects the DMA request for GPDMA input 1: 0 - SD card interface is selected. 1 - Timer 0 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel01(&mut self) -> Dmasel01W<'_, DmacreqselSpec> {
        Dmasel01W::new(self, 1)
    }
    #[doc = "Bit 2 - Selects the DMA request for GPDMA input 2: 0 - SSP0 transmit is selected. 1 - Timer 1 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel02(&mut self) -> Dmasel02W<'_, DmacreqselSpec> {
        Dmasel02W::new(self, 2)
    }
    #[doc = "Bit 3 - Selects the DMA request for GPDMA input 3: 0 - SSP0 receive is selected. 1 - Timer 1 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel03(&mut self) -> Dmasel03W<'_, DmacreqselSpec> {
        Dmasel03W::new(self, 3)
    }
    #[doc = "Bit 4 - Selects the DMA request for GPDMA input 4: 0 - SSP1 transmit is selected. 1 - Timer 2 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel04(&mut self) -> Dmasel04W<'_, DmacreqselSpec> {
        Dmasel04W::new(self, 4)
    }
    #[doc = "Bit 5 - Selects the DMA request for GPDMA input 5: 0 - SSP1 receive is selected. 1 - Timer 2 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel05(&mut self) -> Dmasel05W<'_, DmacreqselSpec> {
        Dmasel05W::new(self, 5)
    }
    #[doc = "Bit 6 - Selects the DMA request for GPDMA input 6: 0 - SSP2 transmit is selected. 1 - I2S channel 0 is selected."]
    #[inline(always)]
    pub fn dmasel06(&mut self) -> Dmasel06W<'_, DmacreqselSpec> {
        Dmasel06W::new(self, 6)
    }
    #[doc = "Bit 7 - Selects the DMA request for GPDMA input 7: 0 - SSP2 receive is selected. 1 - I2S channel 1 is selected."]
    #[inline(always)]
    pub fn dmasel07(&mut self) -> Dmasel07W<'_, DmacreqselSpec> {
        Dmasel07W::new(self, 7)
    }
    #[doc = "Bit 10 - Selects the DMA request for GPDMA input 10: 0 - UART0 transmit is selected. 1 - UART3 transmit is selected."]
    #[inline(always)]
    pub fn dmasel10(&mut self) -> Dmasel10W<'_, DmacreqselSpec> {
        Dmasel10W::new(self, 10)
    }
    #[doc = "Bit 11 - Selects the DMA request for GPDMA input 11: 0 - UART0 receive is selected. 1 - UART3 receive is selected."]
    #[inline(always)]
    pub fn dmasel11(&mut self) -> Dmasel11W<'_, DmacreqselSpec> {
        Dmasel11W::new(self, 11)
    }
    #[doc = "Bit 12 - Selects the DMA request for GPDMA input 12: 0 - UART1 transmit is selected. 1 - UART4 transmit is selected."]
    #[inline(always)]
    pub fn dmasel12(&mut self) -> Dmasel12W<'_, DmacreqselSpec> {
        Dmasel12W::new(self, 12)
    }
    #[doc = "Bit 13 - Selects the DMA request for GPDMA input 13: 0 - UART1 receive is selected. 1 - UART4 receive is selected."]
    #[inline(always)]
    pub fn dmasel13(&mut self) -> Dmasel13W<'_, DmacreqselSpec> {
        Dmasel13W::new(self, 13)
    }
    #[doc = "Bit 14 - Selects the DMA request for GPDMA input 14: 0 - UART2 transmit is selected. 1 - Timer 3 match 0 is selected."]
    #[inline(always)]
    pub fn dmasel14(&mut self) -> Dmasel14W<'_, DmacreqselSpec> {
        Dmasel14W::new(self, 14)
    }
    #[doc = "Bit 15 - Selects the DMA request for GPDMA input 15: 0 - UART2 receive is selected. 1 - Timer 3 match 1 is selected."]
    #[inline(always)]
    pub fn dmasel15(&mut self) -> Dmasel15W<'_, DmacreqselSpec> {
        Dmasel15W::new(self, 15)
    }
}
#[doc = "Selects between alternative requests on DMA channels 0 through 7 and 10 through 15\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacreqsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacreqsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacreqselSpec;
impl crate::RegisterSpec for DmacreqselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacreqsel::R`](R) reader structure"]
impl crate::Readable for DmacreqselSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacreqsel::W`](W) writer structure"]
impl crate::Writable for DmacreqselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACREQSEL to value 0"]
impl crate::Resettable for DmacreqselSpec {}
