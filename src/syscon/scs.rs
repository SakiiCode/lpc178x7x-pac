#[doc = "Register `SCS` reader"]
pub type R = crate::R<ScsSpec>;
#[doc = "Register `SCS` writer"]
pub type W = crate::W<ScsSpec>;
#[doc = "EMC Shift Control. Controls how addresses are output on the EMC address pins for static memories. Also see Section 10.9 in the EMC chapter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Static memory addresses are shifted to match the data bus width. For example, when accessing a 32-bit wide data bus, the address is shifted right 2 places such that bit 2 is the LSB. In this mode, address bit 0 for the this device is connected to address bit 0 of the memory device, thus simplifying memory connections. This also makes a larger memory address range possible, because additional upper address bits can appear on the higher address pins due to the shift."]
    StaticMemoryAddres = 0,
    #[doc = "1: Static memory addresses are always output as byte addresses regardless of the data bus width. For example, when word data is accessed on a 32-bit bus, address bits 1 and 0 will always be 0. In this mode, one or both lower address bits may not be connected to memories that are part of a bus that is wider than 8 bits. This mode matches the operation of LPC23xx and LPC24xx devices."]
    StaticMemoryAddres = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMCSC` reader - EMC Shift Control. Controls how addresses are output on the EMC address pins for static memories. Also see Section 10.9 in the EMC chapter."]
pub type EmcscR = crate::BitReader<Enum>;
impl EmcscR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::StaticMemoryAddres,
            true => Enum::StaticMemoryAddres,
        }
    }
    #[doc = "Static memory addresses are shifted to match the data bus width. For example, when accessing a 32-bit wide data bus, the address is shifted right 2 places such that bit 2 is the LSB. In this mode, address bit 0 for the this device is connected to address bit 0 of the memory device, thus simplifying memory connections. This also makes a larger memory address range possible, because additional upper address bits can appear on the higher address pins due to the shift."]
    #[inline(always)]
    pub fn is_static_memory_addres(&self) -> bool {
        *self == Enum::StaticMemoryAddres
    }
    #[doc = "Static memory addresses are always output as byte addresses regardless of the data bus width. For example, when word data is accessed on a 32-bit bus, address bits 1 and 0 will always be 0. In this mode, one or both lower address bits may not be connected to memories that are part of a bus that is wider than 8 bits. This mode matches the operation of LPC23xx and LPC24xx devices."]
    #[inline(always)]
    pub fn is_static_memory_addres(&self) -> bool {
        *self == Enum::StaticMemoryAddres
    }
}
#[doc = "Field `EMCSC` writer - EMC Shift Control. Controls how addresses are output on the EMC address pins for static memories. Also see Section 10.9 in the EMC chapter."]
pub type EmcscW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> EmcscW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Static memory addresses are shifted to match the data bus width. For example, when accessing a 32-bit wide data bus, the address is shifted right 2 places such that bit 2 is the LSB. In this mode, address bit 0 for the this device is connected to address bit 0 of the memory device, thus simplifying memory connections. This also makes a larger memory address range possible, because additional upper address bits can appear on the higher address pins due to the shift."]
    #[inline(always)]
    pub fn static_memory_addres(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::StaticMemoryAddres)
    }
    #[doc = "Static memory addresses are always output as byte addresses regardless of the data bus width. For example, when word data is accessed on a 32-bit bus, address bits 1 and 0 will always be 0. In this mode, one or both lower address bits may not be connected to memories that are part of a bus that is wider than 8 bits. This mode matches the operation of LPC23xx and LPC24xx devices."]
    #[inline(always)]
    pub fn static_memory_addres(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::StaticMemoryAddres)
    }
}
#[doc = "EMC Reset Disable\\[1\\]. External Memory Controller Reset Disable. Also see Section 10.8 in the EMC chapter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Both EMC resets are asserted when any type of chip reset event occurs. In this mode, all registers and functions of the EMC are initialized upon any reset condition."]
    BothEmcResetsAre_ = 0,
    #[doc = "1: Many portions of the EMC are only reset by a power-on or brown-out event, in order to allow the EMC to retain its state through a warm reset (external reset or watchdog reset). If the EMC is configured correctly, auto-refresh can be maintained through a warm reset."]
    ManyPortionsOfThe = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMCRD` reader - EMC Reset Disable\\[1\\]. External Memory Controller Reset Disable. Also see Section 10.8 in the EMC chapter."]
