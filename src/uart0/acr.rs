#[doc = "Register `ACR` reader"]
pub type R = crate::R<AcrSpec>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<AcrSpec>;
#[doc = "Start bit. This bit is automatically cleared after auto-baud completion.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    #[doc = "0: Auto-baud stop (auto-baud is not running)."]
    AutoBaudStopAuto = 0,
    #[doc = "1: Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion."]
    AutoBaudStartAut = 1,
}
impl From<Start> for bool {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Start bit. This bit is automatically cleared after auto-baud completion."]
pub type StartR = crate::BitReader<Start>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Start {
        match self.bits {
            false => Start::AutoBaudStopAuto,
            true => Start::AutoBaudStartAut,
        }
    }
    #[doc = "Auto-baud stop (auto-baud is not running)."]
    #[inline(always)]
    pub fn is_auto_baud_stop_auto(&self) -> bool {
        *self == Start::AutoBaudStopAuto
    }
    #[doc = "Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion."]
    #[inline(always)]
    pub fn is_auto_baud_start_aut(&self) -> bool {
        *self == Start::AutoBaudStartAut
    }
}
#[doc = "Field `START` writer - Start bit. This bit is automatically cleared after auto-baud completion."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto-baud stop (auto-baud is not running)."]
    #[inline(always)]
    pub fn auto_baud_stop_auto(self) -> &'a mut crate::W<REG> {
        self.variant(Start::AutoBaudStopAuto)
    }
    #[doc = "Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion."]
    #[inline(always)]
    pub fn auto_baud_start_aut(self) -> &'a mut crate::W<REG> {
        self.variant(Start::AutoBaudStartAut)
    }
}
#[doc = "Auto-baud mode select bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: Mode 0."]
    Mode0_ = 0,
    #[doc = "1: Mode 1."]
    Mode1_ = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Auto-baud mode select bit."]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Mode0_,
            true => Mode::Mode1_,
        }
    }
    #[doc = "Mode 0."]
    #[inline(always)]
    pub fn is_mode_0_(&self) -> bool {
        *self == Mode::Mode0_
    }
    #[doc = "Mode 1."]
    #[inline(always)]
    pub fn is_mode_1_(&self) -> bool {
        *self == Mode::Mode1_
    }
}
#[doc = "Field `MODE` writer - Auto-baud mode select bit."]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Mode 0."]
    #[inline(always)]
    pub fn mode_0_(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Mode0_)
    }
    #[doc = "Mode 1."]
    #[inline(always)]
    pub fn mode_1_(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Mode1_)
    }
}
#[doc = "Restart bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autorestart {
    #[doc = "0: No restart."]
    NoRestart_ = 0,
    #[doc = "1: Restart in case of time-out (counter restarts at next UARTn Rx falling edge)"]
    RestartInCaseOfT = 1,
}
impl From<Autorestart> for bool {
    #[inline(always)]
    fn from(variant: Autorestart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTORESTART` reader - Restart bit."]
pub type AutorestartR = crate::BitReader<Autorestart>;
impl AutorestartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autorestart {
        match self.bits {
            false => Autorestart::NoRestart_,
            true => Autorestart::RestartInCaseOfT,
        }
    }
    #[doc = "No restart."]
    #[inline(always)]
    pub fn is_no_restart_(&self) -> bool {
        *self == Autorestart::NoRestart_
    }
    #[doc = "Restart in case of time-out (counter restarts at next UARTn Rx falling edge)"]
    #[inline(always)]
    pub fn is_restart_in_case_of_t(&self) -> bool {
        *self == Autorestart::RestartInCaseOfT
    }
}
#[doc = "Field `AUTORESTART` writer - Restart bit."]
pub type AutorestartW<'a, REG> = crate::BitWriter<'a, REG, Autorestart>;
impl<'a, REG> AutorestartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No restart."]
    #[inline(always)]
    pub fn no_restart_(self) -> &'a mut crate::W<REG> {
        self.variant(Autorestart::NoRestart_)
    }
    #[doc = "Restart in case of time-out (counter restarts at next UARTn Rx falling edge)"]
    #[inline(always)]
    pub fn restart_in_case_of_t(self) -> &'a mut crate::W<REG> {
        self.variant(Autorestart::RestartInCaseOfT)
    }
}
#[doc = "End of auto-baud interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Abeointclr {
    #[doc = "0: No impact."]
    NoImpact_ = 0,
    #[doc = "1: Clear the corresponding interrupt in the IIR."]
    ClearTheCorrespond = 1,
}
impl From<Abeointclr> for bool {
    #[inline(always)]
    fn from(variant: Abeointclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABEOINTCLR` reader - End of auto-baud interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact."]
pub type AbeointclrR = crate::BitReader<Abeointclr>;
impl AbeointclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Abeointclr {
        match self.bits {
            false => Abeointclr::NoImpact_,
            true => Abeointclr::ClearTheCorrespond,
        }
    }
    #[doc = "No impact."]
    #[inline(always)]
    pub fn is_no_impact_(&self) -> bool {
        *self == Abeointclr::NoImpact_
    }
    #[doc = "Clear the corresponding interrupt in the IIR."]
    #[inline(always)]
    pub fn is_clear_the_correspond(&self) -> bool {
        *self == Abeointclr::ClearTheCorrespond
    }
}
#[doc = "Field `ABEOINTCLR` writer - End of auto-baud interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact."]
pub type AbeointclrW<'a, REG> = crate::BitWriter<'a, REG, Abeointclr>;
impl<'a, REG> AbeointclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No impact."]
    #[inline(always)]
    pub fn no_impact_(self) -> &'a mut crate::W<REG> {
        self.variant(Abeointclr::NoImpact_)
    }
    #[doc = "Clear the corresponding interrupt in the IIR."]
    #[inline(always)]
    pub fn clear_the_correspond(self) -> &'a mut crate::W<REG> {
        self.variant(Abeointclr::ClearTheCorrespond)
    }
}
#[doc = "Auto-baud time-out interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Abtointclr {
    #[doc = "0: No impact."]
    NoImpact_ = 0,
    #[doc = "1: Clear the corresponding interrupt in the IIR."]
    ClearTheCorrespond = 1,
}
impl From<Abtointclr> for bool {
    #[inline(always)]
    fn from(variant: Abtointclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABTOINTCLR` reader - Auto-baud time-out interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact."]
pub type AbtointclrR = crate::BitReader<Abtointclr>;
impl AbtointclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Abtointclr {
        match self.bits {
            false => Abtointclr::NoImpact_,
            true => Abtointclr::ClearTheCorrespond,
        }
    }
    #[doc = "No impact."]
    #[inline(always)]
    pub fn is_no_impact_(&self) -> bool {
        *self == Abtointclr::NoImpact_
    }
    #[doc = "Clear the corresponding interrupt in the IIR."]
    #[inline(always)]
    pub fn is_clear_the_correspond(&self) -> bool {
        *self == Abtointclr::ClearTheCorrespond
    }
}
#[doc = "Field `ABTOINTCLR` writer - Auto-baud time-out interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact."]
pub type AbtointclrW<'a, REG> = crate::BitWriter<'a, REG, Abtointclr>;
impl<'a, REG> AbtointclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No impact."]
    #[inline(always)]
    pub fn no_impact_(self) -> &'a mut crate::W<REG> {
        self.variant(Abtointclr::NoImpact_)
    }
    #[doc = "Clear the corresponding interrupt in the IIR."]
    #[inline(always)]
    pub fn clear_the_correspond(self) -> &'a mut crate::W<REG> {
        self.variant(Abtointclr::ClearTheCorrespond)
    }
}
impl R {
    #[doc = "Bit 0 - Start bit. This bit is automatically cleared after auto-baud completion."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Auto-baud mode select bit."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Restart bit."]
    #[inline(always)]
    pub fn autorestart(&self) -> AutorestartR {
        AutorestartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - End of auto-baud interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact."]
    #[inline(always)]
    pub fn abeointclr(&self) -> AbeointclrR {
        AbeointclrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Auto-baud time-out interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact."]
    #[inline(always)]
    pub fn abtointclr(&self) -> AbtointclrR {
        AbtointclrR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start bit. This bit is automatically cleared after auto-baud completion."]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, AcrSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Auto-baud mode select bit."]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, AcrSpec> {
        ModeW::new(self, 1)
    }
    #[doc = "Bit 2 - Restart bit."]
    #[inline(always)]
    pub fn autorestart(&mut self) -> AutorestartW<'_, AcrSpec> {
        AutorestartW::new(self, 2)
    }
    #[doc = "Bit 8 - End of auto-baud interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact."]
    #[inline(always)]
    pub fn abeointclr(&mut self) -> AbeointclrW<'_, AcrSpec> {
        AbeointclrW::new(self, 8)
    }
    #[doc = "Bit 9 - Auto-baud time-out interrupt clear bit (write-only accessible). Writing a 1 will clear the corresponding interrupt in the UnIIR. Writing a 0 has no impact."]
    #[inline(always)]
    pub fn abtointclr(&mut self) -> AbtointclrW<'_, AcrSpec> {
        AbtointclrW::new(self, 9)
    }
}
#[doc = "Auto-baud Control Register. Contains controls for the auto-baud feature.\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AcrSpec;
impl crate::RegisterSpec for AcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr::R`](R) reader structure"]
impl crate::Readable for AcrSpec {}
#[doc = "`write(|w| ..)` method takes [`acr::W`](W) writer structure"]
impl crate::Writable for AcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACR to value 0"]
impl crate::Resettable for AcrSpec {}
