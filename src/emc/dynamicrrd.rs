#[doc = "Register `DYNAMICRRD` reader"]
pub type R = crate::R<DynamicrrdSpec>;
#[doc = "Register `DYNAMICRRD` writer"]
pub type W = crate::W<DynamicrrdSpec>;
#[doc = "Field `TRRD` reader - Active bank A to active bank B latency 0x0 - 0xE = n + 1 clock cycles. The delay is in CCLK cycles. 0xF = 16 clock cycles (POR reset value)."]
pub type TrrdR = crate::FieldReader;
#[doc = "Field `TRRD` writer - Active bank A to active bank B latency 0x0 - 0xE = n + 1 clock cycles. The delay is in CCLK cycles. 0xF = 16 clock cycles (POR reset value)."]
pub type TrrdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Active bank A to active bank B latency 0x0 - 0xE = n + 1 clock cycles. The delay is in CCLK cycles. 0xF = 16 clock cycles (POR reset value)."]
    #[inline(always)]
    pub fn trrd(&self) -> TrrdR {
        TrrdR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Active bank A to active bank B latency 0x0 - 0xE = n + 1 clock cycles. The delay is in CCLK cycles. 0xF = 16 clock cycles (POR reset value)."]
    #[inline(always)]
    pub fn trrd(&mut self) -> TrrdW<'_, DynamicrrdSpec> {
        TrrdW::new(self, 0)
    }
}
#[doc = "Latency for active bank A to active bank B.\n\nYou can [`read`](crate::Reg::read) this register and get [`dynamicrrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dynamicrrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DynamicrrdSpec;
impl crate::RegisterSpec for DynamicrrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dynamicrrd::R`](R) reader structure"]
impl crate::Readable for DynamicrrdSpec {}
#[doc = "`write(|w| ..)` method takes [`dynamicrrd::W`](W) writer structure"]
impl crate::Writable for DynamicrrdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DYNAMICRRD to value 0x0f"]
impl crate::Resettable for DynamicrrdSpec {
    const RESET_VALUE: u32 = 0x0f;
}
