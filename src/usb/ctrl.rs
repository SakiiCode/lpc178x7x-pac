#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Read mode control. Enables reading data from the OUT endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBRxData register. This bit is cleared by hardware when the last word of the current packet is read from USBRxData.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RdEn {
    #[doc = "0: Disabled."]
    Disabled = 0,
    #[doc = "1: Enabled."]
    Enabled = 1,
}
impl From<RdEn> for bool {
    #[inline(always)]
    fn from(variant: RdEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RD_EN` reader - Read mode control. Enables reading data from the OUT endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBRxData register. This bit is cleared by hardware when the last word of the current packet is read from USBRxData."]
pub type RdEnR = crate::BitReader<RdEn>;
impl RdEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> RdEn {
        match self.bits {
            false => RdEn::Disabled,
            true => RdEn::Enabled,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RdEn::Disabled
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RdEn::Enabled
    }
}
#[doc = "Field `RD_EN` writer - Read mode control. Enables reading data from the OUT endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBRxData register. This bit is cleared by hardware when the last word of the current packet is read from USBRxData."]
pub type RdEnW<'a, REG> = crate::BitWriter<'a, REG, RdEn>;
impl<'a, REG> RdEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(RdEn::Disabled)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(RdEn::Enabled)
    }
}
#[doc = "Write mode control. Enables writing data to the IN endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBTxData register. This bit is cleared by hardware when the number of bytes in USBTxLen have been sent.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WrEn {
    #[doc = "0: Disabled."]
    Disabled = 0,
    #[doc = "1: Enabled."]
    Enabled = 1,
}
impl From<WrEn> for bool {
    #[inline(always)]
    fn from(variant: WrEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WR_EN` reader - Write mode control. Enables writing data to the IN endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBTxData register. This bit is cleared by hardware when the number of bytes in USBTxLen have been sent."]
pub type WrEnR = crate::BitReader<WrEn>;
impl WrEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WrEn {
        match self.bits {
            false => WrEn::Disabled,
            true => WrEn::Enabled,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WrEn::Disabled
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WrEn::Enabled
    }
}
#[doc = "Field `WR_EN` writer - Write mode control. Enables writing data to the IN endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBTxData register. This bit is cleared by hardware when the number of bytes in USBTxLen have been sent."]
pub type WrEnW<'a, REG> = crate::BitWriter<'a, REG, WrEn>;
impl<'a, REG> WrEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WrEn::Disabled)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(WrEn::Enabled)
    }
}
#[doc = "Field `LOG_ENDPOINT` reader - Logical Endpoint number."]
pub type LogEndpointR = crate::FieldReader;
#[doc = "Field `LOG_ENDPOINT` writer - Logical Endpoint number."]
pub type LogEndpointW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Read mode control. Enables reading data from the OUT endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBRxData register. This bit is cleared by hardware when the last word of the current packet is read from USBRxData."]
    #[inline(always)]
    pub fn rd_en(&self) -> RdEnR {
        RdEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write mode control. Enables writing data to the IN endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBTxData register. This bit is cleared by hardware when the number of bytes in USBTxLen have been sent."]
    #[inline(always)]
    pub fn wr_en(&self) -> WrEnR {
        WrEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:5 - Logical Endpoint number."]
    #[inline(always)]
    pub fn log_endpoint(&self) -> LogEndpointR {
        LogEndpointR::new(((self.bits >> 2) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Read mode control. Enables reading data from the OUT endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBRxData register. This bit is cleared by hardware when the last word of the current packet is read from USBRxData."]
    #[inline(always)]
    pub fn rd_en(&mut self) -> RdEnW<'_, CtrlSpec> {
        RdEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Write mode control. Enables writing data to the IN endpoint buffer for the endpoint specified in the LOG_ENDPOINT field using the USBTxData register. This bit is cleared by hardware when the number of bytes in USBTxLen have been sent."]
    #[inline(always)]
    pub fn wr_en(&mut self) -> WrEnW<'_, CtrlSpec> {
        WrEnW::new(self, 1)
    }
    #[doc = "Bits 2:5 - Logical Endpoint number."]
    #[inline(always)]
    pub fn log_endpoint(&mut self) -> LogEndpointW<'_, CtrlSpec> {
        LogEndpointW::new(self, 2)
    }
}
#[doc = "USB Control\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
