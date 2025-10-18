#[doc = "Register `EMR` reader"]
pub type R = crate::R<EmrSpec>;
#[doc = "Register `EMR` writer"]
pub type W = crate::W<EmrSpec>;
#[doc = "Field `EM0` reader - External Match 0. When a match occurs between the TC and MR0, this bit can either toggle, go low, go high, or do nothing, depending on bits 5:4 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
pub type Em0R = crate::BitReader;
#[doc = "Field `EM0` writer - External Match 0. When a match occurs between the TC and MR0, this bit can either toggle, go low, go high, or do nothing, depending on bits 5:4 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
pub type Em0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM1` reader - External Match 1. When a match occurs between the TC and MR1, this bit can either toggle, go low, go high, or do nothing, depending on bits 7:6 of this register. This bit can be driven onto a MATn.1 pin, in a positive-logic manner (0 = low, 1 = high)."]
pub type Em1R = crate::BitReader;
#[doc = "Field `EM1` writer - External Match 1. When a match occurs between the TC and MR1, this bit can either toggle, go low, go high, or do nothing, depending on bits 7:6 of this register. This bit can be driven onto a MATn.1 pin, in a positive-logic manner (0 = low, 1 = high)."]
pub type Em1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM2` reader - External Match 2. When a match occurs between the TC and MR2, this bit can either toggle, go low, go high, or do nothing, depending on bits 9:8 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
pub type Em2R = crate::BitReader;
#[doc = "Field `EM2` writer - External Match 2. When a match occurs between the TC and MR2, this bit can either toggle, go low, go high, or do nothing, depending on bits 9:8 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
pub type Em2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EM3` reader - External Match 3. When a match occurs between the TC and MR3, this bit can either toggle, go low, go high, or do nothing, depending on bits 11:10 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
pub type Em3R = crate::BitReader;
#[doc = "Field `EM3` writer - External Match 3. When a match occurs between the TC and MR3, this bit can either toggle, go low, go high, or do nothing, depending on bits 11:10 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
pub type Em3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "External Match Control 0. Determines the functionality of External Match 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Enum {
    #[doc = "0: Do Nothing."]
    DoNothing_ = 0,
    #[doc = "1: Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    ClearTheCorrespond = 1,
    #[doc = "2: Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SetTheCorrespondin = 2,
    #[doc = "3: Toggle the corresponding External Match bit/output."]
    ToggleTheCorrespon = 3,
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
#[doc = "Field `EMC0` reader - External Match Control 0. Determines the functionality of External Match 0."]
pub type Emc0R = crate::FieldReader<Enum>;
impl Emc0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            0 => Enum::DoNothing_,
            1 => Enum::ClearTheCorrespond,
            2 => Enum::SetTheCorrespondin,
            3 => Enum::ToggleTheCorrespon,
            _ => unreachable!(),
        }
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn is_do_nothing_(&self) -> bool {
        *self == Enum::DoNothing_
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn is_clear_the_correspond(&self) -> bool {
        *self == Enum::ClearTheCorrespond
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn is_set_the_correspondin(&self) -> bool {
        *self == Enum::SetTheCorrespondin
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn is_toggle_the_correspon(&self) -> bool {
        *self == Enum::ToggleTheCorrespon
    }
}
#[doc = "Field `EMC0` writer - External Match Control 0. Determines the functionality of External Match 0."]
pub type Emc0W<'a, REG> = crate::FieldWriter<'a, REG, 2, Enum, crate::Safe>;
impl<'a, REG> Emc0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::DoNothing_)
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear_the_correspond(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::ClearTheCorrespond)
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set_the_correspondin(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::SetTheCorrespondin)
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle_the_correspon(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::ToggleTheCorrespon)
    }
}
#[doc = "External Match Control 1. Determines the functionality of External Match 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Enum {
    #[doc = "0: Do Nothing."]
    DoNothing_ = 0,
    #[doc = "1: Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    ClearTheCorrespond = 1,
    #[doc = "2: Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SetTheCorrespondin = 2,
    #[doc = "3: Toggle the corresponding External Match bit/output."]
    ToggleTheCorrespon = 3,
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
#[doc = "Field `EMC1` reader - External Match Control 1. Determines the functionality of External Match 1."]
pub type Emc1R = crate::FieldReader<Enum>;
impl Emc1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            0 => Enum::DoNothing_,
            1 => Enum::ClearTheCorrespond,
            2 => Enum::SetTheCorrespondin,
            3 => Enum::ToggleTheCorrespon,
            _ => unreachable!(),
        }
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn is_do_nothing_(&self) -> bool {
        *self == Enum::DoNothing_
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn is_clear_the_correspond(&self) -> bool {
        *self == Enum::ClearTheCorrespond
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn is_set_the_correspondin(&self) -> bool {
        *self == Enum::SetTheCorrespondin
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn is_toggle_the_correspon(&self) -> bool {
        *self == Enum::ToggleTheCorrespon
    }
}
#[doc = "Field `EMC1` writer - External Match Control 1. Determines the functionality of External Match 1."]
pub type Emc1W<'a, REG> = crate::FieldWriter<'a, REG, 2, Enum, crate::Safe>;
impl<'a, REG> Emc1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::DoNothing_)
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear_the_correspond(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::ClearTheCorrespond)
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set_the_correspondin(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::SetTheCorrespondin)
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle_the_correspon(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::ToggleTheCorrespon)
    }
}
#[doc = "External Match Control 2. Determines the functionality of External Match 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Enum {
    #[doc = "0: Do Nothing."]
    DoNothing_ = 0,
    #[doc = "1: Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    ClearTheCorrespond = 1,
    #[doc = "2: Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SetTheCorrespondin = 2,
    #[doc = "3: Toggle the corresponding External Match bit/output."]
    ToggleTheCorrespon = 3,
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
#[doc = "Field `EMC2` reader - External Match Control 2. Determines the functionality of External Match 2."]
pub type Emc2R = crate::FieldReader<Enum>;
impl Emc2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            0 => Enum::DoNothing_,
            1 => Enum::ClearTheCorrespond,
            2 => Enum::SetTheCorrespondin,
            3 => Enum::ToggleTheCorrespon,
            _ => unreachable!(),
        }
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn is_do_nothing_(&self) -> bool {
        *self == Enum::DoNothing_
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn is_clear_the_correspond(&self) -> bool {
        *self == Enum::ClearTheCorrespond
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn is_set_the_correspondin(&self) -> bool {
        *self == Enum::SetTheCorrespondin
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn is_toggle_the_correspon(&self) -> bool {
        *self == Enum::ToggleTheCorrespon
    }
}
#[doc = "Field `EMC2` writer - External Match Control 2. Determines the functionality of External Match 2."]
pub type Emc2W<'a, REG> = crate::FieldWriter<'a, REG, 2, Enum, crate::Safe>;
impl<'a, REG> Emc2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::DoNothing_)
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear_the_correspond(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::ClearTheCorrespond)
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set_the_correspondin(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::SetTheCorrespondin)
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle_the_correspon(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::ToggleTheCorrespon)
    }
}
#[doc = "External Match Control 3. Determines the functionality of External Match 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Enum {
    #[doc = "0: Do Nothing."]
    DoNothing_ = 0,
    #[doc = "1: Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    ClearTheCorrespond = 1,
    #[doc = "2: Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    SetTheCorrespondin = 2,
    #[doc = "3: Toggle the corresponding External Match bit/output."]
    ToggleTheCorrespon = 3,
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
#[doc = "Field `EMC3` reader - External Match Control 3. Determines the functionality of External Match 3."]
pub type Emc3R = crate::FieldReader<Enum>;
impl Emc3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            0 => Enum::DoNothing_,
            1 => Enum::ClearTheCorrespond,
            2 => Enum::SetTheCorrespondin,
            3 => Enum::ToggleTheCorrespon,
            _ => unreachable!(),
        }
    }
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn is_do_nothing_(&self) -> bool {
        *self == Enum::DoNothing_
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn is_clear_the_correspond(&self) -> bool {
        *self == Enum::ClearTheCorrespond
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn is_set_the_correspondin(&self) -> bool {
        *self == Enum::SetTheCorrespondin
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn is_toggle_the_correspon(&self) -> bool {
        *self == Enum::ToggleTheCorrespon
    }
}
#[doc = "Field `EMC3` writer - External Match Control 3. Determines the functionality of External Match 3."]
pub type Emc3W<'a, REG> = crate::FieldWriter<'a, REG, 2, Enum, crate::Safe>;
impl<'a, REG> Emc3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Do Nothing."]
    #[inline(always)]
    pub fn do_nothing_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::DoNothing_)
    }
    #[doc = "Clear the corresponding External Match bit/output to 0 (MATn.m pin is LOW if pinned out)."]
    #[inline(always)]
    pub fn clear_the_correspond(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::ClearTheCorrespond)
    }
    #[doc = "Set the corresponding External Match bit/output to 1 (MATn.m pin is HIGH if pinned out)."]
    #[inline(always)]
    pub fn set_the_correspondin(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::SetTheCorrespondin)
    }
    #[doc = "Toggle the corresponding External Match bit/output."]
    #[inline(always)]
    pub fn toggle_the_correspon(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::ToggleTheCorrespon)
    }
}
impl R {
    #[doc = "Bit 0 - External Match 0. When a match occurs between the TC and MR0, this bit can either toggle, go low, go high, or do nothing, depending on bits 5:4 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em0(&self) -> Em0R {
        Em0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Match 1. When a match occurs between the TC and MR1, this bit can either toggle, go low, go high, or do nothing, depending on bits 7:6 of this register. This bit can be driven onto a MATn.1 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em1(&self) -> Em1R {
        Em1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - External Match 2. When a match occurs between the TC and MR2, this bit can either toggle, go low, go high, or do nothing, depending on bits 9:8 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em2(&self) -> Em2R {
        Em2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - External Match 3. When a match occurs between the TC and MR3, this bit can either toggle, go low, go high, or do nothing, depending on bits 11:10 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em3(&self) -> Em3R {
        Em3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - External Match Control 0. Determines the functionality of External Match 0."]
    #[inline(always)]
    pub fn emc0(&self) -> Emc0R {
        Emc0R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - External Match Control 1. Determines the functionality of External Match 1."]
    #[inline(always)]
    pub fn emc1(&self) -> Emc1R {
        Emc1R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - External Match Control 2. Determines the functionality of External Match 2."]
    #[inline(always)]
    pub fn emc2(&self) -> Emc2R {
        Emc2R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - External Match Control 3. Determines the functionality of External Match 3."]
    #[inline(always)]
    pub fn emc3(&self) -> Emc3R {
        Emc3R::new(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - External Match 0. When a match occurs between the TC and MR0, this bit can either toggle, go low, go high, or do nothing, depending on bits 5:4 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em0(&mut self) -> Em0W<'_, EmrSpec> {
        Em0W::new(self, 0)
    }
    #[doc = "Bit 1 - External Match 1. When a match occurs between the TC and MR1, this bit can either toggle, go low, go high, or do nothing, depending on bits 7:6 of this register. This bit can be driven onto a MATn.1 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em1(&mut self) -> Em1W<'_, EmrSpec> {
        Em1W::new(self, 1)
    }
    #[doc = "Bit 2 - External Match 2. When a match occurs between the TC and MR2, this bit can either toggle, go low, go high, or do nothing, depending on bits 9:8 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em2(&mut self) -> Em2W<'_, EmrSpec> {
        Em2W::new(self, 2)
    }
    #[doc = "Bit 3 - External Match 3. When a match occurs between the TC and MR3, this bit can either toggle, go low, go high, or do nothing, depending on bits 11:10 of this register. This bit can be driven onto a MATn.0 pin, in a positive-logic manner (0 = low, 1 = high)."]
    #[inline(always)]
    pub fn em3(&mut self) -> Em3W<'_, EmrSpec> {
        Em3W::new(self, 3)
    }
    #[doc = "Bits 4:5 - External Match Control 0. Determines the functionality of External Match 0."]
    #[inline(always)]
    pub fn emc0(&mut self) -> Emc0W<'_, EmrSpec> {
        Emc0W::new(self, 4)
    }
    #[doc = "Bits 6:7 - External Match Control 1. Determines the functionality of External Match 1."]
    #[inline(always)]
    pub fn emc1(&mut self) -> Emc1W<'_, EmrSpec> {
        Emc1W::new(self, 6)
    }
    #[doc = "Bits 8:9 - External Match Control 2. Determines the functionality of External Match 2."]
    #[inline(always)]
    pub fn emc2(&mut self) -> Emc2W<'_, EmrSpec> {
        Emc2W::new(self, 8)
    }
    #[doc = "Bits 10:11 - External Match Control 3. Determines the functionality of External Match 3."]
    #[inline(always)]
    pub fn emc3(&mut self) -> Emc3W<'_, EmrSpec> {
        Emc3W::new(self, 10)
    }
}
#[doc = "External Match Register. The EMR controls the external match pins.\n\nYou can [`read`](crate::Reg::read) this register and get [`emr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EmrSpec;
impl crate::RegisterSpec for EmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`emr::R`](R) reader structure"]
impl crate::Readable for EmrSpec {}
#[doc = "`write(|w| ..)` method takes [`emr::W`](W) writer structure"]
impl crate::Writable for EmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EMR to value 0"]
impl crate::Resettable for EmrSpec {}
