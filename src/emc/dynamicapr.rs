#[doc = "Register `DYNAMICAPR` reader"]
pub type R = crate::R<DynamicaprSpec>;
#[doc = "Register `DYNAMICAPR` writer"]
pub type W = crate::W<DynamicaprSpec>;
#[doc = "Field `TAPR` reader - Last-data-out to active command time. 0x0 - 0xE = n + 1 clock cycles. The delay is in CCLK cycles. 0xF = 16 clock cycles (POR reset value)."]
pub type TaprR = crate::FieldReader;
#[doc = "Field `TAPR` writer - Last-data-out to active command time. 0x0 - 0xE = n + 1 clock cycles. The delay is in CCLK cycles. 0xF = 16 clock cycles (POR reset value)."]
pub type TaprW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Last-data-out to active command time. 0x0 - 0xE = n + 1 clock cycles. The delay is in CCLK cycles. 0xF = 16 clock cycles (POR reset value)."]
    #[inline(always)]
    pub fn tapr(&self) -> TaprR {
        TaprR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Last-data-out to active command time. 0x0 - 0xE = n + 1 clock cycles. The delay is in CCLK cycles. 0xF = 16 clock cycles (POR reset value)."]
    #[inline(always)]
    pub fn tapr(&mut self) -> TaprW<'_, DynamicaprSpec> {
        TaprW::new(self, 0)
    }
}
#[doc = "Last-data-out to active command time.\n\nYou can [`read`](crate::Reg::read) this register and get [`dynamicapr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dynamicapr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DynamicaprSpec;
impl crate::RegisterSpec for DynamicaprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dynamicapr::R`](R) reader structure"]
impl crate::Readable for DynamicaprSpec {}
#[doc = "`write(|w| ..)` method takes [`dynamicapr::W`](W) writer structure"]
impl crate::Writable for DynamicaprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DYNAMICAPR to value 0x0f"]
impl crate::Resettable for DynamicaprSpec {
    const RESET_VALUE: u32 = 0x0f;
}
