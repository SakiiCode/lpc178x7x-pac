#[doc = "Register `CAPCON_SET` writer"]
pub type W = crate::W<CapconSetSpec>;
#[doc = "Field `CAP0MCI0_RE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type Cap0mci0ReSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP0MCI0_FE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type Cap0mci0FeSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP0MCI1_RE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type Cap0mci1ReSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP0MCI1_FE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type Cap0mci1FeSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP0MCI2_RE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type Cap0mci2ReSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP0MCI2_FE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type Cap0mci2FeSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP1MCI0_RE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type Cap1mci0ReSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP1MCI0_FE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type Cap1mci0FeSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP1MCI1_RE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type Cap1mci1ReSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP1MCI1_FE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type Cap1mci1FeSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP1MCI2_RE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type Cap1mci2ReSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP1MCI2_FE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type Cap1mci2FeSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP2MCI0_RE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type Cap2mci0ReSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP2MCI0_FE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type Cap2mci0FeSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP2MCI1_RE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type Cap2mci1ReSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP2MCI1_FE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type Cap2mci1FeSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP2MCI2_RE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type Cap2mci2ReSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP2MCI2_FE_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type Cap2mci2FeSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT0_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type Rt0SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT1_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type Rt1SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RT2_SET` writer - Writing a one sets the corresponding bits in the CAPCON register."]
pub type Rt2SetW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap0mci0_re_set(&mut self) -> Cap0mci0ReSetW<'_, CapconSetSpec> {
        Cap0mci0ReSetW::new(self, 0)
    }
    #[doc = "Bit 1 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap0mci0_fe_set(&mut self) -> Cap0mci0FeSetW<'_, CapconSetSpec> {
        Cap0mci0FeSetW::new(self, 1)
    }
    #[doc = "Bit 2 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap0mci1_re_set(&mut self) -> Cap0mci1ReSetW<'_, CapconSetSpec> {
        Cap0mci1ReSetW::new(self, 2)
    }
    #[doc = "Bit 3 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap0mci1_fe_set(&mut self) -> Cap0mci1FeSetW<'_, CapconSetSpec> {
        Cap0mci1FeSetW::new(self, 3)
    }
    #[doc = "Bit 4 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap0mci2_re_set(&mut self) -> Cap0mci2ReSetW<'_, CapconSetSpec> {
        Cap0mci2ReSetW::new(self, 4)
    }
    #[doc = "Bit 5 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap0mci2_fe_set(&mut self) -> Cap0mci2FeSetW<'_, CapconSetSpec> {
        Cap0mci2FeSetW::new(self, 5)
    }
    #[doc = "Bit 6 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap1mci0_re_set(&mut self) -> Cap1mci0ReSetW<'_, CapconSetSpec> {
        Cap1mci0ReSetW::new(self, 6)
    }
    #[doc = "Bit 7 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap1mci0_fe_set(&mut self) -> Cap1mci0FeSetW<'_, CapconSetSpec> {
        Cap1mci0FeSetW::new(self, 7)
    }
    #[doc = "Bit 8 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap1mci1_re_set(&mut self) -> Cap1mci1ReSetW<'_, CapconSetSpec> {
        Cap1mci1ReSetW::new(self, 8)
    }
    #[doc = "Bit 9 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap1mci1_fe_set(&mut self) -> Cap1mci1FeSetW<'_, CapconSetSpec> {
        Cap1mci1FeSetW::new(self, 9)
    }
    #[doc = "Bit 10 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap1mci2_re_set(&mut self) -> Cap1mci2ReSetW<'_, CapconSetSpec> {
        Cap1mci2ReSetW::new(self, 10)
    }
    #[doc = "Bit 11 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap1mci2_fe_set(&mut self) -> Cap1mci2FeSetW<'_, CapconSetSpec> {
        Cap1mci2FeSetW::new(self, 11)
    }
    #[doc = "Bit 12 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap2mci0_re_set(&mut self) -> Cap2mci0ReSetW<'_, CapconSetSpec> {
        Cap2mci0ReSetW::new(self, 12)
    }
    #[doc = "Bit 13 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap2mci0_fe_set(&mut self) -> Cap2mci0FeSetW<'_, CapconSetSpec> {
        Cap2mci0FeSetW::new(self, 13)
    }
    #[doc = "Bit 14 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap2mci1_re_set(&mut self) -> Cap2mci1ReSetW<'_, CapconSetSpec> {
        Cap2mci1ReSetW::new(self, 14)
    }
    #[doc = "Bit 15 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap2mci1_fe_set(&mut self) -> Cap2mci1FeSetW<'_, CapconSetSpec> {
        Cap2mci1FeSetW::new(self, 15)
    }
    #[doc = "Bit 16 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap2mci2_re_set(&mut self) -> Cap2mci2ReSetW<'_, CapconSetSpec> {
        Cap2mci2ReSetW::new(self, 16)
    }
    #[doc = "Bit 17 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn cap2mci2_fe_set(&mut self) -> Cap2mci2FeSetW<'_, CapconSetSpec> {
        Cap2mci2FeSetW::new(self, 17)
    }
    #[doc = "Bit 18 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn rt0_set(&mut self) -> Rt0SetW<'_, CapconSetSpec> {
        Rt0SetW::new(self, 18)
    }
    #[doc = "Bit 19 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn rt1_set(&mut self) -> Rt1SetW<'_, CapconSetSpec> {
        Rt1SetW::new(self, 19)
    }
    #[doc = "Bit 20 - Writing a one sets the corresponding bits in the CAPCON register."]
    #[inline(always)]
    pub fn rt2_set(&mut self) -> Rt2SetW<'_, CapconSetSpec> {
        Rt2SetW::new(self, 20)
    }
}
#[doc = "Capture Control set address\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capcon_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CapconSetSpec;
impl crate::RegisterSpec for CapconSetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`capcon_set::W`](W) writer structure"]
impl crate::Writable for CapconSetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAPCON_SET to value 0"]
impl crate::Resettable for CapconSetSpec {}
