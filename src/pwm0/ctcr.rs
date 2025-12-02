#[doc = "Register `CTCR` reader"]
pub type R = crate::R<CtcrSpec>;
#[doc = "Register `CTCR` writer"]
pub type W = crate::W<CtcrSpec>;
#[doc = "Counter/ Timer Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mod {
    #[doc = "0: Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale register."]
    Timer = 0,
    #[doc = "1: Rising edge counter Mode: the TC is incremented on rising edges of the PWM_CAP input selected by bits 3:2."]
    CounterRising = 1,
    #[doc = "2: Falling edge counter Mode: the TC is incremented on falling edges of the PWM_CAP input selected by bits 3:2."]
    CounterFalling = 2,
    #[doc = "3: Dual edge counter Mode: the TC is incremented on both edges of the PWM_CAP input selected by bits 3:2."]
    CounterDual = 3,
}
impl From<Mod> for u8 {
    #[inline(always)]
    fn from(variant: Mod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mod {
    type Ux = u8;
}
impl crate::IsEnum for Mod {}
#[doc = "Field `MOD` reader - Counter/ Timer Mode"]
pub type ModR = crate::FieldReader<Mod>;
impl ModR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mod {
        match self.bits {
            0 => Mod::Timer,
            1 => Mod::CounterRising,
            2 => Mod::CounterFalling,
            3 => Mod::CounterDual,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale register."]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == Mod::Timer
    }
    #[doc = "Rising edge counter Mode: the TC is incremented on rising edges of the PWM_CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn is_counter_rising(&self) -> bool {
        *self == Mod::CounterRising
    }
    #[doc = "Falling edge counter Mode: the TC is incremented on falling edges of the PWM_CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn is_counter_falling(&self) -> bool {
        *self == Mod::CounterFalling
    }
    #[doc = "Dual edge counter Mode: the TC is incremented on both edges of the PWM_CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn is_counter_dual(&self) -> bool {
        *self == Mod::CounterDual
    }
}
#[doc = "Field `MOD` writer - Counter/ Timer Mode"]
pub type ModW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mod, crate::Safe>;
impl<'a, REG> ModW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale register."]
    #[inline(always)]
    pub fn timer(self) -> &'a mut crate::W<REG> {
        self.variant(Mod::Timer)
    }
    #[doc = "Rising edge counter Mode: the TC is incremented on rising edges of the PWM_CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn counter_rising(self) -> &'a mut crate::W<REG> {
        self.variant(Mod::CounterRising)
    }
    #[doc = "Falling edge counter Mode: the TC is incremented on falling edges of the PWM_CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn counter_falling(self) -> &'a mut crate::W<REG> {
        self.variant(Mod::CounterFalling)
    }
    #[doc = "Dual edge counter Mode: the TC is incremented on both edges of the PWM_CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn counter_dual(self) -> &'a mut crate::W<REG> {
        self.variant(Mod::CounterDual)
    }
}
#[doc = "Count Input Select. When bits 1:0 are not 00, these bits select which PWM_CAP pin carries the signal used to increment the TC. Other combinations are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cis {
    #[doc = "0: For PWM0: 00 = PWM0_CAP0 (Other combinations are reserved) For PWM1: 00 = PWM1_CAP0, 01 = PWM1_CAP1 (Other combinations are reserved)"]
    Cap0 = 0,
}
impl From<Cis> for u8 {
    #[inline(always)]
    fn from(variant: Cis) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cis {
    type Ux = u8;
}
impl crate::IsEnum for Cis {}
#[doc = "Field `CIS` reader - Count Input Select. When bits 1:0 are not 00, these bits select which PWM_CAP pin carries the signal used to increment the TC. Other combinations are reserved."]
pub type CisR = crate::FieldReader<Cis>;
impl CisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Cis> {
        match self.bits {
            0 => Some(Cis::Cap0),
            _ => None,
        }
    }
    #[doc = "For PWM0: 00 = PWM0_CAP0 (Other combinations are reserved) For PWM1: 00 = PWM1_CAP0, 01 = PWM1_CAP1 (Other combinations are reserved)"]
    #[inline(always)]
    pub fn is_cap0(&self) -> bool {
        *self == Cis::Cap0
    }
}
#[doc = "Field `CIS` writer - Count Input Select. When bits 1:0 are not 00, these bits select which PWM_CAP pin carries the signal used to increment the TC. Other combinations are reserved."]
pub type CisW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cis>;
impl<'a, REG> CisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "For PWM0: 00 = PWM0_CAP0 (Other combinations are reserved) For PWM1: 00 = PWM1_CAP0, 01 = PWM1_CAP1 (Other combinations are reserved)"]
    #[inline(always)]
    pub fn cap0(self) -> &'a mut crate::W<REG> {
        self.variant(Cis::Cap0)
    }
}
impl R {
    #[doc = "Bits 0:1 - Counter/ Timer Mode"]
    #[inline(always)]
    pub fn mod_(&self) -> ModR {
        ModR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Count Input Select. When bits 1:0 are not 00, these bits select which PWM_CAP pin carries the signal used to increment the TC. Other combinations are reserved."]
    #[inline(always)]
    pub fn cis(&self) -> CisR {
        CisR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Counter/ Timer Mode"]
    #[inline(always)]
    pub fn mod_(&mut self) -> ModW<'_, CtcrSpec> {
        ModW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Count Input Select. When bits 1:0 are not 00, these bits select which PWM_CAP pin carries the signal used to increment the TC. Other combinations are reserved."]
    #[inline(always)]
    pub fn cis(&mut self) -> CisW<'_, CtcrSpec> {
        CisW::new(self, 2)
    }
}
#[doc = "Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtcrSpec;
impl crate::RegisterSpec for CtcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctcr::R`](R) reader structure"]
impl crate::Readable for CtcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ctcr::W`](W) writer structure"]
impl crate::Writable for CtcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTCR to value 0"]
impl crate::Resettable for CtcrSpec {}
