#[doc = "Register `DMA1` reader"]
pub type R = crate::R<Dma1Spec>;
#[doc = "Register `DMA1` writer"]
pub type W = crate::W<Dma1Spec>;
#[doc = "Field `RX_DMA1_ENABLE` reader - When 1, enables DMA1 for I2S receive."]
pub type RxDma1EnableR = crate::BitReader;
#[doc = "Field `RX_DMA1_ENABLE` writer - When 1, enables DMA1 for I2S receive."]
pub type RxDma1EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_DMA1_ENABLE` reader - When 1, enables DMA1 for I2S transmit."]
pub type TxDma1EnableR = crate::BitReader;
#[doc = "Field `TX_DMA1_ENABLE` writer - When 1, enables DMA1 for I2S transmit."]
pub type TxDma1EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DEPTH_DMA1` reader - Set the FIFO level that triggers a receive DMA request on DMA1."]
pub type RxDepthDma1R = crate::FieldReader;
#[doc = "Field `RX_DEPTH_DMA1` writer - Set the FIFO level that triggers a receive DMA request on DMA1."]
pub type RxDepthDma1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TX_DEPTH_DMA1` reader - Set the FIFO level that triggers a transmit DMA request on DMA1."]
pub type TxDepthDma1R = crate::FieldReader;
#[doc = "Field `TX_DEPTH_DMA1` writer - Set the FIFO level that triggers a transmit DMA request on DMA1."]
pub type TxDepthDma1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - When 1, enables DMA1 for I2S receive."]
    #[inline(always)]
    pub fn rx_dma1_enable(&self) -> RxDma1EnableR {
        RxDma1EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, enables DMA1 for I2S transmit."]
    #[inline(always)]
    pub fn tx_dma1_enable(&self) -> TxDma1EnableR {
        TxDma1EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Set the FIFO level that triggers a receive DMA request on DMA1."]
    #[inline(always)]
    pub fn rx_depth_dma1(&self) -> RxDepthDma1R {
        RxDepthDma1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Set the FIFO level that triggers a transmit DMA request on DMA1."]
    #[inline(always)]
    pub fn tx_depth_dma1(&self) -> TxDepthDma1R {
        TxDepthDma1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, enables DMA1 for I2S receive."]
    #[inline(always)]
    pub fn rx_dma1_enable(&mut self) -> RxDma1EnableW<'_, Dma1Spec> {
        RxDma1EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - When 1, enables DMA1 for I2S transmit."]
    #[inline(always)]
    pub fn tx_dma1_enable(&mut self) -> TxDma1EnableW<'_, Dma1Spec> {
        TxDma1EnableW::new(self, 1)
    }
    #[doc = "Bits 8:11 - Set the FIFO level that triggers a receive DMA request on DMA1."]
    #[inline(always)]
    pub fn rx_depth_dma1(&mut self) -> RxDepthDma1W<'_, Dma1Spec> {
        RxDepthDma1W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Set the FIFO level that triggers a transmit DMA request on DMA1."]
    #[inline(always)]
    pub fn tx_depth_dma1(&mut self) -> TxDepthDma1W<'_, Dma1Spec> {
        TxDepthDma1W::new(self, 16)
    }
}
#[doc = "I2S DMA Configuration Register 1. Contains control information for DMA request 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dma1Spec;
impl crate::RegisterSpec for Dma1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma1::R`](R) reader structure"]
impl crate::Readable for Dma1Spec {}
#[doc = "`write(|w| ..)` method takes [`dma1::W`](W) writer structure"]
impl crate::Writable for Dma1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA1 to value 0"]
impl crate::Resettable for Dma1Spec {}
