#[doc = "Register `DYNAMICCONTROL` reader"]
pub type R = crate::R<DynamiccontrolSpec>;
#[doc = "Register `DYNAMICCONTROL` writer"]
pub type W = crate::W<DynamiccontrolSpec>;
#[doc = "Dynamic memory clock enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Clock enable of idle devices are deasserted to save power (POR reset value)."]
    Powersave = 0,
    #[doc = "1: All clock enables are driven HIGH continuously.\\[1\\]"]
    High = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CE` reader - Dynamic memory clock enable."]
pub type CeR = crate::BitReader<Enum>;
impl CeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Powersave,
            true => Enum::High,
        }
    }
    #[doc = "Clock enable of idle devices are deasserted to save power (POR reset value)."]
    #[inline(always)]
    pub fn is_powersave(&self) -> bool {
        *self == Enum::Powersave
    }
    #[doc = "All clock enables are driven HIGH continuously.\\[1\\]"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == Enum::High
    }
}
#[doc = "Field `CE` writer - Dynamic memory clock enable."]
pub type CeW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> CeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Clock enable of idle devices are deasserted to save power (POR reset value)."]
    #[inline(always)]
    pub fn powersave(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Powersave)
    }
    #[doc = "All clock enables are driven HIGH continuously.\\[1\\]"]
    #[inline(always)]
    pub fn high(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::High)
    }
}
#[doc = "Dynamic memory clock control. When clock control is LOW the output clock CLKOUT is stopped when there are no SDRAM transactions. The clock is also stopped during self-refresh mode.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: CLKOUT stops when all SDRAMs are idle and during self-refresh mode."]
    Stop = 0,
    #[doc = "1: CLKOUT runs continuously (POR reset value)."]
    Run = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CS` reader - Dynamic memory clock control. When clock control is LOW the output clock CLKOUT is stopped when there are no SDRAM transactions. The clock is also stopped during self-refresh mode."]
pub type CsR = crate::BitReader<Enum>;
impl CsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Stop,
            true => Enum::Run,
        }
    }
    #[doc = "CLKOUT stops when all SDRAMs are idle and during self-refresh mode."]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == Enum::Stop
    }
    #[doc = "CLKOUT runs continuously (POR reset value)."]
    #[inline(always)]
    pub fn is_run(&self) -> bool {
        *self == Enum::Run
    }
}
#[doc = "Field `CS` writer - Dynamic memory clock control. When clock control is LOW the output clock CLKOUT is stopped when there are no SDRAM transactions. The clock is also stopped during self-refresh mode."]
pub type CsW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> CsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLKOUT stops when all SDRAMs are idle and during self-refresh mode."]
    #[inline(always)]
    pub fn stop(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Stop)
    }
    #[doc = "CLKOUT runs continuously (POR reset value)."]
    #[inline(always)]
    pub fn run(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Run)
    }
}
#[doc = "Self-refresh request, EMCSREFREQ. By writing 1 to this bit self-refresh can be entered under software control. Writing 0 to this bit returns the EMC to normal mode. The self-refresh acknowledge bit in the Status register must be polled to discover the current operating mode of the EMC.\\[2\\]\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Normal mode."]
    NormalMode_ = 0,
    #[doc = "1: Enter self-refresh mode (POR reset value)."]
    EnterSelfRefreshM = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SR` reader - Self-refresh request, EMCSREFREQ. By writing 1 to this bit self-refresh can be entered under software control. Writing 0 to this bit returns the EMC to normal mode. The self-refresh acknowledge bit in the Status register must be polled to discover the current operating mode of the EMC.\\[2\\]"]
pub type SrR = crate::BitReader<Enum>;
impl SrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::NormalMode_,
            true => Enum::EnterSelfRefreshM,
        }
    }
    #[doc = "Normal mode."]
    #[inline(always)]
    pub fn is_normal_mode_(&self) -> bool {
        *self == Enum::NormalMode_
    }
    #[doc = "Enter self-refresh mode (POR reset value)."]
    #[inline(always)]
    pub fn is_enter_self_refresh_m(&self) -> bool {
        *self == Enum::EnterSelfRefreshM
    }
}
#[doc = "Field `SR` writer - Self-refresh request, EMCSREFREQ. By writing 1 to this bit self-refresh can be entered under software control. Writing 0 to this bit returns the EMC to normal mode. The self-refresh acknowledge bit in the Status register must be polled to discover the current operating mode of the EMC.\\[2\\]"]
pub type SrW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> SrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode."]
    #[inline(always)]
    pub fn normal_mode_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::NormalMode_)
    }
    #[doc = "Enter self-refresh mode (POR reset value)."]
    #[inline(always)]
    pub fn enter_self_refresh_m(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::EnterSelfRefreshM)
    }
}
#[doc = "Memory clock control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: CLKOUT enabled (POR reset value)."]
    ClkoutEnabledPor_ = 0,
    #[doc = "1: CLKOUT disabled.\\[3\\]"]
    ClkoutDisabled = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MMC` reader - Memory clock control."]
