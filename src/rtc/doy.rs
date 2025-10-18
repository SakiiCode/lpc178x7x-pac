#[doc = "Register `DOY` reader"]
pub type R = crate::R<DoySpec>;
#[doc = "Register `DOY` writer"]
pub type W = crate::W<DoySpec>;
#[doc = "Field `DOY` reader - Day of year value in the range of 1 to 365 (366 for leap years)."]
pub type DoyR = crate::FieldReader<u16>;
#[doc = "Field `DOY` writer - Day of year value in the range of 1 to 365 (366 for leap years)."]
pub type DoyW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - Day of year value in the range of 1 to 365 (366 for leap years)."]
    #[inline(always)]
    pub fn doy(&self) -> DoyR {
        DoyR::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - Day of year value in the range of 1 to 365 (366 for leap years)."]
    #[inline(always)]
    pub fn doy(&mut self) -> DoyW<'_, DoySpec> {
        DoyW::new(self, 0)
    }
}
#[doc = "Day of Year Register\n\nYou can [`read`](crate::Reg::read) this register and get [`doy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`doy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoySpec;
impl crate::RegisterSpec for DoySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doy::R`](R) reader structure"]
impl crate::Readable for DoySpec {}
#[doc = "`write(|w| ..)` method takes [`doy::W`](W) writer structure"]
impl crate::Writable for DoySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOY to value 0"]
impl crate::Resettable for DoySpec {}
