#[doc = "Register `STATE` reader"]
pub type R = crate::R<StateSpec>;
#[doc = "Field `IRQ` reader - This bit reflects the presence of Receive Interrupt or Transmit Interrupt. This is determined by comparing the current FIFO levels to the rx_depth_irq and tx_depth_irq fields in the IRQ register."]
pub type IrqR = crate::BitReader;
#[doc = "Field `DMAREQ1` reader - This bit reflects the presence of Receive or Transmit DMA Request 1. This is determined by comparing the current FIFO levels to the rx_depth_dma1 and tx_depth_dma1 fields in the DMA1 register."]
pub type Dmareq1R = crate::BitReader;
#[doc = "Field `DMAREQ2` reader - This bit reflects the presence of Receive or Transmit DMA Request 2. This is determined by comparing the current FIFO levels to the rx_depth_dma2 and tx_depth_dma2 fields in the DMA2 register."]
pub type Dmareq2R = crate::BitReader;
#[doc = "Field `RX_LEVEL` reader - Reflects the current level of the Receive FIFO."]
pub type RxLevelR = crate::FieldReader;
#[doc = "Field `TX_LEVEL` reader - Reflects the current level of the Transmit FIFO."]
pub type TxLevelR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - This bit reflects the presence of Receive Interrupt or Transmit Interrupt. This is determined by comparing the current FIFO levels to the rx_depth_irq and tx_depth_irq fields in the IRQ register."]
    #[inline(always)]
    pub fn irq(&self) -> IrqR {
        IrqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit reflects the presence of Receive or Transmit DMA Request 1. This is determined by comparing the current FIFO levels to the rx_depth_dma1 and tx_depth_dma1 fields in the DMA1 register."]
    #[inline(always)]
    pub fn dmareq1(&self) -> Dmareq1R {
        Dmareq1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit reflects the presence of Receive or Transmit DMA Request 2. This is determined by comparing the current FIFO levels to the rx_depth_dma2 and tx_depth_dma2 fields in the DMA2 register."]
    #[inline(always)]
    pub fn dmareq2(&self) -> Dmareq2R {
        Dmareq2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Reflects the current level of the Receive FIFO."]
    #[inline(always)]
    pub fn rx_level(&self) -> RxLevelR {
        RxLevelR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Reflects the current level of the Transmit FIFO."]
    #[inline(always)]
    pub fn tx_level(&self) -> TxLevelR {
        TxLevelR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
#[doc = "I2S Status Feedback Register. Contains status information about the I2S interface.\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StateSpec;
impl crate::RegisterSpec for StateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`state::R`](R) reader structure"]
impl crate::Readable for StateSpec {}
#[doc = "`reset()` method sets STATE to value 0x07"]
impl crate::Resettable for StateSpec {
    const RESET_VALUE: u32 = 0x07;
}
