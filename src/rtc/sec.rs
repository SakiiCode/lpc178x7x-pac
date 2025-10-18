#[doc = "Register `SEC` reader"]
pub type R = crate::R<SecSpec>;
#[doc = "Register `SEC` writer"]
pub type W = crate::W<SecSpec>;
#[doc = "Field `SECONDS` reader - Seconds value in the range of 0 to 59"]
pub type SecondsR = crate::FieldReader;
#[doc = "Field `SECONDS` writer - Seconds value in the range of 0 to 59"]
pub type SecondsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Seconds value in the range of 0 to 59"]
    #[inline(always)]
    pub fn seconds(&self) -> SecondsR {
        SecondsR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Seconds value in the range of 0 to 59"]
    #[inline(always)]
    pub fn seconds(&mut self) -> SecondsW<'_, SecSpec> {
        SecondsW::new(self, 0)
    }
}
#[doc = "Seconds Counter\n\nYou can [`read`](crate::Reg::read) this register and get [`sec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecSpec;
impl crate::RegisterSpec for SecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec::R`](R) reader structure"]
impl crate::Readable for SecSpec {}
#[doc = "`write(|w| ..)` method takes [`sec::W`](W) writer structure"]
impl crate::Writable for SecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SEC to value 0"]
impl crate::Resettable for SecSpec {}
