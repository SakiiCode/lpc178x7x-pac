#[doc = "Register `ACR` reader"]
pub type R = crate::R<AcrSpec>;
#[doc = "Register `ACR` writer"]
pub type W = crate::W<AcrSpec>;
#[doc = "Auto-baud start bit. This bit is automatically cleared after auto-baud completion.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Start {
    #[doc = "0: Auto-baud stop (auto-baud is not running)."]
    Stopped = 0,
    #[doc = "1: Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion."]
    Started = 1,
}
impl From<Start> for bool {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `START` reader - Auto-baud start bit. This bit is automatically cleared after auto-baud completion."]
pub type StartR = crate::BitReader<Start>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Start {
        match self.bits {
            false => Start::Stopped,
            true => Start::Started,
        }
    }
    #[doc = "Auto-baud stop (auto-baud is not running)."]
    #[inline(always)]
    pub fn is_stopped(&self) -> bool {
        *self == Start::Stopped
    }
    #[doc = "Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion."]
    #[inline(always)]
    pub fn is_started(&self) -> bool {
        *self == Start::Started
    }
}
#[doc = "Field `START` writer - Auto-baud start bit. This bit is automatically cleared after auto-baud completion."]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG, Start>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto-baud stop (auto-baud is not running)."]
    #[inline(always)]
    pub fn stopped(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Stopped)
    }
    #[doc = "Auto-baud start (auto-baud is running). Auto-baud run bit. This bit is automatically cleared after auto-baud completion."]
    #[inline(always)]
    pub fn started(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Started)
    }
}
#[doc = "Auto-baud mode select bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: Mode 0."]
    Mode0 = 0,
    #[doc = "1: Mode 1."]
    Mode1 = 1,
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
            false => Mode::Mode0,
            true => Mode::Mode1,
        }
    }
    #[doc = "Mode 0."]
    #[inline(always)]
    pub fn is_mode_0(&self) -> bool {
        *self == Mode::Mode0
    }
    #[doc = "Mode 1."]
    #[inline(always)]
    pub fn is_mode_1(&self) -> bool {
        *self == Mode::Mode1
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
    pub fn mode_0(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Mode0)
    }
    #[doc = "Mode 1."]
    #[inline(always)]
    pub fn mode_1(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Mode1)
    }
}
#[doc = "Auto-baud restart bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Autorestart {
    #[doc = "0: No restart"]
    NoRestart = 0,
    #[doc = "1: Restart in case of time-out (counter restarts at next UART1 Rx falling edge)"]
    Restart = 1,
}
impl From<Autorestart> for bool {
    #[inline(always)]
    fn from(variant: Autorestart) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AUTORESTART` reader - Auto-baud restart bit."]
pub type AutorestartR = crate::BitReader<Autorestart>;
impl AutorestartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Autorestart {
        match self.bits {
            false => Autorestart::NoRestart,
            true => Autorestart::Restart,
        }
    }
    #[doc = "No restart"]
    #[inline(always)]
    pub fn is_no_restart(&self) -> bool {
        *self == Autorestart::NoRestart
    }
    #[doc = "Restart in case of time-out (counter restarts at next UART1 Rx falling edge)"]
    #[inline(always)]
    pub fn is_restart(&self) -> bool {
        *self == Autorestart::Restart
    }
}
#[doc = "Field `AUTORESTART` writer - Auto-baud restart bit."]
pub type AutorestartW<'a, REG> = crate::BitWriter<'a, REG, Autorestart>;
impl<'a, REG> AutorestartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No restart"]
    #[inline(always)]
    pub fn no_restart(self) -> &'a mut crate::W<REG> {
        self.variant(Autorestart::NoRestart)
    }
    #[doc = "Restart in case of time-out (counter restarts at next UART1 Rx falling edge)"]
    #[inline(always)]
    pub fn restart(self) -> &'a mut crate::W<REG> {
        self.variant(Autorestart::Restart)
    }
}
#[doc = "End of auto-baud interrupt clear bit (write-only).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Abeointclr {
    #[doc = "0: Writing a 0 has no impact."]
    Disabled = 0,
    #[doc = "1: Writing a 1 will clear the corresponding interrupt in the IIR."]
    Enabled = 1,
}
impl From<Abeointclr> for bool {
    #[inline(always)]
    fn from(variant: Abeointclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABEOINTCLR` reader - End of auto-baud interrupt clear bit (write-only)."]
pub type AbeointclrR = crate::BitReader<Abeointclr>;
impl AbeointclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Abeointclr {
        match self.bits {
            false => Abeointclr::Disabled,
            true => Abeointclr::Enabled,
        }
    }
    #[doc = "Writing a 0 has no impact."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Abeointclr::Disabled
    }
    #[doc = "Writing a 1 will clear the corresponding interrupt in the IIR."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Abeointclr::Enabled
    }
}
#[doc = "Field `ABEOINTCLR` writer - End of auto-baud interrupt clear bit (write-only)."]
pub type AbeointclrW<'a, REG> = crate::BitWriter<'a, REG, Abeointclr>;
impl<'a, REG> AbeointclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing a 0 has no impact."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Abeointclr::Disabled)
    }
    #[doc = "Writing a 1 will clear the corresponding interrupt in the IIR."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Abeointclr::Enabled)
    }
}
#[doc = "Auto-baud time-out interrupt clear bit (write-only).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Abtointclr {
    #[doc = "0: Writing a 0 has no impact."]
    Disabled = 0,
    #[doc = "1: Writing a 1 will clear the corresponding interrupt in the IIR."]
    Enabled = 1,
}
impl From<Abtointclr> for bool {
    #[inline(always)]
    fn from(variant: Abtointclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABTOINTCLR` reader - Auto-baud time-out interrupt clear bit (write-only)."]
pub type AbtointclrR = crate::BitReader<Abtointclr>;
impl AbtointclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Abtointclr {
        match self.bits {
            false => Abtointclr::Disabled,
            true => Abtointclr::Enabled,
        }
    }
    #[doc = "Writing a 0 has no impact."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Abtointclr::Disabled
    }
    #[doc = "Writing a 1 will clear the corresponding interrupt in the IIR."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Abtointclr::Enabled
    }
}
#[doc = "Field `ABTOINTCLR` writer - Auto-baud time-out interrupt clear bit (write-only)."]
pub type AbtointclrW<'a, REG> = crate::BitWriter<'a, REG, Abtointclr>;
impl<'a, REG> AbtointclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing a 0 has no impact."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Abtointclr::Disabled)
    }
    #[doc = "Writing a 1 will clear the corresponding interrupt in the IIR."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Abtointclr::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Auto-baud start bit. This bit is automatically cleared after auto-baud completion."]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Auto-baud mode select bit."]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Auto-baud restart bit."]
    #[inline(always)]
    pub fn autorestart(&self) -> AutorestartR {
        AutorestartR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - End of auto-baud interrupt clear bit (write-only)."]
    #[inline(always)]
    pub fn abeointclr(&self) -> AbeointclrR {
        AbeointclrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Auto-baud time-out interrupt clear bit (write-only)."]
    #[inline(always)]
    pub fn abtointclr(&self) -> AbtointclrR {
        AbtointclrR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Auto-baud start bit. This bit is automatically cleared after auto-baud completion."]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, AcrSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Auto-baud mode select bit."]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, AcrSpec> {
        ModeW::new(self, 1)
    }
    #[doc = "Bit 2 - Auto-baud restart bit."]
    #[inline(always)]
    pub fn autorestart(&mut self) -> AutorestartW<'_, AcrSpec> {
        AutorestartW::new(self, 2)
    }
    #[doc = "Bit 8 - End of auto-baud interrupt clear bit (write-only)."]
    #[inline(always)]
    pub fn abeointclr(&mut self) -> AbeointclrW<'_, AcrSpec> {
        AbeointclrW::new(self, 8)
    }
    #[doc = "Bit 9 - Auto-baud time-out interrupt clear bit (write-only)."]
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
