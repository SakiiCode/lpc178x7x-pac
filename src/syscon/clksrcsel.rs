#[doc = "Register `CLKSRCSEL` reader"]
pub type R = crate::R<ClksrcselSpec>;
#[doc = "Register `CLKSRCSEL` writer"]
pub type W = crate::W<ClksrcselSpec>;
#[doc = "Selects the clock source for sysclk and PLL0 as follows:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clksrc {
    #[doc = "0: Selects the Internal RC oscillator as the sysclk and PLL0 clock source (default)."]
    SelectsTheInternal = 0,
    #[doc = "1: Selects the main oscillator as the sysclk and PLL0 clock source."]
    SelectsTheMainOsc = 1,
}
impl From<Clksrc> for bool {
    #[inline(always)]
    fn from(variant: Clksrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKSRC` reader - Selects the clock source for sysclk and PLL0 as follows:"]
pub type ClksrcR = crate::BitReader<Clksrc>;
impl ClksrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clksrc {
        match self.bits {
            false => Clksrc::SelectsTheInternal,
            true => Clksrc::SelectsTheMainOsc,
        }
    }
    #[doc = "Selects the Internal RC oscillator as the sysclk and PLL0 clock source (default)."]
    #[inline(always)]
    pub fn is_selects_the_internal(&self) -> bool {
        *self == Clksrc::SelectsTheInternal
    }
    #[doc = "Selects the main oscillator as the sysclk and PLL0 clock source."]
    #[inline(always)]
    pub fn is_selects_the_main_osc(&self) -> bool {
        *self == Clksrc::SelectsTheMainOsc
    }
}
#[doc = "Field `CLKSRC` writer - Selects the clock source for sysclk and PLL0 as follows:"]
pub type ClksrcW<'a, REG> = crate::BitWriter<'a, REG, Clksrc>;
impl<'a, REG> ClksrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Selects the Internal RC oscillator as the sysclk and PLL0 clock source (default)."]
    #[inline(always)]
    pub fn selects_the_internal(self) -> &'a mut crate::W<REG> {
        self.variant(Clksrc::SelectsTheInternal)
    }
    #[doc = "Selects the main oscillator as the sysclk and PLL0 clock source."]
    #[inline(always)]
    pub fn selects_the_main_osc(self) -> &'a mut crate::W<REG> {
        self.variant(Clksrc::SelectsTheMainOsc)
    }
}
impl R {
    #[doc = "Bit 0 - Selects the clock source for sysclk and PLL0 as follows:"]
    #[inline(always)]
    pub fn clksrc(&self) -> ClksrcR {
        ClksrcR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects the clock source for sysclk and PLL0 as follows:"]
    #[inline(always)]
    pub fn clksrc(&mut self) -> ClksrcW<'_, ClksrcselSpec> {
        ClksrcW::new(self, 0)
    }
}
#[doc = "Clock Source Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clksrcsel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clksrcsel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClksrcselSpec;
impl crate::RegisterSpec for ClksrcselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clksrcsel::R`](R) reader structure"]
impl crate::Readable for ClksrcselSpec {}
#[doc = "`write(|w| ..)` method takes [`clksrcsel::W`](W) writer structure"]
impl crate::Writable for ClksrcselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKSRCSEL to value 0"]
impl crate::Resettable for ClksrcselSpec {}
