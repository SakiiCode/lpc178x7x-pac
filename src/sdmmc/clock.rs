#[doc = "Register `CLOCK` reader"]
pub type R = crate::R<ClockSpec>;
#[doc = "Register `CLOCK` writer"]
pub type W = crate::W<ClockSpec>;
#[doc = "Field `CLKDIV` reader - Bus clock period: SD_CLK frequency = MCLK / \\[2x(ClkDiv+1)\\]."]
pub type ClkdivR = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - Bus clock period: SD_CLK frequency = MCLK / \\[2x(ClkDiv+1)\\]."]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Enable SD card bus clock:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enable {
    #[doc = "0: Clock disabled."]
    ClockDisabled_ = 0,
    #[doc = "1: Clock enabled."]
    ClockEnabled_ = 1,
}
impl From<Enable> for bool {
    #[inline(always)]
    fn from(variant: Enable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENABLE` reader - Enable SD card bus clock:"]
pub type EnableR = crate::BitReader<Enable>;
impl EnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enable {
        match self.bits {
            false => Enable::ClockDisabled_,
            true => Enable::ClockEnabled_,
        }
    }
    #[doc = "Clock disabled."]
    #[inline(always)]
    pub fn is_clock_disabled_(&self) -> bool {
        *self == Enable::ClockDisabled_
    }
    #[doc = "Clock enabled."]
    #[inline(always)]
    pub fn is_clock_enabled_(&self) -> bool {
        *self == Enable::ClockEnabled_
    }
}
#[doc = "Field `ENABLE` writer - Enable SD card bus clock:"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG, Enable>;
impl<'a, REG> EnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock disabled."]
    #[inline(always)]
    pub fn clock_disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::ClockDisabled_)
    }
    #[doc = "Clock enabled."]
    #[inline(always)]
    pub fn clock_enabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enable::ClockEnabled_)
    }
}
#[doc = "Disable SD_CLK output when bus is idle:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pwrsave {
    #[doc = "0: Always enabled."]
    AlwaysEnabled = 0,
    #[doc = "1: Clock enabled when bus is active."]
    ActiveBusOnly = 1,
}
impl From<Pwrsave> for bool {
    #[inline(always)]
    fn from(variant: Pwrsave) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PWRSAVE` reader - Disable SD_CLK output when bus is idle:"]
pub type PwrsaveR = crate::BitReader<Pwrsave>;
impl PwrsaveR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pwrsave {
        match self.bits {
            false => Pwrsave::AlwaysEnabled,
            true => Pwrsave::ActiveBusOnly,
        }
    }
    #[doc = "Always enabled."]
    #[inline(always)]
    pub fn is_always_enabled(&self) -> bool {
        *self == Pwrsave::AlwaysEnabled
    }
    #[doc = "Clock enabled when bus is active."]
    #[inline(always)]
    pub fn is_active_bus_only(&self) -> bool {
        *self == Pwrsave::ActiveBusOnly
    }
}
#[doc = "Field `PWRSAVE` writer - Disable SD_CLK output when bus is idle:"]
pub type PwrsaveW<'a, REG> = crate::BitWriter<'a, REG, Pwrsave>;
impl<'a, REG> PwrsaveW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Always enabled."]
    #[inline(always)]
    pub fn always_enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrsave::AlwaysEnabled)
    }
    #[doc = "Clock enabled when bus is active."]
    #[inline(always)]
    pub fn active_bus_only(self) -> &'a mut crate::W<REG> {
        self.variant(Pwrsave::ActiveBusOnly)
    }
}
#[doc = "Enable bypass of clock divide logic:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bypass {
    #[doc = "0: Disable bypass."]
    Disabled = 0,
    #[doc = "1: Enable bypass. MCLK driven to card bus output (SD_CLK)."]
    Enabled = 1,
}
impl From<Bypass> for bool {
    #[inline(always)]
    fn from(variant: Bypass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BYPASS` reader - Enable bypass of clock divide logic:"]
pub type BypassR = crate::BitReader<Bypass>;
impl BypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bypass {
        match self.bits {
            false => Bypass::Disabled,
            true => Bypass::Enabled,
        }
    }
    #[doc = "Disable bypass."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Bypass::Disabled
    }
    #[doc = "Enable bypass. MCLK driven to card bus output (SD_CLK)."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Bypass::Enabled
    }
}
#[doc = "Field `BYPASS` writer - Enable bypass of clock divide logic:"]
pub type BypassW<'a, REG> = crate::BitWriter<'a, REG, Bypass>;
impl<'a, REG> BypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable bypass."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Bypass::Disabled)
    }
    #[doc = "Enable bypass. MCLK driven to card bus output (SD_CLK)."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Bypass::Enabled)
    }
}
#[doc = "Enable wide bus mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Widebus {
    #[doc = "0: Standard bus mode (only SD_DAT\\[0\\] used)."]
    Standard = 0,
    #[doc = "1: Wide bus mode (SD_DAT\\[3:0\\] used)"]
    Wide = 1,
}
impl From<Widebus> for bool {
    #[inline(always)]
    fn from(variant: Widebus) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WIDEBUS` reader - Enable wide bus mode."]
pub type WidebusR = crate::BitReader<Widebus>;
impl WidebusR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Widebus {
        match self.bits {
            false => Widebus::Standard,
            true => Widebus::Wide,
        }
    }
    #[doc = "Standard bus mode (only SD_DAT\\[0\\] used)."]
    #[inline(always)]
    pub fn is_standard(&self) -> bool {
        *self == Widebus::Standard
    }
    #[doc = "Wide bus mode (SD_DAT\\[3:0\\] used)"]
    #[inline(always)]
    pub fn is_wide(&self) -> bool {
        *self == Widebus::Wide
    }
}
#[doc = "Field `WIDEBUS` writer - Enable wide bus mode."]
pub type WidebusW<'a, REG> = crate::BitWriter<'a, REG, Widebus>;
impl<'a, REG> WidebusW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Standard bus mode (only SD_DAT\\[0\\] used)."]
    #[inline(always)]
    pub fn standard(self) -> &'a mut crate::W<REG> {
        self.variant(Widebus::Standard)
    }
    #[doc = "Wide bus mode (SD_DAT\\[3:0\\] used)"]
    #[inline(always)]
    pub fn wide(self) -> &'a mut crate::W<REG> {
        self.variant(Widebus::Wide)
    }
}
impl R {
    #[doc = "Bits 0:7 - Bus clock period: SD_CLK frequency = MCLK / \\[2x(ClkDiv+1)\\]."]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Enable SD card bus clock:"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Disable SD_CLK output when bus is idle:"]
    #[inline(always)]
    pub fn pwrsave(&self) -> PwrsaveR {
        PwrsaveR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable bypass of clock divide logic:"]
    #[inline(always)]
    pub fn bypass(&self) -> BypassR {
        BypassR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable wide bus mode."]
    #[inline(always)]
    pub fn widebus(&self) -> WidebusR {
        WidebusR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bus clock period: SD_CLK frequency = MCLK / \\[2x(ClkDiv+1)\\]."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> ClkdivW<'_, ClockSpec> {
        ClkdivW::new(self, 0)
    }
    #[doc = "Bit 8 - Enable SD card bus clock:"]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, ClockSpec> {
        EnableW::new(self, 8)
    }
    #[doc = "Bit 9 - Disable SD_CLK output when bus is idle:"]
    #[inline(always)]
    pub fn pwrsave(&mut self) -> PwrsaveW<'_, ClockSpec> {
        PwrsaveW::new(self, 9)
    }
    #[doc = "Bit 10 - Enable bypass of clock divide logic:"]
    #[inline(always)]
    pub fn bypass(&mut self) -> BypassW<'_, ClockSpec> {
        BypassW::new(self, 10)
    }
    #[doc = "Bit 11 - Enable wide bus mode."]
    #[inline(always)]
    pub fn widebus(&mut self) -> WidebusW<'_, ClockSpec> {
        WidebusW::new(self, 11)
    }
}
#[doc = "Clock control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClockSpec;
impl crate::RegisterSpec for ClockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clock::R`](R) reader structure"]
impl crate::Readable for ClockSpec {}
#[doc = "`write(|w| ..)` method takes [`clock::W`](W) writer structure"]
impl crate::Writable for ClockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLOCK to value 0"]
impl crate::Resettable for ClockSpec {}
