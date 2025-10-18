#[doc = "Register `DMAINTEN` reader"]
pub type R = crate::R<DmaintenSpec>;
#[doc = "Register `DMAINTEN` writer"]
pub type W = crate::W<DmaintenSpec>;
#[doc = "End of Transfer Interrupt enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disabled."]
    Disabled_ = 0,
    #[doc = "1: Enabled."]
    Enabled_ = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOT` reader - End of Transfer Interrupt enable bit."]
pub type EotR = crate::BitReader<Enum>;
impl EotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disabled_,
            true => Enum::Enabled_,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == Enum::Disabled_
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled_(&self) -> bool {
        *self == Enum::Enabled_
    }
}
#[doc = "Field `EOT` writer - End of Transfer Interrupt enable bit."]
pub type EotW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> EotW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled_)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Enabled_)
    }
}
#[doc = "New DD Request Interrupt enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disabled."]
    Disabled_ = 0,
    #[doc = "1: Enabled."]
    Enabled_ = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NDDR` reader - New DD Request Interrupt enable bit."]
pub type NddrR = crate::BitReader<Enum>;
impl NddrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disabled_,
            true => Enum::Enabled_,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == Enum::Disabled_
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled_(&self) -> bool {
        *self == Enum::Enabled_
    }
}
#[doc = "Field `NDDR` writer - New DD Request Interrupt enable bit."]
pub type NddrW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> NddrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled_)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Enabled_)
    }
}
#[doc = "System Error Interrupt enable bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disabled."]
    Disabled_ = 0,
    #[doc = "1: Enabled."]
    Enabled_ = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR` reader - System Error Interrupt enable bit."]
pub type ErrR = crate::BitReader<Enum>;
impl ErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disabled_,
            true => Enum::Enabled_,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == Enum::Disabled_
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled_(&self) -> bool {
        *self == Enum::Enabled_
    }
}
#[doc = "Field `ERR` writer - System Error Interrupt enable bit."]
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> ErrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled_)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Enabled_)
    }
}
impl R {
    #[doc = "Bit 0 - End of Transfer Interrupt enable bit."]
    #[inline(always)]
    pub fn eot(&self) -> EotR {
        EotR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - New DD Request Interrupt enable bit."]
    #[inline(always)]
    pub fn nddr(&self) -> NddrR {
        NddrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System Error Interrupt enable bit."]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - End of Transfer Interrupt enable bit."]
    #[inline(always)]
    pub fn eot(&mut self) -> EotW<'_, DmaintenSpec> {
        EotW::new(self, 0)
    }
    #[doc = "Bit 1 - New DD Request Interrupt enable bit."]
    #[inline(always)]
    pub fn nddr(&mut self) -> NddrW<'_, DmaintenSpec> {
        NddrW::new(self, 1)
    }
    #[doc = "Bit 2 - System Error Interrupt enable bit."]
    #[inline(always)]
    pub fn err(&mut self) -> ErrW<'_, DmaintenSpec> {
        ErrW::new(self, 2)
    }
}
#[doc = "USB DMA Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`dmainten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmainten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaintenSpec;
impl crate::RegisterSpec for DmaintenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmainten::R`](R) reader structure"]
impl crate::Readable for DmaintenSpec {}
#[doc = "`write(|w| ..)` method takes [`dmainten::W`](W) writer structure"]
impl crate::Writable for DmaintenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMAINTEN to value 0"]
impl crate::Resettable for DmaintenSpec {}
