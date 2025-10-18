#[doc = "Register `USBINTST` reader"]
pub type R = crate::R<UsbintstSpec>;
#[doc = "Register `USBINTST` writer"]
pub type W = crate::W<UsbintstSpec>;
#[doc = "Field `USB_INT_REQ_LP` reader - Low priority interrupt line status. This bit is read-only."]
pub type UsbIntReqLpR = crate::BitReader;
#[doc = "Field `USB_INT_REQ_LP` writer - Low priority interrupt line status. This bit is read-only."]
pub type UsbIntReqLpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_INT_REQ_HP` reader - High priority interrupt line status. This bit is read-only."]
pub type UsbIntReqHpR = crate::BitReader;
#[doc = "Field `USB_INT_REQ_HP` writer - High priority interrupt line status. This bit is read-only."]
pub type UsbIntReqHpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_INT_REQ_DMA` reader - DMA interrupt line status. This bit is read-only."]
pub type UsbIntReqDmaR = crate::BitReader;
#[doc = "Field `USB_INT_REQ_DMA` writer - DMA interrupt line status. This bit is read-only."]
pub type UsbIntReqDmaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_HOST_INT` reader - USB host interrupt line status. This bit is read-only."]
pub type UsbHostIntR = crate::BitReader;
#[doc = "Field `USB_HOST_INT` writer - USB host interrupt line status. This bit is read-only."]
pub type UsbHostIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_ATX_INT` reader - External ATX interrupt line status. This bit is read-only."]
pub type UsbAtxIntR = crate::BitReader;
#[doc = "Field `USB_ATX_INT` writer - External ATX interrupt line status. This bit is read-only."]
pub type UsbAtxIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_OTG_INT` reader - OTG interrupt line status. This bit is read-only."]
pub type UsbOtgIntR = crate::BitReader;
#[doc = "Field `USB_OTG_INT` writer - OTG interrupt line status. This bit is read-only."]
pub type UsbOtgIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_I2C_INT` reader - I2C module interrupt line status. This bit is read-only."]
pub type UsbI2cIntR = crate::BitReader;
#[doc = "Field `USB_I2C_INT` writer - I2C module interrupt line status. This bit is read-only."]
pub type UsbI2cIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_NEED_CLK` reader - USB need clock indicator. This bit is read-only. This bit is set to 1 when USB activity or a change of state on the USB data pins is detected, and it indicates that a PLL supplied clock of 48 MHz is needed. Once USB_NEED_CLK becomes one, it resets to zero 5 ms after the last packet has been received/sent, or 2 ms after the Suspend Change (SUS_CH) interrupt has occurred. A change of this bit from 0 to 1 can wake up the microcontroller if activity on the USB bus is selected to wake up the part from the Power-down mode (see Section 4.7.9 Wake-up from Reduced Power Modes for details). Also see Section 4.5.8 PLLs and Power-down mode and Section 4.7.10 Power Control for Peripherals register (PCONP - 0x400F C0C4) for considerations about the PLL and invoking the Power-down mode. This bit is read-only."]
pub type UsbNeedClkR = crate::BitReader;
#[doc = "Field `USB_NEED_CLK` writer - USB need clock indicator. This bit is read-only. This bit is set to 1 when USB activity or a change of state on the USB data pins is detected, and it indicates that a PLL supplied clock of 48 MHz is needed. Once USB_NEED_CLK becomes one, it resets to zero 5 ms after the last packet has been received/sent, or 2 ms after the Suspend Change (SUS_CH) interrupt has occurred. A change of this bit from 0 to 1 can wake up the microcontroller if activity on the USB bus is selected to wake up the part from the Power-down mode (see Section 4.7.9 Wake-up from Reduced Power Modes for details). Also see Section 4.5.8 PLLs and Power-down mode and Section 4.7.10 Power Control for Peripherals register (PCONP - 0x400F C0C4) for considerations about the PLL and invoking the Power-down mode. This bit is read-only."]
pub type UsbNeedClkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_USB_INTS` reader - Enable all USB interrupts. When this bit is cleared, the NVIC does not see the ORed output of the USB interrupt lines."]
pub type EnUsbIntsR = crate::BitReader;
#[doc = "Field `EN_USB_INTS` writer - Enable all USB interrupts. When this bit is cleared, the NVIC does not see the ORed output of the USB interrupt lines."]
pub type EnUsbIntsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Low priority interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_int_req_lp(&self) -> UsbIntReqLpR {
        UsbIntReqLpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High priority interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_int_req_hp(&self) -> UsbIntReqHpR {
        UsbIntReqHpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_int_req_dma(&self) -> UsbIntReqDmaR {
        UsbIntReqDmaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - USB host interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_host_int(&self) -> UsbHostIntR {
        UsbHostIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - External ATX interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_atx_int(&self) -> UsbAtxIntR {
        UsbAtxIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - OTG interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_otg_int(&self) -> UsbOtgIntR {
        UsbOtgIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - I2C module interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_i2c_int(&self) -> UsbI2cIntR {
        UsbI2cIntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - USB need clock indicator. This bit is read-only. This bit is set to 1 when USB activity or a change of state on the USB data pins is detected, and it indicates that a PLL supplied clock of 48 MHz is needed. Once USB_NEED_CLK becomes one, it resets to zero 5 ms after the last packet has been received/sent, or 2 ms after the Suspend Change (SUS_CH) interrupt has occurred. A change of this bit from 0 to 1 can wake up the microcontroller if activity on the USB bus is selected to wake up the part from the Power-down mode (see Section 4.7.9 Wake-up from Reduced Power Modes for details). Also see Section 4.5.8 PLLs and Power-down mode and Section 4.7.10 Power Control for Peripherals register (PCONP - 0x400F C0C4) for considerations about the PLL and invoking the Power-down mode. This bit is read-only."]
    #[inline(always)]
    pub fn usb_need_clk(&self) -> UsbNeedClkR {
        UsbNeedClkR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 31 - Enable all USB interrupts. When this bit is cleared, the NVIC does not see the ORed output of the USB interrupt lines."]
    #[inline(always)]
    pub fn en_usb_ints(&self) -> EnUsbIntsR {
        EnUsbIntsR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Low priority interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_int_req_lp(&mut self) -> UsbIntReqLpW<'_, UsbintstSpec> {
        UsbIntReqLpW::new(self, 0)
    }
    #[doc = "Bit 1 - High priority interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_int_req_hp(&mut self) -> UsbIntReqHpW<'_, UsbintstSpec> {
        UsbIntReqHpW::new(self, 1)
    }
    #[doc = "Bit 2 - DMA interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_int_req_dma(&mut self) -> UsbIntReqDmaW<'_, UsbintstSpec> {
        UsbIntReqDmaW::new(self, 2)
    }
    #[doc = "Bit 3 - USB host interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_host_int(&mut self) -> UsbHostIntW<'_, UsbintstSpec> {
        UsbHostIntW::new(self, 3)
    }
    #[doc = "Bit 4 - External ATX interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_atx_int(&mut self) -> UsbAtxIntW<'_, UsbintstSpec> {
        UsbAtxIntW::new(self, 4)
    }
    #[doc = "Bit 5 - OTG interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_otg_int(&mut self) -> UsbOtgIntW<'_, UsbintstSpec> {
        UsbOtgIntW::new(self, 5)
    }
    #[doc = "Bit 6 - I2C module interrupt line status. This bit is read-only."]
    #[inline(always)]
    pub fn usb_i2c_int(&mut self) -> UsbI2cIntW<'_, UsbintstSpec> {
        UsbI2cIntW::new(self, 6)
    }
    #[doc = "Bit 8 - USB need clock indicator. This bit is read-only. This bit is set to 1 when USB activity or a change of state on the USB data pins is detected, and it indicates that a PLL supplied clock of 48 MHz is needed. Once USB_NEED_CLK becomes one, it resets to zero 5 ms after the last packet has been received/sent, or 2 ms after the Suspend Change (SUS_CH) interrupt has occurred. A change of this bit from 0 to 1 can wake up the microcontroller if activity on the USB bus is selected to wake up the part from the Power-down mode (see Section 4.7.9 Wake-up from Reduced Power Modes for details). Also see Section 4.5.8 PLLs and Power-down mode and Section 4.7.10 Power Control for Peripherals register (PCONP - 0x400F C0C4) for considerations about the PLL and invoking the Power-down mode. This bit is read-only."]
    #[inline(always)]
    pub fn usb_need_clk(&mut self) -> UsbNeedClkW<'_, UsbintstSpec> {
        UsbNeedClkW::new(self, 8)
    }
    #[doc = "Bit 31 - Enable all USB interrupts. When this bit is cleared, the NVIC does not see the ORed output of the USB interrupt lines."]
    #[inline(always)]
    pub fn en_usb_ints(&mut self) -> EnUsbIntsW<'_, UsbintstSpec> {
        EnUsbIntsW::new(self, 31)
    }
}
#[doc = "USB Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`usbintst::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbintst::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbintstSpec;
impl crate::RegisterSpec for UsbintstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbintst::R`](R) reader structure"]
impl crate::Readable for UsbintstSpec {}
#[doc = "`write(|w| ..)` method takes [`usbintst::W`](W) writer structure"]
impl crate::Writable for UsbintstSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USBINTST to value 0x8000_0000"]
impl crate::Resettable for UsbintstSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
