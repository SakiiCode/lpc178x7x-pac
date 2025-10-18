#[doc = "Register `ADOW` reader"]
pub type R = crate::R<AdowSpec>;
#[doc = "Register `ADOW` writer"]
pub type W = crate::W<AdowSpec>;
#[doc = "Field `DOW` reader - Day of week value in the range of 0 to 6."]
pub type DowR = crate::FieldReader;
#[doc = "Field `DOW` writer - Day of week value in the range of 0 to 6."]
pub type DowW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Day of week value in the range of 0 to 6."]
    #[inline(always)]
    pub fn dow(&self) -> DowR {
        DowR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Day of week value in the range of 0 to 6."]
    #[inline(always)]
    pub fn dow(&mut self) -> DowW<'_, AdowSpec> {
        DowW::new(self, 0)
    }
}
#[doc = "Alarm value for Day of Week\n\nYou can [`read`](crate::Reg::read) this register and get [`adow::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adow::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdowSpec;
impl crate::RegisterSpec for AdowSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adow::R`](R) reader structure"]
impl crate::Readable for AdowSpec {}
#[doc = "`write(|w| ..)` method takes [`adow::W`](W) writer structure"]
impl crate::Writable for AdowSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADOW to value 0"]
impl crate::Resettable for AdowSpec {}
