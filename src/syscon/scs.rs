#[doc = "Register `SCS` reader"]
pub type R = crate::R<ScsSpec>;
#[doc = "Register `SCS` writer"]
pub type W = crate::W<ScsSpec>;
#[doc = "EMC Shift Control. Controls how addresses are output on the EMC address pins for static memories. Also see Section 10.9 in the EMC chapter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Emcsc {
    #[doc = "0: Static memory addresses are shifted to match the data bus width. For example, when accessing a 32-bit wide data bus, the address is shifted right 2 places such that bit 2 is the LSB. In this mode, address bit 0 for the this device is connected to address bit 0 of the memory device, thus simplifying memory connections. This also makes a larger memory address range possible, because additional upper address bits can appear on the higher address pins due to the shift."]
    MatchDataBus = 0,
    #[doc = "1: Static memory addresses are always output as byte addresses regardless of the data bus width. For example, when word data is accessed on a 32-bit bus, address bits 1 and 0 will always be 0. In this mode, one or both lower address bits may not be connected to memories that are part of a bus that is wider than 8 bits. This mode matches the operation of LPC23xx and LPC24xx devices."]
    NoShift = 1,
}
impl From<Emcsc> for bool {
    #[inline(always)]
    fn from(variant: Emcsc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMCSC` reader - EMC Shift Control. Controls how addresses are output on the EMC address pins for static memories. Also see Section 10.9 in the EMC chapter."]
pub type EmcscR = crate::BitReader<Emcsc>;
impl EmcscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Emcsc {
        match self.bits {
            false => Emcsc::MatchDataBus,
            true => Emcsc::NoShift,
        }
    }
    #[doc = "Static memory addresses are shifted to match the data bus width. For example, when accessing a 32-bit wide data bus, the address is shifted right 2 places such that bit 2 is the LSB. In this mode, address bit 0 for the this device is connected to address bit 0 of the memory device, thus simplifying memory connections. This also makes a larger memory address range possible, because additional upper address bits can appear on the higher address pins due to the shift."]
    #[inline(always)]
    pub fn is_match_data_bus(&self) -> bool {
        *self == Emcsc::MatchDataBus
    }
    #[doc = "Static memory addresses are always output as byte addresses regardless of the data bus width. For example, when word data is accessed on a 32-bit bus, address bits 1 and 0 will always be 0. In this mode, one or both lower address bits may not be connected to memories that are part of a bus that is wider than 8 bits. This mode matches the operation of LPC23xx and LPC24xx devices."]
    #[inline(always)]
    pub fn is_no_shift(&self) -> bool {
        *self == Emcsc::NoShift
    }
}
#[doc = "Field `EMCSC` writer - EMC Shift Control. Controls how addresses are output on the EMC address pins for static memories. Also see Section 10.9 in the EMC chapter."]
pub type EmcscW<'a, REG> = crate::BitWriter<'a, REG, Emcsc>;
impl<'a, REG> EmcscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Static memory addresses are shifted to match the data bus width. For example, when accessing a 32-bit wide data bus, the address is shifted right 2 places such that bit 2 is the LSB. In this mode, address bit 0 for the this device is connected to address bit 0 of the memory device, thus simplifying memory connections. This also makes a larger memory address range possible, because additional upper address bits can appear on the higher address pins due to the shift."]
    #[inline(always)]
    pub fn match_data_bus(self) -> &'a mut crate::W<REG> {
        self.variant(Emcsc::MatchDataBus)
    }
    #[doc = "Static memory addresses are always output as byte addresses regardless of the data bus width. For example, when word data is accessed on a 32-bit bus, address bits 1 and 0 will always be 0. In this mode, one or both lower address bits may not be connected to memories that are part of a bus that is wider than 8 bits. This mode matches the operation of LPC23xx and LPC24xx devices."]
    #[inline(always)]
    pub fn no_shift(self) -> &'a mut crate::W<REG> {
        self.variant(Emcsc::NoShift)
    }
}
#[doc = "EMC Reset Disable\\[1\\]. External Memory Controller Reset Disable. Also see Section 10.8 in the EMC chapter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Emcrd {
    #[doc = "0: Both EMC resets are asserted when any type of chip reset event occurs. In this mode, all registers and functions of the EMC are initialized upon any reset condition."]
    BothEmcResetsAre_ = 0,
    #[doc = "1: Many portions of the EMC are only reset by a power-on or brown-out event, in order to allow the EMC to retain its state through a warm reset (external reset or watchdog reset). If the EMC is configured correctly, auto-refresh can be maintained through a warm reset."]
    ManyPortionsOfThe = 1,
}
impl From<Emcrd> for bool {
    #[inline(always)]
    fn from(variant: Emcrd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMCRD` reader - EMC Reset Disable\\[1\\]. External Memory Controller Reset Disable. Also see Section 10.8 in the EMC chapter."]
pub type EmcrdR = crate::BitReader<Emcrd>;
impl EmcrdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Emcrd {
        match self.bits {
            false => Emcrd::BothEmcResetsAre_,
            true => Emcrd::ManyPortionsOfThe,
        }
    }
    #[doc = "Both EMC resets are asserted when any type of chip reset event occurs. In this mode, all registers and functions of the EMC are initialized upon any reset condition."]
    #[inline(always)]
    pub fn is_both_emc_resets_are_(&self) -> bool {
        *self == Emcrd::BothEmcResetsAre_
    }
    #[doc = "Many portions of the EMC are only reset by a power-on or brown-out event, in order to allow the EMC to retain its state through a warm reset (external reset or watchdog reset). If the EMC is configured correctly, auto-refresh can be maintained through a warm reset."]
    #[inline(always)]
    pub fn is_many_portions_of_the(&self) -> bool {
        *self == Emcrd::ManyPortionsOfThe
    }
}
#[doc = "Field `EMCRD` writer - EMC Reset Disable\\[1\\]. External Memory Controller Reset Disable. Also see Section 10.8 in the EMC chapter."]
pub type EmcrdW<'a, REG> = crate::BitWriter<'a, REG, Emcrd>;
impl<'a, REG> EmcrdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Both EMC resets are asserted when any type of chip reset event occurs. In this mode, all registers and functions of the EMC are initialized upon any reset condition."]
    #[inline(always)]
    pub fn both_emc_resets_are_(self) -> &'a mut crate::W<REG> {
        self.variant(Emcrd::BothEmcResetsAre_)
    }
    #[doc = "Many portions of the EMC are only reset by a power-on or brown-out event, in order to allow the EMC to retain its state through a warm reset (external reset or watchdog reset). If the EMC is configured correctly, auto-refresh can be maintained through a warm reset."]
    #[inline(always)]
    pub fn many_portions_of_the(self) -> &'a mut crate::W<REG> {
        self.variant(Emcrd::ManyPortionsOfThe)
    }
}
#[doc = "External Memory Controller burst control. Also see Section 10.10 in the EMC chapter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Emcbc {
    #[doc = "0: Burst enabled."]
    BurstEnabled_ = 0,
    #[doc = "1: Burst disabled. This mode can be used to prevent multiple sequential accesses to memory mapped I/O devices connected to EMC static memory chip selects. These unrequested accesses can cause issues with some I/O devices."]
    BurstDisabledThis = 1,
}
impl From<Emcbc> for bool {
    #[inline(always)]
    fn from(variant: Emcbc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMCBC` reader - External Memory Controller burst control. Also see Section 10.10 in the EMC chapter."]
pub type EmcbcR = crate::BitReader<Emcbc>;
impl EmcbcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Emcbc {
        match self.bits {
            false => Emcbc::BurstEnabled_,
            true => Emcbc::BurstDisabledThis,
        }
    }
    #[doc = "Burst enabled."]
    #[inline(always)]
    pub fn is_burst_enabled_(&self) -> bool {
        *self == Emcbc::BurstEnabled_
    }
    #[doc = "Burst disabled. This mode can be used to prevent multiple sequential accesses to memory mapped I/O devices connected to EMC static memory chip selects. These unrequested accesses can cause issues with some I/O devices."]
    #[inline(always)]
    pub fn is_burst_disabled_this(&self) -> bool {
        *self == Emcbc::BurstDisabledThis
    }
}
#[doc = "Field `EMCBC` writer - External Memory Controller burst control. Also see Section 10.10 in the EMC chapter."]
pub type EmcbcW<'a, REG> = crate::BitWriter<'a, REG, Emcbc>;
impl<'a, REG> EmcbcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Burst enabled."]
    #[inline(always)]
    pub fn burst_enabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Emcbc::BurstEnabled_)
    }
    #[doc = "Burst disabled. This mode can be used to prevent multiple sequential accesses to memory mapped I/O devices connected to EMC static memory chip selects. These unrequested accesses can cause issues with some I/O devices."]
    #[inline(always)]
    pub fn burst_disabled_this(self) -> &'a mut crate::W<REG> {
        self.variant(Emcbc::BurstDisabledThis)
    }
}
#[doc = "MCIPWR Active Level\\[1\\]. Selects the active level of the SD card interface signal SD_PWR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mcipwral {
    #[doc = "0: SD_PWR is active low (inverted output of the SD Card interface block)."]
    SdPwrIsActiveLow = 0,
    #[doc = "1: SD_PWR is active high (follows the output of the SD Card interface block)."]
    SdPwrIsActiveHig = 1,
}
impl From<Mcipwral> for bool {
    #[inline(always)]
    fn from(variant: Mcipwral) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCIPWRAL` reader - MCIPWR Active Level\\[1\\]. Selects the active level of the SD card interface signal SD_PWR."]
pub type McipwralR = crate::BitReader<Mcipwral>;
impl McipwralR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mcipwral {
        match self.bits {
            false => Mcipwral::SdPwrIsActiveLow,
            true => Mcipwral::SdPwrIsActiveHig,
        }
    }
    #[doc = "SD_PWR is active low (inverted output of the SD Card interface block)."]
    #[inline(always)]
    pub fn is_sd_pwr_is_active_low(&self) -> bool {
        *self == Mcipwral::SdPwrIsActiveLow
    }
    #[doc = "SD_PWR is active high (follows the output of the SD Card interface block)."]
    #[inline(always)]
    pub fn is_sd_pwr_is_active_hig(&self) -> bool {
        *self == Mcipwral::SdPwrIsActiveHig
    }
}
#[doc = "Field `MCIPWRAL` writer - MCIPWR Active Level\\[1\\]. Selects the active level of the SD card interface signal SD_PWR."]
pub type McipwralW<'a, REG> = crate::BitWriter<'a, REG, Mcipwral>;
impl<'a, REG> McipwralW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SD_PWR is active low (inverted output of the SD Card interface block)."]
    #[inline(always)]
    pub fn sd_pwr_is_active_low(self) -> &'a mut crate::W<REG> {
        self.variant(Mcipwral::SdPwrIsActiveLow)
    }
    #[doc = "SD_PWR is active high (follows the output of the SD Card interface block)."]
    #[inline(always)]
    pub fn sd_pwr_is_active_hig(self) -> &'a mut crate::W<REG> {
        self.variant(Mcipwral::SdPwrIsActiveHig)
    }
}
#[doc = "Main oscillator range select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oscrs {
    #[doc = "0: The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    From1To20 = 0,
    #[doc = "1: The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    From15To25 = 1,
}
impl From<Oscrs> for bool {
    #[inline(always)]
    fn from(variant: Oscrs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCRS` reader - Main oscillator range select."]
pub type OscrsR = crate::BitReader<Oscrs>;
impl OscrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oscrs {
        match self.bits {
            false => Oscrs::From1To20,
            true => Oscrs::From15To25,
        }
    }
    #[doc = "The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    #[inline(always)]
    pub fn is_from_1_to_20(&self) -> bool {
        *self == Oscrs::From1To20
    }
    #[doc = "The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    #[inline(always)]
    pub fn is_from_15_to_25(&self) -> bool {
        *self == Oscrs::From15To25
    }
}
#[doc = "Field `OSCRS` writer - Main oscillator range select."]
pub type OscrsW<'a, REG> = crate::BitWriter<'a, REG, Oscrs>;
impl<'a, REG> OscrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    #[inline(always)]
    pub fn from_1_to_20(self) -> &'a mut crate::W<REG> {
        self.variant(Oscrs::From1To20)
    }
    #[doc = "The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    #[inline(always)]
    pub fn from_15_to_25(self) -> &'a mut crate::W<REG> {
        self.variant(Oscrs::From15To25)
    }
}
#[doc = "Main oscillator enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oscen {
    #[doc = "0: The main oscillator is disabled."]
    Disabled = 0,
    #[doc = "1: The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    Enabled = 1,
}
impl From<Oscen> for bool {
    #[inline(always)]
    fn from(variant: Oscen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCEN` reader - Main oscillator enable."]
pub type OscenR = crate::BitReader<Oscen>;
impl OscenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oscen {
        match self.bits {
            false => Oscen::Disabled,
            true => Oscen::Enabled,
        }
    }
    #[doc = "The main oscillator is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Oscen::Disabled
    }
    #[doc = "The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Oscen::Enabled
    }
}
#[doc = "Field `OSCEN` writer - Main oscillator enable."]
pub type OscenW<'a, REG> = crate::BitWriter<'a, REG, Oscen>;
impl<'a, REG> OscenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The main oscillator is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Oscen::Disabled)
    }
    #[doc = "The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Oscen::Enabled)
    }
}
#[doc = "Main oscillator status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oscstat {
    #[doc = "0: The main oscillator is not ready to be used as a clock source."]
    NotReady = 0,
    #[doc = "1: The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    Ready = 1,
}
impl From<Oscstat> for bool {
    #[inline(always)]
    fn from(variant: Oscstat) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCSTAT` reader - Main oscillator status."]
pub type OscstatR = crate::BitReader<Oscstat>;
impl OscstatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oscstat {
        match self.bits {
            false => Oscstat::NotReady,
            true => Oscstat::Ready,
        }
    }
    #[doc = "The main oscillator is not ready to be used as a clock source."]
    #[inline(always)]
    pub fn is_not_ready(&self) -> bool {
        *self == Oscstat::NotReady
    }
    #[doc = "The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    #[inline(always)]
    pub fn is_ready(&self) -> bool {
        *self == Oscstat::Ready
    }
}
#[doc = "Field `OSCSTAT` writer - Main oscillator status."]
pub type OscstatW<'a, REG> = crate::BitWriter<'a, REG, Oscstat>;
impl<'a, REG> OscstatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The main oscillator is not ready to be used as a clock source."]
    #[inline(always)]
    pub fn not_ready(self) -> &'a mut crate::W<REG> {
        self.variant(Oscstat::NotReady)
    }
    #[doc = "The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    #[inline(always)]
    pub fn ready(self) -> &'a mut crate::W<REG> {
        self.variant(Oscstat::Ready)
    }
}
impl R {
    #[doc = "Bit 0 - EMC Shift Control. Controls how addresses are output on the EMC address pins for static memories. Also see Section 10.9 in the EMC chapter."]
    #[inline(always)]
    pub fn emcsc(&self) -> EmcscR {
        EmcscR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EMC Reset Disable\\[1\\]. External Memory Controller Reset Disable. Also see Section 10.8 in the EMC chapter."]
    #[inline(always)]
    pub fn emcrd(&self) -> EmcrdR {
        EmcrdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Memory Controller burst control. Also see Section 10.10 in the EMC chapter."]
    #[inline(always)]
    pub fn emcbc(&self) -> EmcbcR {
        EmcbcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - MCIPWR Active Level\\[1\\]. Selects the active level of the SD card interface signal SD_PWR."]
    #[inline(always)]
    pub fn mcipwral(&self) -> McipwralR {
        McipwralR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Main oscillator range select."]
    #[inline(always)]
    pub fn oscrs(&self) -> OscrsR {
        OscrsR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Main oscillator enable."]
    #[inline(always)]
    pub fn oscen(&self) -> OscenR {
        OscenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Main oscillator status."]
    #[inline(always)]
    pub fn oscstat(&self) -> OscstatR {
        OscstatR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EMC Shift Control. Controls how addresses are output on the EMC address pins for static memories. Also see Section 10.9 in the EMC chapter."]
    #[inline(always)]
    pub fn emcsc(&mut self) -> EmcscW<'_, ScsSpec> {
        EmcscW::new(self, 0)
    }
    #[doc = "Bit 1 - EMC Reset Disable\\[1\\]. External Memory Controller Reset Disable. Also see Section 10.8 in the EMC chapter."]
    #[inline(always)]
    pub fn emcrd(&mut self) -> EmcrdW<'_, ScsSpec> {
        EmcrdW::new(self, 1)
    }
    #[doc = "Bit 2 - External Memory Controller burst control. Also see Section 10.10 in the EMC chapter."]
    #[inline(always)]
    pub fn emcbc(&mut self) -> EmcbcW<'_, ScsSpec> {
        EmcbcW::new(self, 2)
    }
    #[doc = "Bit 3 - MCIPWR Active Level\\[1\\]. Selects the active level of the SD card interface signal SD_PWR."]
    #[inline(always)]
    pub fn mcipwral(&mut self) -> McipwralW<'_, ScsSpec> {
        McipwralW::new(self, 3)
    }
    #[doc = "Bit 4 - Main oscillator range select."]
    #[inline(always)]
    pub fn oscrs(&mut self) -> OscrsW<'_, ScsSpec> {
        OscrsW::new(self, 4)
    }
    #[doc = "Bit 5 - Main oscillator enable."]
    #[inline(always)]
    pub fn oscen(&mut self) -> OscenW<'_, ScsSpec> {
        OscenW::new(self, 5)
    }
    #[doc = "Bit 6 - Main oscillator status."]
    #[inline(always)]
    pub fn oscstat(&mut self) -> OscstatW<'_, ScsSpec> {
        OscstatW::new(self, 6)
    }
}
#[doc = "System Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`scs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScsSpec;
impl crate::RegisterSpec for ScsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scs::R`](R) reader structure"]
impl crate::Readable for ScsSpec {}
#[doc = "`write(|w| ..)` method takes [`scs::W`](W) writer structure"]
impl crate::Writable for ScsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCS to value 0"]
impl crate::Resettable for ScsSpec {}
