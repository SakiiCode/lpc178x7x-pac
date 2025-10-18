#[doc = "Register `STCTRL` reader"]
pub type R = crate::R<StctrlSpec>;
#[doc = "Register `STCTRL` writer"]
pub type W = crate::W<StctrlSpec>;
#[doc = "Field `PORT_FUNC` reader - Controls connection of USB functions (see Figure 51). Bit 0 is set or cleared by hardware when B_HNP_TRACK or A_HNP_TRACK is set and HNP succeeds. See Section 14.9. 00: U1 = device (OTG), U2 = host 01: U1 = host (OTG), U2 = host 10: Reserved 11: U1 = host, U2 = device In a device-only configuration, the following values are allowed: 00: U1 = device. The USB device controller signals are mapped to the U1 port: USB_CONNECT1, USB_UP_LED1, USB_D+1, USB_D-1. 11: U2 = device. The USB device controller signals are mapped to the U2 port: USB_CONNECT2, USB_UP_LED2, USB_D+2, USB_D-2."]
pub type PortFuncR = crate::FieldReader;
#[doc = "Field `PORT_FUNC` writer - Controls connection of USB functions (see Figure 51). Bit 0 is set or cleared by hardware when B_HNP_TRACK or A_HNP_TRACK is set and HNP succeeds. See Section 14.9. 00: U1 = device (OTG), U2 = host 01: U1 = host (OTG), U2 = host 10: Reserved 11: U1 = host, U2 = device In a device-only configuration, the following values are allowed: 00: U1 = device. The USB device controller signals are mapped to the U1 port: USB_CONNECT1, USB_UP_LED1, USB_D+1, USB_D-1. 11: U2 = device. The USB device controller signals are mapped to the U2 port: USB_CONNECT2, USB_UP_LED2, USB_D+2, USB_D-2."]
pub type PortFuncW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TMR_SCALE` reader - Timer scale selection. This field determines the duration of each timer count. 00: 10 ms (100 KHz) 01: 100 ms (10 KHz) 10: 1000 ms (1 KHz) 11: Reserved"]
pub type TmrScaleR = crate::FieldReader;
#[doc = "Field `TMR_SCALE` writer - Timer scale selection. This field determines the duration of each timer count. 00: 10 ms (100 KHz) 01: 100 ms (10 KHz) 10: 1000 ms (1 KHz) 11: Reserved"]
pub type TmrScaleW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TMR_MODE` reader - Timer mode selection. 0: monoshot 1: free running"]
pub type TmrModeR = crate::BitReader;
#[doc = "Field `TMR_MODE` writer - Timer mode selection. 0: monoshot 1: free running"]
pub type TmrModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR_EN` reader - Timer enable. When set, TMR_CNT increments. When cleared, TMR_CNT is reset to 0."]
pub type TmrEnR = crate::BitReader;
#[doc = "Field `TMR_EN` writer - Timer enable. When set, TMR_CNT increments. When cleared, TMR_CNT is reset to 0."]
pub type TmrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR_RST` reader - Timer reset. Writing one to this bit resets TMR_CNT to 0. This provides a single bit control for the software to restart the timer when the timer is enabled."]
pub type TmrRstR = crate::BitReader;
#[doc = "Field `TMR_RST` writer - Timer reset. Writing one to this bit resets TMR_CNT to 0. This provides a single bit control for the software to restart the timer when the timer is enabled."]
pub type TmrRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `B_HNP_TRACK` reader - Enable HNP tracking for B-device (peripheral), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
pub type BHnpTrackR = crate::BitReader;
#[doc = "Field `B_HNP_TRACK` writer - Enable HNP tracking for B-device (peripheral), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
pub type BHnpTrackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A_HNP_TRACK` reader - Enable HNP tracking for A-device (host), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
pub type AHnpTrackR = crate::BitReader;
#[doc = "Field `A_HNP_TRACK` writer - Enable HNP tracking for A-device (host), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
pub type AHnpTrackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PU_REMOVED` reader - When the B-device changes its role from peripheral to host, software sets this bit when it removes the D+ pull-up, see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
pub type PuRemovedR = crate::BitReader;
#[doc = "Field `PU_REMOVED` writer - When the B-device changes its role from peripheral to host, software sets this bit when it removes the D+ pull-up, see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
pub type PuRemovedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMR_CNT` reader - Current timer count value."]
pub type TmrCntR = crate::FieldReader<u16>;
#[doc = "Field `TMR_CNT` writer - Current timer count value."]
pub type TmrCntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - Controls connection of USB functions (see Figure 51). Bit 0 is set or cleared by hardware when B_HNP_TRACK or A_HNP_TRACK is set and HNP succeeds. See Section 14.9. 00: U1 = device (OTG), U2 = host 01: U1 = host (OTG), U2 = host 10: Reserved 11: U1 = host, U2 = device In a device-only configuration, the following values are allowed: 00: U1 = device. The USB device controller signals are mapped to the U1 port: USB_CONNECT1, USB_UP_LED1, USB_D+1, USB_D-1. 11: U2 = device. The USB device controller signals are mapped to the U2 port: USB_CONNECT2, USB_UP_LED2, USB_D+2, USB_D-2."]
    #[inline(always)]
    pub fn port_func(&self) -> PortFuncR {
        PortFuncR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Timer scale selection. This field determines the duration of each timer count. 00: 10 ms (100 KHz) 01: 100 ms (10 KHz) 10: 1000 ms (1 KHz) 11: Reserved"]
    #[inline(always)]
    pub fn tmr_scale(&self) -> TmrScaleR {
        TmrScaleR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Timer mode selection. 0: monoshot 1: free running"]
    #[inline(always)]
    pub fn tmr_mode(&self) -> TmrModeR {
        TmrModeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Timer enable. When set, TMR_CNT increments. When cleared, TMR_CNT is reset to 0."]
    #[inline(always)]
    pub fn tmr_en(&self) -> TmrEnR {
        TmrEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer reset. Writing one to this bit resets TMR_CNT to 0. This provides a single bit control for the software to restart the timer when the timer is enabled."]
    #[inline(always)]
    pub fn tmr_rst(&self) -> TmrRstR {
        TmrRstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable HNP tracking for B-device (peripheral), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    pub fn b_hnp_track(&self) -> BHnpTrackR {
        BHnpTrackR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable HNP tracking for A-device (host), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    pub fn a_hnp_track(&self) -> AHnpTrackR {
        AHnpTrackR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When the B-device changes its role from peripheral to host, software sets this bit when it removes the D+ pull-up, see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    pub fn pu_removed(&self) -> PuRemovedR {
        PuRemovedR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:31 - Current timer count value."]
    #[inline(always)]
    pub fn tmr_cnt(&self) -> TmrCntR {
        TmrCntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Controls connection of USB functions (see Figure 51). Bit 0 is set or cleared by hardware when B_HNP_TRACK or A_HNP_TRACK is set and HNP succeeds. See Section 14.9. 00: U1 = device (OTG), U2 = host 01: U1 = host (OTG), U2 = host 10: Reserved 11: U1 = host, U2 = device In a device-only configuration, the following values are allowed: 00: U1 = device. The USB device controller signals are mapped to the U1 port: USB_CONNECT1, USB_UP_LED1, USB_D+1, USB_D-1. 11: U2 = device. The USB device controller signals are mapped to the U2 port: USB_CONNECT2, USB_UP_LED2, USB_D+2, USB_D-2."]
    #[inline(always)]
    pub fn port_func(&mut self) -> PortFuncW<'_, StctrlSpec> {
        PortFuncW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Timer scale selection. This field determines the duration of each timer count. 00: 10 ms (100 KHz) 01: 100 ms (10 KHz) 10: 1000 ms (1 KHz) 11: Reserved"]
    #[inline(always)]
    pub fn tmr_scale(&mut self) -> TmrScaleW<'_, StctrlSpec> {
        TmrScaleW::new(self, 2)
    }
    #[doc = "Bit 4 - Timer mode selection. 0: monoshot 1: free running"]
    #[inline(always)]
    pub fn tmr_mode(&mut self) -> TmrModeW<'_, StctrlSpec> {
        TmrModeW::new(self, 4)
    }
    #[doc = "Bit 5 - Timer enable. When set, TMR_CNT increments. When cleared, TMR_CNT is reset to 0."]
    #[inline(always)]
    pub fn tmr_en(&mut self) -> TmrEnW<'_, StctrlSpec> {
        TmrEnW::new(self, 5)
    }
    #[doc = "Bit 6 - Timer reset. Writing one to this bit resets TMR_CNT to 0. This provides a single bit control for the software to restart the timer when the timer is enabled."]
    #[inline(always)]
    pub fn tmr_rst(&mut self) -> TmrRstW<'_, StctrlSpec> {
        TmrRstW::new(self, 6)
    }
    #[doc = "Bit 8 - Enable HNP tracking for B-device (peripheral), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    pub fn b_hnp_track(&mut self) -> BHnpTrackW<'_, StctrlSpec> {
        BHnpTrackW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable HNP tracking for A-device (host), see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    pub fn a_hnp_track(&mut self) -> AHnpTrackW<'_, StctrlSpec> {
        AHnpTrackW::new(self, 9)
    }
    #[doc = "Bit 10 - When the B-device changes its role from peripheral to host, software sets this bit when it removes the D+ pull-up, see Section 14.9. Hardware clears this bit when HNP_SUCCESS or HNP_FAILURE is set."]
    #[inline(always)]
    pub fn pu_removed(&mut self) -> PuRemovedW<'_, StctrlSpec> {
        PuRemovedW::new(self, 10)
    }
    #[doc = "Bits 16:31 - Current timer count value."]
    #[inline(always)]
    pub fn tmr_cnt(&mut self) -> TmrCntW<'_, StctrlSpec> {
        TmrCntW::new(self, 16)
    }
}
#[doc = "OTG Status and Control and USB port select\n\nYou can [`read`](crate::Reg::read) this register and get [`stctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StctrlSpec;
impl crate::RegisterSpec for StctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stctrl::R`](R) reader structure"]
impl crate::Readable for StctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`stctrl::W`](W) writer structure"]
impl crate::Writable for StctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STCTRL to value 0"]
impl crate::Resettable for StctrlSpec {}
