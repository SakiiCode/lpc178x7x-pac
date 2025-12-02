#[doc = "Register `EXTPOLAR` reader"]
pub type R = crate::R<ExtpolarSpec>;
#[doc = "Register `EXTPOLAR` writer"]
pub type W = crate::W<ExtpolarSpec>;
#[doc = "External interrupt polarity for EINT0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extpolar0 {
    #[doc = "0: Low-active or falling-edge sensitive (depending on EXTMODE0)."]
    ActiveLow = 0,
    #[doc = "1: High-active or rising-edge sensitive (depending on EXTMODE0)."]
    ActiveHigh = 1,
}
impl From<Extpolar0> for bool {
    #[inline(always)]
    fn from(variant: Extpolar0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTPOLAR0` reader - External interrupt polarity for EINT0."]
pub type Extpolar0R = crate::BitReader<Extpolar0>;
impl Extpolar0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extpolar0 {
        match self.bits {
            false => Extpolar0::ActiveLow,
            true => Extpolar0::ActiveHigh,
        }
    }
    #[doc = "Low-active or falling-edge sensitive (depending on EXTMODE0)."]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == Extpolar0::ActiveLow
    }
    #[doc = "High-active or rising-edge sensitive (depending on EXTMODE0)."]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == Extpolar0::ActiveHigh
    }
}
#[doc = "Field `EXTPOLAR0` writer - External interrupt polarity for EINT0."]
pub type Extpolar0W<'a, REG> = crate::BitWriter<'a, REG, Extpolar0>;
impl<'a, REG> Extpolar0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low-active or falling-edge sensitive (depending on EXTMODE0)."]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(Extpolar0::ActiveLow)
    }
    #[doc = "High-active or rising-edge sensitive (depending on EXTMODE0)."]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(Extpolar0::ActiveHigh)
    }
}
#[doc = "External interrupt polarity for EINT1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extpolar1 {
    #[doc = "0: Low-active or falling-edge sensitive (depending on EXTMODE1)."]
    ActiveLow = 0,
    #[doc = "1: High-active or rising-edge sensitive (depending on EXTMODE1)."]
    ActiveHigh = 1,
}
impl From<Extpolar1> for bool {
    #[inline(always)]
    fn from(variant: Extpolar1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTPOLAR1` reader - External interrupt polarity for EINT1."]
pub type Extpolar1R = crate::BitReader<Extpolar1>;
impl Extpolar1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extpolar1 {
        match self.bits {
            false => Extpolar1::ActiveLow,
            true => Extpolar1::ActiveHigh,
        }
    }
    #[doc = "Low-active or falling-edge sensitive (depending on EXTMODE1)."]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == Extpolar1::ActiveLow
    }
    #[doc = "High-active or rising-edge sensitive (depending on EXTMODE1)."]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == Extpolar1::ActiveHigh
    }
}
#[doc = "Field `EXTPOLAR1` writer - External interrupt polarity for EINT1."]
pub type Extpolar1W<'a, REG> = crate::BitWriter<'a, REG, Extpolar1>;
impl<'a, REG> Extpolar1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low-active or falling-edge sensitive (depending on EXTMODE1)."]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(Extpolar1::ActiveLow)
    }
    #[doc = "High-active or rising-edge sensitive (depending on EXTMODE1)."]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(Extpolar1::ActiveHigh)
    }
}
#[doc = "External interrupt polarity for EINT2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extpolar2 {
    #[doc = "0: Low-active or falling-edge sensitive (depending on EXTMODE2)."]
    ActiveLow = 0,
    #[doc = "1: High-active or rising-edge sensitive (depending on EXTMODE2)."]
    ActiveHigh = 1,
}
impl From<Extpolar2> for bool {
    #[inline(always)]
    fn from(variant: Extpolar2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTPOLAR2` reader - External interrupt polarity for EINT2."]
pub type Extpolar2R = crate::BitReader<Extpolar2>;
impl Extpolar2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extpolar2 {
        match self.bits {
            false => Extpolar2::ActiveLow,
            true => Extpolar2::ActiveHigh,
        }
    }
    #[doc = "Low-active or falling-edge sensitive (depending on EXTMODE2)."]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == Extpolar2::ActiveLow
    }
    #[doc = "High-active or rising-edge sensitive (depending on EXTMODE2)."]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == Extpolar2::ActiveHigh
    }
}
#[doc = "Field `EXTPOLAR2` writer - External interrupt polarity for EINT2."]
pub type Extpolar2W<'a, REG> = crate::BitWriter<'a, REG, Extpolar2>;
impl<'a, REG> Extpolar2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low-active or falling-edge sensitive (depending on EXTMODE2)."]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(Extpolar2::ActiveLow)
    }
    #[doc = "High-active or rising-edge sensitive (depending on EXTMODE2)."]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(Extpolar2::ActiveHigh)
    }
}
#[doc = "External interrupt polarity for EINT3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extpolar3 {
    #[doc = "0: Low-active or falling-edge sensitive (depending on EXTMODE3)."]
    ActiveLow = 0,
    #[doc = "1: High-active or rising-edge sensitive (depending on EXTMODE3)."]
    ActiveHigh = 1,
}
impl From<Extpolar3> for bool {
    #[inline(always)]
    fn from(variant: Extpolar3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTPOLAR3` reader - External interrupt polarity for EINT3."]
pub type Extpolar3R = crate::BitReader<Extpolar3>;
impl Extpolar3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extpolar3 {
        match self.bits {
            false => Extpolar3::ActiveLow,
            true => Extpolar3::ActiveHigh,
        }
    }
    #[doc = "Low-active or falling-edge sensitive (depending on EXTMODE3)."]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == Extpolar3::ActiveLow
    }
    #[doc = "High-active or rising-edge sensitive (depending on EXTMODE3)."]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == Extpolar3::ActiveHigh
    }
}
#[doc = "Field `EXTPOLAR3` writer - External interrupt polarity for EINT3."]
pub type Extpolar3W<'a, REG> = crate::BitWriter<'a, REG, Extpolar3>;
impl<'a, REG> Extpolar3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Low-active or falling-edge sensitive (depending on EXTMODE3)."]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(Extpolar3::ActiveLow)
    }
    #[doc = "High-active or rising-edge sensitive (depending on EXTMODE3)."]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(Extpolar3::ActiveHigh)
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