pub type MmcR = crate::BitReader<Enum>;
impl MmcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::ClkoutEnabledPor_,
            true => Enum::ClkoutDisabled,
        }
    }
    #[doc = "CLKOUT enabled (POR reset value)."]
    #[inline(always)]
    pub fn is_clkout_enabled_por_(&self) -> bool {
        *self == Enum::ClkoutEnabledPor_
    }
    #[doc = "CLKOUT disabled.\\[3\\]"]
    #[inline(always)]
    pub fn is_clkout_disabled(&self) -> bool {
        *self == Enum::ClkoutDisabled
    }
}
#[doc = "Field `MMC` writer - Memory clock control."]
pub type MmcW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> MmcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CLKOUT enabled (POR reset value)."]
    #[inline(always)]
    pub fn clkout_enabled_por_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::ClkoutEnabledPor_)
    }
    #[doc = "CLKOUT disabled.\\[3\\]"]
    #[inline(always)]
    pub fn clkout_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::ClkoutDisabled)
    }
}
#[doc = "SDRAM initialization.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Enum {
    #[doc = "0: Issue SDRAM NORMAL operation command (POR reset value)."]
    Normal = 0,
    #[doc = "1: Issue SDRAM MODE command."]
    Mode = 1,
    #[doc = "2: Issue SDRAM PALL (precharge all) command."]
    Pall = 2,
    #[doc = "3: Issue SDRAM NOP (no operation) command)"]
    Nop = 3,
}
impl From<Enum> for u8 {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Enum {
    type Ux = u8;
}
impl crate::IsEnum for Enum {}
#[doc = "Field `I` reader - SDRAM initialization."]
pub type IR = crate::FieldReader<Enum>;
impl IR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            0 => Enum::Normal,
            1 => Enum::Mode,
            2 => Enum::Pall,
            3 => Enum::Nop,
            _ => unreachable!(),
        }
    }
    #[doc = "Issue SDRAM NORMAL operation command (POR reset value)."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Enum::Normal
    }
    #[doc = "Issue SDRAM MODE command."]
    #[inline(always)]
    pub fn is_mode(&self) -> bool {
        *self == Enum::Mode
    }
    #[doc = "Issue SDRAM PALL (precharge all) command."]
    #[inline(always)]
    pub fn is_pall(&self) -> bool {
        *self == Enum::Pall
    }
    #[doc = "Issue SDRAM NOP (no operation) command)"]
    #[inline(always)]
    pub fn is_nop(&self) -> bool {
        *self == Enum::Nop
    }
}
#[doc = "Field `I` writer - SDRAM initialization."]
pub type IW<'a, REG> = crate::FieldWriter<'a, REG, 2, Enum, crate::Safe>;
impl<'a, REG> IW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Issue SDRAM NORMAL operation command (POR reset value)."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Normal)
    }
    #[doc = "Issue SDRAM MODE command."]
    #[inline(always)]
    pub fn mode(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Mode)
    }
    #[doc = "Issue SDRAM PALL (precharge all) command."]
    #[inline(always)]
    pub fn pall(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Pall)
    }
    #[doc = "Issue SDRAM NOP (no operation) command)"]
    #[inline(always)]
    pub fn nop(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Nop)
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
