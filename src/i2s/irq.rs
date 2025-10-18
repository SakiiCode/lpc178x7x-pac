#[doc = "Register `IRQ` reader"]
pub type R = crate::R<IrqSpec>;
#[doc = "Register `IRQ` writer"]
pub type W = crate::W<IrqSpec>;
#[doc = "Field `RX_IRQ_ENABLE` reader - When 1, enables I2S receive interrupt."]
pub type RxIrqEnableR = crate::BitReader;
#[doc = "Field `RX_IRQ_ENABLE` writer - When 1, enables I2S receive interrupt."]
pub type RxIrqEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_IRQ_ENABLE` reader - When 1, enables I2S transmit interrupt."]
pub type TxIrqEnableR = crate::BitReader;
#[doc = "Field `TX_IRQ_ENABLE` writer - When 1, enables I2S transmit interrupt."]
pub type TxIrqEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DEPTH_IRQ` reader - Set the FIFO level on which to create an irq request."]
pub type RxDepthIrqR = crate::FieldReader;
#[doc = "Field `RX_DEPTH_IRQ` writer - Set the FIFO level on which to create an irq request."]
pub type RxDepthIrqW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TX_DEPTH_IRQ` reader - Set the FIFO level on which to create an irq request."]
pub type TxDepthIrqR = crate::FieldReader;
#[doc = "Field `TX_DEPTH_IRQ` writer - Set the FIFO level on which to create an irq request."]
pub type TxDepthIrqW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - When 1, enables I2S receive interrupt."]
    #[inline(always)]
    pub fn rx_irq_enable(&self) -> RxIrqEnableR {
        RxIrqEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, enables I2S transmit interrupt."]
    #[inline(always)]
    pub fn tx_irq_enable(&self) -> TxIrqEnableR {
        TxIrqEnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Set the FIFO level on which to create an irq request."]
    #[inline(always)]
    pub fn rx_depth_irq(&self) -> RxDepthIrqR {
        RxDepthIrqR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Set the FIFO level on which to create an irq request."]
    #[inline(always)]
    pub fn tx_depth_irq(&self) -> TxDepthIrqR {
        TxDepthIrqR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, enables I2S receive interrupt."]
    #[inline(always)]
    pub fn rx_irq_enable(&mut self) -> RxIrqEnableW<'_, IrqSpec> {
        RxIrqEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - When 1, enables I2S transmit interrupt."]
    #[inline(always)]
    pub fn tx_irq_enable(&mut self) -> TxIrqEnableW<'_, IrqSpec> {
        TxIrqEnableW::new(self, 1)
    }
    #[doc = "Bits 8:11 - Set the FIFO level on which to create an irq request."]
    #[inline(always)]
    pub fn rx_depth_irq(&mut self) -> RxDepthIrqW<'_, IrqSpec> {
        RxDepthIrqW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Set the FIFO level on which to create an irq request."]
    #[inline(always)]
    pub fn tx_depth_irq(&mut self) -> TxDepthIrqW<'_, IrqSpec> {
        TxDepthIrqW::new(self, 16)
    }
}
#[doc = "I2S Interrupt Request Control Register. Contains bits that control how the I2S interrupt request is generated.\n\nYou can [`read`](crate::Reg::read) this register and get [`irq::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IrqSpec;
impl crate::RegisterSpec for IrqSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`irq::R`](R) reader structure"]
impl crate::Readable for IrqSpec {}
#[doc = "`write(|w| ..)` method takes [`irq::W`](W) writer structure"]
impl crate::Writable for IrqSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IRQ to value 0"]
impl crate::Resettable for IrqSpec {}
