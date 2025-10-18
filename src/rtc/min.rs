#[doc = "Register `MIN` reader"]
pub type R = crate::R<MinSpec>;
#[doc = "Register `MIN` writer"]
pub type W = crate::W<MinSpec>;
#[doc = "Field `MINUTES` reader - Minutes value in the range of 0 to 59"]
pub type MinutesR = crate::FieldReader;
#[doc = "Field `MINUTES` writer - Minutes value in the range of 0 to 59"]
pub type MinutesW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Minutes value in the range of 0 to 59"]
    #[inline(always)]
    pub fn minutes(&self) -> MinutesR {
        MinutesR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Minutes value in the range of 0 to 59"]
    #[inline(always)]
    pub fn minutes(&mut self) -> MinutesW<'_, MinSpec> {
        MinutesW::new(self, 0)
    }
}
#[doc = "Minutes Register\n\nYou can [`read`](crate::Reg::read) this register and get [`min::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`min::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MinSpec;
impl crate::RegisterSpec for MinSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`min::R`](R) reader structure"]
impl crate::Readable for MinSpec {}
#[doc = "`write(|w| ..)` method takes [`min::W`](W) writer structure"]
impl crate::Writable for MinSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MIN to value 0"]
impl crate::Resettable for MinSpec {}
