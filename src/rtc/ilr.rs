#[doc = "Register `ILR` reader"]
pub type R = crate::R<IlrSpec>;
#[doc = "Register `ILR` writer"]
pub type W = crate::W<IlrSpec>;
#[doc = "Field `RTCCIF` reader - When one, the Counter Increment Interrupt block generated an interrupt. Writing a one to this bit location clears the counter increment interrupt."]
pub type RtccifR = crate::BitReader;
#[doc = "Field `RTCCIF` writer - When one, the Counter Increment Interrupt block generated an interrupt. Writing a one to this bit location clears the counter increment interrupt."]
pub type RtccifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCALF` reader - When one, the alarm registers generated an interrupt. Writing a one to this bit location clears the alarm interrupt."]
pub type RtcalfR = crate::BitReader;
#[doc = "Field `RTCALF` writer - When one, the alarm registers generated an interrupt. Writing a one to this bit location clears the alarm interrupt."]
pub type RtcalfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When one, the Counter Increment Interrupt block generated an interrupt. Writing a one to this bit location clears the counter increment interrupt."]
    #[inline(always)]
    pub fn rtccif(&self) -> RtccifR {
        RtccifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When one, the alarm registers generated an interrupt. Writing a one to this bit location clears the alarm interrupt."]
    #[inline(always)]
    pub fn rtcalf(&self) -> RtcalfR {
        RtcalfR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When one, the Counter Increment Interrupt block generated an interrupt. Writing a one to this bit location clears the counter increment interrupt."]
    #[inline(always)]
    pub fn rtccif(&mut self) -> RtccifW<'_, IlrSpec> {
        RtccifW::new(self, 0)
    }
    #[doc = "Bit 1 - When one, the alarm registers generated an interrupt. Writing a one to this bit location clears the alarm interrupt."]
    #[inline(always)]
    pub fn rtcalf(&mut self) -> RtcalfW<'_, IlrSpec> {
        RtcalfW::new(self, 1)
    }
}
#[doc = "Interrupt Location Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ilr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ilr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IlrSpec;
impl crate::RegisterSpec for IlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ilr::R`](R) reader structure"]
impl crate::Readable for IlrSpec {}
#[doc = "`write(|w| ..)` method takes [`ilr::W`](W) writer structure"]
impl crate::Writable for IlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ILR to value 0"]
impl crate::Resettable for IlrSpec {}
