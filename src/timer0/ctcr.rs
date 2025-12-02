#[doc = "Register `CTCR` reader"]
pub type R = crate::R<CtcrSpec>;
#[doc = "Register `CTCR` writer"]
pub type W = crate::W<CtcrSpec>;
#[doc = "Counter/Timer Mode This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ctmode {
    #[doc = "0: Timer Mode: every rising PCLK edge"]
    Trising = 0,
    #[doc = "1: Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    Crising = 1,
    #[doc = "2: Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    Cfalling = 2,
    #[doc = "3: Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2."]
    Cdualedge = 3,
}
impl From<Ctmode> for u8 {
    #[inline(always)]
    fn from(variant: Ctmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ctmode {
    type Ux = u8;
}
impl crate::IsEnum for Ctmode {}
#[doc = "Field `CTMODE` reader - Counter/Timer Mode This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
pub type CtmodeR = crate::FieldReader<Ctmode>;
impl CtmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctmode {
        match self.bits {
            0 => Ctmode::Trising,
            1 => Ctmode::Crising,
            2 => Ctmode::Cfalling,
            3 => Ctmode::Cdualedge,
            _ => unreachable!(),
        }
    }
    #[doc = "Timer Mode: every rising PCLK edge"]
    #[inline(always)]
    pub fn is_trising(&self) -> bool {
        *self == Ctmode::Trising
    }
    #[doc = "Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn is_crising(&self) -> bool {
        *self == Ctmode::Crising
    }
    #[doc = "Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn is_cfalling(&self) -> bool {
        *self == Ctmode::Cfalling
    }
    #[doc = "Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn is_cdualedge(&self) -> bool {
        *self == Ctmode::Cdualedge
    }
}
#[doc = "Field `CTMODE` writer - Counter/Timer Mode This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
pub type CtmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ctmode, crate::Safe>;
impl<'a, REG> CtmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Timer Mode: every rising PCLK edge"]
    #[inline(always)]
    pub fn trising(self) -> &'a mut crate::W<REG> {
        self.variant(Ctmode::Trising)
    }
    #[doc = "Counter Mode: TC is incremented on rising edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn crising(self) -> &'a mut crate::W<REG> {
        self.variant(Ctmode::Crising)
    }
    #[doc = "Counter Mode: TC is incremented on falling edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn cfalling(self) -> &'a mut crate::W<REG> {
        self.variant(Ctmode::Cfalling)
    }
    #[doc = "Counter Mode: TC is incremented on both edges on the CAP input selected by bits 3:2."]
    #[inline(always)]
    pub fn cdualedge(self) -> &'a mut crate::W<REG> {
        self.variant(Ctmode::Cdualedge)
    }
}
#[doc = "Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the TnCTCR, the 3 bits for that input in the Capture Control Register (TnCCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Cinsel {
    #[doc = "0: CAPn.0 for TIMERn"]
    Capn0 = 0,
    #[doc = "1: CAPn.1 for TIMERn"]
    Capn1 = 1,
}
impl From<Cinsel> for u8 {
    #[inline(always)]
    fn from(variant: Cinsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Cinsel {
    type Ux = u8;
}
impl crate::IsEnum for Cinsel {}
#[doc = "Field `CINSEL` reader - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the TnCTCR, the 3 bits for that input in the Capture Control Register (TnCCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
pub type CinselR = crate::FieldReader<Cinsel>;
impl CinselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cinsel {
        match self.bits {
            0 => Cinsel::Capn0,
            1 => Cinsel::Capn1,
            _ => unreachable!(),
        }
    }
    #[doc = "CAPn.0 for TIMERn"]
    #[inline(always)]
    pub fn is_capn_0(&self) -> bool {
        *self == Cinsel::Capn0
    }
    #[doc = "CAPn.1 for TIMERn"]
    #[inline(always)]
    pub fn is_capn_1(&self) -> bool {
        *self == Cinsel::Capn1
    }
}
#[doc = "Field `CINSEL` writer - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the TnCTCR, the 3 bits for that input in the Capture Control Register (TnCCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
pub type CinselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Cinsel>;
impl<'a, REG> CinselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CAPn.0 for TIMERn"]
    #[inline(always)]
    pub fn capn_0(self) -> &'a mut crate::W<REG> {
        self.variant(Cinsel::Capn0)
    }
    #[doc = "CAPn.1 for TIMERn"]
    #[inline(always)]
    pub fn capn_1(self) -> &'a mut crate::W<REG> {
        self.variant(Cinsel::Capn1)
    }
}
impl R {
    #[doc = "Bits 0:1 - Counter/Timer Mode This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
    #[inline(always)]
    pub fn ctmode(&self) -> CtmodeR {
        CtmodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the TnCTCR, the 3 bits for that input in the Capture Control Register (TnCCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
    #[inline(always)]
    pub fn cinsel(&self) -> CinselR {
        CinselR::new(((self.bits >> 2) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Counter/Timer Mode This field selects which rising PCLK edges can increment Timer's Prescale Counter (PC), or clear PC and increment Timer Counter (TC). Timer Mode: the TC is incremented when the Prescale Counter matches the Prescale Register."]
    #[inline(always)]
    pub fn ctmode(&mut self) -> CtmodeW<'_, CtcrSpec> {
        CtmodeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Count Input Select When bits 1:0 in this register are not 00, these bits select which CAP pin is sampled for clocking. Note: If Counter mode is selected for a particular CAPn input in the TnCTCR, the 3 bits for that input in the Capture Control Register (TnCCR) must be programmed as 000. However, capture and/or interrupt can be selected for the other 3 CAPn inputs in the same timer."]
    #[inline(always)]
    pub fn cinsel(&mut self) -> CinselW<'_, CtcrSpec> {
        CinselW::new(self, 2)
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
