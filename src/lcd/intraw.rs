#[doc = "Register `INTRAW` reader"]
pub type R = crate::R<IntrawSpec>;
#[doc = "Field `FUFRIS` reader - FIFO underflow raw interrupt status. Set when either the upper or lower DMA FIFOs have been read accessed when empty causing an underflow condition to occur. Generates an interrupt if the FUFIM bit in the LCD_INTMSK register is set."]
pub type FufrisR = crate::BitReader;
#[doc = "Field `LNBURIS` reader - LCD next address base update raw interrupt status. Mode dependent. Set when the current base address registers have been successfully updated by the next address registers. Signifies that a new next address can be loaded if double buffering is in use. Generates an interrupt if the LNBUIM bit in the LCD_INTMSK register is set."]
pub type LnburisR = crate::BitReader;
#[doc = "Field `VCOMPRIS` reader - Vertical compare raw interrupt status. Set when one of the four vertical regions is reached, as selected by the LcdVComp bits in the LCD_CTRL register. Generates an interrupt if the VCompIM bit in the LCD_INTMSK register is set."]
pub type VcomprisR = crate::BitReader;
#[doc = "Field `BERRAW` reader - AHB master bus error raw interrupt status. Set when the AHB master interface receives a bus error response from a slave. Generates an interrupt if the BERIM bit in the LCD_INTMSK register is set."]
pub type BerrawR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - FIFO underflow raw interrupt status. Set when either the upper or lower DMA FIFOs have been read accessed when empty causing an underflow condition to occur. Generates an interrupt if the FUFIM bit in the LCD_INTMSK register is set."]
    #[inline(always)]
    pub fn fufris(&self) -> FufrisR {
        FufrisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD next address base update raw interrupt status. Mode dependent. Set when the current base address registers have been successfully updated by the next address registers. Signifies that a new next address can be loaded if double buffering is in use. Generates an interrupt if the LNBUIM bit in the LCD_INTMSK register is set."]
    #[inline(always)]
    pub fn lnburis(&self) -> LnburisR {
        LnburisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Vertical compare raw interrupt status. Set when one of the four vertical regions is reached, as selected by the LcdVComp bits in the LCD_CTRL register. Generates an interrupt if the VCompIM bit in the LCD_INTMSK register is set."]
    #[inline(always)]
    pub fn vcompris(&self) -> VcomprisR {
        VcomprisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB master bus error raw interrupt status. Set when the AHB master interface receives a bus error response from a slave. Generates an interrupt if the BERIM bit in the LCD_INTMSK register is set."]
    #[inline(always)]
    pub fn berraw(&self) -> BerrawR {
        BerrawR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Raw Interrupt Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intraw::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntrawSpec;
impl crate::RegisterSpec for IntrawSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intraw::R`](R) reader structure"]
impl crate::Readable for IntrawSpec {}
#[doc = "`reset()` method sets INTRAW to value 0"]
impl crate::Resettable for IntrawSpec {}
