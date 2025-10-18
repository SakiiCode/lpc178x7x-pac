#[doc = "Register `POL` reader"]
pub type R = crate::R<PolSpec>;
#[doc = "Register `POL` writer"]
pub type W = crate::W<PolSpec>;
#[doc = "Field `PCD_LO` reader - Lower five bits of panel clock divisor. The ten-bit PCD field, comprising PCD_HI (bits 31:27 of this register) and PCD_LO, is used to derive the LCD panel clock frequency LCD_DCLK from the input clock, LCD_DCLK = LCDCLK/(PCD+2). For monochrome STN displays with a 4 or 8-bit interface, the panel clock is a factor of four and eight down from the actual individual pixel clock rate. For color STN displays, 22/3 pixels are output per LCD_DCLK cycle, so the panel clock is 0.375 times the pixel rate. For TFT displays, the pixel clock divider can be bypassed by setting the BCD bit in this register. Note: data path latency forces some restrictions on the usable minimum values for the panel clock divider in STN modes: Single panel color mode, PCD = 1 (LCD_DCLK = LCDCLK/3). Dual panel color mode, PCD = 4 (LCD_DCLK = LCDCLK/6). Single panel monochrome 4-bit interface mode, PCD = 2(LCD_DCLK = LCDCLK/4). Dual panel monochrome 4-bit interface mode and single panel monochrome 8-bit interface mode, PCD = 6(LCD_DCLK = LCDCLK/8). Dual panel monochrome 8-bit interface mode, PCD = 14(LCD_DCLK = LCDCLK/16)."]
pub type PcdLoR = crate::FieldReader;
#[doc = "Field `PCD_LO` writer - Lower five bits of panel clock divisor. The ten-bit PCD field, comprising PCD_HI (bits 31:27 of this register) and PCD_LO, is used to derive the LCD panel clock frequency LCD_DCLK from the input clock, LCD_DCLK = LCDCLK/(PCD+2). For monochrome STN displays with a 4 or 8-bit interface, the panel clock is a factor of four and eight down from the actual individual pixel clock rate. For color STN displays, 22/3 pixels are output per LCD_DCLK cycle, so the panel clock is 0.375 times the pixel rate. For TFT displays, the pixel clock divider can be bypassed by setting the BCD bit in this register. Note: data path latency forces some restrictions on the usable minimum values for the panel clock divider in STN modes: Single panel color mode, PCD = 1 (LCD_DCLK = LCDCLK/3). Dual panel color mode, PCD = 4 (LCD_DCLK = LCDCLK/6). Single panel monochrome 4-bit interface mode, PCD = 2(LCD_DCLK = LCDCLK/4). Dual panel monochrome 4-bit interface mode and single panel monochrome 8-bit interface mode, PCD = 6(LCD_DCLK = LCDCLK/8). Dual panel monochrome 8-bit interface mode, PCD = 14(LCD_DCLK = LCDCLK/16)."]
pub type PcdLoW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `CLKSEL` reader - Clock Select. This bit controls the selection of the source for LCDCLK. 0 = the clock source for the LCD block is CCLK. 1 = the clock source for the LCD block is LCD_CLKIN (external clock input for the LVD)."]
pub type ClkselR = crate::BitReader;
#[doc = "Field `CLKSEL` writer - Clock Select. This bit controls the selection of the source for LCDCLK. 0 = the clock source for the LCD block is CCLK. 1 = the clock source for the LCD block is LCD_CLKIN (external clock input for the LVD)."]
pub type ClkselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACB` reader - AC bias pin frequency. The AC bias pin frequency is only applicable to STN displays. These require the pixel voltage polarity to periodically reverse to prevent damage caused by DC charge accumulation. Program this field with the required value minus one to apply the number of line clocks between each toggle of the AC bias pin, LCD_ENAB_M. This field has no effect if the LCD is operating in TFT mode, when the LCD_ENAB_M pin is used as a data enable signal."]
pub type AcbR = crate::FieldReader;
#[doc = "Field `ACB` writer - AC bias pin frequency. The AC bias pin frequency is only applicable to STN displays. These require the pixel voltage polarity to periodically reverse to prevent damage caused by DC charge accumulation. Program this field with the required value minus one to apply the number of line clocks between each toggle of the AC bias pin, LCD_ENAB_M. This field has no effect if the LCD is operating in TFT mode, when the LCD_ENAB_M pin is used as a data enable signal."]
pub type AcbW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `IVS` reader - Invert vertical synchronization. The IVS bit inverts the polarity of the LCD_FP signal. 0 = LCD_FP pin is active HIGH and inactive LOW. 1 = LCD_FP pin is active LOW and inactive HIGH."]
pub type IvsR = crate::BitReader;
#[doc = "Field `IVS` writer - Invert vertical synchronization. The IVS bit inverts the polarity of the LCD_FP signal. 0 = LCD_FP pin is active HIGH and inactive LOW. 1 = LCD_FP pin is active LOW and inactive HIGH."]
pub type IvsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IHS` reader - Invert horizontal synchronization. The IHS bit inverts the polarity of the LCD_LP signal. 0 = LCD_LP pin is active HIGH and inactive LOW. 1 = LCD_LP pin is active LOW and inactive HIGH."]
pub type IhsR = crate::BitReader;
#[doc = "Field `IHS` writer - Invert horizontal synchronization. The IHS bit inverts the polarity of the LCD_LP signal. 0 = LCD_LP pin is active HIGH and inactive LOW. 1 = LCD_LP pin is active LOW and inactive HIGH."]
pub type IhsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPC` reader - Invert panel clock. The IPC bit selects the edge of the panel clock on which pixel data is driven out onto the LCD data lines. 0 = Data is driven on the LCD data lines on the rising edge of LCD_DCLK. 1 = Data is driven on the LCD data lines on the falling edge of LCD_DCLK."]
pub type IpcR = crate::BitReader;
#[doc = "Field `IPC` writer - Invert panel clock. The IPC bit selects the edge of the panel clock on which pixel data is driven out onto the LCD data lines. 0 = Data is driven on the LCD data lines on the rising edge of LCD_DCLK. 1 = Data is driven on the LCD data lines on the falling edge of LCD_DCLK."]
pub type IpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOE` reader - Invert output enable. This bit selects the active polarity of the output enable signal in TFT mode. In this mode, the LCD_ENAB_M pin is used as an enable that indicates to the LCD panel when valid display data is available. In active display mode, data is driven onto the LCD data lines at the programmed edge of LCD_DCLK when LCD_ENAB_M is in its active state. 0 = LCD_ENAB_M output pin is active HIGH in TFT mode. 1 = LCD_ENAB_M output pin is active LOW in TFT mode."]
pub type IoeR = crate::BitReader;
#[doc = "Field `IOE` writer - Invert output enable. This bit selects the active polarity of the output enable signal in TFT mode. In this mode, the LCD_ENAB_M pin is used as an enable that indicates to the LCD panel when valid display data is available. In active display mode, data is driven onto the LCD data lines at the programmed edge of LCD_DCLK when LCD_ENAB_M is in its active state. 0 = LCD_ENAB_M output pin is active HIGH in TFT mode. 1 = LCD_ENAB_M output pin is active LOW in TFT mode."]
pub type IoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPL` reader - Clocks per line. This field specifies the number of actual LCD_DCLK clocks to the LCD panel on each line. This is the number of PPL divided by either 1 (for TFT), 4 or 8 (for monochrome passive), 2 2/3 (for color passive), minus one. This must be correctly programmed in addition to the PPL bit in the LCD_TIMH register for the LCD display to work correctly."]
pub type CplR = crate::FieldReader<u16>;
#[doc = "Field `CPL` writer - Clocks per line. This field specifies the number of actual LCD_DCLK clocks to the LCD panel on each line. This is the number of PPL divided by either 1 (for TFT), 4 or 8 (for monochrome passive), 2 2/3 (for color passive), minus one. This must be correctly programmed in addition to the PPL bit in the LCD_TIMH register for the LCD display to work correctly."]
pub type CplW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `BCD` reader - Bypass pixel clock divider. Setting this to 1 bypasses the pixel clock divider logic. This is mainly used for TFT displays."]
pub type BcdR = crate::BitReader;
#[doc = "Field `BCD` writer - Bypass pixel clock divider. Setting this to 1 bypasses the pixel clock divider logic. This is mainly used for TFT displays."]
pub type BcdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCD_HI` reader - Upper five bits of panel clock divisor. See description for PCD_LO, in bits \\[4:0\\] of this register."]
pub type PcdHiR = crate::FieldReader;
#[doc = "Field `PCD_HI` writer - Upper five bits of panel clock divisor. See description for PCD_LO, in bits \\[4:0\\] of this register."]
pub type PcdHiW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Lower five bits of panel clock divisor. The ten-bit PCD field, comprising PCD_HI (bits 31:27 of this register) and PCD_LO, is used to derive the LCD panel clock frequency LCD_DCLK from the input clock, LCD_DCLK = LCDCLK/(PCD+2). For monochrome STN displays with a 4 or 8-bit interface, the panel clock is a factor of four and eight down from the actual individual pixel clock rate. For color STN displays, 22/3 pixels are output per LCD_DCLK cycle, so the panel clock is 0.375 times the pixel rate. For TFT displays, the pixel clock divider can be bypassed by setting the BCD bit in this register. Note: data path latency forces some restrictions on the usable minimum values for the panel clock divider in STN modes: Single panel color mode, PCD = 1 (LCD_DCLK = LCDCLK/3). Dual panel color mode, PCD = 4 (LCD_DCLK = LCDCLK/6). Single panel monochrome 4-bit interface mode, PCD = 2(LCD_DCLK = LCDCLK/4). Dual panel monochrome 4-bit interface mode and single panel monochrome 8-bit interface mode, PCD = 6(LCD_DCLK = LCDCLK/8). Dual panel monochrome 8-bit interface mode, PCD = 14(LCD_DCLK = LCDCLK/16)."]
    #[inline(always)]
    pub fn pcd_lo(&self) -> PcdLoR {
        PcdLoR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Clock Select. This bit controls the selection of the source for LCDCLK. 0 = the clock source for the LCD block is CCLK. 1 = the clock source for the LCD block is LCD_CLKIN (external clock input for the LVD)."]
    #[inline(always)]
    pub fn clksel(&self) -> ClkselR {
        ClkselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:10 - AC bias pin frequency. The AC bias pin frequency is only applicable to STN displays. These require the pixel voltage polarity to periodically reverse to prevent damage caused by DC charge accumulation. Program this field with the required value minus one to apply the number of line clocks between each toggle of the AC bias pin, LCD_ENAB_M. This field has no effect if the LCD is operating in TFT mode, when the LCD_ENAB_M pin is used as a data enable signal."]
    #[inline(always)]
    pub fn acb(&self) -> AcbR {
        AcbR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - Invert vertical synchronization. The IVS bit inverts the polarity of the LCD_FP signal. 0 = LCD_FP pin is active HIGH and inactive LOW. 1 = LCD_FP pin is active LOW and inactive HIGH."]
    #[inline(always)]
    pub fn ivs(&self) -> IvsR {
        IvsR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Invert horizontal synchronization. The IHS bit inverts the polarity of the LCD_LP signal. 0 = LCD_LP pin is active HIGH and inactive LOW. 1 = LCD_LP pin is active LOW and inactive HIGH."]
    #[inline(always)]
    pub fn ihs(&self) -> IhsR {
        IhsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Invert panel clock. The IPC bit selects the edge of the panel clock on which pixel data is driven out onto the LCD data lines. 0 = Data is driven on the LCD data lines on the rising edge of LCD_DCLK. 1 = Data is driven on the LCD data lines on the falling edge of LCD_DCLK."]
    #[inline(always)]
    pub fn ipc(&self) -> IpcR {
        IpcR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Invert output enable. This bit selects the active polarity of the output enable signal in TFT mode. In this mode, the LCD_ENAB_M pin is used as an enable that indicates to the LCD panel when valid display data is available. In active display mode, data is driven onto the LCD data lines at the programmed edge of LCD_DCLK when LCD_ENAB_M is in its active state. 0 = LCD_ENAB_M output pin is active HIGH in TFT mode. 1 = LCD_ENAB_M output pin is active LOW in TFT mode."]
    #[inline(always)]
    pub fn ioe(&self) -> IoeR {
        IoeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:25 - Clocks per line. This field specifies the number of actual LCD_DCLK clocks to the LCD panel on each line. This is the number of PPL divided by either 1 (for TFT), 4 or 8 (for monochrome passive), 2 2/3 (for color passive), minus one. This must be correctly programmed in addition to the PPL bit in the LCD_TIMH register for the LCD display to work correctly."]
    #[inline(always)]
    pub fn cpl(&self) -> CplR {
        CplR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 26 - Bypass pixel clock divider. Setting this to 1 bypasses the pixel clock divider logic. This is mainly used for TFT displays."]
    #[inline(always)]
    pub fn bcd(&self) -> BcdR {
        BcdR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - Upper five bits of panel clock divisor. See description for PCD_LO, in bits \\[4:0\\] of this register."]
    #[inline(always)]
    pub fn pcd_hi(&self) -> PcdHiR {
        PcdHiR::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Lower five bits of panel clock divisor. The ten-bit PCD field, comprising PCD_HI (bits 31:27 of this register) and PCD_LO, is used to derive the LCD panel clock frequency LCD_DCLK from the input clock, LCD_DCLK = LCDCLK/(PCD+2). For monochrome STN displays with a 4 or 8-bit interface, the panel clock is a factor of four and eight down from the actual individual pixel clock rate. For color STN displays, 22/3 pixels are output per LCD_DCLK cycle, so the panel clock is 0.375 times the pixel rate. For TFT displays, the pixel clock divider can be bypassed by setting the BCD bit in this register. Note: data path latency forces some restrictions on the usable minimum values for the panel clock divider in STN modes: Single panel color mode, PCD = 1 (LCD_DCLK = LCDCLK/3). Dual panel color mode, PCD = 4 (LCD_DCLK = LCDCLK/6). Single panel monochrome 4-bit interface mode, PCD = 2(LCD_DCLK = LCDCLK/4). Dual panel monochrome 4-bit interface mode and single panel monochrome 8-bit interface mode, PCD = 6(LCD_DCLK = LCDCLK/8). Dual panel monochrome 8-bit interface mode, PCD = 14(LCD_DCLK = LCDCLK/16)."]
    #[inline(always)]
    pub fn pcd_lo(&mut self) -> PcdLoW<'_, PolSpec> {
        PcdLoW::new(self, 0)
    }
    #[doc = "Bit 5 - Clock Select. This bit controls the selection of the source for LCDCLK. 0 = the clock source for the LCD block is CCLK. 1 = the clock source for the LCD block is LCD_CLKIN (external clock input for the LVD)."]
    #[inline(always)]
    pub fn clksel(&mut self) -> ClkselW<'_, PolSpec> {
        ClkselW::new(self, 5)
    }
    #[doc = "Bits 6:10 - AC bias pin frequency. The AC bias pin frequency is only applicable to STN displays. These require the pixel voltage polarity to periodically reverse to prevent damage caused by DC charge accumulation. Program this field with the required value minus one to apply the number of line clocks between each toggle of the AC bias pin, LCD_ENAB_M. This field has no effect if the LCD is operating in TFT mode, when the LCD_ENAB_M pin is used as a data enable signal."]
    #[inline(always)]
    pub fn acb(&mut self) -> AcbW<'_, PolSpec> {
        AcbW::new(self, 6)
    }
    #[doc = "Bit 11 - Invert vertical synchronization. The IVS bit inverts the polarity of the LCD_FP signal. 0 = LCD_FP pin is active HIGH and inactive LOW. 1 = LCD_FP pin is active LOW and inactive HIGH."]
    #[inline(always)]
    pub fn ivs(&mut self) -> IvsW<'_, PolSpec> {
        IvsW::new(self, 11)
    }
    #[doc = "Bit 12 - Invert horizontal synchronization. The IHS bit inverts the polarity of the LCD_LP signal. 0 = LCD_LP pin is active HIGH and inactive LOW. 1 = LCD_LP pin is active LOW and inactive HIGH."]
    #[inline(always)]
    pub fn ihs(&mut self) -> IhsW<'_, PolSpec> {
        IhsW::new(self, 12)
    }
    #[doc = "Bit 13 - Invert panel clock. The IPC bit selects the edge of the panel clock on which pixel data is driven out onto the LCD data lines. 0 = Data is driven on the LCD data lines on the rising edge of LCD_DCLK. 1 = Data is driven on the LCD data lines on the falling edge of LCD_DCLK."]
    #[inline(always)]
    pub fn ipc(&mut self) -> IpcW<'_, PolSpec> {
        IpcW::new(self, 13)
    }
    #[doc = "Bit 14 - Invert output enable. This bit selects the active polarity of the output enable signal in TFT mode. In this mode, the LCD_ENAB_M pin is used as an enable that indicates to the LCD panel when valid display data is available. In active display mode, data is driven onto the LCD data lines at the programmed edge of LCD_DCLK when LCD_ENAB_M is in its active state. 0 = LCD_ENAB_M output pin is active HIGH in TFT mode. 1 = LCD_ENAB_M output pin is active LOW in TFT mode."]
    #[inline(always)]
    pub fn ioe(&mut self) -> IoeW<'_, PolSpec> {
        IoeW::new(self, 14)
    }
    #[doc = "Bits 16:25 - Clocks per line. This field specifies the number of actual LCD_DCLK clocks to the LCD panel on each line. This is the number of PPL divided by either 1 (for TFT), 4 or 8 (for monochrome passive), 2 2/3 (for color passive), minus one. This must be correctly programmed in addition to the PPL bit in the LCD_TIMH register for the LCD display to work correctly."]
    #[inline(always)]
    pub fn cpl(&mut self) -> CplW<'_, PolSpec> {
        CplW::new(self, 16)
    }
    #[doc = "Bit 26 - Bypass pixel clock divider. Setting this to 1 bypasses the pixel clock divider logic. This is mainly used for TFT displays."]
    #[inline(always)]
    pub fn bcd(&mut self) -> BcdW<'_, PolSpec> {
        BcdW::new(self, 26)
    }
    #[doc = "Bits 27:31 - Upper five bits of panel clock divisor. See description for PCD_LO, in bits \\[4:0\\] of this register."]
    #[inline(always)]
    pub fn pcd_hi(&mut self) -> PcdHiW<'_, PolSpec> {
        PcdHiW::new(self, 27)
    }
}
#[doc = "Clock and Signal Polarity Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PolSpec;
impl crate::RegisterSpec for PolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pol::R`](R) reader structure"]
impl crate::Readable for PolSpec {}
#[doc = "`write(|w| ..)` method takes [`pol::W`](W) writer structure"]
impl crate::Writable for PolSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets POL to value 0"]
impl crate::Resettable for PolSpec {}
