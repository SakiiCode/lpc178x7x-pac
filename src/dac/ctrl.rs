#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "DMA interrupt request\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntDmaReq {
    #[doc = "0: Clear on any write to the DACR register."]
    ClearOnWrite = 0,
    #[doc = "1: Set by hardware when the timer times out."]
    SetByHardware = 1,
}
impl From<IntDmaReq> for bool {
    #[inline(always)]
    fn from(variant: IntDmaReq) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INT_DMA_REQ` reader - DMA interrupt request"]
pub type IntDmaReqR = crate::BitReader<IntDmaReq>;
impl IntDmaReqR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IntDmaReq {
        match self.bits {
            false => IntDmaReq::ClearOnWrite,
            true => IntDmaReq::SetByHardware,
        }
    }
    #[doc = "Clear on any write to the DACR register."]
    #[inline(always)]
    pub fn is_clear_on_write(&self) -> bool {
        *self == IntDmaReq::ClearOnWrite
    }
    #[doc = "Set by hardware when the timer times out."]
    #[inline(always)]
    pub fn is_set_by_hardware(&self) -> bool {
        *self == IntDmaReq::SetByHardware
    }
}
#[doc = "Field `INT_DMA_REQ` writer - DMA interrupt request"]
pub type IntDmaReqW<'a, REG> = crate::BitWriter<'a, REG, IntDmaReq>;
impl<'a, REG> IntDmaReqW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clear on any write to the DACR register."]
    #[inline(always)]
    pub fn clear_on_write(self) -> &'a mut crate::W<REG> {
        self.variant(IntDmaReq::ClearOnWrite)
    }
    #[doc = "Set by hardware when the timer times out."]
    #[inline(always)]
    pub fn set_by_hardware(self) -> &'a mut crate::W<REG> {
        self.variant(IntDmaReq::SetByHardware)
    }
}
#[doc = "Double buffering\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DblbufEna {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable. When this bit and the CNT_ENA bit are both set, the double-buffering feature in the DACR register will be enabled. Writes to the DACR register are written to a pre-buffer and then transferred to the DACR on the next time-out of the counter."]
    Enable = 1,
}
impl From<DblbufEna> for bool {
    #[inline(always)]
    fn from(variant: DblbufEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DBLBUF_ENA` reader - Double buffering"]
pub type DblbufEnaR = crate::BitReader<DblbufEna>;
impl DblbufEnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DblbufEna {
        match self.bits {
            false => DblbufEna::Disable,
            true => DblbufEna::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DblbufEna::Disable
    }
    #[doc = "Enable. When this bit and the CNT_ENA bit are both set, the double-buffering feature in the DACR register will be enabled. Writes to the DACR register are written to a pre-buffer and then transferred to the DACR on the next time-out of the counter."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DblbufEna::Enable
    }
}
#[doc = "Field `DBLBUF_ENA` writer - Double buffering"]
pub type DblbufEnaW<'a, REG> = crate::BitWriter<'a, REG, DblbufEna>;
impl<'a, REG> DblbufEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DblbufEna::Disable)
    }
    #[doc = "Enable. When this bit and the CNT_ENA bit are both set, the double-buffering feature in the DACR register will be enabled. Writes to the DACR register are written to a pre-buffer and then transferred to the DACR on the next time-out of the counter."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DblbufEna::Enable)
    }
}
#[doc = "Time-out counter operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CntEna {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable"]
    Enable = 1,
}
impl From<CntEna> for bool {
    #[inline(always)]
    fn from(variant: CntEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNT_ENA` reader - Time-out counter operation"]
pub type CntEnaR = crate::BitReader<CntEna>;
impl CntEnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CntEna {
        match self.bits {
            false => CntEna::Disable,
            true => CntEna::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == CntEna::Disable
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == CntEna::Enable
    }
}
#[doc = "Field `CNT_ENA` writer - Time-out counter operation"]
pub type CntEnaW<'a, REG> = crate::BitWriter<'a, REG, CntEna>;
impl<'a, REG> CntEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(CntEna::Disable)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(CntEna::Enable)
    }
}
#[doc = "DMA access\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DmaEna {
    #[doc = "0: Disable"]
    Disable = 0,
    #[doc = "1: Enable. DMA Burst Request Input 7 is enabled for the DAC (see Table 672)."]
    Enable = 1,
}
impl From<DmaEna> for bool {
    #[inline(always)]
    fn from(variant: DmaEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMA_ENA` reader - DMA access"]
pub type DmaEnaR = crate::BitReader<DmaEna>;
impl DmaEnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DmaEna {
        match self.bits {
            false => DmaEna::Disable,
            true => DmaEna::Enable,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == DmaEna::Disable
    }
    #[doc = "Enable. DMA Burst Request Input 7 is enabled for the DAC (see Table 672)."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == DmaEna::Enable
    }
}
#[doc = "Field `DMA_ENA` writer - DMA access"]
pub type DmaEnaW<'a, REG> = crate::BitWriter<'a, REG, DmaEna>;
impl<'a, REG> DmaEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(DmaEna::Disable)
    }
    #[doc = "Enable. DMA Burst Request Input 7 is enabled for the DAC (see Table 672)."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(DmaEna::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - DMA interrupt request"]
    #[inline(always)]
    pub fn int_dma_req(&self) -> IntDmaReqR {
        IntDmaReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Double buffering"]
    #[inline(always)]
    pub fn dblbuf_ena(&self) -> DblbufEnaR {
        DblbufEnaR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Time-out counter operation"]
    #[inline(always)]
    pub fn cnt_ena(&self) -> CntEnaR {
        CntEnaR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DMA access"]
    #[inline(always)]
    pub fn dma_ena(&self) -> DmaEnaR {
        DmaEnaR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA interrupt request"]
    #[inline(always)]
    pub fn int_dma_req(&mut self) -> IntDmaReqW<'_, CtrlSpec> {
        IntDmaReqW::new(self, 0)
    }
    #[doc = "Bit 1 - Double buffering"]
    #[inline(always)]
    pub fn dblbuf_ena(&mut self) -> DblbufEnaW<'_, CtrlSpec> {
        DblbufEnaW::new(self, 1)
    }
    #[doc = "Bit 2 - Time-out counter operation"]
    #[inline(always)]
    pub fn cnt_ena(&mut self) -> CntEnaW<'_, CtrlSpec> {
        CntEnaW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA access"]
    #[inline(always)]
    pub fn dma_ena(&mut self) -> DmaEnaW<'_, CtrlSpec> {
        DmaEnaW::new(self, 3)
    }
}
#[doc = "DAC Control register. This register controls DMA and timer operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {}
