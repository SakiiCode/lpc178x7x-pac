#[doc = "Register `INTSTAT` reader"]
pub type R = crate::R<IntstatSpec>;
#[doc = "Field `FUFMIS` reader - FIFO underflow masked interrupt status. Set when the both the FUFRIS bit in the LCD_INTRAW register and the FUFIM bit in the LCD_INTMSK register are set."]
pub type FufmisR = crate::BitReader;
#[doc = "Field `LNBUMIS` reader - LCD next address base update masked interrupt status. Set when the both the LNBURIS bit in the LCD_INTRAW register and the LNBUIM bit in the LCD_INTMSK register are set."]
pub type LnbumisR = crate::BitReader;
#[doc = "Field `VCOMPMIS` reader - Vertical compare masked interrupt status. Set when the both the VCompRIS bit in the LCD_INTRAW register and the VCompIM bit in the LCD_INTMSK register are set."]
pub type VcompmisR = crate::BitReader;
#[doc = "Field `BERMIS` reader - AHB master bus error masked interrupt status. Set when the both the BERRAW bit in the LCD_INTRAW register and the BERIM bit in the LCD_INTMSK register are set."]
pub type BermisR = crate::BitReader;
impl R {
    #[doc = "Bit 1 - FIFO underflow masked interrupt status. Set when the both the FUFRIS bit in the LCD_INTRAW register and the FUFIM bit in the LCD_INTMSK register are set."]
    #[inline(always)]
    pub fn fufmis(&self) -> FufmisR {
        FufmisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LCD next address base update masked interrupt status. Set when the both the LNBURIS bit in the LCD_INTRAW register and the LNBUIM bit in the LCD_INTMSK register are set."]
    #[inline(always)]
    pub fn lnbumis(&self) -> LnbumisR {
        LnbumisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Vertical compare masked interrupt status. Set when the both the VCompRIS bit in the LCD_INTRAW register and the VCompIM bit in the LCD_INTMSK register are set."]
    #[inline(always)]
    pub fn vcompmis(&self) -> VcompmisR {
        VcompmisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB master bus error masked interrupt status. Set when the both the BERRAW bit in the LCD_INTRAW register and the BERIM bit in the LCD_INTMSK register are set."]
    #[inline(always)]
    pub fn bermis(&self) -> BermisR {
        BermisR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Masked Interrupt Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatSpec;
impl crate::RegisterSpec for IntstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstat::R`](R) reader structure"]
impl crate::Readable for IntstatSpec {}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for IntstatSpec {}
