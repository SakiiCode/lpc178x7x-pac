#[doc = "Register `CCR` reader"]
pub type R = crate::R<CcrSpec>;
#[doc = "Register `CCR` writer"]
pub type W = crate::W<CcrSpec>;
#[doc = "Capture on PWMn_CAP0 rising edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cap0R {
    #[doc = "0: Disabled. This feature is disabled."]
    DisabledThisFeatu = 0,
    #[doc = "1: Rising edge. A synchronously sampled rising edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of the TC."]
    RisingEdgeASynch = 1,
}
impl From<Cap0R> for bool {
    #[inline(always)]
    fn from(variant: Cap0R) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP0_R` reader - Capture on PWMn_CAP0 rising edge"]
pub type Cap0RR = crate::BitReader<Cap0R>;
impl Cap0RR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cap0R {
        match self.bits {
            false => Cap0R::DisabledThisFeatu,
            true => Cap0R::RisingEdgeASynch,
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn is_disabled_this_featu(&self) -> bool {
        *self == Cap0R::DisabledThisFeatu
    }
    #[doc = "Rising edge. A synchronously sampled rising edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of the TC."]
    #[inline(always)]
    pub fn is_rising_edge_a_synch(&self) -> bool {
        *self == Cap0R::RisingEdgeASynch
    }
}
#[doc = "Field `CAP0_R` writer - Capture on PWMn_CAP0 rising edge"]
pub type Cap0RW<'a, REG> = crate::BitWriter<'a, REG, Cap0R>;
impl<'a, REG> Cap0RW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn disabled_this_featu(self) -> &'a mut crate::W<REG> {
        self.variant(Cap0R::DisabledThisFeatu)
    }
    #[doc = "Rising edge. A synchronously sampled rising edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of the TC."]
    #[inline(always)]
    pub fn rising_edge_a_synch(self) -> &'a mut crate::W<REG> {
        self.variant(Cap0R::RisingEdgeASynch)
    }
}
#[doc = "Capture on PWMn_CAP0 falling edge\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cap0F {
    #[doc = "0: Disabled. This feature is disabled."]
    DisabledThisFeatu = 0,
    #[doc = "1: Falling edge. A synchronously sampled falling edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of TC."]
    FallingEdgeASync = 1,
}
impl From<Cap0F> for bool {
    #[inline(always)]
    fn from(variant: Cap0F) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP0_F` reader - Capture on PWMn_CAP0 falling edge"]
pub type Cap0FR = crate::BitReader<Cap0F>;
impl Cap0FR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cap0F {
        match self.bits {
            false => Cap0F::DisabledThisFeatu,
            true => Cap0F::FallingEdgeASync,
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn is_disabled_this_featu(&self) -> bool {
        *self == Cap0F::DisabledThisFeatu
    }
    #[doc = "Falling edge. A synchronously sampled falling edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn is_falling_edge_a_sync(&self) -> bool {
        *self == Cap0F::FallingEdgeASync
    }
}
#[doc = "Field `CAP0_F` writer - Capture on PWMn_CAP0 falling edge"]
pub type Cap0FW<'a, REG> = crate::BitWriter<'a, REG, Cap0F>;
impl<'a, REG> Cap0FW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn disabled_this_featu(self) -> &'a mut crate::W<REG> {
        self.variant(Cap0F::DisabledThisFeatu)
    }
    #[doc = "Falling edge. A synchronously sampled falling edge on PWMn_CAP0 will cause CR0 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn falling_edge_a_sync(self) -> &'a mut crate::W<REG> {
        self.variant(Cap0F::FallingEdgeASync)
    }
}
#[doc = "Interrupt on PWMn_CAP0 event\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cap0I {
    #[doc = "0: Disabled. This feature is disabled."]
    DisabledThisFeatu = 0,
    #[doc = "1: Interrupt. A CR0 load due to a PWMn_CAP0 event will generate an interrupt."]
    InterruptACr0Loa = 1,
}
impl From<Cap0I> for bool {
    #[inline(always)]
    fn from(variant: Cap0I) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP0_I` reader - Interrupt on PWMn_CAP0 event"]
pub type Cap0IR = crate::BitReader<Cap0I>;
impl Cap0IR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cap0I {
        match self.bits {
            false => Cap0I::DisabledThisFeatu,
            true => Cap0I::InterruptACr0Loa,
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn is_disabled_this_featu(&self) -> bool {
        *self == Cap0I::DisabledThisFeatu
    }
    #[doc = "Interrupt. A CR0 load due to a PWMn_CAP0 event will generate an interrupt."]
    #[inline(always)]
    pub fn is_interrupt_a_cr0_loa(&self) -> bool {
        *self == Cap0I::InterruptACr0Loa
    }
}
#[doc = "Field `CAP0_I` writer - Interrupt on PWMn_CAP0 event"]
pub type Cap0IW<'a, REG> = crate::BitWriter<'a, REG, Cap0I>;
impl<'a, REG> Cap0IW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn disabled_this_featu(self) -> &'a mut crate::W<REG> {
        self.variant(Cap0I::DisabledThisFeatu)
    }
    #[doc = "Interrupt. A CR0 load due to a PWMn_CAP0 event will generate an interrupt."]
    #[inline(always)]
    pub fn interrupt_a_cr0_loa(self) -> &'a mut crate::W<REG> {
        self.variant(Cap0I::InterruptACr0Loa)
    }
}
#[doc = "Capture on PWMn_CAP1 rising edge. Reserved for PWM0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cap1R {
    #[doc = "0: Disabled. This feature is disabled."]
    DisabledThisFeatu = 0,
    #[doc = "1: Rising edge. A synchronously sampled rising edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of the TC."]
    RisingEdgeASynch = 1,
}
impl From<Cap1R> for bool {
    #[inline(always)]
    fn from(variant: Cap1R) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP1_R` reader - Capture on PWMn_CAP1 rising edge. Reserved for PWM0."]
pub type Cap1RR = crate::BitReader<Cap1R>;
impl Cap1RR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cap1R {
        match self.bits {
            false => Cap1R::DisabledThisFeatu,
            true => Cap1R::RisingEdgeASynch,
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn is_disabled_this_featu(&self) -> bool {
        *self == Cap1R::DisabledThisFeatu
    }
    #[doc = "Rising edge. A synchronously sampled rising edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of the TC."]
    #[inline(always)]
    pub fn is_rising_edge_a_synch(&self) -> bool {
        *self == Cap1R::RisingEdgeASynch
    }
}
#[doc = "Field `CAP1_R` writer - Capture on PWMn_CAP1 rising edge. Reserved for PWM0."]
pub type Cap1RW<'a, REG> = crate::BitWriter<'a, REG, Cap1R>;
impl<'a, REG> Cap1RW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn disabled_this_featu(self) -> &'a mut crate::W<REG> {
        self.variant(Cap1R::DisabledThisFeatu)
    }
    #[doc = "Rising edge. A synchronously sampled rising edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of the TC."]
    #[inline(always)]
    pub fn rising_edge_a_synch(self) -> &'a mut crate::W<REG> {
        self.variant(Cap1R::RisingEdgeASynch)
    }
}
#[doc = "Capture on PWMn_CAP1 falling edge. Reserved for PWM0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cap1F {
    #[doc = "0: Disabled. This feature is disabled."]
    DisabledThisFeatu = 0,
    #[doc = "1: Falling edge. A synchronously sampled falling edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of TC."]
    FallingEdgeASync = 1,
}
impl From<Cap1F> for bool {
    #[inline(always)]
    fn from(variant: Cap1F) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP1_F` reader - Capture on PWMn_CAP1 falling edge. Reserved for PWM0."]
pub type Cap1FR = crate::BitReader<Cap1F>;
impl Cap1FR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cap1F {
        match self.bits {
            false => Cap1F::DisabledThisFeatu,
            true => Cap1F::FallingEdgeASync,
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn is_disabled_this_featu(&self) -> bool {
        *self == Cap1F::DisabledThisFeatu
    }
    #[doc = "Falling edge. A synchronously sampled falling edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn is_falling_edge_a_sync(&self) -> bool {
        *self == Cap1F::FallingEdgeASync
    }
}
#[doc = "Field `CAP1_F` writer - Capture on PWMn_CAP1 falling edge. Reserved for PWM0."]
pub type Cap1FW<'a, REG> = crate::BitWriter<'a, REG, Cap1F>;
impl<'a, REG> Cap1FW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn disabled_this_featu(self) -> &'a mut crate::W<REG> {
        self.variant(Cap1F::DisabledThisFeatu)
    }
    #[doc = "Falling edge. A synchronously sampled falling edge on PWMn_CAP1 will cause CR1 to be loaded with the contents of TC."]
    #[inline(always)]
    pub fn falling_edge_a_sync(self) -> &'a mut crate::W<REG> {
        self.variant(Cap1F::FallingEdgeASync)
    }
}
#[doc = "Interrupt on PWMn_CAP1 event. Reserved for PWM0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cap1I {
    #[doc = "0: Disabled. This feature is disabled."]
    DisabledThisFeatu = 0,
    #[doc = "1: Interrupt. A CR1 load due to a PWMn_CAP1 event will generate an interrupt."]
    InterruptACr1Loa = 1,
}
impl From<Cap1I> for bool {
    #[inline(always)]
    fn from(variant: Cap1I) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CAP1_I` reader - Interrupt on PWMn_CAP1 event. Reserved for PWM0."]
pub type Cap1IR = crate::BitReader<Cap1I>;
impl Cap1IR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cap1I {
        match self.bits {
            false => Cap1I::DisabledThisFeatu,
            true => Cap1I::InterruptACr1Loa,
        }
    }
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn is_disabled_this_featu(&self) -> bool {
        *self == Cap1I::DisabledThisFeatu
    }
    #[doc = "Interrupt. A CR1 load due to a PWMn_CAP1 event will generate an interrupt."]
    #[inline(always)]
    pub fn is_interrupt_a_cr1_loa(&self) -> bool {
        *self == Cap1I::InterruptACr1Loa
    }
}
#[doc = "Field `CAP1_I` writer - Interrupt on PWMn_CAP1 event. Reserved for PWM0."]
pub type Cap1IW<'a, REG> = crate::BitWriter<'a, REG, Cap1I>;
impl<'a, REG> Cap1IW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. This feature is disabled."]
    #[inline(always)]
    pub fn disabled_this_featu(self) -> &'a mut crate::W<REG> {
        self.variant(Cap1I::DisabledThisFeatu)
    }
    #[doc = "Interrupt. A CR1 load due to a PWMn_CAP1 event will generate an interrupt."]
    #[inline(always)]
    pub fn interrupt_a_cr1_loa(self) -> &'a mut crate::W<REG> {
        self.variant(Cap1I::InterruptACr1Loa)
    }
}
impl R {
    #[doc = "Bit 0 - Capture on PWMn_CAP0 rising edge"]
    #[inline(always)]
    pub fn cap0_r(&self) -> Cap0RR {
        Cap0RR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Capture on PWMn_CAP0 falling edge"]
    #[inline(always)]
    pub fn cap0_f(&self) -> Cap0FR {
        Cap0FR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt on PWMn_CAP0 event"]
    #[inline(always)]
    pub fn cap0_i(&self) -> Cap0IR {
        Cap0IR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Capture on PWMn_CAP1 rising edge. Reserved for PWM0."]
    #[inline(always)]
    pub fn cap1_r(&self) -> Cap1RR {
        Cap1RR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Capture on PWMn_CAP1 falling edge. Reserved for PWM0."]
    #[inline(always)]
    pub fn cap1_f(&self) -> Cap1FR {
        Cap1FR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt on PWMn_CAP1 event. Reserved for PWM0."]
    #[inline(always)]
    pub fn cap1_i(&self) -> Cap1IR {
        Cap1IR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Capture on PWMn_CAP0 rising edge"]
    #[inline(always)]
    pub fn cap0_r(&mut self) -> Cap0RW<'_, CcrSpec> {
        Cap0RW::new(self, 0)
    }
    #[doc = "Bit 1 - Capture on PWMn_CAP0 falling edge"]
    #[inline(always)]
    pub fn cap0_f(&mut self) -> Cap0FW<'_, CcrSpec> {
        Cap0FW::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt on PWMn_CAP0 event"]
    #[inline(always)]
    pub fn cap0_i(&mut self) -> Cap0IW<'_, CcrSpec> {
        Cap0IW::new(self, 2)
    }
    #[doc = "Bit 3 - Capture on PWMn_CAP1 rising edge. Reserved for PWM0."]
    #[inline(always)]
    pub fn cap1_r(&mut self) -> Cap1RW<'_, CcrSpec> {
        Cap1RW::new(self, 3)
    }
    #[doc = "Bit 4 - Capture on PWMn_CAP1 falling edge. Reserved for PWM0."]
    #[inline(always)]
    pub fn cap1_f(&mut self) -> Cap1FW<'_, CcrSpec> {
        Cap1FW::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt on PWMn_CAP1 event. Reserved for PWM0."]
    #[inline(always)]
    pub fn cap1_i(&mut self) -> Cap1IW<'_, CcrSpec> {
        Cap1IW::new(self, 5)
    }
}
#[doc = "Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated for a capture event.\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcrSpec;
impl crate::RegisterSpec for CcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr::R`](R) reader structure"]
impl crate::Readable for CcrSpec {}
#[doc = "`write(|w| ..)` method takes [`ccr::W`](W) writer structure"]
impl crate::Writable for CcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CcrSpec {}
