#[doc = "Register `DYNAMICCONTROL` reader"]
pub type R = crate::R<DynamiccontrolSpec>;
#[doc = "Register `DYNAMICCONTROL` writer"]
pub type W = crate::W<DynamiccontrolSpec>;
#[doc = "Dynamic memory clock enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ce {
    #[doc = "0: Clock enable of idle devices are deasserted to save power (POR reset value)."]
    Powersave = 0,
    #[doc = "1: All clock enables are driven HIGH continuously.\\[1\\]"]
    High = 1,
}
impl From<Ce> for bool {
    #[inline(always)]
    fn from(variant: Ce) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CE` reader - Dynamic memory clock enable."]
pub type CeR = crate::BitReader<Ce>;
impl CeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ce {
        match self.bits {
            false => Ce::Powersave,
            true => Ce::High,
        }
    }
    #[doc = "Clock enable of idle devices are deasserted to save power (POR reset value)."]
    #[inline(always)]
    pub fn is_powersave(&self) -> bool {
        *self == Ce::Powersave
    }
    #[doc = "All clock enables are driven HIGH continuously.\\[1\\]"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Ce::High
    }
}
#[doc = "Field `CE` writer - Dynamic memory clock enable."]
pub type CeW<'a, REG> = crate::BitWriter<'a, REG, Ce>;
impl<'a, REG> CeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock enable of idle devices are deasserted to save power (POR reset value)."]
    #[inline(always)]
    pub fn powersave(self) -> &'a mut crate::W<REG> {
        self.variant(Ce::Powersave)
    }
    #[doc = "All clock enables are driven HIGH continuously.\\[1\\]"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Ce::High)
    }
}
#[doc = "Dynamic memory clock control. When clock control is LOW the output clock CLKOUT is stopped when there are no SDRAM transactions. The clock is also stopped during self-refresh mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cs {
    #[doc = "0: CLKOUT stops when all SDRAMs are idle and during self-refresh mode."]
    StopOnIdle = 0,
    #[doc = "1: CLKOUT runs continuously (POR reset value)."]
    Continuous = 1,
}
impl From<Cs> for bool {
    #[inline(always)]
    fn from(variant: Cs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CS` reader - Dynamic memory clock control. When clock control is LOW the output clock CLKOUT is stopped when there are no SDRAM transactions. The clock is also stopped during self-refresh mode."]
pub type CsR = crate::BitReader<Cs>;
impl CsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cs {
        match self.bits {
            false => Cs::StopOnIdle,
            true => Cs::Continuous,
        }
    }
    #[doc = "CLKOUT stops when all SDRAMs are idle and during self-refresh mode."]
    #[inline(always)]
    pub fn is_stop_on_idle(&self) -> bool {
        *self == Cs::StopOnIdle
    }
    #[doc = "CLKOUT runs continuously (POR reset value)."]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Cs::Continuous
    }
}
#[doc = "Field `CS` writer - Dynamic memory clock control. When clock control is LOW the output clock CLKOUT is stopped when there are no SDRAM transactions. The clock is also stopped during self-refresh mode."]
pub type CsW<'a, REG> = crate::BitWriter<'a, REG, Cs>;
impl<'a, REG> CsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLKOUT stops when all SDRAMs are idle and during self-refresh mode."]
    #[inline(always)]
    pub fn stop_on_idle(self) -> &'a mut crate::W<REG> {
        self.variant(Cs::StopOnIdle)
    }
    #[doc = "CLKOUT runs continuously (POR reset value)."]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(Cs::Continuous)
    }
}
#[doc = "Self-refresh request, EMCSREFREQ. By writing 1 to this bit self-refresh can be entered under software control. Writing 0 to this bit returns the EMC to normal mode. The self-refresh acknowledge bit in the Status register must be polled to discover the current operating mode of the EMC.\\[2\\]\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sr {
    #[doc = "0: Normal mode."]
    NormalMode = 0,
    #[doc = "1: Enter self-refresh mode (POR reset value)."]
    SelfRefreshMode = 1,
}
impl From<Sr> for bool {
    #[inline(always)]
    fn from(variant: Sr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR` reader - Self-refresh request, EMCSREFREQ. By writing 1 to this bit self-refresh can be entered under software control. Writing 0 to this bit returns the EMC to normal mode. The self-refresh acknowledge bit in the Status register must be polled to discover the current operating mode of the EMC.\\[2\\]"]
pub type SrR = crate::BitReader<Sr>;
impl SrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sr {
        match self.bits {
            false => Sr::NormalMode,
            true => Sr::SelfRefreshMode,
        }
    }
    #[doc = "Normal mode."]
    #[inline(always)]
    pub fn is_normal_mode(&self) -> bool {
        *self == Sr::NormalMode
    }
    #[doc = "Enter self-refresh mode (POR reset value)."]
    #[inline(always)]
    pub fn is_self_refresh_mode(&self) -> bool {
        *self == Sr::SelfRefreshMode
    }
}
#[doc = "Field `SR` writer - Self-refresh request, EMCSREFREQ. By writing 1 to this bit self-refresh can be entered under software control. Writing 0 to this bit returns the EMC to normal mode. The self-refresh acknowledge bit in the Status register must be polled to discover the current operating mode of the EMC.\\[2\\]"]
pub type SrW<'a, REG> = crate::BitWriter<'a, REG, Sr>;
impl<'a, REG> SrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode."]
    #[inline(always)]
    pub fn normal_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Sr::NormalMode)
    }
    #[doc = "Enter self-refresh mode (POR reset value)."]
    #[inline(always)]
    pub fn self_refresh_mode(self) -> &'a mut crate::W<REG> {
        self.variant(Sr::SelfRefreshMode)
    }
}
#[doc = "Memory clock control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mmc {
    #[doc = "0: CLKOUT enabled (POR reset value)."]
    Enabled = 0,
    #[doc = "1: CLKOUT disabled.\\[3\\]"]
    Disabled = 1,
}
impl From<Mmc> for bool {
    #[inline(always)]
    fn from(variant: Mmc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMC` reader - Memory clock control."]
pub type MmcR = crate::BitReader<Mmc>;
impl MmcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mmc {
        match self.bits {
            false => Mmc::Enabled,
            true => Mmc::Disabled,
        }
    }
    #[doc = "CLKOUT enabled (POR reset value)."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Mmc::Enabled
    }
    #[doc = "CLKOUT disabled.\\[3\\]"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Mmc::Disabled
    }
}
#[doc = "Field `MMC` writer - Memory clock control."]
pub type MmcW<'a, REG> = crate::BitWriter<'a, REG, Mmc>;
impl<'a, REG> MmcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLKOUT enabled (POR reset value)."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mmc::Enabled)
    }
    #[doc = "CLKOUT disabled.\\[3\\]"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Mmc::Disabled)
    }
}
#[doc = "SDRAM initialization.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum I {
    #[doc = "0: Issue SDRAM NORMAL operation command (POR reset value)."]
    Normal = 0,
    #[doc = "1: Issue SDRAM MODE command."]
    Mode = 1,
    #[doc = "2: Issue SDRAM PALL (precharge all) command."]
    Pall = 2,
    #[doc = "3: Issue SDRAM NOP (no operation) command)"]
    Nop = 3,
}
impl From<I> for u8 {
    #[inline(always)]
    fn from(variant: I) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for I {
    type Ux = u8;
}
impl crate::IsEnum for I {}
#[doc = "Field `I` reader - SDRAM initialization."]
pub type IR = crate::FieldReader<I>;
impl IR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> I {
        match self.bits {
            0 => I::Normal,
            1 => I::Mode,
            2 => I::Pall,
            3 => I::Nop,
            _ => unreachable!(),
        }
    }
    #[doc = "Issue SDRAM NORMAL operation command (POR reset value)."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == I::Normal
    }
    #[doc = "Issue SDRAM MODE command."]
    #[inline(always)]
    pub fn is_mode(&self) -> bool {
        *self == I::Mode
    }
    #[doc = "Issue SDRAM PALL (precharge all) command."]
    #[inline(always)]
    pub fn is_pall(&self) -> bool {
        *self == I::Pall
    }
    #[doc = "Issue SDRAM NOP (no operation) command)"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == I::Nop
    }
}
#[doc = "Field `I` writer - SDRAM initialization."]
pub type IW<'a, REG> = crate::FieldWriter<'a, REG, 2, I, crate::Safe>;
impl<'a, REG> IW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Issue SDRAM NORMAL operation command (POR reset value)."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(I::Normal)
    }
    #[doc = "Issue SDRAM MODE command."]
    #[inline(always)]
    pub fn mode(self) -> &'a mut crate::W<REG> {
        self.variant(I::Mode)
    }
    #[doc = "Issue SDRAM PALL (precharge all) command."]
    #[inline(always)]
    pub fn pall(self) -> &'a mut crate::W<REG> {
        self.variant(I::Pall)
    }
    #[doc = "Issue SDRAM NOP (no operation) command)"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut crate::W<REG> {
        self.variant(I::Nop)
    }
}
impl R {
    #[doc = "Bit 0 - Dynamic memory clock enable."]
    #[inline(always)]
    pub fn ce(&self) -> CeR {
        CeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Dynamic memory clock control. When clock control is LOW the output clock CLKOUT is stopped when there are no SDRAM transactions. The clock is also stopped during self-refresh mode."]
    #[inline(always)]
    pub fn cs(&self) -> CsR {
        CsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Self-refresh request, EMCSREFREQ. By writing 1 to this bit self-refresh can be entered under software control. Writing 0 to this bit returns the EMC to normal mode. The self-refresh acknowledge bit in the Status register must be polled to discover the current operating mode of the EMC.\\[2\\]"]
    #[inline(always)]
    pub fn sr(&self) -> SrR {
        SrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Memory clock control."]
    #[inline(always)]
    pub fn mmc(&self) -> MmcR {
        MmcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 7:8 - SDRAM initialization."]
    #[inline(always)]
    pub fn i(&self) -> IR {
        IR::new(((self.bits >> 7) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Dynamic memory clock enable."]
    #[inline(always)]
    pub fn ce(&mut self) -> CeW<'_, DynamiccontrolSpec> {
        CeW::new(self, 0)
    }
    #[doc = "Bit 1 - Dynamic memory clock control. When clock control is LOW the output clock CLKOUT is stopped when there are no SDRAM transactions. The clock is also stopped during self-refresh mode."]
    #[inline(always)]
    pub fn cs(&mut self) -> CsW<'_, DynamiccontrolSpec> {
        CsW::new(self, 1)
    }
    #[doc = "Bit 2 - Self-refresh request, EMCSREFREQ. By writing 1 to this bit self-refresh can be entered under software control. Writing 0 to this bit returns the EMC to normal mode. The self-refresh acknowledge bit in the Status register must be polled to discover the current operating mode of the EMC.\\[2\\]"]
    #[inline(always)]
    pub fn sr(&mut self) -> SrW<'_, DynamiccontrolSpec> {
        SrW::new(self, 2)
    }
    #[doc = "Bit 5 - Memory clock control."]
    #[inline(always)]
    pub fn mmc(&mut self) -> MmcW<'_, DynamiccontrolSpec> {
        MmcW::new(self, 5)
    }
    #[doc = "Bits 7:8 - SDRAM initialization."]
    #[inline(always)]
    pub fn i(&mut self) -> IW<'_, DynamiccontrolSpec> {
        IW::new(self, 7)
    }
}
#[doc = "Controls dynamic memory operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`dynamiccontrol::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dynamiccontrol::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DynamiccontrolSpec;
impl crate::RegisterSpec for DynamiccontrolSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dynamiccontrol::R`](R) reader structure"]
impl crate::Readable for DynamiccontrolSpec {}
#[doc = "`write(|w| ..)` method takes [`dynamiccontrol::W`](W) writer structure"]
impl crate::Writable for DynamiccontrolSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DYNAMICCONTROL to value 0x06"]
impl crate::Resettable for DynamiccontrolSpec {
    const RESET_VALUE: u32 = 0x06;
}