pub type EmcrdR = crate::BitReader<Enum>;
impl EmcrdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::BothEmcResetsAre_,
            true => Enum::ManyPortionsOfThe,
        }
    }
    #[doc = "Both EMC resets are asserted when any type of chip reset event occurs. In this mode, all registers and functions of the EMC are initialized upon any reset condition."]
    #[inline(always)]
    pub fn is_both_emc_resets_are_(&self) -> bool {
        *self == Enum::BothEmcResetsAre_
    }
    #[doc = "Many portions of the EMC are only reset by a power-on or brown-out event, in order to allow the EMC to retain its state through a warm reset (external reset or watchdog reset). If the EMC is configured correctly, auto-refresh can be maintained through a warm reset."]
    #[inline(always)]
    pub fn is_many_portions_of_the(&self) -> bool {
        *self == Enum::ManyPortionsOfThe
    }
}
#[doc = "Field `EMCRD` writer - EMC Reset Disable\\[1\\]. External Memory Controller Reset Disable. Also see Section 10.8 in the EMC chapter."]
pub type EmcrdW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> EmcrdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Both EMC resets are asserted when any type of chip reset event occurs. In this mode, all registers and functions of the EMC are initialized upon any reset condition."]
    #[inline(always)]
    pub fn both_emc_resets_are_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::BothEmcResetsAre_)
    }
    #[doc = "Many portions of the EMC are only reset by a power-on or brown-out event, in order to allow the EMC to retain its state through a warm reset (external reset or watchdog reset). If the EMC is configured correctly, auto-refresh can be maintained through a warm reset."]
    #[inline(always)]
    pub fn many_portions_of_the(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::ManyPortionsOfThe)
    }
}
#[doc = "External Memory Controller burst control. Also see Section 10.10 in the EMC chapter.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Burst enabled."]
    BurstEnabled_ = 0,
    #[doc = "1: Burst disabled. This mode can be used to prevent multiple sequential accesses to memory mapped I/O devices connected to EMC static memory chip selects. These unrequested accesses can cause issues with some I/O devices."]
    BurstDisabledThis = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EMCBC` reader - External Memory Controller burst control. Also see Section 10.10 in the EMC chapter."]
pub type EmcbcR = crate::BitReader<Enum>;
impl EmcbcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::BurstEnabled_,
            true => Enum::BurstDisabledThis,
        }
    }
    #[doc = "Burst enabled."]
    #[inline(always)]
    pub fn is_burst_enabled_(&self) -> bool {
        *self == Enum::BurstEnabled_
    }
    #[doc = "Burst disabled. This mode can be used to prevent multiple sequential accesses to memory mapped I/O devices connected to EMC static memory chip selects. These unrequested accesses can cause issues with some I/O devices."]
    #[inline(always)]
    pub fn is_burst_disabled_this(&self) -> bool {
        *self == Enum::BurstDisabledThis
    }
}
#[doc = "Field `EMCBC` writer - External Memory Controller burst control. Also see Section 10.10 in the EMC chapter."]
pub type EmcbcW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> EmcbcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Burst enabled."]
    #[inline(always)]
    pub fn burst_enabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::BurstEnabled_)
    }
    #[doc = "Burst disabled. This mode can be used to prevent multiple sequential accesses to memory mapped I/O devices connected to EMC static memory chip selects. These unrequested accesses can cause issues with some I/O devices."]
    #[inline(always)]
    pub fn burst_disabled_this(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::BurstDisabledThis)
    }
}
#[doc = "MCIPWR Active Level\\[1\\]. Selects the active level of the SD card interface signal SD_PWR.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: SD_PWR is active low (inverted output of the SD Card interface block)."]
    SdPwrIsActiveLow = 0,
    #[doc = "1: SD_PWR is active high (follows the output of the SD Card interface block)."]
    SdPwrIsActiveHig = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MCIPWRAL` reader - MCIPWR Active Level\\[1\\]. Selects the active level of the SD card interface signal SD_PWR."]
