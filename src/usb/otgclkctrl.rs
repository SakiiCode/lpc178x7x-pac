#[doc = "Register `OTGCLKCTRL` reader"]
pub type R = crate::R<OtgclkctrlSpec>;
#[doc = "Register `OTGCLKCTRL` writer"]
pub type W = crate::W<OtgclkctrlSpec>;
#[doc = "Host clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HostClkEn {
    #[doc = "0: Disable the Host clock."]
    Disabled = 0,
    #[doc = "1: Enable the Host clock."]
    Enabled = 1,
}
impl From<HostClkEn> for bool {
    #[inline(always)]
    fn from(variant: HostClkEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HOST_CLK_EN` reader - Host clock enable"]
pub type HostClkEnR = crate::BitReader<HostClkEn>;
impl HostClkEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HostClkEn {
        match self.bits {
            false => HostClkEn::Disabled,
            true => HostClkEn::Enabled,
        }
    }
    #[doc = "Disable the Host clock."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HostClkEn::Disabled
    }
    #[doc = "Enable the Host clock."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HostClkEn::Enabled
    }
}
#[doc = "Field `HOST_CLK_EN` writer - Host clock enable"]
pub type HostClkEnW<'a, REG> = crate::BitWriter<'a, REG, HostClkEn>;
impl<'a, REG> HostClkEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the Host clock."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(HostClkEn::Disabled)
    }
    #[doc = "Enable the Host clock."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(HostClkEn::Enabled)
    }
}
#[doc = "Device clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DevClkEn {
    #[doc = "0: Disable the Device clock."]
    Disabled = 0,
    #[doc = "1: Enable the Device clock."]
    Enabled = 1,
}
impl From<DevClkEn> for bool {
    #[inline(always)]
    fn from(variant: DevClkEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DEV_CLK_EN` reader - Device clock enable"]
pub type DevClkEnR = crate::BitReader<DevClkEn>;
impl DevClkEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DevClkEn {
        match self.bits {
            false => DevClkEn::Disabled,
            true => DevClkEn::Enabled,
        }
    }
    #[doc = "Disable the Device clock."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DevClkEn::Disabled
    }
    #[doc = "Enable the Device clock."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DevClkEn::Enabled
    }
}
#[doc = "Field `DEV_CLK_EN` writer - Device clock enable"]
pub type DevClkEnW<'a, REG> = crate::BitWriter<'a, REG, DevClkEn>;
impl<'a, REG> DevClkEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the Device clock."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(DevClkEn::Disabled)
    }
    #[doc = "Enable the Device clock."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(DevClkEn::Enabled)
    }
}
#[doc = "I2C clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2cClkEn {
    #[doc = "0: Disable the I2C clock."]
    Disabled = 0,
    #[doc = "1: Enable the I2C clock."]
    Enabled = 1,
}
impl From<I2cClkEn> for bool {
    #[inline(always)]
    fn from(variant: I2cClkEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I2C_CLK_EN` reader - I2C clock enable"]
pub type I2cClkEnR = crate::BitReader<I2cClkEn>;
impl I2cClkEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I2cClkEn {
        match self.bits {
            false => I2cClkEn::Disabled,
            true => I2cClkEn::Enabled,
        }
    }
    #[doc = "Disable the I2C clock."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == I2cClkEn::Disabled
    }
    #[doc = "Enable the I2C clock."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == I2cClkEn::Enabled
    }
}
#[doc = "Field `I2C_CLK_EN` writer - I2C clock enable"]
pub type I2cClkEnW<'a, REG> = crate::BitWriter<'a, REG, I2cClkEn>;
impl<'a, REG> I2cClkEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the I2C clock."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2cClkEn::Disabled)
    }
    #[doc = "Enable the I2C clock."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(I2cClkEn::Enabled)
    }
}
#[doc = "OTG clock enable. In device-only applications, this bit enables access to the PORTSEL register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OtgClkEn {
    #[doc = "0: Disable the OTG clock."]
    Disabled = 0,
    #[doc = "1: Enable the OTG clock."]
    Enabled = 1,
}
impl From<OtgClkEn> for bool {
    #[inline(always)]
    fn from(variant: OtgClkEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OTG_CLK_EN` reader - OTG clock enable. In device-only applications, this bit enables access to the PORTSEL register."]
pub type OtgClkEnR = crate::BitReader<OtgClkEn>;
impl OtgClkEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OtgClkEn {
        match self.bits {
            false => OtgClkEn::Disabled,
            true => OtgClkEn::Enabled,
        }
    }
    #[doc = "Disable the OTG clock."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OtgClkEn::Disabled
    }
    #[doc = "Enable the OTG clock."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OtgClkEn::Enabled
    }
}
#[doc = "Field `OTG_CLK_EN` writer - OTG clock enable. In device-only applications, this bit enables access to the PORTSEL register."]
pub type OtgClkEnW<'a, REG> = crate::BitWriter<'a, REG, OtgClkEn>;
impl<'a, REG> OtgClkEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the OTG clock."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(OtgClkEn::Disabled)
    }
    #[doc = "Enable the OTG clock."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(OtgClkEn::Enabled)
    }
}
#[doc = "AHB master clock enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AhbClkEn {
    #[doc = "0: Disable the AHB clock."]
    Disabled = 0,
    #[doc = "1: Enable the AHB clock."]
    Enabled = 1,
}
impl From<AhbClkEn> for bool {
    #[inline(always)]
    fn from(variant: AhbClkEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AHB_CLK_EN` reader - AHB master clock enable"]
pub type AhbClkEnR = crate::BitReader<AhbClkEn>;
impl AhbClkEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AhbClkEn {
        match self.bits {
            false => AhbClkEn::Disabled,
            true => AhbClkEn::Enabled,
        }
    }
    #[doc = "Disable the AHB clock."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == AhbClkEn::Disabled
    }
    #[doc = "Enable the AHB clock."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == AhbClkEn::Enabled
    }
}
#[doc = "Field `AHB_CLK_EN` writer - AHB master clock enable"]
pub type AhbClkEnW<'a, REG> = crate::BitWriter<'a, REG, AhbClkEn>;
impl<'a, REG> AhbClkEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the AHB clock."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(AhbClkEn::Disabled)
    }
    #[doc = "Enable the AHB clock."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(AhbClkEn::Enabled)
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
