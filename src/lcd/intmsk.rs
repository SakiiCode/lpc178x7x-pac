#[doc = "Register `INTMSK` reader"]
pub type R = crate::R<IntmskSpec>;
#[doc = "Register `INTMSK` writer"]
pub type W = crate::W<IntmskSpec>;
#[doc = "Field `FUFIM` reader - FIFO underflow interrupt enable. 0: The FIFO underflow interrupt is disabled. 1: Interrupt will be generated when the FIFO underflows."]
pub type FufimR = crate::BitReader;
#[doc = "Field `FUFIM` writer - FIFO underflow interrupt enable. 0: The FIFO underflow interrupt is disabled. 1: Interrupt will be generated when the FIFO underflows."]
pub type FufimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LNBUIM` reader - LCD next base address update interrupt enable. 0: The base address update interrupt is disabled. 1: Interrupt will be generated when the LCD base address registers have been updated from the next address registers."]
pub type LnbuimR = crate::BitReader;
#[doc = "Field `LNBUIM` writer - LCD next base address update interrupt enable. 0: The base address update interrupt is disabled. 1: Interrupt will be generated when the LCD base address registers have been updated from the next address registers."]
pub type LnbuimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VCOMPIM` reader - Vertical compare interrupt enable. 0: The vertical compare time interrupt is disabled. 1: Interrupt will be generated when the vertical compare time (as defined by LcdVComp field in the LCD_CTRL register) is reached."]
pub type VcompimR = crate::BitReader;
#[doc = "Field `VCOMPIM` writer - Vertical compare interrupt enable. 0: The vertical compare time interrupt is disabled. 1: Interrupt will be generated when the vertical compare time (as defined by LcdVComp field in the LCD_CTRL register) is reached."]
pub type VcompimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BERIM` reader - AHB master error interrupt enable. 0: The AHB Master error interrupt is disabled. 1: Interrupt will be generated when an AHB Master error occurs."]
pub type BerimR = crate::BitReader;
#[doc = "Field `BERIM` writer - AHB master error interrupt enable. 0: The AHB Master error interrupt is disabled. 1: Interrupt will be generated when an AHB Master error occurs."]
pub type BerimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - FIFO underflow interrupt enable. 0: The FIFO underflow interrupt is disabled. 1: Interrupt will be generated when the FIFO underflows."]
    #[inline(always)]
    pub fn fufim(&self) -> FufimR {
        FufimR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD next base address update interrupt enable. 0: The base address update interrupt is disabled. 1: Interrupt will be generated when the LCD base address registers have been updated from the next address registers."]
    #[inline(always)]
    pub fn lnbuim(&self) -> LnbuimR {
        LnbuimR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Vertical compare interrupt enable. 0: The vertical compare time interrupt is disabled. 1: Interrupt will be generated when the vertical compare time (as defined by LcdVComp field in the LCD_CTRL register) is reached."]
    #[inline(always)]
    pub fn vcompim(&self) -> VcompimR {
        VcompimR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB master error interrupt enable. 0: The AHB Master error interrupt is disabled. 1: Interrupt will be generated when an AHB Master error occurs."]
    #[inline(always)]
    pub fn berim(&self) -> BerimR {
        BerimR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - FIFO underflow interrupt enable. 0: The FIFO underflow interrupt is disabled. 1: Interrupt will be generated when the FIFO underflows."]
    #[inline(always)]
    pub fn fufim(&mut self) -> FufimW<'_, IntmskSpec> {
        FufimW::new(self, 1)
    }
    #[doc = "Bit 2 - LCD next base address update interrupt enable. 0: The base address update interrupt is disabled. 1: Interrupt will be generated when the LCD base address registers have been updated from the next address registers."]
    #[inline(always)]
    pub fn lnbuim(&mut self) -> LnbuimW<'_, IntmskSpec> {
        LnbuimW::new(self, 2)
    }
    #[doc = "Bit 3 - Vertical compare interrupt enable. 0: The vertical compare time interrupt is disabled. 1: Interrupt will be generated when the vertical compare time (as defined by LcdVComp field in the LCD_CTRL register) is reached."]
    #[inline(always)]
    pub fn vcompim(&mut self) -> VcompimW<'_, IntmskSpec> {
        VcompimW::new(self, 3)
    }
    #[doc = "Bit 4 - AHB master error interrupt enable. 0: The AHB Master error interrupt is disabled. 1: Interrupt will be generated when an AHB Master error occurs."]
    #[inline(always)]
    pub fn berim(&mut self) -> BerimW<'_, IntmskSpec> {
        BerimW::new(self, 4)
    }
}
#[doc = "Interrupt Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`intmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntmskSpec;
impl crate::RegisterSpec for IntmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intmsk::R`](R) reader structure"]
impl crate::Readable for IntmskSpec {}
#[doc = "`write(|w| ..)` method takes [`intmsk::W`](W) writer structure"]
impl crate::Writable for IntmskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTMSK to value 0"]
impl crate::Resettable for IntmskSpec {}
