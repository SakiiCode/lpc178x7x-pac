#[doc = "Register `DMA2` reader"]
pub type R = crate::R<Dma2Spec>;
#[doc = "Register `DMA2` writer"]
pub type W = crate::W<Dma2Spec>;
#[doc = "Field `RX_DMA2_ENABLE` reader - When 1, enables DMA1 for I2S receive."]
pub type RxDma2EnableR = crate::BitReader;
#[doc = "Field `RX_DMA2_ENABLE` writer - When 1, enables DMA1 for I2S receive."]
pub type RxDma2EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DMA2_ENABLE` reader - When 1, enables DMA1 for I2S transmit."]
pub type TxDma2EnableR = crate::BitReader;
#[doc = "Field `TX_DMA2_ENABLE` writer - When 1, enables DMA1 for I2S transmit."]
pub type TxDma2EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DEPTH_DMA2` reader - Set the FIFO level that triggers a receive DMA request on DMA2."]
pub type RxDepthDma2R = crate::FieldReader;
#[doc = "Field `RX_DEPTH_DMA2` writer - Set the FIFO level that triggers a receive DMA request on DMA2."]
pub type RxDepthDma2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TX_DEPTH_DMA2` reader - Set the FIFO level that triggers a transmit DMA request on DMA2."]
pub type TxDepthDma2R = crate::FieldReader;
#[doc = "Field `TX_DEPTH_DMA2` writer - Set the FIFO level that triggers a transmit DMA request on DMA2."]
pub type TxDepthDma2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - When 1, enables DMA1 for I2S receive."]
    #[inline(always)]
    pub fn rx_dma2_enable(&self) -> RxDma2EnableR {
        RxDma2EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, enables DMA1 for I2S transmit."]
    #[inline(always)]
    pub fn tx_dma2_enable(&self) -> TxDma2EnableR {
        TxDma2EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Set the FIFO level that triggers a receive DMA request on DMA2."]
    #[inline(always)]
    pub fn rx_depth_dma2(&self) -> RxDepthDma2R {
        RxDepthDma2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Set the FIFO level that triggers a transmit DMA request on DMA2."]
    #[inline(always)]
    pub fn tx_depth_dma2(&self) -> TxDepthDma2R {
        TxDepthDma2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, enables DMA1 for I2S receive."]
    #[inline(always)]
    pub fn rx_dma2_enable(&mut self) -> RxDma2EnableW<'_, Dma2Spec> {
        RxDma2EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - When 1, enables DMA1 for I2S transmit."]
    #[inline(always)]
    pub fn tx_dma2_enable(&mut self) -> TxDma2EnableW<'_, Dma2Spec> {
        TxDma2EnableW::new(self, 1)
    }
    #[doc = "Bits 8:11 - Set the FIFO level that triggers a receive DMA request on DMA2."]
    #[inline(always)]
    pub fn rx_depth_dma2(&mut self) -> RxDepthDma2W<'_, Dma2Spec> {
        RxDepthDma2W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Set the FIFO level that triggers a transmit DMA request on DMA2."]
    #[inline(always)]
    pub fn tx_depth_dma2(&mut self) -> TxDepthDma2W<'_, Dma2Spec> {
        TxDepthDma2W::new(self, 16)
    }
}
#[doc = "I2S DMA Configuration Register 2. Contains control information for DMA request 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma2Spec;
impl crate::RegisterSpec for Dma2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma2::R`](R) reader structure"]
impl crate::Readable for Dma2Spec {}
#[doc = "`write(|w| ..)` method takes [`dma2::W`](W) writer structure"]
impl crate::Writable for Dma2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA2 to value 0"]
impl crate::Resettable for Dma2Spec {}
