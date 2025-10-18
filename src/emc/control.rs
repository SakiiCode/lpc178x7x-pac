#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "EMC Enable. Indicates if the EMC is enabled or disabled:\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled (POR and warm reset value)."]
    Enabled = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `E` reader - EMC Enable. Indicates if the EMC is enabled or disabled:"]
pub type ER = crate::BitReader<Enum>;
impl ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disabled,
            true => Enum::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Enum::Disabled
    }
    #[doc = "Enabled (POR and warm reset value)."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Enum::Enabled
    }
}
#[doc = "Field `E` writer - EMC Enable. Indicates if the EMC is enabled or disabled:"]
pub type EW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled)
    }
    #[doc = "Enabled (POR and warm reset value)."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Enabled)
    }
}
#[doc = "Address mirror. Indicates normal or reset memory map:\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Normal memory map."]
    Normal = 0,
    #[doc = "1: Reset memory map. Static memory EMC_CS1 is mirrored onto EMC_CS0 and EMC_DYCS0 (POR reset value)."]
    Reset = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `M` reader - Address mirror. Indicates normal or reset memory map:"]
pub type MR = crate::BitReader<Enum>;
impl MR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Normal,
            true => Enum::Reset,
        }
    }
    #[doc = "Normal memory map."]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Enum::Normal
    }
    #[doc = "Reset memory map. Static memory EMC_CS1 is mirrored onto EMC_CS0 and EMC_DYCS0 (POR reset value)."]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == Enum::Reset
    }
}
#[doc = "Field `M` writer - Address mirror. Indicates normal or reset memory map:"]
pub type MW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> MW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal memory map."]
    #[inline(always)]
    pub fn normal(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Normal)
    }
    #[doc = "Reset memory map. Static memory EMC_CS1 is mirrored onto EMC_CS0 and EMC_DYCS0 (POR reset value)."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Reset)
    }
}
#[doc = "Low-power mode. Indicates normal, or low-power mode:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Normal mode (warm reset value)."]
    Warmreset = 0,
    #[doc = "1: Low-power mode. Entering low-power mode reduces memory controller power consumption. Dynamic memory is refreshed as necessary. The memory controller returns to normal functional mode by clearing the low-power mode bit (L), or by POR. This bit must only be modified when the EMC is in idle state.\\[1\\]"]
    Lowpower = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `L` reader - Low-power mode. Indicates normal, or low-power mode:"]
pub type LR = crate::BitReader<Enum>;
impl LR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Warmreset,
            true => Enum::Lowpower,
        }
    }
    #[doc = "Normal mode (warm reset value)."]
    #[inline(always)]
    pub fn is_warmreset(&self) -> bool {
        *self == Enum::Warmreset
    }
    #[doc = "Low-power mode. Entering low-power mode reduces memory controller power consumption. Dynamic memory is refreshed as necessary. The memory controller returns to normal functional mode by clearing the low-power mode bit (L), or by POR. This bit must only be modified when the EMC is in idle state.\\[1\\]"]
    #[inline(always)]
    pub fn is_lowpower(&self) -> bool {
        *self == Enum::Lowpower
    }
}
#[doc = "Field `L` writer - Low-power mode. Indicates normal, or low-power mode:"]
pub type LW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> LW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Normal mode (warm reset value)."]
    #[inline(always)]
    pub fn warmreset(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Warmreset)
    }
    #[doc = "Low-power mode. Entering low-power mode reduces memory controller power consumption. Dynamic memory is refreshed as necessary. The memory controller returns to normal functional mode by clearing the low-power mode bit (L), or by POR. This bit must only be modified when the EMC is in idle state.\\[1\\]"]
    #[inline(always)]
    pub fn lowpower(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Lowpower)
    }
}
impl R {
    #[doc = "Bit 0 - EMC Enable. Indicates if the EMC is enabled or disabled:"]
    #[inline(always)]
    pub fn e(&self) -> ER {
        ER::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Address mirror. Indicates normal or reset memory map:"]
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Low-power mode. Indicates normal, or low-power mode:"]
    #[inline(always)]
    pub fn l(&self) -> LR {
        LR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EMC Enable. Indicates if the EMC is enabled or disabled:"]
    #[inline(always)]
    pub fn e(&mut self) -> EW<'_, ControlSpec> {
        EW::new(self, 0)
    }
    #[doc = "Bit 1 - Address mirror. Indicates normal or reset memory map:"]
    #[inline(always)]
    pub fn m(&mut self) -> MW<'_, ControlSpec> {
        MW::new(self, 1)
    }
    #[doc = "Bit 2 - Low-power mode. Indicates normal, or low-power mode:"]
    #[inline(always)]
    pub fn l(&mut self) -> LW<'_, ControlSpec> {
        LW::new(self, 2)
    }
}
#[doc = "Controls operation of the memory controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONTROL to value 0x03"]
impl crate::Resettable for ControlSpec {
    const RESET_VALUE: u32 = 0x03;
}