pub type McipwralR = crate::BitReader<Enum>;
impl McipwralR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::SdPwrIsActiveLow,
            true => Enum::SdPwrIsActiveHig,
        }
    }
    #[doc = "SD_PWR is active low (inverted output of the SD Card interface block)."]
    #[inline(always)]
    pub fn is_sd_pwr_is_active_low(&self) -> bool {
        *self == Enum::SdPwrIsActiveLow
    }
    #[doc = "SD_PWR is active high (follows the output of the SD Card interface block)."]
    #[inline(always)]
    pub fn is_sd_pwr_is_active_hig(&self) -> bool {
        *self == Enum::SdPwrIsActiveHig
    }
}
#[doc = "Field `MCIPWRAL` writer - MCIPWR Active Level\\[1\\]. Selects the active level of the SD card interface signal SD_PWR."]
pub type McipwralW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> McipwralW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SD_PWR is active low (inverted output of the SD Card interface block)."]
    #[inline(always)]
    pub fn sd_pwr_is_active_low(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::SdPwrIsActiveLow)
    }
    #[doc = "SD_PWR is active high (follows the output of the SD Card interface block)."]
    #[inline(always)]
    pub fn sd_pwr_is_active_hig(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::SdPwrIsActiveHig)
    }
}
#[doc = "Main oscillator range select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    TheFrequencyRange_ = 0,
    #[doc = "1: The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    TheFrequencyRange_ = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCRS` reader - Main oscillator range select."]
pub type OscrsR = crate::BitReader<Enum>;
impl OscrsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::TheFrequencyRange_,
            true => Enum::TheFrequencyRange_,
        }
    }
    #[doc = "The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    #[inline(always)]
    pub fn is_the_frequency_range_(&self) -> bool {
        *self == Enum::TheFrequencyRange_
    }
    #[doc = "The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    #[inline(always)]
    pub fn is_the_frequency_range_(&self) -> bool {
        *self == Enum::TheFrequencyRange_
    }
}
#[doc = "Field `OSCRS` writer - Main oscillator range select."]
pub type OscrsW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> OscrsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The frequency range of the main oscillator is 1 MHz to 20 MHz."]
    #[inline(always)]
    pub fn the_frequency_range_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::TheFrequencyRange_)
    }
    #[doc = "The frequency range of the main oscillator is 15 MHz to 25 MHz."]
    #[inline(always)]
    pub fn the_frequency_range_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::TheFrequencyRange_)
    }
}
#[doc = "Main oscillator enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: The main oscillator is disabled."]
    TheMainOscillator_ = 0,
    #[doc = "1: The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    TheMainOscillator_ = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCEN` reader - Main oscillator enable."]
pub type OscenR = crate::BitReader<Enum>;
impl OscenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::TheMainOscillator_,
            true => Enum::TheMainOscillator_,
        }
    }
    #[doc = "The main oscillator is disabled."]
    #[inline(always)]
    pub fn is_the_main_oscillator_(&self) -> bool {
        *self == Enum::TheMainOscillator_
    }
    #[doc = "The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    #[inline(always)]
    pub fn is_the_main_oscillator_(&self) -> bool {
        *self == Enum::TheMainOscillator_
    }
}
#[doc = "Field `OSCEN` writer - Main oscillator enable."]
pub type OscenW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> OscenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The main oscillator is disabled."]
    #[inline(always)]
    pub fn the_main_oscillator_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::TheMainOscillator_)
    }
    #[doc = "The main oscillator is enabled, and will start up if the correct external circuitry is connected to the XTAL1 and XTAL2 pins."]
    #[inline(always)]
    pub fn the_main_oscillator_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::TheMainOscillator_)
    }
}
#[doc = "Main oscillator status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: The main oscillator is not ready to be used as a clock source."]
    TheMainOscillator_ = 0,
    #[doc = "1: The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    TheMainOscillator_ = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OSCSTAT` reader - Main oscillator status."]
pub type OscstatR = crate::BitReader<Enum>;
impl OscstatR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::TheMainOscillator_,
            true => Enum::TheMainOscillator_,
        }
    }
    #[doc = "The main oscillator is not ready to be used as a clock source."]
    #[inline(always)]
    pub fn is_the_main_oscillator_(&self) -> bool {
        *self == Enum::TheMainOscillator_
    }
    #[doc = "The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    #[inline(always)]
    pub fn is_the_main_oscillator_(&self) -> bool {
        *self == Enum::TheMainOscillator_
    }
}
#[doc = "Field `OSCSTAT` writer - Main oscillator status."]
pub type OscstatW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> OscstatW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The main oscillator is not ready to be used as a clock source."]
    #[inline(always)]
    pub fn the_main_oscillator_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::TheMainOscillator_)
    }
    #[doc = "The main oscillator is ready to be used as a clock source. The main oscillator must be enabled via the OSCEN bit."]
    #[inline(always)]
    pub fn the_main_oscillator_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::TheMainOscillator_)
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
