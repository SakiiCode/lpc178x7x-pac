#[doc = "Register `MMCTRL` reader"]
pub type R = crate::R<MmctrlSpec>;
#[doc = "Register `MMCTRL` writer"]
pub type W = crate::W<MmctrlSpec>;
#[doc = "Monitor mode enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MmEna {
    #[doc = "0: Monitor mode disabled."]
    MonitorModeDisable = 0,
    #[doc = "1: The I 2C module will enter monitor mode. In this mode the SDA output will be forced high. This will prevent the I2C module from outputting data of any kind (including ACK) onto the I2C data bus. Depending on the state of the ENA_SCL bit, the output may be also forced high, preventing the module from having control over the I2C clock line."]
    TheI2cModuleWill = 1,
}
impl From<MmEna> for bool {
    #[inline(always)]
    fn from(variant: MmEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MM_ENA` reader - Monitor mode enable."]
pub type MmEnaR = crate::BitReader<MmEna>;
impl MmEnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MmEna {
        match self.bits {
            false => MmEna::MonitorModeDisable,
            true => MmEna::TheI2cModuleWill,
        }
    }
    #[doc = "Monitor mode disabled."]
    #[inline(always)]
    pub fn is_monitor_mode_disable(&self) -> bool {
        *self == MmEna::MonitorModeDisable
    }
    #[doc = "The I 2C module will enter monitor mode. In this mode the SDA output will be forced high. This will prevent the I2C module from outputting data of any kind (including ACK) onto the I2C data bus. Depending on the state of the ENA_SCL bit, the output may be also forced high, preventing the module from having control over the I2C clock line."]
    #[inline(always)]
    pub fn is_the_i_2c_module_will(&self) -> bool {
        *self == MmEna::TheI2cModuleWill
    }
}
#[doc = "Field `MM_ENA` writer - Monitor mode enable."]
pub type MmEnaW<'a, REG> = crate::BitWriter<'a, REG, MmEna>;
impl<'a, REG> MmEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Monitor mode disabled."]
    #[inline(always)]
    pub fn monitor_mode_disable(self) -> &'a mut crate::W<REG> {
        self.variant(MmEna::MonitorModeDisable)
    }
    #[doc = "The I 2C module will enter monitor mode. In this mode the SDA output will be forced high. This will prevent the I2C module from outputting data of any kind (including ACK) onto the I2C data bus. Depending on the state of the ENA_SCL bit, the output may be also forced high, preventing the module from having control over the I2C clock line."]
    #[inline(always)]
    pub fn the_i_2c_module_will(self) -> &'a mut crate::W<REG> {
        self.variant(MmEna::TheI2cModuleWill)
    }
}
#[doc = "SCL output enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnaScl {
    #[doc = "0: When this bit is cleared to 0, the SCL output will be forced high when the module is in monitor mode. As described above, this will prevent the module from having any control over the I2C clock line."]
    WhenThisBitIsCle = 0,
    #[doc = "1: When this bit is set, the I2C module may exercise the same control over the clock line that it would in normal operation. This means that, acting as a slave peripheral, the I2C module can stretch the clock line (hold it low) until it has had time to respond to an I2C interrupt.\\[1\\]"]
    WhenThisBitIsSet = 1,
}
impl From<EnaScl> for bool {
    #[inline(always)]
    fn from(variant: EnaScl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ENA_SCL` reader - SCL output enable."]
pub type EnaSclR = crate::BitReader<EnaScl>;
impl EnaSclR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EnaScl {
        match self.bits {
            false => EnaScl::WhenThisBitIsCle,
            true => EnaScl::WhenThisBitIsSet,
        }
    }
    #[doc = "When this bit is cleared to 0, the SCL output will be forced high when the module is in monitor mode. As described above, this will prevent the module from having any control over the I2C clock line."]
    #[inline(always)]
    pub fn is_when_this_bit_is_cle(&self) -> bool {
        *self == EnaScl::WhenThisBitIsCle
    }
    #[doc = "When this bit is set, the I2C module may exercise the same control over the clock line that it would in normal operation. This means that, acting as a slave peripheral, the I2C module can stretch the clock line (hold it low) until it has had time to respond to an I2C interrupt.\\[1\\]"]
    #[inline(always)]
    pub fn is_when_this_bit_is_set(&self) -> bool {
        *self == EnaScl::WhenThisBitIsSet
    }
}
#[doc = "Field `ENA_SCL` writer - SCL output enable."]
pub type EnaSclW<'a, REG> = crate::BitWriter<'a, REG, EnaScl>;
impl<'a, REG> EnaSclW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When this bit is cleared to 0, the SCL output will be forced high when the module is in monitor mode. As described above, this will prevent the module from having any control over the I2C clock line."]
    #[inline(always)]
    pub fn when_this_bit_is_cle(self) -> &'a mut crate::W<REG> {
        self.variant(EnaScl::WhenThisBitIsCle)
    }
    #[doc = "When this bit is set, the I2C module may exercise the same control over the clock line that it would in normal operation. This means that, acting as a slave peripheral, the I2C module can stretch the clock line (hold it low) until it has had time to respond to an I2C interrupt.\\[1\\]"]
    #[inline(always)]
    pub fn when_this_bit_is_set(self) -> &'a mut crate::W<REG> {
        self.variant(EnaScl::WhenThisBitIsSet)
    }
}
#[doc = "Select interrupt register match.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MatchAll {
    #[doc = "0: When this bit is cleared, an interrupt will only be generated when a match occurs to one of the (up-to) four address registers described above. That is, the module will respond as a normal slave as far as address-recognition is concerned."]
    WhenThisBitIsCle = 0,
    #[doc = "1: When this bit is set to 1 and the I2C is in monitor mode, an interrupt will be generated on ANY address received. This will enable the part to monitor all traffic on the bus."]
    WhenThisBitIsSet = 1,
}
impl From<MatchAll> for bool {
    #[inline(always)]
    fn from(variant: MatchAll) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MATCH_ALL` reader - Select interrupt register match."]
pub type MatchAllR = crate::BitReader<MatchAll>;
impl MatchAllR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MatchAll {
        match self.bits {
            false => MatchAll::WhenThisBitIsCle,
            true => MatchAll::WhenThisBitIsSet,
        }
    }
    #[doc = "When this bit is cleared, an interrupt will only be generated when a match occurs to one of the (up-to) four address registers described above. That is, the module will respond as a normal slave as far as address-recognition is concerned."]
    #[inline(always)]
    pub fn is_when_this_bit_is_cle(&self) -> bool {
        *self == MatchAll::WhenThisBitIsCle
    }
    #[doc = "When this bit is set to 1 and the I2C is in monitor mode, an interrupt will be generated on ANY address received. This will enable the part to monitor all traffic on the bus."]
    #[inline(always)]
    pub fn is_when_this_bit_is_set(&self) -> bool {
        *self == MatchAll::WhenThisBitIsSet
    }
}
#[doc = "Field `MATCH_ALL` writer - Select interrupt register match."]
pub type MatchAllW<'a, REG> = crate::BitWriter<'a, REG, MatchAll>;
impl<'a, REG> MatchAllW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "When this bit is cleared, an interrupt will only be generated when a match occurs to one of the (up-to) four address registers described above. That is, the module will respond as a normal slave as far as address-recognition is concerned."]
    #[inline(always)]
    pub fn when_this_bit_is_cle(self) -> &'a mut crate::W<REG> {
        self.variant(MatchAll::WhenThisBitIsCle)
    }
    #[doc = "When this bit is set to 1 and the I2C is in monitor mode, an interrupt will be generated on ANY address received. This will enable the part to monitor all traffic on the bus."]
    #[inline(always)]
    pub fn when_this_bit_is_set(self) -> &'a mut crate::W<REG> {
        self.variant(MatchAll::WhenThisBitIsSet)
    }
}
impl R {
    #[doc = "Bit 0 - Monitor mode enable."]
    #[inline(always)]
    pub fn mm_ena(&self) -> MmEnaR {
        MmEnaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SCL output enable."]
    #[inline(always)]
    pub fn ena_scl(&self) -> EnaSclR {
        EnaSclR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Select interrupt register match."]
    #[inline(always)]
    pub fn match_all(&self) -> MatchAllR {
        MatchAllR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Monitor mode enable."]
    #[inline(always)]
    pub fn mm_ena(&mut self) -> MmEnaW<'_, MmctrlSpec> {
        MmEnaW::new(self, 0)
    }
    #[doc = "Bit 1 - SCL output enable."]
    #[inline(always)]
    pub fn ena_scl(&mut self) -> EnaSclW<'_, MmctrlSpec> {
        EnaSclW::new(self, 1)
    }
    #[doc = "Bit 2 - Select interrupt register match."]
    #[inline(always)]
    pub fn match_all(&mut self) -> MatchAllW<'_, MmctrlSpec> {
        MatchAllW::new(self, 2)
    }
}
#[doc = "Monitor mode control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mmctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MmctrlSpec;
impl crate::RegisterSpec for MmctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mmctrl::R`](R) reader structure"]
impl crate::Readable for MmctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`mmctrl::W`](W) writer structure"]
impl crate::Writable for MmctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MMCTRL to value 0"]
impl crate::Resettable for MmctrlSpec {}
