#[doc = "Register `DYNAMICRASCAS%s` reader"]
pub type R = crate::R<DynamicrascasSpec>;
#[doc = "Register `DYNAMICRASCAS%s` writer"]
pub type W = crate::W<DynamicrascasSpec>;
#[doc = "RAS latency (active to read/write delay).\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ras {
    #[doc = "1: One CCLK cycle."]
    OneCclk = 1,
    #[doc = "2: Two CCLK cycles."]
    TwoCclk = 2,
    #[doc = "3: Three CCLK cycles (POR reset value)."]
    ThreeCclk = 3,
}
impl From<Ras> for u8 {
    #[inline(always)]
    fn from(variant: Ras) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ras {
    type Ux = u8;
}
impl crate::IsEnum for Ras {}
#[doc = "Field `RAS` reader - RAS latency (active to read/write delay)."]
pub type RasR = crate::FieldReader<Ras>;
impl RasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ras {
        match self.bits {
            1 => Ras::OneCclk,
            2 => Ras::TwoCclk,
            3 => Ras::ThreeCclk,
            _ => unreachable!(),
        }
    }
    #[doc = "One CCLK cycle."]
    #[inline(always)]
    pub fn is_one_cclk(&self) -> bool {
        *self == Ras::OneCclk
    }
    #[doc = "Two CCLK cycles."]
    #[inline(always)]
    pub fn is_two_cclk(&self) -> bool {
        *self == Ras::TwoCclk
    }
    #[doc = "Three CCLK cycles (POR reset value)."]
    #[inline(always)]
    pub fn is_three_cclk(&self) -> bool {
        *self == Ras::ThreeCclk
    }
}
#[doc = "Field `RAS` writer - RAS latency (active to read/write delay)."]
pub type RasW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ras>;
impl<'a, REG> RasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One CCLK cycle."]
    #[inline(always)]
    pub fn one_cclk(self) -> &'a mut crate::W<REG> {
        self.variant(Ras::OneCclk)
    }
    #[doc = "Two CCLK cycles."]
    #[inline(always)]
    pub fn two_cclk(self) -> &'a mut crate::W<REG> {
        self.variant(Ras::TwoCclk)
    }
    #[doc = "Three CCLK cycles (POR reset value)."]
    #[inline(always)]
    pub fn three_cclk(self) -> &'a mut crate::W<REG> {
        self.variant(Ras::ThreeCclk)
    }
}
#[doc = "CAS latency.\n\nValue on reset: 3"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cas {
    #[doc = "1: One CCLK cycle."]
    OneCycle = 1,
    #[doc = "2: Two CCLK cycles."]
    TwoCycles = 2,
    #[doc = "3: Three CCLK cycles (POR reset value)."]
    ThreeCycles = 3,
}
impl From<Cas> for u8 {
    #[inline(always)]
    fn from(variant: Cas) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cas {
    type Ux = u8;
}
impl crate::IsEnum for Cas {}
#[doc = "Field `CAS` reader - CAS latency."]
pub type CasR = crate::FieldReader<Cas>;
impl CasR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cas {
        match self.bits {
            1 => Cas::OneCycle,
            2 => Cas::TwoCycles,
            3 => Cas::ThreeCycles,
            _ => unreachable!(),
        }
    }
    #[doc = "One CCLK cycle."]
    #[inline(always)]
    pub fn is_one_cycle(&self) -> bool {
        *self == Cas::OneCycle
    }
    #[doc = "Two CCLK cycles."]
    #[inline(always)]
    pub fn is_two_cycles(&self) -> bool {
        *self == Cas::TwoCycles
    }
    #[doc = "Three CCLK cycles (POR reset value)."]
    #[inline(always)]
    pub fn is_three_cycles(&self) -> bool {
        *self == Cas::ThreeCycles
    }
}
#[doc = "Field `CAS` writer - CAS latency."]
pub type CasW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cas>;
impl<'a, REG> CasW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "One CCLK cycle."]
    #[inline(always)]
    pub fn one_cycle(self) -> &'a mut crate::W<REG> {
        self.variant(Cas::OneCycle)
    }
    #[doc = "Two CCLK cycles."]
    #[inline(always)]
    pub fn two_cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Cas::TwoCycles)
    }
    #[doc = "Three CCLK cycles (POR reset value)."]
    #[inline(always)]
    pub fn three_cycles(self) -> &'a mut crate::W<REG> {
        self.variant(Cas::ThreeCycles)
    }
}
impl R {
    #[doc = "Bits 0:1 - RAS latency (active to read/write delay)."]
    #[inline(always)]
    pub fn ras(&self) -> RasR {
        RasR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - CAS latency."]
    #[inline(always)]
    pub fn cas(&self) -> CasR {
        CasR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - RAS latency (active to read/write delay)."]
    #[inline(always)]
    pub fn ras(&mut self) -> RasW<'_, DynamicrascasSpec> {
        RasW::new(self, 0)
    }
    #[doc = "Bits 8:9 - CAS latency."]
    #[inline(always)]
    pub fn cas(&mut self) -> CasW<'_, DynamicrascasSpec> {
        CasW::new(self, 8)
    }
}
#[doc = "RAS and CAS latencies for EMC_DYCS0.\n\nYou can [`read`](crate::Reg::read) this register and get [`dynamicrascas::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dynamicrascas::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DynamicrascasSpec;
impl crate::RegisterSpec for DynamicrascasSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dynamicrascas::R`](R) reader structure"]
impl crate::Readable for DynamicrascasSpec {}
#[doc = "`write(|w| ..)` method takes [`dynamicrascas::W`](W) writer structure"]
impl crate::Writable for DynamicrascasSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DYNAMICRASCAS%s to value 0x0303"]
impl crate::Resettable for DynamicrascasSpec {
    const RESET_VALUE: u32 = 0x0303;
}
