#[doc = "Register `EXTMODE` reader"]
pub type R = crate::R<ExtmodeSpec>;
#[doc = "Register `EXTMODE` writer"]
pub type W = crate::W<ExtmodeSpec>;
#[doc = "Level or edge sensitivity select for EINT0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Level sensitive."]
    LevelSensitive_ = 0,
    #[doc = "1: Edge sensitive."]
    EdgeSensitive_ = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTMODE0` reader - Level or edge sensitivity select for EINT0."]
pub type Extmode0R = crate::BitReader<Enum>;
impl Extmode0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::LevelSensitive_,
            true => Enum::EdgeSensitive_,
        }
    }
    #[doc = "Level sensitive."]
    #[inline(always)]
    pub fn is_level_sensitive_(&self) -> bool {
        *self == Enum::LevelSensitive_
    }
    #[doc = "Edge sensitive."]
    #[inline(always)]
    pub fn is_edge_sensitive_(&self) -> bool {
        *self == Enum::EdgeSensitive_
    }
}
#[doc = "Field `EXTMODE0` writer - Level or edge sensitivity select for EINT0."]
pub type Extmode0W<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Extmode0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive."]
    #[inline(always)]
    pub fn level_sensitive_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::LevelSensitive_)
    }
    #[doc = "Edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::EdgeSensitive_)
    }
}
#[doc = "Level or edge sensitivity select for EINT1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Level sensitive."]
    LevelSensitive_ = 0,
    #[doc = "1: Edge sensitive."]
    EdgeSensitive_ = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTMODE1` reader - Level or edge sensitivity select for EINT1."]
pub type Extmode1R = crate::BitReader<Enum>;
impl Extmode1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::LevelSensitive_,
            true => Enum::EdgeSensitive_,
        }
    }
    #[doc = "Level sensitive."]
    #[inline(always)]
    pub fn is_level_sensitive_(&self) -> bool {
        *self == Enum::LevelSensitive_
    }
    #[doc = "Edge sensitive."]
    #[inline(always)]
    pub fn is_edge_sensitive_(&self) -> bool {
        *self == Enum::EdgeSensitive_
    }
}
#[doc = "Field `EXTMODE1` writer - Level or edge sensitivity select for EINT1."]
pub type Extmode1W<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Extmode1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive."]
    #[inline(always)]
    pub fn level_sensitive_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::LevelSensitive_)
    }
    #[doc = "Edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::EdgeSensitive_)
    }
}
#[doc = "Level or edge sensitivity select for EINT2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Level sensitive."]
    LevelSensitive_ = 0,
    #[doc = "1: Edge sensitive."]
    EdgeSensitive_ = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTMODE2` reader - Level or edge sensitivity select for EINT2."]
pub type Extmode2R = crate::BitReader<Enum>;
impl Extmode2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::LevelSensitive_,
            true => Enum::EdgeSensitive_,
        }
    }
    #[doc = "Level sensitive."]
    #[inline(always)]
    pub fn is_level_sensitive_(&self) -> bool {
        *self == Enum::LevelSensitive_
    }
    #[doc = "Edge sensitive."]
    #[inline(always)]
    pub fn is_edge_sensitive_(&self) -> bool {
        *self == Enum::EdgeSensitive_
    }
}
#[doc = "Field `EXTMODE2` writer - Level or edge sensitivity select for EINT2."]
pub type Extmode2W<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Extmode2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive."]
    #[inline(always)]
    pub fn level_sensitive_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::LevelSensitive_)
    }
    #[doc = "Edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::EdgeSensitive_)
    }
}
#[doc = "Level or edge sensitivity select for EINT3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Level sensitive."]
    LevelSensitive_ = 0,
    #[doc = "1: Edge sensitive."]
    EdgeSensitive_ = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EXTMODE3` reader - Level or edge sensitivity select for EINT3."]
pub type Extmode3R = crate::BitReader<Enum>;
impl Extmode3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::LevelSensitive_,
            true => Enum::EdgeSensitive_,
        }
    }
    #[doc = "Level sensitive."]
    #[inline(always)]
    pub fn is_level_sensitive_(&self) -> bool {
        *self == Enum::LevelSensitive_
    }
    #[doc = "Edge sensitive."]
    #[inline(always)]
    pub fn is_edge_sensitive_(&self) -> bool {
        *self == Enum::EdgeSensitive_
    }
}
#[doc = "Field `EXTMODE3` writer - Level or edge sensitivity select for EINT3."]
pub type Extmode3W<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Extmode3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Level sensitive."]
    #[inline(always)]
    pub fn level_sensitive_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::LevelSensitive_)
    }
    #[doc = "Edge sensitive."]
    #[inline(always)]
    pub fn edge_sensitive_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::EdgeSensitive_)
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
