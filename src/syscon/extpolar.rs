#[doc = "Register `EXTPOLAR` reader"]
pub type R = crate::R<ExtpolarSpec>;
#[doc = "Register `EXTPOLAR` writer"]
pub type W = crate::W<ExtpolarSpec>;
#[doc = "External interrupt polarity for EINT0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Low-active or falling-edge sensitive (depending on EXTMODE0)."]
    LowActiveOrFallin = 0,
    #[doc = "1: High-active or rising-edge sensitive (depending on EXTMODE0)."]
    HighActiveOrRisin = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTPOLAR0` reader - External interrupt polarity for EINT0."]
pub type Extpolar0R = crate::BitReader<Enum>;
impl Extpolar0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::LowActiveOrFallin,
            true => Enum::HighActiveOrRisin,
        }
    }
    #[doc = "Low-active or falling-edge sensitive (depending on EXTMODE0)."]
    #[inline(always)]
    pub fn is_low_active_or_fallin(&self) -> bool {
        *self == Enum::LowActiveOrFallin
    }
    #[doc = "High-active or rising-edge sensitive (depending on EXTMODE0)."]
    #[inline(always)]
    pub fn is_high_active_or_risin(&self) -> bool {
        *self == Enum::HighActiveOrRisin
    }
}
#[doc = "Field `EXTPOLAR0` writer - External interrupt polarity for EINT0."]
pub type Extpolar0W<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Extpolar0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low-active or falling-edge sensitive (depending on EXTMODE0)."]
    #[inline(always)]
    pub fn low_active_or_fallin(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::LowActiveOrFallin)
    }
    #[doc = "High-active or rising-edge sensitive (depending on EXTMODE0)."]
    #[inline(always)]
    pub fn high_active_or_risin(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::HighActiveOrRisin)
    }
}
#[doc = "External interrupt polarity for EINT1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Low-active or falling-edge sensitive (depending on EXTMODE1)."]
    LowActiveOrFallin = 0,
    #[doc = "1: High-active or rising-edge sensitive (depending on EXTMODE1)."]
    HighActiveOrRisin = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTPOLAR1` reader - External interrupt polarity for EINT1."]
pub type Extpolar1R = crate::BitReader<Enum>;
impl Extpolar1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::LowActiveOrFallin,
            true => Enum::HighActiveOrRisin,
        }
    }
    #[doc = "Low-active or falling-edge sensitive (depending on EXTMODE1)."]
    #[inline(always)]
    pub fn is_low_active_or_fallin(&self) -> bool {
        *self == Enum::LowActiveOrFallin
    }
    #[doc = "High-active or rising-edge sensitive (depending on EXTMODE1)."]
    #[inline(always)]
    pub fn is_high_active_or_risin(&self) -> bool {
        *self == Enum::HighActiveOrRisin
    }
}
#[doc = "Field `EXTPOLAR1` writer - External interrupt polarity for EINT1."]
pub type Extpolar1W<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Extpolar1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low-active or falling-edge sensitive (depending on EXTMODE1)."]
    #[inline(always)]
    pub fn low_active_or_fallin(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::LowActiveOrFallin)
    }
    #[doc = "High-active or rising-edge sensitive (depending on EXTMODE1)."]
    #[inline(always)]
    pub fn high_active_or_risin(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::HighActiveOrRisin)
    }
}
#[doc = "External interrupt polarity for EINT2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Low-active or falling-edge sensitive (depending on EXTMODE2)."]
    LowActiveOrFallin = 0,
    #[doc = "1: High-active or rising-edge sensitive (depending on EXTMODE2)."]
    HighActiveOrRisin = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTPOLAR2` reader - External interrupt polarity for EINT2."]
pub type Extpolar2R = crate::BitReader<Enum>;
impl Extpolar2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::LowActiveOrFallin,
            true => Enum::HighActiveOrRisin,
        }
    }
    #[doc = "Low-active or falling-edge sensitive (depending on EXTMODE2)."]
    #[inline(always)]
    pub fn is_low_active_or_fallin(&self) -> bool {
        *self == Enum::LowActiveOrFallin
    }
    #[doc = "High-active or rising-edge sensitive (depending on EXTMODE2)."]
    #[inline(always)]
    pub fn is_high_active_or_risin(&self) -> bool {
        *self == Enum::HighActiveOrRisin
    }
}
#[doc = "Field `EXTPOLAR2` writer - External interrupt polarity for EINT2."]
pub type Extpolar2W<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Extpolar2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low-active or falling-edge sensitive (depending on EXTMODE2)."]
    #[inline(always)]
    pub fn low_active_or_fallin(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::LowActiveOrFallin)
    }
    #[doc = "High-active or rising-edge sensitive (depending on EXTMODE2)."]
    #[inline(always)]
    pub fn high_active_or_risin(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::HighActiveOrRisin)
    }
}
#[doc = "External interrupt polarity for EINT3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Low-active or falling-edge sensitive (depending on EXTMODE3)."]
    LowActiveOrFallin = 0,
    #[doc = "1: High-active or rising-edge sensitive (depending on EXTMODE3)."]
    HighActiveOrRisin = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTPOLAR3` reader - External interrupt polarity for EINT3."]
