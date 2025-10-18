#[doc = "Register `CON_SET` writer"]
pub type W = crate::W<ConSetSpec>;
#[doc = "Field `RUN0_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type Run0SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CENTER0_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type Center0SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLA0_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type Pola0SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTE0_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type Dte0SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISUP0_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type Disup0SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUN1_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type Run1SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CENTER1_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type Center1SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLA1_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type Pola1SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTE1_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type Dte1SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISUP1_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type Disup1SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUN2_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type Run2SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CENTER2_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type Center2SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLA2_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type Pola2SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTE2_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type Dte2SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISUP2_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type Disup2SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INVBDC_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type InvbdcSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACMODE_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type AcmodeSetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCMODE_SET` writer - Writing a one sets the corresponding bit in the CON register."]
pub type DcmodeSetW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn run0_set(&mut self) -> Run0SetW<'_, ConSetSpec> {
        Run0SetW::new(self, 0)
    }
    #[doc = "Bit 1 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn center0_set(&mut self) -> Center0SetW<'_, ConSetSpec> {
        Center0SetW::new(self, 1)
    }
    #[doc = "Bit 2 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn pola0_set(&mut self) -> Pola0SetW<'_, ConSetSpec> {
        Pola0SetW::new(self, 2)
    }
    #[doc = "Bit 3 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn dte0_set(&mut self) -> Dte0SetW<'_, ConSetSpec> {
        Dte0SetW::new(self, 3)
    }
    #[doc = "Bit 4 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn disup0_set(&mut self) -> Disup0SetW<'_, ConSetSpec> {
        Disup0SetW::new(self, 4)
    }
    #[doc = "Bit 8 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn run1_set(&mut self) -> Run1SetW<'_, ConSetSpec> {
        Run1SetW::new(self, 8)
    }
    #[doc = "Bit 9 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn center1_set(&mut self) -> Center1SetW<'_, ConSetSpec> {
        Center1SetW::new(self, 9)
    }
    #[doc = "Bit 10 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn pola1_set(&mut self) -> Pola1SetW<'_, ConSetSpec> {
        Pola1SetW::new(self, 10)
    }
    #[doc = "Bit 11 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn dte1_set(&mut self) -> Dte1SetW<'_, ConSetSpec> {
        Dte1SetW::new(self, 11)
    }
    #[doc = "Bit 12 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn disup1_set(&mut self) -> Disup1SetW<'_, ConSetSpec> {
        Disup1SetW::new(self, 12)
    }
    #[doc = "Bit 16 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn run2_set(&mut self) -> Run2SetW<'_, ConSetSpec> {
        Run2SetW::new(self, 16)
    }
    #[doc = "Bit 17 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn center2_set(&mut self) -> Center2SetW<'_, ConSetSpec> {
        Center2SetW::new(self, 17)
    }
    #[doc = "Bit 18 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn pola2_set(&mut self) -> Pola2SetW<'_, ConSetSpec> {
        Pola2SetW::new(self, 18)
    }
    #[doc = "Bit 19 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn dte2_set(&mut self) -> Dte2SetW<'_, ConSetSpec> {
        Dte2SetW::new(self, 19)
    }
    #[doc = "Bit 20 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn disup2_set(&mut self) -> Disup2SetW<'_, ConSetSpec> {
        Disup2SetW::new(self, 20)
    }
    #[doc = "Bit 29 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn invbdc_set(&mut self) -> InvbdcSetW<'_, ConSetSpec> {
        InvbdcSetW::new(self, 29)
    }
    #[doc = "Bit 30 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn acmode_set(&mut self) -> AcmodeSetW<'_, ConSetSpec> {
        AcmodeSetW::new(self, 30)
    }
    #[doc = "Bit 31 - Writing a one sets the corresponding bit in the CON register."]
    #[inline(always)]
    pub fn dcmode_set(&mut self) -> DcmodeSetW<'_, ConSetSpec> {
        DcmodeSetW::new(self, 31)
    }
}
#[doc = "PWM Control set address\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`con_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConSetSpec;
impl crate::RegisterSpec for ConSetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`con_set::W`](W) writer structure"]
impl crate::Writable for ConSetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CON_SET to value 0"]
impl crate::Resettable for ConSetSpec {}
