#[doc = "Register `AHRS` reader"]
pub type R = crate::R<AhrsSpec>;
#[doc = "Register `AHRS` writer"]
pub type W = crate::W<AhrsSpec>;
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
    pub fn hours(&mut self) -> HoursW<'_, AhrsSpec> {
        HoursW::new(self, 0)
    }
}
#[doc = "Alarm value for Hours\n\nYou can [`read`](crate::Reg::read) this register and get [`ahrs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahrs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhrsSpec;
impl crate::RegisterSpec for AhrsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahrs::R`](R) reader structure"]
impl crate::Readable for AhrsSpec {}
#[doc = "`write(|w| ..)` method takes [`ahrs::W`](W) writer structure"]
impl crate::Writable for AhrsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHRS to value 0"]
impl crate::Resettable for AhrsSpec {}
