#[doc = "Register `CCP` reader"]
pub type R = crate::R<CcpSpec>;
#[doc = "Register `CCP` writer"]
pub type W = crate::W<CcpSpec>;
#[doc = "Communication pattern output A, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccpa0 {
    #[doc = "0: MCOA0 passive."]
    Mcoa0Passive_ = 0,
    #[doc = "1: internal MCOA0."]
    InternalMcoa0_ = 1,
}
impl From<Ccpa0> for bool {
    #[inline(always)]
    fn from(variant: Ccpa0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCPA0` reader - Communication pattern output A, channel 0."]
pub type Ccpa0R = crate::BitReader<Ccpa0>;
impl Ccpa0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccpa0 {
        match self.bits {
            false => Ccpa0::Mcoa0Passive_,
            true => Ccpa0::InternalMcoa0_,
        }
    }
    #[doc = "MCOA0 passive."]
    #[inline(always)]
    pub fn is_mcoa0_passive_(&self) -> bool {
        *self == Ccpa0::Mcoa0Passive_
    }
    #[doc = "internal MCOA0."]
    #[inline(always)]
    pub fn is_internal_mcoa0_(&self) -> bool {
        *self == Ccpa0::InternalMcoa0_
    }
}
#[doc = "Field `CCPA0` writer - Communication pattern output A, channel 0."]
pub type Ccpa0W<'a, REG> = crate::BitWriter<'a, REG, Ccpa0>;
impl<'a, REG> Ccpa0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCOA0 passive."]
    #[inline(always)]
    pub fn mcoa0_passive_(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpa0::Mcoa0Passive_)
    }
    #[doc = "internal MCOA0."]
    #[inline(always)]
    pub fn internal_mcoa0_(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpa0::InternalMcoa0_)
    }
}
#[doc = "Communication pattern output B, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccpb0 {
    #[doc = "0: MCOB0 passive."]
    Mcob0Passive_ = 0,
    #[doc = "1: MCOB0 tracks internal MCOA0."]
    Mcob0TracksInterna = 1,
}
impl From<Ccpb0> for bool {
    #[inline(always)]
    fn from(variant: Ccpb0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCPB0` reader - Communication pattern output B, channel 0."]
pub type Ccpb0R = crate::BitReader<Ccpb0>;
impl Ccpb0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccpb0 {
        match self.bits {
            false => Ccpb0::Mcob0Passive_,
            true => Ccpb0::Mcob0TracksInterna,
        }
    }
    #[doc = "MCOB0 passive."]
    #[inline(always)]
    pub fn is_mcob0_passive_(&self) -> bool {
        *self == Ccpb0::Mcob0Passive_
    }
    #[doc = "MCOB0 tracks internal MCOA0."]
    #[inline(always)]
    pub fn is_mcob0_tracks_interna(&self) -> bool {
        *self == Ccpb0::Mcob0TracksInterna
    }
}
#[doc = "Field `CCPB0` writer - Communication pattern output B, channel 0."]
pub type Ccpb0W<'a, REG> = crate::BitWriter<'a, REG, Ccpb0>;
impl<'a, REG> Ccpb0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCOB0 passive."]
    #[inline(always)]
    pub fn mcob0_passive_(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpb0::Mcob0Passive_)
    }
    #[doc = "MCOB0 tracks internal MCOA0."]
    #[inline(always)]
    pub fn mcob0_tracks_interna(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpb0::Mcob0TracksInterna)
    }
}
#[doc = "Communication pattern output A, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccpa1 {
    #[doc = "0: MCOA1 passive."]
    Mcoa1Passive_ = 0,
    #[doc = "1: MCOA1 tracks internal MCOA0."]
    Mcoa1TracksInterna = 1,
}
impl From<Ccpa1> for bool {
    #[inline(always)]
    fn from(variant: Ccpa1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCPA1` reader - Communication pattern output A, channel 1."]
pub type Ccpa1R = crate::BitReader<Ccpa1>;
impl Ccpa1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccpa1 {
        match self.bits {
            false => Ccpa1::Mcoa1Passive_,
            true => Ccpa1::Mcoa1TracksInterna,
        }
    }
    #[doc = "MCOA1 passive."]
    #[inline(always)]
    pub fn is_mcoa1_passive_(&self) -> bool {
        *self == Ccpa1::Mcoa1Passive_
    }
    #[doc = "MCOA1 tracks internal MCOA0."]
    #[inline(always)]
    pub fn is_mcoa1_tracks_interna(&self) -> bool {
        *self == Ccpa1::Mcoa1TracksInterna
    }
}
#[doc = "Field `CCPA1` writer - Communication pattern output A, channel 1."]
pub type Ccpa1W<'a, REG> = crate::BitWriter<'a, REG, Ccpa1>;
impl<'a, REG> Ccpa1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCOA1 passive."]
    #[inline(always)]
    pub fn mcoa1_passive_(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpa1::Mcoa1Passive_)
    }
    #[doc = "MCOA1 tracks internal MCOA0."]
    #[inline(always)]
    pub fn mcoa1_tracks_interna(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpa1::Mcoa1TracksInterna)
    }
}
#[doc = "Communication pattern output B, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccpb1 {
    #[doc = "0: MCOB1 passive."]
    Mcob1Passive_ = 0,
    #[doc = "1: MCOB1 tracks internal MCOA0."]
    Mcob1TracksInterna = 1,
}
impl From<Ccpb1> for bool {
    #[inline(always)]
    fn from(variant: Ccpb1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCPB1` reader - Communication pattern output B, channel 1."]
pub type Ccpb1R = crate::BitReader<Ccpb1>;
impl Ccpb1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccpb1 {
        match self.bits {
            false => Ccpb1::Mcob1Passive_,
            true => Ccpb1::Mcob1TracksInterna,
        }
    }
    #[doc = "MCOB1 passive."]
    #[inline(always)]
    pub fn is_mcob1_passive_(&self) -> bool {
        *self == Ccpb1::Mcob1Passive_
    }
    #[doc = "MCOB1 tracks internal MCOA0."]
    #[inline(always)]
    pub fn is_mcob1_tracks_interna(&self) -> bool {
        *self == Ccpb1::Mcob1TracksInterna
    }
}
#[doc = "Field `CCPB1` writer - Communication pattern output B, channel 1."]
pub type Ccpb1W<'a, REG> = crate::BitWriter<'a, REG, Ccpb1>;
impl<'a, REG> Ccpb1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCOB1 passive."]
    #[inline(always)]
    pub fn mcob1_passive_(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpb1::Mcob1Passive_)
    }
    #[doc = "MCOB1 tracks internal MCOA0."]
    #[inline(always)]
    pub fn mcob1_tracks_interna(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpb1::Mcob1TracksInterna)
    }
}
#[doc = "Communication pattern output A, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccpa2 {
    #[doc = "0: MCOA2 passive."]
    Mcoa2Passive_ = 0,
    #[doc = "1: MCOA2 tracks internal MCOA0."]
    Mcoa2TracksInterna = 1,
}
impl From<Ccpa2> for bool {
    #[inline(always)]
    fn from(variant: Ccpa2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCPA2` reader - Communication pattern output A, channel 2."]
pub type Ccpa2R = crate::BitReader<Ccpa2>;
impl Ccpa2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccpa2 {
        match self.bits {
            false => Ccpa2::Mcoa2Passive_,
            true => Ccpa2::Mcoa2TracksInterna,
        }
    }
    #[doc = "MCOA2 passive."]
    #[inline(always)]
    pub fn is_mcoa2_passive_(&self) -> bool {
        *self == Ccpa2::Mcoa2Passive_
    }
    #[doc = "MCOA2 tracks internal MCOA0."]
    #[inline(always)]
    pub fn is_mcoa2_tracks_interna(&self) -> bool {
        *self == Ccpa2::Mcoa2TracksInterna
    }
}
#[doc = "Field `CCPA2` writer - Communication pattern output A, channel 2."]
pub type Ccpa2W<'a, REG> = crate::BitWriter<'a, REG, Ccpa2>;
impl<'a, REG> Ccpa2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCOA2 passive."]
    #[inline(always)]
    pub fn mcoa2_passive_(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpa2::Mcoa2Passive_)
    }
    #[doc = "MCOA2 tracks internal MCOA0."]
    #[inline(always)]
    pub fn mcoa2_tracks_interna(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpa2::Mcoa2TracksInterna)
    }
}
#[doc = "Communication pattern output B, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccpb2 {
    #[doc = "0: MCOB2 passive."]
    Mcob2Passive_ = 0,
    #[doc = "1: MCOB2 tracks internal MCOA0."]
    Mcob2TracksInterna = 1,
}
impl From<Ccpb2> for bool {
    #[inline(always)]
    fn from(variant: Ccpb2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCPB2` reader - Communication pattern output B, channel 2."]
pub type Ccpb2R = crate::BitReader<Ccpb2>;
impl Ccpb2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccpb2 {
        match self.bits {
            false => Ccpb2::Mcob2Passive_,
            true => Ccpb2::Mcob2TracksInterna,
        }
    }
    #[doc = "MCOB2 passive."]
    #[inline(always)]
    pub fn is_mcob2_passive_(&self) -> bool {
        *self == Ccpb2::Mcob2Passive_
    }
    #[doc = "MCOB2 tracks internal MCOA0."]
    #[inline(always)]
    pub fn is_mcob2_tracks_interna(&self) -> bool {
        *self == Ccpb2::Mcob2TracksInterna
    }
}
#[doc = "Field `CCPB2` writer - Communication pattern output B, channel 2."]
pub type Ccpb2W<'a, REG> = crate::BitWriter<'a, REG, Ccpb2>;
impl<'a, REG> Ccpb2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "MCOB2 passive."]
    #[inline(always)]
    pub fn mcob2_passive_(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpb2::Mcob2Passive_)
    }
    #[doc = "MCOB2 tracks internal MCOA0."]
    #[inline(always)]
    pub fn mcob2_tracks_interna(self) -> &'a mut crate::W<REG> {
        self.variant(Ccpb2::Mcob2TracksInterna)
    }
}
impl R {
    #[doc = "Bit 0 - Communication pattern output A, channel 0."]
    #[inline(always)]
    pub fn ccpa0(&self) -> Ccpa0R {
        Ccpa0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Communication pattern output B, channel 0."]
    #[inline(always)]
    pub fn ccpb0(&self) -> Ccpb0R {
        Ccpb0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Communication pattern output A, channel 1."]
    #[inline(always)]
    pub fn ccpa1(&self) -> Ccpa1R {
        Ccpa1R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Communication pattern output B, channel 1."]
    #[inline(always)]
    pub fn ccpb1(&self) -> Ccpb1R {
        Ccpb1R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Communication pattern output A, channel 2."]
    #[inline(always)]
    pub fn ccpa2(&self) -> Ccpa2R {
        Ccpa2R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Communication pattern output B, channel 2."]
    #[inline(always)]
    pub fn ccpb2(&self) -> Ccpb2R {
        Ccpb2R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Communication pattern output A, channel 0."]
    #[inline(always)]
    pub fn ccpa0(&mut self) -> Ccpa0W<'_, CcpSpec> {
        Ccpa0W::new(self, 0)
    }
    #[doc = "Bit 1 - Communication pattern output B, channel 0."]
    #[inline(always)]
    pub fn ccpb0(&mut self) -> Ccpb0W<'_, CcpSpec> {
        Ccpb0W::new(self, 1)
    }
    #[doc = "Bit 2 - Communication pattern output A, channel 1."]
    #[inline(always)]
    pub fn ccpa1(&mut self) -> Ccpa1W<'_, CcpSpec> {
        Ccpa1W::new(self, 2)
    }
    #[doc = "Bit 3 - Communication pattern output B, channel 1."]
    #[inline(always)]
    pub fn ccpb1(&mut self) -> Ccpb1W<'_, CcpSpec> {
        Ccpb1W::new(self, 3)
    }
    #[doc = "Bit 4 - Communication pattern output A, channel 2."]
    #[inline(always)]
    pub fn ccpa2(&mut self) -> Ccpa2W<'_, CcpSpec> {
        Ccpa2W::new(self, 4)
    }
    #[doc = "Bit 5 - Communication pattern output B, channel 2."]
    #[inline(always)]
    pub fn ccpb2(&mut self) -> Ccpb2W<'_, CcpSpec> {
        Ccpb2W::new(self, 5)
    }
}
#[doc = "Communication Pattern register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcpSpec;
impl crate::RegisterSpec for CcpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccp::R`](R) reader structure"]
impl crate::Readable for CcpSpec {}
#[doc = "`write(|w| ..)` method takes [`ccp::W`](W) writer structure"]
impl crate::Writable for CcpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCP to value 0"]
impl crate::Resettable for CcpSpec {}
