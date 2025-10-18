#[doc = "Register `AMON` reader"]
pub type R = crate::R<AmonSpec>;
#[doc = "Register `AMON` writer"]
pub type W = crate::W<AmonSpec>;
#[doc = "Field `MONTH` reader - Month value in the range of 1 to 12."]
pub type MonthR = crate::FieldReader;
#[doc = "Field `MONTH` writer - Month value in the range of 1 to 12."]
pub type MonthW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Month value in the range of 1 to 12."]
    #[inline(always)]
    pub fn month(&self) -> MonthR {
        MonthR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Month value in the range of 1 to 12."]
    #[inline(always)]
    pub fn month(&mut self) -> MonthW<'_, AmonSpec> {
        MonthW::new(self, 0)
    }
}
#[doc = "Alarm value for Months\n\nYou can [`read`](crate::Reg::read) this register and get [`amon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AmonSpec;
impl crate::RegisterSpec for AmonSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`amon::R`](R) reader structure"]
impl crate::Readable for AmonSpec {}
#[doc = "`write(|w| ..)` method takes [`amon::W`](W) writer structure"]
impl crate::Writable for AmonSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AMON to value 0"]
impl crate::Resettable for AmonSpec {}
