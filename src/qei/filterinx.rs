#[doc = "Register `FILTERINX` reader"]
pub type R = crate::R<FilterinxSpec>;
#[doc = "Register `FILTERINX` writer"]
pub type W = crate::W<FilterinxSpec>;
#[doc = "Field `FITLINX` reader - Digital filter sampling delay for the index."]
pub type FitlinxR = crate::FieldReader<u32>;
#[doc = "Field `FITLINX` writer - Digital filter sampling delay for the index."]
pub type FitlinxW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Digital filter sampling delay for the index."]
    #[inline(always)]
    pub fn fitlinx(&self) -> FitlinxR {
        FitlinxR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Digital filter sampling delay for the index."]
    #[inline(always)]
    pub fn fitlinx(&mut self) -> FitlinxW<'_, FilterinxSpec> {
        FitlinxW::new(self, 0)
    }
}
#[doc = "Digital filter register on IDX\n\nYou can [`read`](crate::Reg::read) this register and get [`filterinx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`filterinx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FilterinxSpec;
impl crate::RegisterSpec for FilterinxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`filterinx::R`](R) reader structure"]
impl crate::Readable for FilterinxSpec {}
#[doc = "`write(|w| ..)` method takes [`filterinx::W`](W) writer structure"]
impl crate::Writable for FilterinxSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FILTERINX to value 0"]
impl crate::Resettable for FilterinxSpec {}
