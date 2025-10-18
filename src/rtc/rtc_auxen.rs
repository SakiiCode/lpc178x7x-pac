#[doc = "Register `RTC_AUXEN` reader"]
pub type R = crate::R<RtcAuxenSpec>;
#[doc = "Register `RTC_AUXEN` writer"]
pub type W = crate::W<RtcAuxenSpec>;
#[doc = "Field `RTC_OSCFEN` reader - Oscillator Fail Detect interrupt enable. When 0: the RTC Oscillator Fail detect interrupt is disabled. When 1: the RTC Oscillator Fail detect interrupt is enabled. See Section 30.6.2.5."]
pub type RtcOscfenR = crate::BitReader;
#[doc = "Field `RTC_OSCFEN` writer - Oscillator Fail Detect interrupt enable. When 0: the RTC Oscillator Fail detect interrupt is disabled. When 1: the RTC Oscillator Fail detect interrupt is enabled. See Section 30.6.2.5."]
pub type RtcOscfenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 4 - Oscillator Fail Detect interrupt enable. When 0: the RTC Oscillator Fail detect interrupt is disabled. When 1: the RTC Oscillator Fail detect interrupt is enabled. See Section 30.6.2.5."]
    #[inline(always)]
    pub fn rtc_oscfen(&self) -> RtcOscfenR {
        RtcOscfenR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Oscillator Fail Detect interrupt enable. When 0: the RTC Oscillator Fail detect interrupt is disabled. When 1: the RTC Oscillator Fail detect interrupt is enabled. See Section 30.6.2.5."]
    #[inline(always)]
    pub fn rtc_oscfen(&mut self) -> RtcOscfenW<'_, RtcAuxenSpec> {
        RtcOscfenW::new(self, 4)
    }
}
#[doc = "RTC Auxiliary Enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_auxen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_auxen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcAuxenSpec;
impl crate::RegisterSpec for RtcAuxenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_auxen::R`](R) reader structure"]
impl crate::Readable for RtcAuxenSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_auxen::W`](W) writer structure"]
impl crate::Writable for RtcAuxenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_AUXEN to value 0"]
impl crate::Resettable for RtcAuxenSpec {}
