#[doc = "Register `MAXPOS` reader"]
pub type R = crate::R<MaxposSpec>;
#[doc = "Register `MAXPOS` writer"]
pub type W = crate::W<MaxposSpec>;
#[doc = "Field `MAXPOS` reader - Current maximum position value."]
pub type MaxposR = crate::FieldReader<u32>;
#[doc = "Field `MAXPOS` writer - Current maximum position value."]
pub type MaxposW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Current maximum position value."]
    #[inline(always)]
    pub fn maxpos(&self) -> MaxposR {
        MaxposR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Current maximum position value."]
    #[inline(always)]
    pub fn maxpos(&mut self) -> MaxposW<'_, MaxposSpec> {
        MaxposW::new(self, 0)
    }
}
#[doc = "Maximum position register\n\nYou can [`read`](crate::Reg::read) this register and get [`maxpos::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxpos::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxposSpec;
impl crate::RegisterSpec for MaxposSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maxpos::R`](R) reader structure"]
impl crate::Readable for MaxposSpec {}
#[doc = "`write(|w| ..)` method takes [`maxpos::W`](W) writer structure"]
impl crate::Writable for MaxposSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAXPOS to value 0"]
impl crate::Resettable for MaxposSpec {}
