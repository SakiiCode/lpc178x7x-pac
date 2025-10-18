#[doc = "Register `DYNAMICRP` reader"]
pub type R = crate::R<DynamicrpSpec>;
#[doc = "Register `DYNAMICRP` writer"]
pub type W = crate::W<DynamicrpSpec>;
#[doc = "Field `TRP` reader - Precharge command period. 0x0 - 0xE = n + 1 clock cycles. The delay is in CCLK cycles. 0xF = 16 clock cycles (POR reset value)."]
pub type TrpR = crate::FieldReader;
#[doc = "Field `TRP` writer - Precharge command period. 0x0 - 0xE = n + 1 clock cycles. The delay is in CCLK cycles. 0xF = 16 clock cycles (POR reset value)."]
pub type TrpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Precharge command period. 0x0 - 0xE = n + 1 clock cycles. The delay is in CCLK cycles. 0xF = 16 clock cycles (POR reset value)."]
    #[inline(always)]
    pub fn trp(&self) -> TrpR {
        TrpR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Precharge command period. 0x0 - 0xE = n + 1 clock cycles. The delay is in CCLK cycles. 0xF = 16 clock cycles (POR reset value)."]
    #[inline(always)]
    pub fn trp(&mut self) -> TrpW<'_, DynamicrpSpec> {
        TrpW::new(self, 0)
    }
}
#[doc = "Precharge command period.\n\nYou can [`read`](crate::Reg::read) this register and get [`dynamicrp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dynamicrp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DynamicrpSpec;
impl crate::RegisterSpec for DynamicrpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dynamicrp::R`](R) reader structure"]
impl crate::Readable for DynamicrpSpec {}
#[doc = "`write(|w| ..)` method takes [`dynamicrp::W`](W) writer structure"]
impl crate::Writable for DynamicrpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DYNAMICRP to value 0x0f"]
impl crate::Resettable for DynamicrpSpec {
    const RESET_VALUE: u32 = 0x0f;
}
