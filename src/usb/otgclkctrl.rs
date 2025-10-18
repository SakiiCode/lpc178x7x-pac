#[doc = "Register `OTGCLKCTRL` reader"]
pub type R = crate::R<OtgclkctrlSpec>;
#[doc = "Register `OTGCLKCTRL` writer"]
pub type W = crate::W<OtgclkctrlSpec>;
#[doc = "Host clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disable the Host clock."]
    DisableTheHostClo = 0,
    #[doc = "1: Enable the Host clock."]
    EnableTheHostCloc = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOST_CLK_EN` reader - Host clock enable"]
pub type HostClkEnR = crate::BitReader<Enum>;
impl HostClkEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::DisableTheHostClo,
            true => Enum::EnableTheHostCloc,
        }
    }
    #[doc = "Disable the Host clock."]
    #[inline(always)]
    pub fn is_disable_the_host_clo(&self) -> bool {
        *self == Enum::DisableTheHostClo
    }
    #[doc = "Enable the Host clock."]
    #[inline(always)]
    pub fn is_enable_the_host_cloc(&self) -> bool {
        *self == Enum::EnableTheHostCloc
    }
}
#[doc = "Field `HOST_CLK_EN` writer - Host clock enable"]
pub type HostClkEnW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> HostClkEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the Host clock."]
    #[inline(always)]
    pub fn disable_the_host_clo(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::DisableTheHostClo)
    }
    #[doc = "Enable the Host clock."]
    #[inline(always)]
    pub fn enable_the_host_cloc(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::EnableTheHostCloc)
    }
}
#[doc = "Device clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disable the Device clock."]
    DisableTheDeviceC = 0,
    #[doc = "1: Enable the Device clock."]
    EnableTheDeviceCl = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEV_CLK_EN` reader - Device clock enable"]
pub type DevClkEnR = crate::BitReader<Enum>;
impl DevClkEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::DisableTheDeviceC,
            true => Enum::EnableTheDeviceCl,
        }
    }
    #[doc = "Disable the Device clock."]
    #[inline(always)]
    pub fn is_disable_the_device_c(&self) -> bool {
        *self == Enum::DisableTheDeviceC
    }
    #[doc = "Enable the Device clock."]
    #[inline(always)]
    pub fn is_enable_the_device_cl(&self) -> bool {
        *self == Enum::EnableTheDeviceCl
    }
}
#[doc = "Field `DEV_CLK_EN` writer - Device clock enable"]
pub type DevClkEnW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> DevClkEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the Device clock."]
    #[inline(always)]
    pub fn disable_the_device_c(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::DisableTheDeviceC)
    }
    #[doc = "Enable the Device clock."]
    #[inline(always)]
    pub fn enable_the_device_cl(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::EnableTheDeviceCl)
    }
}
#[doc = "I2C clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disable the I2C clock."]
    DisableTheI2cCloc = 0,
    #[doc = "1: Enable the I2C clock."]
    EnableTheI2cClock = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_CLK_EN` reader - I2C clock enable"]
pub type I2cClkEnR = crate::BitReader<Enum>;
impl I2cClkEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::DisableTheI2cCloc,
            true => Enum::EnableTheI2cClock,
        }
    }
    #[doc = "Disable the I2C clock."]
    #[inline(always)]
    pub fn is_disable_the_i2c_cloc(&self) -> bool {
        *self == Enum::DisableTheI2cCloc
    }
    #[doc = "Enable the I2C clock."]
    #[inline(always)]
    pub fn is_enable_the_i2c_clock(&self) -> bool {
        *self == Enum::EnableTheI2cClock
    }
}
#[doc = "Field `I2C_CLK_EN` writer - I2C clock enable"]
pub type I2cClkEnW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> I2cClkEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the I2C clock."]
    #[inline(always)]
    pub fn disable_the_i2c_cloc(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::DisableTheI2cCloc)
    }
    #[doc = "Enable the I2C clock."]
    #[inline(always)]
    pub fn enable_the_i2c_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::EnableTheI2cClock)
    }
}
#[doc = "OTG clock enable. In device-only applications, this bit enables access to the PORTSEL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disable the OTG clock."]
    DisableTheOtgCloc = 0,
    #[doc = "1: Enable the OTG clock."]
    EnableTheOtgClock = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTG_CLK_EN` reader - OTG clock enable. In device-only applications, this bit enables access to the PORTSEL register."]