pub type Extpolar3R = crate::BitReader<Enum>;
impl Extpolar3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::LowActiveOrFallin,
            true => Enum::HighActiveOrRisin,
        }
    }
    #[doc = "Low-active or falling-edge sensitive (depending on EXTMODE3)."]
    #[inline(always)]
    pub fn is_low_active_or_fallin(&self) -> bool {
        *self == Enum::LowActiveOrFallin
    }
    #[doc = "High-active or rising-edge sensitive (depending on EXTMODE3)."]
    #[inline(always)]
    pub fn is_high_active_or_risin(&self) -> bool {
        *self == Enum::HighActiveOrRisin
    }
}
#[doc = "Field `EXTPOLAR3` writer - External interrupt polarity for EINT3."]
pub type Extpolar3W<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Extpolar3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low-active or falling-edge sensitive (depending on EXTMODE3)."]
    #[inline(always)]
    pub fn low_active_or_fallin(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::LowActiveOrFallin)
    }
    #[doc = "High-active or rising-edge sensitive (depending on EXTMODE3)."]
    #[inline(always)]
    pub fn high_active_or_risin(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::HighActiveOrRisin)
    }
}
impl R {
    #[doc = "Bit 0 - External interrupt polarity for EINT0."]
    #[inline(always)]
    pub fn extpolar0(&self) -> Extpolar0R {
        Extpolar0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External interrupt polarity for EINT1."]
    #[inline(always)]
    pub fn extpolar1(&self) -> Extpolar1R {
        Extpolar1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External interrupt polarity for EINT2."]
    #[inline(always)]
    pub fn extpolar2(&self) -> Extpolar2R {
        Extpolar2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External interrupt polarity for EINT3."]
    #[inline(always)]
    pub fn extpolar3(&self) -> Extpolar3R {
        Extpolar3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - External interrupt polarity for EINT0."]
    #[inline(always)]
    pub fn extpolar0(&mut self) -> Extpolar0W<'_, ExtpolarSpec> {
        Extpolar0W::new(self, 0)
    }
    #[doc = "Bit 1 - External interrupt polarity for EINT1."]
    #[inline(always)]
    pub fn extpolar1(&mut self) -> Extpolar1W<'_, ExtpolarSpec> {
        Extpolar1W::new(self, 1)
    }
    #[doc = "Bit 2 - External interrupt polarity for EINT2."]
    #[inline(always)]
    pub fn extpolar2(&mut self) -> Extpolar2W<'_, ExtpolarSpec> {
        Extpolar2W::new(self, 2)
    }
    #[doc = "Bit 3 - External interrupt polarity for EINT3."]
    #[inline(always)]
    pub fn extpolar3(&mut self) -> Extpolar3W<'_, ExtpolarSpec> {
        Extpolar3W::new(self, 3)
    }
}
#[doc = "External Interrupt Polarity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extpolar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extpolar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtpolarSpec;
impl crate::RegisterSpec for ExtpolarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extpolar::R`](R) reader structure"]
impl crate::Readable for ExtpolarSpec {}
#[doc = "`write(|w| ..)` method takes [`extpolar::W`](W) writer structure"]
impl crate::Writable for ExtpolarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTPOLAR to value 0"]
impl crate::Resettable for ExtpolarSpec {}
