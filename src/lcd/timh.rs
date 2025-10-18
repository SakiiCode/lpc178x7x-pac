#[doc = "Register `TIMH` reader"]
pub type R = crate::R<TimhSpec>;
#[doc = "Register `TIMH` writer"]
pub type W = crate::W<TimhSpec>;
#[doc = "Field `PPL` reader - Pixels-per-line. The PPL bit field specifies the number of pixels in each line or row of the screen. PPL is a 6-bit value that represents between 16 and 1024 pixels per line. PPL counts the number of pixel clocks that occur before the HFP is applied. Program the value required divided by 16, minus 1. Actual pixels-per-line = 16 * (PPL + 1). For example, to obtain 320 pixels per line, program PPL as (320/16) -1 = 19."]
pub type PplR = crate::FieldReader;
#[doc = "Field `PPL` writer - Pixels-per-line. The PPL bit field specifies the number of pixels in each line or row of the screen. PPL is a 6-bit value that represents between 16 and 1024 pixels per line. PPL counts the number of pixel clocks that occur before the HFP is applied. Program the value required divided by 16, minus 1. Actual pixels-per-line = 16 * (PPL + 1). For example, to obtain 320 pixels per line, program PPL as (320/16) -1 = 19."]
pub type PplW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `HSW` reader - Horizontal synchronization pulse width. The 8-bit HSW field specifies the pulse width of the line clock in passive mode, or the horizontal synchronization pulse in active mode. Program with desired value minus 1."]
pub type HswR = crate::FieldReader;
#[doc = "Field `HSW` writer - Horizontal synchronization pulse width. The 8-bit HSW field specifies the pulse width of the line clock in passive mode, or the horizontal synchronization pulse in active mode. Program with desired value minus 1."]
pub type HswW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HFP` reader - Horizontal front porch. The 8-bit HFP field sets the number of pixel clock intervals at the end of each line or row of pixels, before the LCD line clock is pulsed. When a complete line of pixels is transmitted to the LCD driver, the value in HFP counts the number of pixel clocks to wait before asserting the line clock. HFP can generate a period of 1-256 pixel clock cycles. Program with desired value minus 1."]
pub type HfpR = crate::FieldReader;
#[doc = "Field `HFP` writer - Horizontal front porch. The 8-bit HFP field sets the number of pixel clock intervals at the end of each line or row of pixels, before the LCD line clock is pulsed. When a complete line of pixels is transmitted to the LCD driver, the value in HFP counts the number of pixel clocks to wait before asserting the line clock. HFP can generate a period of 1-256 pixel clock cycles. Program with desired value minus 1."]
pub type HfpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HBP` reader - Horizontal back porch. The 8-bit HBP field is used to specify the number of pixel clock periods inserted at the beginning of each line or row of pixels. After the line clock for the previous line has been deasserted, the value in HBP counts the number of pixel clocks to wait before starting the next display line. HBP can generate a delay of 1-256 pixel clock cycles. Program with desired value minus 1."]
pub type HbpR = crate::FieldReader;
#[doc = "Field `HBP` writer - Horizontal back porch. The 8-bit HBP field is used to specify the number of pixel clock periods inserted at the beginning of each line or row of pixels. After the line clock for the previous line has been deasserted, the value in HBP counts the number of pixel clocks to wait before starting the next display line. HBP can generate a delay of 1-256 pixel clock cycles. Program with desired value minus 1."]
pub type HbpW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 2:7 - Pixels-per-line. The PPL bit field specifies the number of pixels in each line or row of the screen. PPL is a 6-bit value that represents between 16 and 1024 pixels per line. PPL counts the number of pixel clocks that occur before the HFP is applied. Program the value required divided by 16, minus 1. Actual pixels-per-line = 16 * (PPL + 1). For example, to obtain 320 pixels per line, program PPL as (320/16) -1 = 19."]
    #[inline(always)]
    pub fn ppl(&self) -> PplR {
        PplR::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:15 - Horizontal synchronization pulse width. The 8-bit HSW field specifies the pulse width of the line clock in passive mode, or the horizontal synchronization pulse in active mode. Program with desired value minus 1."]
    #[inline(always)]
    pub fn hsw(&self) -> HswR {
        HswR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Horizontal front porch. The 8-bit HFP field sets the number of pixel clock intervals at the end of each line or row of pixels, before the LCD line clock is pulsed. When a complete line of pixels is transmitted to the LCD driver, the value in HFP counts the number of pixel clocks to wait before asserting the line clock. HFP can generate a period of 1-256 pixel clock cycles. Program with desired value minus 1."]
    #[inline(always)]
    pub fn hfp(&self) -> HfpR {
        HfpR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Horizontal back porch. The 8-bit HBP field is used to specify the number of pixel clock periods inserted at the beginning of each line or row of pixels. After the line clock for the previous line has been deasserted, the value in HBP counts the number of pixel clocks to wait before starting the next display line. HBP can generate a delay of 1-256 pixel clock cycles. Program with desired value minus 1."]
    #[inline(always)]
    pub fn hbp(&self) -> HbpR {
        HbpR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 2:7 - Pixels-per-line. The PPL bit field specifies the number of pixels in each line or row of the screen. PPL is a 6-bit value that represents between 16 and 1024 pixels per line. PPL counts the number of pixel clocks that occur before the HFP is applied. Program the value required divided by 16, minus 1. Actual pixels-per-line = 16 * (PPL + 1). For example, to obtain 320 pixels per line, program PPL as (320/16) -1 = 19."]
    #[inline(always)]
    pub fn ppl(&mut self) -> PplW<'_, TimhSpec> {
        PplW::new(self, 2)
    }
    #[doc = "Bits 8:15 - Horizontal synchronization pulse width. The 8-bit HSW field specifies the pulse width of the line clock in passive mode, or the horizontal synchronization pulse in active mode. Program with desired value minus 1."]
    #[inline(always)]
    pub fn hsw(&mut self) -> HswW<'_, TimhSpec> {
        HswW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Horizontal front porch. The 8-bit HFP field sets the number of pixel clock intervals at the end of each line or row of pixels, before the LCD line clock is pulsed. When a complete line of pixels is transmitted to the LCD driver, the value in HFP counts the number of pixel clocks to wait before asserting the line clock. HFP can generate a period of 1-256 pixel clock cycles. Program with desired value minus 1."]
    #[inline(always)]
    pub fn hfp(&mut self) -> HfpW<'_, TimhSpec> {
        HfpW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Horizontal back porch. The 8-bit HBP field is used to specify the number of pixel clock periods inserted at the beginning of each line or row of pixels. After the line clock for the previous line has been deasserted, the value in HBP counts the number of pixel clocks to wait before starting the next display line. HBP can generate a delay of 1-256 pixel clock cycles. Program with desired value minus 1."]
    #[inline(always)]
    pub fn hbp(&mut self) -> HbpW<'_, TimhSpec> {
        HbpW::new(self, 24)
    }
}
#[doc = "Horizontal Timing Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`timh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TimhSpec;
impl crate::RegisterSpec for TimhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timh::R`](R) reader structure"]
impl crate::Readable for TimhSpec {}
#[doc = "`write(|w| ..)` method takes [`timh::W`](W) writer structure"]
impl crate::Writable for TimhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMH to value 0"]
impl crate::Resettable for TimhSpec {}