pub type OtgClkEnR = crate::BitReader<Enum>;
impl OtgClkEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::DisableTheOtgCloc,
            true => Enum::EnableTheOtgClock,
        }
    }
    #[doc = "Disable the OTG clock."]
    #[inline(always)]
    pub fn is_disable_the_otg_cloc(&self) -> bool {
        *self == Enum::DisableTheOtgCloc
    }
    #[doc = "Enable the OTG clock."]
    #[inline(always)]
    pub fn is_enable_the_otg_clock(&self) -> bool {
        *self == Enum::EnableTheOtgClock
    }
}
#[doc = "Field `OTG_CLK_EN` writer - OTG clock enable. In device-only applications, this bit enables access to the PORTSEL register."]
pub type OtgClkEnW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> OtgClkEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the OTG clock."]
    #[inline(always)]
    pub fn disable_the_otg_cloc(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::DisableTheOtgCloc)
    }
    #[doc = "Enable the OTG clock."]
    #[inline(always)]
    pub fn enable_the_otg_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::EnableTheOtgClock)
    }
}
#[doc = "AHB master clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disable the AHB clock."]
    DisableTheAhbCloc = 0,
    #[doc = "1: Enable the AHB clock."]
    EnableTheAhbClock = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHB_CLK_EN` reader - AHB master clock enable"]
pub type AhbClkEnR = crate::BitReader<Enum>;
impl AhbClkEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::DisableTheAhbCloc,
            true => Enum::EnableTheAhbClock,
        }
    }
    #[doc = "Disable the AHB clock."]
    #[inline(always)]
    pub fn is_disable_the_ahb_cloc(&self) -> bool {
        *self == Enum::DisableTheAhbCloc
    }
    #[doc = "Enable the AHB clock."]
    #[inline(always)]
    pub fn is_enable_the_ahb_clock(&self) -> bool {
        *self == Enum::EnableTheAhbClock
    }
}
#[doc = "Field `AHB_CLK_EN` writer - AHB master clock enable"]
pub type AhbClkEnW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> AhbClkEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the AHB clock."]
    #[inline(always)]
    pub fn disable_the_ahb_cloc(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::DisableTheAhbCloc)
    }
    #[doc = "Enable the AHB clock."]
    #[inline(always)]
    pub fn enable_the_ahb_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::EnableTheAhbClock)
    }
}
impl R {
    #[doc = "Bit 0 - Host clock enable"]
    #[inline(always)]
    pub fn host_clk_en(&self) -> HostClkEnR {
        HostClkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Device clock enable"]
    #[inline(always)]
    pub fn dev_clk_en(&self) -> DevClkEnR {
        DevClkEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C clock enable"]
    #[inline(always)]
    pub fn i2c_clk_en(&self) -> I2cClkEnR {
        I2cClkEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - OTG clock enable. In device-only applications, this bit enables access to the PORTSEL register."]
    #[inline(always)]
    pub fn otg_clk_en(&self) -> OtgClkEnR {
        OtgClkEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AHB master clock enable"]
    #[inline(always)]
    pub fn ahb_clk_en(&self) -> AhbClkEnR {
        AhbClkEnR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Host clock enable"]
    #[inline(always)]
    pub fn host_clk_en(&mut self) -> HostClkEnW<'_, OtgclkctrlSpec> {
        HostClkEnW::new(self, 0)
    }
    #[doc = "Bit 1 - Device clock enable"]
    #[inline(always)]
    pub fn dev_clk_en(&mut self) -> DevClkEnW<'_, OtgclkctrlSpec> {
        DevClkEnW::new(self, 1)
    }
    #[doc = "Bit 2 - I2C clock enable"]
    #[inline(always)]
    pub fn i2c_clk_en(&mut self) -> I2cClkEnW<'_, OtgclkctrlSpec> {
        I2cClkEnW::new(self, 2)
    }
    #[doc = "Bit 3 - OTG clock enable. In device-only applications, this bit enables access to the PORTSEL register."]
    #[inline(always)]
    pub fn otg_clk_en(&mut self) -> OtgClkEnW<'_, OtgclkctrlSpec> {
        OtgClkEnW::new(self, 3)
    }
    #[doc = "Bit 4 - AHB master clock enable"]
    #[inline(always)]
    pub fn ahb_clk_en(&mut self) -> AhbClkEnW<'_, OtgclkctrlSpec> {
        AhbClkEnW::new(self, 4)
    }
}
#[doc = "OTG clock controller\n\nYou can [`read`](crate::Reg::read) this register and get [`otgclkctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otgclkctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgclkctrlSpec;
impl crate::RegisterSpec for OtgclkctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otgclkctrl::R`](R) reader structure"]
impl crate::Readable for OtgclkctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`otgclkctrl::W`](W) writer structure"]
impl crate::Writable for OtgclkctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTGCLKCTRL to value 0"]
impl crate::Resettable for OtgclkctrlSpec {}
