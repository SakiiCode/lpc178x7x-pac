#[doc = "Register `LIM[%s]` reader"]
pub type R = crate::R<LimSpec>;
#[doc = "Register `LIM[%s]` writer"]
pub type W = crate::W<LimSpec>;
#[doc = "Field `MCLIM` reader - Limit value."]
pub type MclimR = crate::FieldReader<u32>;
#[doc = "Field `MCLIM` writer - Limit value."]
pub type MclimW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Limit value."]
    #[inline(always)]
    pub fn mclim(&self) -> MclimR {
        MclimR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Limit value."]
    #[inline(always)]
    pub fn mclim(&mut self) -> MclimW<'_, LimSpec> {
        MclimW::new(self, 0)
    }
}
#[doc = "Limit register\n\nYou can [`read`](crate::Reg::read) this register and get [`lim::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lim::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LimSpec;
impl crate::RegisterSpec for LimSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lim::R`](R) reader structure"]
impl crate::Readable for LimSpec {}
#[doc = "`write(|w| ..)` method takes [`lim::W`](W) writer structure"]
impl crate::Writable for LimSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LIM[%s] to value 0"]
impl crate::Resettable for LimSpec {}
