#[doc = "Register `MAC1` reader"]
pub type R = crate::R<Mac1Spec>;
#[doc = "Register `MAC1` writer"]
pub type W = crate::W<Mac1Spec>;
#[doc = "Field `RXENABLE` reader - RECEIVE ENABLE. Set this to allow receive frames to be received. Internally the MAC synchronizes this control bit to the incoming receive stream."]
pub type RxenableR = crate::BitReader;
#[doc = "Field `RXENABLE` writer - RECEIVE ENABLE. Set this to allow receive frames to be received. Internally the MAC synchronizes this control bit to the incoming receive stream."]
pub type RxenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PARF` reader - PASS ALL RECEIVE FRAMES. When enabled (set to 1), the MAC will pass all frames regardless of type (normal vs. Control). When disabled, the MAC does not pass valid Control frames."]
pub type ParfR = crate::BitReader;
#[doc = "Field `PARF` writer - PASS ALL RECEIVE FRAMES. When enabled (set to 1), the MAC will pass all frames regardless of type (normal vs. Control). When disabled, the MAC does not pass valid Control frames."]
pub type ParfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFLOWCTRL` reader - RX FLOW CONTROL. When enabled (set to 1), the MAC acts upon received PAUSE Flow Control frames. When disabled, received PAUSE Flow Control frames are ignored."]
pub type RxflowctrlR = crate::BitReader;
#[doc = "Field `RXFLOWCTRL` writer - RX FLOW CONTROL. When enabled (set to 1), the MAC acts upon received PAUSE Flow Control frames. When disabled, received PAUSE Flow Control frames are ignored."]
pub type RxflowctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFLOWCTRL` reader - TX FLOW CONTROL. When enabled (set to 1), PAUSE Flow Control frames are allowed to be transmitted. When disabled, Flow Control frames are blocked."]
pub type TxflowctrlR = crate::BitReader;
#[doc = "Field `TXFLOWCTRL` writer - TX FLOW CONTROL. When enabled (set to 1), PAUSE Flow Control frames are allowed to be transmitted. When disabled, Flow Control frames are blocked."]
pub type TxflowctrlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LOOPBACK` reader - Setting this bit will cause the MAC Transmit interface to be looped back to the MAC Receive interface. Clearing this bit results in normal operation."]
pub type LoopbackR = crate::BitReader;
#[doc = "Field `LOOPBACK` writer - Setting this bit will cause the MAC Transmit interface to be looped back to the MAC Receive interface. Clearing this bit results in normal operation."]
pub type LoopbackW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETTX` reader - Setting this bit will put the Transmit Function logic in reset."]
pub type ResettxR = crate::BitReader;
#[doc = "Field `RESETTX` writer - Setting this bit will put the Transmit Function logic in reset."]
pub type ResettxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETMCSTX` reader - Setting this bit resets the MAC Control Sublayer / Transmit logic. The MCS logic implements flow control."]
pub type ResetmcstxR = crate::BitReader;
#[doc = "Field `RESETMCSTX` writer - Setting this bit resets the MAC Control Sublayer / Transmit logic. The MCS logic implements flow control."]
pub type ResetmcstxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETRX` reader - Setting this bit will put the Ethernet receive logic in reset."]
pub type ResetrxR = crate::BitReader;
#[doc = "Field `RESETRX` writer - Setting this bit will put the Ethernet receive logic in reset."]
pub type ResetrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETMCSRX` reader - Setting this bit resets the MAC Control Sublayer / Receive logic. The MCS logic implements flow control."]
pub type ResetmcsrxR = crate::BitReader;
#[doc = "Field `RESETMCSRX` writer - Setting this bit resets the MAC Control Sublayer / Receive logic. The MCS logic implements flow control."]
pub type ResetmcsrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SIMRESET` reader - SIMULATION RESET. Setting this bit will cause a reset to the random number generator within the Transmit Function."]
pub type SimresetR = crate::BitReader;
#[doc = "Field `SIMRESET` writer - SIMULATION RESET. Setting this bit will cause a reset to the random number generator within the Transmit Function."]
pub type SimresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOFTRESET` reader - SOFT RESET. Setting this bit will put all modules within the MAC in reset except the Host Interface."]
pub type SoftresetR = crate::BitReader;
#[doc = "Field `SOFTRESET` writer - SOFT RESET. Setting this bit will put all modules within the MAC in reset except the Host Interface."]
pub type SoftresetW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RECEIVE ENABLE. Set this to allow receive frames to be received. Internally the MAC synchronizes this control bit to the incoming receive stream."]
    #[inline(always)]
    pub fn rxenable(&self) -> RxenableR {
        RxenableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PASS ALL RECEIVE FRAMES. When enabled (set to 1), the MAC will pass all frames regardless of type (normal vs. Control). When disabled, the MAC does not pass valid Control frames."]
    #[inline(always)]
    pub fn parf(&self) -> ParfR {
        ParfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX FLOW CONTROL. When enabled (set to 1), the MAC acts upon received PAUSE Flow Control frames. When disabled, received PAUSE Flow Control frames are ignored."]
    #[inline(always)]
    pub fn rxflowctrl(&self) -> RxflowctrlR {
        RxflowctrlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TX FLOW CONTROL. When enabled (set to 1), PAUSE Flow Control frames are allowed to be transmitted. When disabled, Flow Control frames are blocked."]
    #[inline(always)]
    pub fn txflowctrl(&self) -> TxflowctrlR {
        TxflowctrlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Setting this bit will cause the MAC Transmit interface to be looped back to the MAC Receive interface. Clearing this bit results in normal operation."]
    #[inline(always)]
    pub fn loopback(&self) -> LoopbackR {
        LoopbackR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Setting this bit will put the Transmit Function logic in reset."]
    #[inline(always)]
    pub fn resettx(&self) -> ResettxR {
        ResettxR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Setting this bit resets the MAC Control Sublayer / Transmit logic. The MCS logic implements flow control."]
    #[inline(always)]
    pub fn resetmcstx(&self) -> ResetmcstxR {
        ResetmcstxR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Setting this bit will put the Ethernet receive logic in reset."]
    #[inline(always)]
    pub fn resetrx(&self) -> ResetrxR {
        ResetrxR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Setting this bit resets the MAC Control Sublayer / Receive logic. The MCS logic implements flow control."]
    #[inline(always)]
    pub fn resetmcsrx(&self) -> ResetmcsrxR {
        ResetmcsrxR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - SIMULATION RESET. Setting this bit will cause a reset to the random number generator within the Transmit Function."]
    #[inline(always)]
    pub fn simreset(&self) -> SimresetR {
        SimresetR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SOFT RESET. Setting this bit will put all modules within the MAC in reset except the Host Interface."]
    #[inline(always)]
    pub fn softreset(&self) -> SoftresetR {
        SoftresetR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RECEIVE ENABLE. Set this to allow receive frames to be received. Internally the MAC synchronizes this control bit to the incoming receive stream."]
    #[inline(always)]
    pub fn rxenable(&mut self) -> RxenableW<'_, Mac1Spec> {
        RxenableW::new(self, 0)
    }
    #[doc = "Bit 1 - PASS ALL RECEIVE FRAMES. When enabled (set to 1), the MAC will pass all frames regardless of type (normal vs. Control). When disabled, the MAC does not pass valid Control frames."]
    #[inline(always)]
    pub fn parf(&mut self) -> ParfW<'_, Mac1Spec> {
        ParfW::new(self, 1)
    }
    #[doc = "Bit 2 - RX FLOW CONTROL. When enabled (set to 1), the MAC acts upon received PAUSE Flow Control frames. When disabled, received PAUSE Flow Control frames are ignored."]
    #[inline(always)]
    pub fn rxflowctrl(&mut self) -> RxflowctrlW<'_, Mac1Spec> {
        RxflowctrlW::new(self, 2)
    }
    #[doc = "Bit 3 - TX FLOW CONTROL. When enabled (set to 1), PAUSE Flow Control frames are allowed to be transmitted. When disabled, Flow Control frames are blocked."]
    #[inline(always)]
    pub fn txflowctrl(&mut self) -> TxflowctrlW<'_, Mac1Spec> {
        TxflowctrlW::new(self, 3)
    }
    #[doc = "Bit 4 - Setting this bit will cause the MAC Transmit interface to be looped back to the MAC Receive interface. Clearing this bit results in normal operation."]
    #[inline(always)]
    pub fn loopback(&mut self) -> LoopbackW<'_, Mac1Spec> {
        LoopbackW::new(self, 4)
    }
    #[doc = "Bit 8 - Setting this bit will put the Transmit Function logic in reset."]
    #[inline(always)]
    pub fn resettx(&mut self) -> ResettxW<'_, Mac1Spec> {
        ResettxW::new(self, 8)
    }
    #[doc = "Bit 9 - Setting this bit resets the MAC Control Sublayer / Transmit logic. The MCS logic implements flow control."]
    #[inline(always)]
    pub fn resetmcstx(&mut self) -> ResetmcstxW<'_, Mac1Spec> {
        ResetmcstxW::new(self, 9)
    }
    #[doc = "Bit 10 - Setting this bit will put the Ethernet receive logic in reset."]
    #[inline(always)]
    pub fn resetrx(&mut self) -> ResetrxW<'_, Mac1Spec> {
        ResetrxW::new(self, 10)
    }
    #[doc = "Bit 11 - Setting this bit resets the MAC Control Sublayer / Receive logic. The MCS logic implements flow control."]
    #[inline(always)]
    pub fn resetmcsrx(&mut self) -> ResetmcsrxW<'_, Mac1Spec> {
        ResetmcsrxW::new(self, 11)
    }
    #[doc = "Bit 14 - SIMULATION RESET. Setting this bit will cause a reset to the random number generator within the Transmit Function."]
    #[inline(always)]
    pub fn simreset(&mut self) -> SimresetW<'_, Mac1Spec> {
        SimresetW::new(self, 14)
    }
    #[doc = "Bit 15 - SOFT RESET. Setting this bit will put all modules within the MAC in reset except the Host Interface."]
    #[inline(always)]
    pub fn softreset(&mut self) -> SoftresetW<'_, Mac1Spec> {
        SoftresetW::new(self, 15)
    }
}
#[doc = "MAC configuration register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`mac1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mac1Spec;
impl crate::RegisterSpec for Mac1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac1::R`](R) reader structure"]
impl crate::Readable for Mac1Spec {}
#[doc = "`write(|w| ..)` method takes [`mac1::W`](W) writer structure"]
impl crate::Writable for Mac1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAC1 to value 0x8000"]
impl crate::Resettable for Mac1Spec {
    const RESET_VALUE: u32 = 0x8000;
}
