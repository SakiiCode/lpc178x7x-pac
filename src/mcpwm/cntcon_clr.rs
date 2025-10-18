#[doc = "Register `CNTCON_CLR` writer"]
pub type W = crate::W<CntconClrSpec>;
#[doc = "Field `TC0MCI0_RE_CLR` writer - Writing a one clears the corresponding bit in the CNTCON register."]
pub type Tc0mci0ReClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC0MCI0_FE_CLR` writer - Writing a one clears the corresponding bit in the CNTCON register."]
pub type Tc0mci0FeClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC0MCI1_RE_CLR` writer - Writing a one clears the corresponding bit in the CNTCON register."]
pub type Tc0mci1ReClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC0MCI1_FE_CLR` writer - Writing a one clears the corresponding bit in the CNTCON register."]
pub type Tc0mci1FeClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC0MCI2_RE` writer - Writing a one clears the corresponding bit in the CNTCON register."]
pub type Tc0mci2ReW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC0MCI2_FE_CLR` writer - Writing a one clears the corresponding bit in the CNTCON register."]
pub type Tc0mci2FeClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC1MCI0_RE_CLR` writer - Writing a one clears the corresponding bit in the CNTCON register."]
pub type Tc1mci0ReClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC1MCI0_FE_CLR` writer - Writing a one clears the corresponding bit in the CNTCON register."]
pub type Tc1mci0FeClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC1MCI1_RE_CLR` writer - Writing a one clears the corresponding bit in the CNTCON register."]
pub type Tc1mci1ReClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC1MCI1_FE_CLR` writer - Writing a one clears the corresponding bit in the CNTCON register."]
pub type Tc1mci1FeClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC1MCI2_RE_CLR` writer - Writing a one clears the corresponding bit in the CNTCON register."]
pub type Tc1mci2ReClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC1MCI2_FE_CLR` writer - Writing a one clears the corresponding bit in the CNTCON register."]
pub type Tc1mci2FeClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC2MCI0_RE_CLR` writer - Writing a one clears the corresponding bit in the CNTCON register."]
pub type Tc2mci0ReClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC2MCI0_FE_CLR` writer - Writing a one clears the corresponding bit in the CNTCON register."]
pub type Tc2mci0FeClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC2MCI1_RE_CLR` writer - Writing a one clears the corresponding bit in the CNTCON register."]
pub type Tc2mci1ReClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC2MCI1_FE_CLR` writer - Writing a one clears the corresponding bit in the CNTCON register."]
pub type Tc2mci1FeClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC2MCI2_RE_CLR` writer - Writing a one clears the corresponding bit in the CNTCON register."]
pub type Tc2mci2ReClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC2MCI2_FE_CLR` writer - Writing a one clears the corresponding bit in the CNTCON register."]
pub type Tc2mci2FeClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTR0_CLR` writer - Writing a one clears the corresponding bit in the CNTCON register."]
pub type Cntr0ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTR1_CLR` writer - Writing a one clears the corresponding bit in the CNTCON register."]
pub type Cntr1ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNTR2_CLR` writer - Writing a one clears the corresponding bit in the CNTCON register."]
pub type Cntr2ClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc0mci0_re_clr(&mut self) -> Tc0mci0ReClrW<'_, CntconClrSpec> {
        Tc0mci0ReClrW::new(self, 0)
    }
    #[doc = "Bit 1 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc0mci0_fe_clr(&mut self) -> Tc0mci0FeClrW<'_, CntconClrSpec> {
        Tc0mci0FeClrW::new(self, 1)
    }
    #[doc = "Bit 2 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc0mci1_re_clr(&mut self) -> Tc0mci1ReClrW<'_, CntconClrSpec> {
        Tc0mci1ReClrW::new(self, 2)
    }
    #[doc = "Bit 3 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc0mci1_fe_clr(&mut self) -> Tc0mci1FeClrW<'_, CntconClrSpec> {
        Tc0mci1FeClrW::new(self, 3)
    }
    #[doc = "Bit 4 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc0mci2_re(&mut self) -> Tc0mci2ReW<'_, CntconClrSpec> {
        Tc0mci2ReW::new(self, 4)
    }
    #[doc = "Bit 5 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc0mci2_fe_clr(&mut self) -> Tc0mci2FeClrW<'_, CntconClrSpec> {
        Tc0mci2FeClrW::new(self, 5)
    }
    #[doc = "Bit 6 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc1mci0_re_clr(&mut self) -> Tc1mci0ReClrW<'_, CntconClrSpec> {
        Tc1mci0ReClrW::new(self, 6)
    }
    #[doc = "Bit 7 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc1mci0_fe_clr(&mut self) -> Tc1mci0FeClrW<'_, CntconClrSpec> {
        Tc1mci0FeClrW::new(self, 7)
    }
    #[doc = "Bit 8 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc1mci1_re_clr(&mut self) -> Tc1mci1ReClrW<'_, CntconClrSpec> {
        Tc1mci1ReClrW::new(self, 8)
    }
    #[doc = "Bit 9 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc1mci1_fe_clr(&mut self) -> Tc1mci1FeClrW<'_, CntconClrSpec> {
        Tc1mci1FeClrW::new(self, 9)
    }
    #[doc = "Bit 10 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc1mci2_re_clr(&mut self) -> Tc1mci2ReClrW<'_, CntconClrSpec> {
        Tc1mci2ReClrW::new(self, 10)
    }
    #[doc = "Bit 11 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc1mci2_fe_clr(&mut self) -> Tc1mci2FeClrW<'_, CntconClrSpec> {
        Tc1mci2FeClrW::new(self, 11)
    }
    #[doc = "Bit 12 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc2mci0_re_clr(&mut self) -> Tc2mci0ReClrW<'_, CntconClrSpec> {
        Tc2mci0ReClrW::new(self, 12)
    }
    #[doc = "Bit 13 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc2mci0_fe_clr(&mut self) -> Tc2mci0FeClrW<'_, CntconClrSpec> {
        Tc2mci0FeClrW::new(self, 13)
    }
    #[doc = "Bit 14 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc2mci1_re_clr(&mut self) -> Tc2mci1ReClrW<'_, CntconClrSpec> {
        Tc2mci1ReClrW::new(self, 14)
    }
    #[doc = "Bit 15 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc2mci1_fe_clr(&mut self) -> Tc2mci1FeClrW<'_, CntconClrSpec> {
        Tc2mci1FeClrW::new(self, 15)
    }
    #[doc = "Bit 16 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc2mci2_re_clr(&mut self) -> Tc2mci2ReClrW<'_, CntconClrSpec> {
        Tc2mci2ReClrW::new(self, 16)
    }
    #[doc = "Bit 17 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn tc2mci2_fe_clr(&mut self) -> Tc2mci2FeClrW<'_, CntconClrSpec> {
        Tc2mci2FeClrW::new(self, 17)
    }
    #[doc = "Bit 29 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn cntr0_clr(&mut self) -> Cntr0ClrW<'_, CntconClrSpec> {
        Cntr0ClrW::new(self, 29)
    }
    #[doc = "Bit 30 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn cntr1_clr(&mut self) -> Cntr1ClrW<'_, CntconClrSpec> {
        Cntr1ClrW::new(self, 30)
    }
    #[doc = "Bit 31 - Writing a one clears the corresponding bit in the CNTCON register."]
    #[inline(always)]
    pub fn cntr2_clr(&mut self) -> Cntr2ClrW<'_, CntconClrSpec> {
        Cntr2ClrW::new(self, 31)
    }
}
#[doc = "Count Control clear address\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntcon_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntconClrSpec;
impl crate::RegisterSpec for CntconClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cntcon_clr::W`](W) writer structure"]
impl crate::Writable for CntconClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNTCON_CLR to value 0"]
impl crate::Resettable for CntconClrSpec {}
