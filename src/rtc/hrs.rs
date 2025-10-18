#[doc = "Register `HRS` reader"]
pub type R = crate::R<HrsSpec>;
#[doc = "Register `HRS` writer"]
pub type W = crate::W<HrsSpec>;
#[doc = "Field `HOURS` reader - Hours value in the range of 0 to 23"]
pub type HoursR = crate::FieldReader;
#[doc = "Field `HOURS` writer - Hours value in the range of 0 to 23"]
pub type HoursW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Hours value in the range of 0 to 23"]
    #[inline(always)]
    pub fn hours(&self) -> HoursR {
        HoursR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Hours value in the range of 0 to 23"]
    #[inline(always)]
    pub fn hours(&mut self) -> HoursW<'_, HrsSpec> {
        HoursW::new(self, 0)
    }
}
#[doc = "Hours Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hrs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hrs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HrsSpec;
impl crate::RegisterSpec for HrsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hrs::R`](R) reader structure"]
impl crate::Readable for HrsSpec {}
#[doc = "`write(|w| ..)` method takes [`hrs::W`](W) writer structure"]
impl crate::Writable for HrsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HRS to value 0"]
impl crate::Resettable for HrsSpec {}
