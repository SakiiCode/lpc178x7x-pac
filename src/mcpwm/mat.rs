#[doc = "Register `MAT[%s]` reader"]
pub type R = crate::R<MatSpec>;
#[doc = "Register `MAT[%s]` writer"]
pub type W = crate::W<MatSpec>;
#[doc = "Field `MCMAT` reader - Match value."]
pub type McmatR = crate::FieldReader<u32>;
#[doc = "Field `MCMAT` writer - Match value."]
pub type McmatW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Match value."]
    #[inline(always)]
    pub fn mcmat(&self) -> McmatR {
        McmatR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Match value."]
    #[inline(always)]
    pub fn mcmat(&mut self) -> McmatW<'_, MatSpec> {
        McmatW::new(self, 0)
    }
}
#[doc = "Match register\n\nYou can [`read`](crate::Reg::read) this register and get [`mat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MatSpec;
impl crate::RegisterSpec for MatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mat::R`](R) reader structure"]
impl crate::Readable for MatSpec {}
#[doc = "`write(|w| ..)` method takes [`mat::W`](W) writer structure"]
impl crate::Writable for MatSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAT[%s] to value 0"]
impl crate::Resettable for MatSpec {}
