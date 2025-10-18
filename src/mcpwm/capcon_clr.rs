#[doc = "Register `CAPCON_CLR` writer"]
pub type W = crate::W<CapconClrSpec>;
#[doc = "Field `CAP0MCI0_RE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub type Cap0mci0ReClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP0MCI0_FE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub type Cap0mci0FeClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP0MCI1_RE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub type Cap0mci1ReClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP0MCI1_FE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub type Cap0mci1FeClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP0MCI2_RE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub type Cap0mci2ReClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP0MCI2_FE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub type Cap0mci2FeClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP1MCI0_RE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub type Cap1mci0ReClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP1MCI0_FE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub type Cap1mci0FeClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP1MCI1_RE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub type Cap1mci1ReClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP1MCI1_FE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub type Cap1mci1FeClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP1MCI2_RE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub type Cap1mci2ReClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP1MCI2_FE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub type Cap1mci2FeClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP2MCI0_RE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub type Cap2mci0ReClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP2MCI0_FE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub type Cap2mci0FeClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP2MCI1_RE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub type Cap2mci1ReClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP2MCI1_FE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub type Cap2mci1FeClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP2MCI2_RE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub type Cap2mci2ReClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP2MCI2_FE_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub type Cap2mci2FeClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT0_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub type Rt0ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT1_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub type Rt1ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT2_CLR` writer - Writing a one clears the corresponding bits in the CAPCON register."]
pub type Rt2ClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap0mci0_re_clr(&mut self) -> Cap0mci0ReClrW<'_, CapconClrSpec> {
        Cap0mci0ReClrW::new(self, 0)
    }
    #[doc = "Bit 1 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap0mci0_fe_clr(&mut self) -> Cap0mci0FeClrW<'_, CapconClrSpec> {
        Cap0mci0FeClrW::new(self, 1)
    }
    #[doc = "Bit 2 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap0mci1_re_clr(&mut self) -> Cap0mci1ReClrW<'_, CapconClrSpec> {
        Cap0mci1ReClrW::new(self, 2)
    }
    #[doc = "Bit 3 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap0mci1_fe_clr(&mut self) -> Cap0mci1FeClrW<'_, CapconClrSpec> {
        Cap0mci1FeClrW::new(self, 3)
    }
    #[doc = "Bit 4 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap0mci2_re_clr(&mut self) -> Cap0mci2ReClrW<'_, CapconClrSpec> {
        Cap0mci2ReClrW::new(self, 4)
    }
    #[doc = "Bit 5 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap0mci2_fe_clr(&mut self) -> Cap0mci2FeClrW<'_, CapconClrSpec> {
        Cap0mci2FeClrW::new(self, 5)
    }
    #[doc = "Bit 6 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap1mci0_re_clr(&mut self) -> Cap1mci0ReClrW<'_, CapconClrSpec> {
        Cap1mci0ReClrW::new(self, 6)
    }
    #[doc = "Bit 7 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap1mci0_fe_clr(&mut self) -> Cap1mci0FeClrW<'_, CapconClrSpec> {
        Cap1mci0FeClrW::new(self, 7)
    }
    #[doc = "Bit 8 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap1mci1_re_clr(&mut self) -> Cap1mci1ReClrW<'_, CapconClrSpec> {
        Cap1mci1ReClrW::new(self, 8)
    }
    #[doc = "Bit 9 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap1mci1_fe_clr(&mut self) -> Cap1mci1FeClrW<'_, CapconClrSpec> {
        Cap1mci1FeClrW::new(self, 9)
    }
    #[doc = "Bit 10 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap1mci2_re_clr(&mut self) -> Cap1mci2ReClrW<'_, CapconClrSpec> {
        Cap1mci2ReClrW::new(self, 10)
    }
    #[doc = "Bit 11 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap1mci2_fe_clr(&mut self) -> Cap1mci2FeClrW<'_, CapconClrSpec> {
        Cap1mci2FeClrW::new(self, 11)
    }
    #[doc = "Bit 12 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap2mci0_re_clr(&mut self) -> Cap2mci0ReClrW<'_, CapconClrSpec> {
        Cap2mci0ReClrW::new(self, 12)
    }
    #[doc = "Bit 13 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap2mci0_fe_clr(&mut self) -> Cap2mci0FeClrW<'_, CapconClrSpec> {
        Cap2mci0FeClrW::new(self, 13)
    }
    #[doc = "Bit 14 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap2mci1_re_clr(&mut self) -> Cap2mci1ReClrW<'_, CapconClrSpec> {
        Cap2mci1ReClrW::new(self, 14)
    }
    #[doc = "Bit 15 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap2mci1_fe_clr(&mut self) -> Cap2mci1FeClrW<'_, CapconClrSpec> {
        Cap2mci1FeClrW::new(self, 15)
    }
    #[doc = "Bit 16 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap2mci2_re_clr(&mut self) -> Cap2mci2ReClrW<'_, CapconClrSpec> {
        Cap2mci2ReClrW::new(self, 16)
    }
    #[doc = "Bit 17 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap2mci2_fe_clr(&mut self) -> Cap2mci2FeClrW<'_, CapconClrSpec> {
        Cap2mci2FeClrW::new(self, 17)
    }
    #[doc = "Bit 18 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn rt0_clr(&mut self) -> Rt0ClrW<'_, CapconClrSpec> {
        Rt0ClrW::new(self, 18)
    }
    #[doc = "Bit 19 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn rt1_clr(&mut self) -> Rt1ClrW<'_, CapconClrSpec> {
        Rt1ClrW::new(self, 19)
    }
    #[doc = "Bit 20 - Writing a one clears the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn rt2_clr(&mut self) -> Rt2ClrW<'_, CapconClrSpec> {
        Rt2ClrW::new(self, 20)
    }
}
#[doc = "Event Control clear address\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capcon_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CapconClrSpec;
impl crate::RegisterSpec for CapconClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`capcon_clr::W`](W) writer structure"]
impl crate::Writable for CapconClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAPCON_CLR to value 0"]
impl crate::Resettable for CapconClrSpec {}
