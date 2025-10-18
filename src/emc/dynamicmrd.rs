#[doc = "Register `DYNAMICMRD` reader"]
pub type R = crate::R<DynamicmrdSpec>;
#[doc = "Register `DYNAMICMRD` writer"]
pub type W = crate::W<DynamicmrdSpec>;
#[doc = "Field `TMRD` reader - Load mode register to active command time. 0x0 - 0xE = n + 1 clock cycles. The delay is in CCLK cycles. 0xF = 16 clock cycles (POR reset value)."]
pub type TmrdR = crate::FieldReader;
#[doc = "Field `TMRD` writer - Load mode register to active command time. 0x0 - 0xE = n + 1 clock cycles. The delay is in CCLK cycles. 0xF = 16 clock cycles (POR reset value)."]
pub type TmrdW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Load mode register to active command time. 0x0 - 0xE = n + 1 clock cycles. The delay is in CCLK cycles. 0xF = 16 clock cycles (POR reset value)."]
    #[inline(always)]
    pub fn tmrd(&self) -> TmrdR {
        TmrdR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Load mode register to active command time. 0x0 - 0xE = n + 1 clock cycles. The delay is in CCLK cycles. 0xF = 16 clock cycles (POR reset value)."]
    #[inline(always)]
    pub fn tmrd(&mut self) -> TmrdW<'_, DynamicmrdSpec> {
        TmrdW::new(self, 0)
    }
}
#[doc = "Time for load mode register to active command.\n\nYou can [`read`](crate::Reg::read) this register and get [`dynamicmrd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dynamicmrd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DynamicmrdSpec;
impl crate::RegisterSpec for DynamicmrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dynamicmrd::R`](R) reader structure"]
impl crate::Readable for DynamicmrdSpec {}
#[doc = "`write(|w| ..)` method takes [`dynamicmrd::W`](W) writer structure"]
impl crate::Writable for DynamicmrdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DYNAMICMRD to value 0x0f"]
impl crate::Resettable for DynamicmrdSpec {
    const RESET_VALUE: u32 = 0x0f;
}
