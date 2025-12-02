#[doc = "Register `EMCCLKSEL` reader"]
pub type R = crate::R<EmcclkselSpec>;
#[doc = "Register `EMCCLKSEL` writer"]
pub type W = crate::W<EmcclkselSpec>;
#[doc = "Selects the EMC clock rate relative to the CPU clock.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Emcdiv {
    #[doc = "0: The EMC uses the same clock as the CPU."]
    FullRate = 0,
    #[doc = "1: The EMC uses a clock at half the rate of the CPU."]
    HalfRate = 1,
}
impl From<Emcdiv> for bool {
    #[inline(always)]
    fn from(variant: Emcdiv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMCDIV` reader - Selects the EMC clock rate relative to the CPU clock."]
pub type EmcdivR = crate::BitReader<Emcdiv>;
impl EmcdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Emcdiv {
        match self.bits {
            false => Emcdiv::FullRate,
            true => Emcdiv::HalfRate,
        }
    }
    #[doc = "The EMC uses the same clock as the CPU."]
    #[inline(always)]
    pub fn is_full_rate(&self) -> bool {
        *self == Emcdiv::FullRate
    }
    #[doc = "The EMC uses a clock at half the rate of the CPU."]
    #[inline(always)]
    pub fn is_half_rate(&self) -> bool {
        *self == Emcdiv::HalfRate
    }
}
#[doc = "Field `EMCDIV` writer - Selects the EMC clock rate relative to the CPU clock."]
pub type EmcdivW<'a, REG> = crate::BitWriter<'a, REG, Emcdiv>;
impl<'a, REG> EmcdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The EMC uses the same clock as the CPU."]
    #[inline(always)]
    pub fn full_rate(self) -> &'a mut crate::W<REG> {
        self.variant(Emcdiv::FullRate)
    }
    #[doc = "The EMC uses a clock at half the rate of the CPU."]
    #[inline(always)]
    pub fn half_rate(self) -> &'a mut crate::W<REG> {
        self.variant(Emcdiv::HalfRate)
    }
}
impl R {
    #[doc = "Bit 0 - Selects the EMC clock rate relative to the CPU clock."]
    #[inline(always)]
    pub fn emcdiv(&self) -> EmcdivR {
        EmcdivR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selects the EMC clock rate relative to the CPU clock."]
    #[inline(always)]
    pub fn emcdiv(&mut self) -> EmcdivW<'_, EmcclkselSpec> {
        EmcdivW::new(self, 0)
    }
}
#[doc = "External Memory Controller Clock Selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`emcclksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emcclksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmcclkselSpec;
impl crate::RegisterSpec for EmcclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emcclksel::R`](R) reader structure"]
impl crate::Readable for EmcclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`emcclksel::W`](W) writer structure"]
impl crate::Writable for EmcclkselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMCCLKSEL to value 0"]
impl crate::Resettable for EmcclkselSpec {}
