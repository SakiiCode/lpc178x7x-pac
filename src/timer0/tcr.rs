#[doc = "Register `TCR` reader"]
pub type R = crate::R<TcrSpec>;
#[doc = "Register `TCR` writer"]
pub type W = crate::W<TcrSpec>;
#[doc = "Field `CEN` reader - When one, the Timer Counter and Prescale Counter are enabled for counting. When zero, the counters are disabled."]
pub type CenR = crate::BitReader;
#[doc = "Field `CEN` writer - When one, the Timer Counter and Prescale Counter are enabled for counting. When zero, the counters are disabled."]
pub type CenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRST` reader - When one, the Timer Counter and the Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until TCR\\[1\\] is returned to zero."]
pub type CrstR = crate::BitReader;
#[doc = "Field `CRST` writer - When one, the Timer Counter and the Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until TCR\\[1\\] is returned to zero."]
pub type CrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When one, the Timer Counter and Prescale Counter are enabled for counting. When zero, the counters are disabled."]
    #[inline(always)]
    pub fn cen(&self) -> CenR {
        CenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When one, the Timer Counter and the Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until TCR\\[1\\] is returned to zero."]
    #[inline(always)]
    pub fn crst(&self) -> CrstR {
        CrstR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When one, the Timer Counter and Prescale Counter are enabled for counting. When zero, the counters are disabled."]
    #[inline(always)]
    pub fn cen(&mut self) -> CenW<'_, TcrSpec> {
        CenW::new(self, 0)
    }
    #[doc = "Bit 1 - When one, the Timer Counter and the Prescale Counter are synchronously reset on the next positive edge of PCLK. The counters remain reset until TCR\\[1\\] is returned to zero."]
    #[inline(always)]
    pub fn crst(&mut self) -> CrstW<'_, TcrSpec> {
        CrstW::new(self, 1)
    }
}
#[doc = "Timer Control Register. The TCR is used to control the Timer Counter functions. The Timer Counter can be disabled or reset through the TCR.\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcrSpec;
impl crate::RegisterSpec for TcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tcr::R`](R) reader structure"]
impl crate::Readable for TcrSpec {}
#[doc = "`write(|w| ..)` method takes [`tcr::W`](W) writer structure"]
impl crate::Writable for TcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TCR to value 0"]
impl crate::Resettable for TcrSpec {}
