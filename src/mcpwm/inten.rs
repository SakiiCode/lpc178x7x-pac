#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Limit interrupt for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ilim0 {
    #[doc = "0: Interrupt disabled."]
    InterruptDisabled_ = 0,
    #[doc = "1: Interrupt enabled."]
    InterruptEnabled_ = 1,
}
impl From<Ilim0> for bool {
    #[inline(always)]
    fn from(variant: Ilim0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILIM0` reader - Limit interrupt for channel 0."]
pub type Ilim0R = crate::BitReader<Ilim0>;
impl Ilim0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ilim0 {
        match self.bits {
            false => Ilim0::InterruptDisabled_,
            true => Ilim0::InterruptEnabled_,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == Ilim0::InterruptDisabled_
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == Ilim0::InterruptEnabled_
    }
}
#[doc = "Match interrupt for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Imat0 {
    #[doc = "0: Interrupt disabled."]
    InterruptDisabled_ = 0,
    #[doc = "1: Interrupt enabled."]
    InterruptEnabled_ = 1,
}
impl From<Imat0> for bool {
    #[inline(always)]
    fn from(variant: Imat0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMAT0` reader - Match interrupt for channel 0."]
pub type Imat0R = crate::BitReader<Imat0>;
impl Imat0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Imat0 {
        match self.bits {
            false => Imat0::InterruptDisabled_,
            true => Imat0::InterruptEnabled_,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == Imat0::InterruptDisabled_
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == Imat0::InterruptEnabled_
    }
}
#[doc = "Capture interrupt for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icap0 {
    #[doc = "0: Interrupt disabled."]
    InterruptDisabled_ = 0,
    #[doc = "1: Interrupt enabled."]
    InterruptEnabled_ = 1,
}
impl From<Icap0> for bool {
    #[inline(always)]
    fn from(variant: Icap0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICAP0` reader - Capture interrupt for channel 0."]
pub type Icap0R = crate::BitReader<Icap0>;
impl Icap0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Icap0 {
        match self.bits {
            false => Icap0::InterruptDisabled_,
            true => Icap0::InterruptEnabled_,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == Icap0::InterruptDisabled_
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == Icap0::InterruptEnabled_
    }
}
#[doc = "Limit interrupt for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ilim1 {
    #[doc = "0: Interrupt disabled."]
    InterruptDisabled_ = 0,
    #[doc = "1: Interrupt enabled."]
    InterruptEnabled_ = 1,
}
impl From<Ilim1> for bool {
    #[inline(always)]
    fn from(variant: Ilim1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILIM1` reader - Limit interrupt for channel 1."]
pub type Ilim1R = crate::BitReader<Ilim1>;
impl Ilim1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ilim1 {
        match self.bits {
            false => Ilim1::InterruptDisabled_,
            true => Ilim1::InterruptEnabled_,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == Ilim1::InterruptDisabled_
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == Ilim1::InterruptEnabled_
    }
}
#[doc = "Match interrupt for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Imat1 {
    #[doc = "0: Interrupt disabled."]
    InterruptDisabled_ = 0,
    #[doc = "1: Interrupt enabled."]
    InterruptEnabled_ = 1,
}
impl From<Imat1> for bool {
    #[inline(always)]
    fn from(variant: Imat1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMAT1` reader - Match interrupt for channel 1."]
pub type Imat1R = crate::BitReader<Imat1>;
impl Imat1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Imat1 {
        match self.bits {
            false => Imat1::InterruptDisabled_,
            true => Imat1::InterruptEnabled_,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == Imat1::InterruptDisabled_
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == Imat1::InterruptEnabled_
    }
}
#[doc = "Capture interrupt for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icap1 {
    #[doc = "0: Interrupt disabled."]
    InterruptDisabled_ = 0,
    #[doc = "1: Interrupt enabled."]
    InterruptEnabled_ = 1,
}
impl From<Icap1> for bool {
    #[inline(always)]
    fn from(variant: Icap1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICAP1` reader - Capture interrupt for channel 1."]
pub type Icap1R = crate::BitReader<Icap1>;
impl Icap1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Icap1 {
        match self.bits {
            false => Icap1::InterruptDisabled_,
            true => Icap1::InterruptEnabled_,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == Icap1::InterruptDisabled_
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == Icap1::InterruptEnabled_
    }
}
#[doc = "Limit interrupt for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ilim2 {
    #[doc = "0: Interrupt disabled."]
    InterruptDisabled_ = 0,
    #[doc = "1: Interrupt enabled."]
    InterruptEnabled_ = 1,
}
impl From<Ilim2> for bool {
    #[inline(always)]
    fn from(variant: Ilim2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILIM2` reader - Limit interrupt for channel 2."]
pub type Ilim2R = crate::BitReader<Ilim2>;
impl Ilim2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ilim2 {
        match self.bits {
            false => Ilim2::InterruptDisabled_,
            true => Ilim2::InterruptEnabled_,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == Ilim2::InterruptDisabled_
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == Ilim2::InterruptEnabled_
    }
}
#[doc = "Match interrupt for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Imat2 {
    #[doc = "0: Interrupt disabled."]
    InterruptDisabled_ = 0,
    #[doc = "1: Interrupt enabled."]
    InterruptEnabled_ = 1,
}
impl From<Imat2> for bool {
    #[inline(always)]
    fn from(variant: Imat2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMAT2` reader - Match interrupt for channel 2."]
pub type Imat2R = crate::BitReader<Imat2>;
impl Imat2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Imat2 {
        match self.bits {
            false => Imat2::InterruptDisabled_,
            true => Imat2::InterruptEnabled_,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == Imat2::InterruptDisabled_
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == Imat2::InterruptEnabled_
    }
}
#[doc = "Capture interrupt for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icap2 {
    #[doc = "0: Interrupt disabled."]
    InterruptDisabled_ = 0,
    #[doc = "1: Interrupt enabled."]
    InterruptEnabled_ = 1,
}
impl From<Icap2> for bool {
    #[inline(always)]
    fn from(variant: Icap2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICAP2` reader - Capture interrupt for channel 2."]
pub type Icap2R = crate::BitReader<Icap2>;
impl Icap2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Icap2 {
        match self.bits {
            false => Icap2::InterruptDisabled_,
            true => Icap2::InterruptEnabled_,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == Icap2::InterruptDisabled_
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == Icap2::InterruptEnabled_
    }
}
#[doc = "Fast abort interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Abort {
    #[doc = "0: Interrupt disabled."]
    InterruptDisabled_ = 0,
    #[doc = "1: Interrupt enabled."]
    InterruptEnabled_ = 1,
}
impl From<Abort> for bool {
    #[inline(always)]
    fn from(variant: Abort) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT` reader - Fast abort interrupt."]
pub type AbortR = crate::BitReader<Abort>;
impl AbortR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Abort {
        match self.bits {
            false => Abort::InterruptDisabled_,
            true => Abort::InterruptEnabled_,
        }
    }
    #[doc = "Interrupt disabled."]
    #[inline(always)]
    pub fn is_interrupt_disabled_(&self) -> bool {
        *self == Abort::InterruptDisabled_
    }
    #[doc = "Interrupt enabled."]
    #[inline(always)]
    pub fn is_interrupt_enabled_(&self) -> bool {
        *self == Abort::InterruptEnabled_
    }
}
impl R {
    #[doc = "Bit 0 - Limit interrupt for channel 0."]
    #[inline(always)]
    pub fn ilim0(&self) -> Ilim0R {
        Ilim0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Match interrupt for channel 0."]
    #[inline(always)]
    pub fn imat0(&self) -> Imat0R {
        Imat0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture interrupt for channel 0."]
    #[inline(always)]
    pub fn icap0(&self) -> Icap0R {
        Icap0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Limit interrupt for channel 1."]
    #[inline(always)]
    pub fn ilim1(&self) -> Ilim1R {
        Ilim1R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Match interrupt for channel 1."]
    #[inline(always)]
    pub fn imat1(&self) -> Imat1R {
        Imat1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture interrupt for channel 1."]
    #[inline(always)]
    pub fn icap1(&self) -> Icap1R {
        Icap1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Limit interrupt for channel 2."]
    #[inline(always)]
    pub fn ilim2(&self) -> Ilim2R {
        Ilim2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Match interrupt for channel 2."]
    #[inline(always)]
    pub fn imat2(&self) -> Imat2R {
        Imat2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture interrupt for channel 2."]
    #[inline(always)]
    pub fn icap2(&self) -> Icap2R {
        Icap2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Fast abort interrupt."]
    #[inline(always)]
    pub fn abort(&self) -> AbortR {
        AbortR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Interrupt Enable read address\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {}
