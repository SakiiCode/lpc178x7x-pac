#[doc = "Register `CCR` reader"]
pub type R = crate::R<CcrSpec>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CcrSpec>;
#[doc = "Clock Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Clken {
    #[doc = "1: The time counters are enabled."]
    Enabled = 1,
    #[doc = "0: The time counters are disabled so that they may be initialized."]
    Disabled = 0,
}
impl From<Clken> for bool {
    #[inline(always)]
    fn from(variant: Clken) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKEN` reader - Clock Enable."]
pub type ClkenR = crate::BitReader<Clken>;
impl ClkenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Clken {
        match self.bits {
            true => Clken::Enabled,
            false => Clken::Disabled,
        }
    }
    #[doc = "The time counters are enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Clken::Enabled
    }
    #[doc = "The time counters are disabled so that they may be initialized."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Clken::Disabled
    }
}
#[doc = "Field `CLKEN` writer - Clock Enable."]
pub type ClkenW<'a, REG> = crate::BitWriter<'a, REG, Clken>;
impl<'a, REG> ClkenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The time counters are enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Clken::Enabled)
    }
    #[doc = "The time counters are disabled so that they may be initialized."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Clken::Disabled)
    }
}
#[doc = "CTC Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ctcrst {
    #[doc = "1: When one, the elements in the internal oscillator divider are reset, and remain reset until CCR\\[1\\] is changed to zero. This is the divider that generates the 1 Hz clock from the 32.768 kHz crystal. The state of the divider is not visible to software."]
    Reset = 1,
    #[doc = "0: No effect."]
    NoEffect_ = 0,
}
impl From<Ctcrst> for bool {
    #[inline(always)]
    fn from(variant: Ctcrst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CTCRST` reader - CTC Reset."]
pub type CtcrstR = crate::BitReader<Ctcrst>;
impl CtcrstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ctcrst {
        match self.bits {
            true => Ctcrst::Reset,
            false => Ctcrst::NoEffect_,
        }
    }
    #[doc = "When one, the elements in the internal oscillator divider are reset, and remain reset until CCR\\[1\\] is changed to zero. This is the divider that generates the 1 Hz clock from the 32.768 kHz crystal. The state of the divider is not visible to software."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Ctcrst::Reset
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn is_no_effect_(&self) -> bool {
        *self == Ctcrst::NoEffect_
    }
}
#[doc = "Field `CTCRST` writer - CTC Reset."]
pub type CtcrstW<'a, REG> = crate::BitWriter<'a, REG, Ctcrst>;
impl<'a, REG> CtcrstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When one, the elements in the internal oscillator divider are reset, and remain reset until CCR\\[1\\] is changed to zero. This is the divider that generates the 1 Hz clock from the 32.768 kHz crystal. The state of the divider is not visible to software."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Ctcrst::Reset)
    }
    #[doc = "No effect."]
    #[inline(always)]
    pub fn no_effect_(self) -> &'a mut crate::W<REG> {
        self.variant(Ctcrst::NoEffect_)
    }
}
#[doc = "Calibration counter enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccalen {
    #[doc = "1: The calibration counter is disabled and reset to zero."]
    Disabled = 1,
    #[doc = "0: The calibration counter is enabled and counting, using the 1 Hz clock. When the calibration counter is equal to the value of the CALIBRATION register, the counter resets and repeats counting up to the value of the CALIBRATION register. See Section 30.6.4.2 and Section 30.6.5."]
    Enabled = 0,
}
impl From<Ccalen> for bool {
    #[inline(always)]
    fn from(variant: Ccalen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCALEN` reader - Calibration counter enable."]
pub type CcalenR = crate::BitReader<Ccalen>;
impl CcalenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccalen {
        match self.bits {
            true => Ccalen::Disabled,
            false => Ccalen::Enabled,
        }
    }
    #[doc = "The calibration counter is disabled and reset to zero."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ccalen::Disabled
    }
    #[doc = "The calibration counter is enabled and counting, using the 1 Hz clock. When the calibration counter is equal to the value of the CALIBRATION register, the counter resets and repeats counting up to the value of the CALIBRATION register. See Section 30.6.4.2 and Section 30.6.5."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ccalen::Enabled
    }
}
#[doc = "Field `CCALEN` writer - Calibration counter enable."]
pub type CcalenW<'a, REG> = crate::BitWriter<'a, REG, Ccalen>;
impl<'a, REG> CcalenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The calibration counter is disabled and reset to zero."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ccalen::Disabled)
    }
    #[doc = "The calibration counter is enabled and counting, using the 1 Hz clock. When the calibration counter is equal to the value of the CALIBRATION register, the counter resets and repeats counting up to the value of the CALIBRATION register. See Section 30.6.4.2 and Section 30.6.5."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ccalen::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Clock Enable."]
    #[inline(always)]
    pub fn clken(&self) -> ClkenR {
        ClkenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTC Reset."]
    #[inline(always)]
    pub fn ctcrst(&self) -> CtcrstR {
        CtcrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Calibration counter enable."]
    #[inline(always)]
    pub fn ccalen(&self) -> CcalenR {
        CcalenR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock Enable."]
    #[inline(always)]
    pub fn clken(&mut self) -> ClkenW<'_, CcrSpec> {
        ClkenW::new(self, 0)
    }
    #[doc = "Bit 1 - CTC Reset."]
    #[inline(always)]
    pub fn ctcrst(&mut self) -> CtcrstW<'_, CcrSpec> {
        CtcrstW::new(self, 1)
    }
    #[doc = "Bit 4 - Calibration counter enable."]
    #[inline(always)]
    pub fn ccalen(&mut self) -> CcalenW<'_, CcrSpec> {
        CcalenW::new(self, 4)
    }
}
#[doc = "Clock Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcrSpec;
impl crate::RegisterSpec for CcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CcrSpec {}
