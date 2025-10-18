#[doc = "Register `EXTMODE` reader"]
pub type R = crate::R<ExtmodeSpec>;
#[doc = "Register `EXTMODE` writer"]
pub type W = crate::W<ExtmodeSpec>;
#[doc = "Level or edge sensitivity select for EINT0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extmode0 {
    #[doc = "0: Level sensitive."]
    LevelSensitive_ = 0,
    #[doc = "1: Edge sensitive."]
    EdgeSensitive_ = 1,
}
impl From<Extmode0> for bool {
    #[inline(always)]
    fn from(variant: Extmode0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTMODE0` reader - Level or edge sensitivity select for EINT0."]
pub type Extmode0R = crate::BitReader<Extmode0>;
impl Extmode0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extmode0 {
        match self.bits {
            false => Extmode0::LevelSensitive_,
            true => Extmode0::EdgeSensitive_,
        }
    }
    #[doc = "Level sensitive."]
    #[inline(always)]
    pub fn is_level_sensitive_(&self) -> bool {
        *self == Extmode0::LevelSensitive_
    }
    #[doc = "Edge sensitive."]
    #[inline(always)]
    pub fn is_edge_sensitive_(&self) -> bool {
        *self == Extmode0::EdgeSensitive_
    }
}
#[doc = "Field `EXTMODE0` writer - Level or edge sensitivity select for EINT0."]
pub type Extmode0W<'a, REG> = crate::BitWriter<'a, REG, Extmode0>;
impl<'a, REG> Extmode0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive."]
    #[inline(always)]
    pub fn level_sensitive_(self) -> &'a mut crate::W<REG> {
        self.variant(Extmode0::LevelSensitive_)
    }
    #[doc = "Edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive_(self) -> &'a mut crate::W<REG> {
        self.variant(Extmode0::EdgeSensitive_)
    }
}
#[doc = "Level or edge sensitivity select for EINT1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extmode1 {
    #[doc = "0: Level sensitive."]
    LevelSensitive_ = 0,
    #[doc = "1: Edge sensitive."]
    EdgeSensitive_ = 1,
}
impl From<Extmode1> for bool {
    #[inline(always)]
    fn from(variant: Extmode1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTMODE1` reader - Level or edge sensitivity select for EINT1."]
pub type Extmode1R = crate::BitReader<Extmode1>;
impl Extmode1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extmode1 {
        match self.bits {
            false => Extmode1::LevelSensitive_,
            true => Extmode1::EdgeSensitive_,
        }
    }
    #[doc = "Level sensitive."]
    #[inline(always)]
    pub fn is_level_sensitive_(&self) -> bool {
        *self == Extmode1::LevelSensitive_
    }
    #[doc = "Edge sensitive."]
    #[inline(always)]
    pub fn is_edge_sensitive_(&self) -> bool {
        *self == Extmode1::EdgeSensitive_
    }
}
#[doc = "Field `EXTMODE1` writer - Level or edge sensitivity select for EINT1."]
pub type Extmode1W<'a, REG> = crate::BitWriter<'a, REG, Extmode1>;
impl<'a, REG> Extmode1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive."]
    #[inline(always)]
    pub fn level_sensitive_(self) -> &'a mut crate::W<REG> {
        self.variant(Extmode1::LevelSensitive_)
    }
    #[doc = "Edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive_(self) -> &'a mut crate::W<REG> {
        self.variant(Extmode1::EdgeSensitive_)
    }
}
#[doc = "Level or edge sensitivity select for EINT2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extmode2 {
    #[doc = "0: Level sensitive."]
    LevelSensitive_ = 0,
    #[doc = "1: Edge sensitive."]
    EdgeSensitive_ = 1,
}
impl From<Extmode2> for bool {
    #[inline(always)]
    fn from(variant: Extmode2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTMODE2` reader - Level or edge sensitivity select for EINT2."]
pub type Extmode2R = crate::BitReader<Extmode2>;
impl Extmode2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extmode2 {
        match self.bits {
            false => Extmode2::LevelSensitive_,
            true => Extmode2::EdgeSensitive_,
        }
    }
    #[doc = "Level sensitive."]
    #[inline(always)]
    pub fn is_level_sensitive_(&self) -> bool {
        *self == Extmode2::LevelSensitive_
    }
    #[doc = "Edge sensitive."]
    #[inline(always)]
    pub fn is_edge_sensitive_(&self) -> bool {
        *self == Extmode2::EdgeSensitive_
    }
}
#[doc = "Field `EXTMODE2` writer - Level or edge sensitivity select for EINT2."]
pub type Extmode2W<'a, REG> = crate::BitWriter<'a, REG, Extmode2>;
impl<'a, REG> Extmode2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive."]
    #[inline(always)]
    pub fn level_sensitive_(self) -> &'a mut crate::W<REG> {
        self.variant(Extmode2::LevelSensitive_)
    }
    #[doc = "Edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive_(self) -> &'a mut crate::W<REG> {
        self.variant(Extmode2::EdgeSensitive_)
    }
}
#[doc = "Level or edge sensitivity select for EINT3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Extmode3 {
    #[doc = "0: Level sensitive."]
    LevelSensitive_ = 0,
    #[doc = "1: Edge sensitive."]
    EdgeSensitive_ = 1,
}
impl From<Extmode3> for bool {
    #[inline(always)]
    fn from(variant: Extmode3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTMODE3` reader - Level or edge sensitivity select for EINT3."]
pub type Extmode3R = crate::BitReader<Extmode3>;
impl Extmode3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Extmode3 {
        match self.bits {
            false => Extmode3::LevelSensitive_,
            true => Extmode3::EdgeSensitive_,
        }
    }
    #[doc = "Level sensitive."]
    #[inline(always)]
    pub fn is_level_sensitive_(&self) -> bool {
        *self == Extmode3::LevelSensitive_
    }
    #[doc = "Edge sensitive."]
    #[inline(always)]
    pub fn is_edge_sensitive_(&self) -> bool {
        *self == Extmode3::EdgeSensitive_
    }
}
#[doc = "Field `EXTMODE3` writer - Level or edge sensitivity select for EINT3."]
pub type Extmode3W<'a, REG> = crate::BitWriter<'a, REG, Extmode3>;
impl<'a, REG> Extmode3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive."]
    #[inline(always)]
    pub fn level_sensitive_(self) -> &'a mut crate::W<REG> {
        self.variant(Extmode3::LevelSensitive_)
    }
    #[doc = "Edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive_(self) -> &'a mut crate::W<REG> {
        self.variant(Extmode3::EdgeSensitive_)
    }
}
impl R {
    #[doc = "Bit 0 - Level or edge sensitivity select for EINT0."]
    #[inline(always)]
    pub fn extmode0(&self) -> Extmode0R {
        Extmode0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Level or edge sensitivity select for EINT1."]
    #[inline(always)]
    pub fn extmode1(&self) -> Extmode1R {
        Extmode1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Level or edge sensitivity select for EINT2."]
    #[inline(always)]
    pub fn extmode2(&self) -> Extmode2R {
        Extmode2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Level or edge sensitivity select for EINT3."]
    #[inline(always)]
    pub fn extmode3(&self) -> Extmode3R {
        Extmode3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Level or edge sensitivity select for EINT0."]
    #[inline(always)]
    pub fn extmode0(&mut self) -> Extmode0W<'_, ExtmodeSpec> {
        Extmode0W::new(self, 0)
    }
    #[doc = "Bit 1 - Level or edge sensitivity select for EINT1."]
    #[inline(always)]
    pub fn extmode1(&mut self) -> Extmode1W<'_, ExtmodeSpec> {
        Extmode1W::new(self, 1)
    }
    #[doc = "Bit 2 - Level or edge sensitivity select for EINT2."]
    #[inline(always)]
    pub fn extmode2(&mut self) -> Extmode2W<'_, ExtmodeSpec> {
        Extmode2W::new(self, 2)
    }
    #[doc = "Bit 3 - Level or edge sensitivity select for EINT3."]
    #[inline(always)]
    pub fn extmode3(&mut self) -> Extmode3W<'_, ExtmodeSpec> {
        Extmode3W::new(self, 3)
    }
}
#[doc = "External Interrupt Mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`extmode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extmode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtmodeSpec;
impl crate::RegisterSpec for ExtmodeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extmode::R`](R) reader structure"]
impl crate::Readable for ExtmodeSpec {}
#[doc = "`write(|w| ..)` method takes [`extmode::W`](W) writer structure"]
impl crate::Writable for ExtmodeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTMODE to value 0"]
impl crate::Resettable for ExtmodeSpec {}
