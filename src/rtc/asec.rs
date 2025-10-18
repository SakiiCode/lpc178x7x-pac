#[doc = "Register `ASEC` reader"]
pub type R = crate::R<AsecSpec>;
#[doc = "Register `ASEC` writer"]
pub type W = crate::W<AsecSpec>;
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
    pub fn seconds(&mut self) -> SecondsW<'_, AsecSpec> {
        SecondsW::new(self, 0)
    }
}
#[doc = "Alarm value for Seconds\n\nYou can [`read`](crate::Reg::read) this register and get [`asec::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`asec::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AsecSpec;
impl crate::RegisterSpec for AsecSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`asec::R`](R) reader structure"]
impl crate::Readable for AsecSpec {}
#[doc = "`write(|w| ..)` method takes [`asec::W`](W) writer structure"]
impl crate::Writable for AsecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ASEC to value 0"]
impl crate::Resettable for AsecSpec {}
