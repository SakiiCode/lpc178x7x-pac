#[doc = "Register `DEVINTSET` writer"]
pub type W = crate::W<DevintsetSpec>;
#[doc = "Field `FRAMESET` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
pub type FramesetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP_FASTSET` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
pub type EpFastsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP_SLOWSET` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
pub type EpSlowsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEV_STATSET` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
pub type DevStatsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCEMPTYSET` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
pub type CcemptysetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDFULLSET` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
pub type CdfullsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RxENDPKTSET` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
pub type RxEndpktsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TxENDPKTSET` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
pub type TxEndpktsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EP_RLZEDSET` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
pub type EpRlzedsetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_INTSET` writer - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
pub type ErrIntsetW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
    #[inline(always)]
    pub fn frameset(&mut self) -> FramesetW<'_, DevintsetSpec> {
        FramesetW::new(self, 0)
    }
    #[doc = "Bit 1 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
    #[inline(always)]
    pub fn ep_fastset(&mut self) -> EpFastsetW<'_, DevintsetSpec> {
        EpFastsetW::new(self, 1)
    }
    #[doc = "Bit 2 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
    #[inline(always)]
    pub fn ep_slowset(&mut self) -> EpSlowsetW<'_, DevintsetSpec> {
        EpSlowsetW::new(self, 2)
    }
    #[doc = "Bit 3 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
    #[inline(always)]
    pub fn dev_statset(&mut self) -> DevStatsetW<'_, DevintsetSpec> {
        DevStatsetW::new(self, 3)
    }
    #[doc = "Bit 4 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
    #[inline(always)]
    pub fn ccemptyset(&mut self) -> CcemptysetW<'_, DevintsetSpec> {
        CcemptysetW::new(self, 4)
    }
    #[doc = "Bit 5 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
    #[inline(always)]
    pub fn cdfullset(&mut self) -> CdfullsetW<'_, DevintsetSpec> {
        CdfullsetW::new(self, 5)
    }
    #[doc = "Bit 6 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
    #[inline(always)]
    pub fn rx_endpktset(&mut self) -> RxEndpktsetW<'_, DevintsetSpec> {
        RxEndpktsetW::new(self, 6)
    }
    #[doc = "Bit 7 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
    #[inline(always)]
    pub fn tx_endpktset(&mut self) -> TxEndpktsetW<'_, DevintsetSpec> {
        TxEndpktsetW::new(self, 7)
    }
    #[doc = "Bit 8 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
    #[inline(always)]
    pub fn ep_rlzedset(&mut self) -> EpRlzedsetW<'_, DevintsetSpec> {
        EpRlzedsetW::new(self, 8)
    }
    #[doc = "Bit 9 - 0 = No effect. 1 = The corresponding bit in USBDevIntSt (Section 13.10.3.2) is set."]
    #[inline(always)]
    pub fn err_intset(&mut self) -> ErrIntsetW<'_, DevintsetSpec> {
        ErrIntsetW::new(self, 9)
    }
}
#[doc = "USB Device Interrupt Set\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devintset::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevintsetSpec;
impl crate::RegisterSpec for DevintsetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`devintset::W`](W) writer structure"]
impl crate::Writable for DevintsetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DEVINTSET to value 0"]
impl crate::Resettable for DevintsetSpec {}
