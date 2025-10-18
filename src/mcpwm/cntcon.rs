#[doc = "Register `CNTCON` reader"]
pub type R = crate::R<CntconSpec>;
#[doc = "Counter 0 rising edge mode, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: A rising edge on MCI0 does not affect counter 0."]
    ARisingEdgeOnMci = 0,
    #[doc = "1: If MODE0 is 1, counter 0 advances on a rising edge on MCI0."]
    Rising = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC0MCI0_RE` reader - Counter 0 rising edge mode, channel 0."]
pub type Tc0mci0ReR = crate::BitReader<Enum>;
impl Tc0mci0ReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::ARisingEdgeOnMci,
            true => Enum::Rising,
        }
    }
    #[doc = "A rising edge on MCI0 does not affect counter 0."]
    #[inline(always)]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == Enum::ARisingEdgeOnMci
    }
    #[doc = "If MODE0 is 1, counter 0 advances on a rising edge on MCI0."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Enum::Rising
    }
}
#[doc = "Counter 0 falling edge mode, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: A falling edge on MCI0 does not affect counter 0."]
    AFallingEdgeOnMc = 0,
    #[doc = "1: If MODE0 is 1, counter 0 advances on a falling edge on MCI0."]
    Falling = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC0MCI0_FE` reader - Counter 0 falling edge mode, channel 0."]
pub type Tc0mci0FeR = crate::BitReader<Enum>;
impl Tc0mci0FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::AFallingEdgeOnMc,
            true => Enum::Falling,
        }
    }
    #[doc = "A falling edge on MCI0 does not affect counter 0."]
    #[inline(always)]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == Enum::AFallingEdgeOnMc
    }
    #[doc = "If MODE0 is 1, counter 0 advances on a falling edge on MCI0."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Enum::Falling
    }
}
#[doc = "Counter 0 rising edge mode, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: A rising edge on MCI1 does not affect counter 0."]
    ARisingEdgeOnMci = 0,
    #[doc = "1: If MODE0 is 1, counter 0 advances on a rising edge on MCI1."]
    Rising = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC0MCI1_RE` reader - Counter 0 rising edge mode, channel 1."]
pub type Tc0mci1ReR = crate::BitReader<Enum>;
impl Tc0mci1ReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::ARisingEdgeOnMci,
            true => Enum::Rising,
        }
    }
    #[doc = "A rising edge on MCI1 does not affect counter 0."]
    #[inline(always)]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == Enum::ARisingEdgeOnMci
    }
    #[doc = "If MODE0 is 1, counter 0 advances on a rising edge on MCI1."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Enum::Rising
    }
}
#[doc = "Counter 0 falling edge mode, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: A falling edge on MCI1 does not affect counter 0."]
    AFallingEdgeOnMc = 0,
    #[doc = "1: If MODE0 is 1, counter 0 advances on a falling edge on MCI1."]
    Falling = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC0MCI1_FE` reader - Counter 0 falling edge mode, channel 1."]
pub type Tc0mci1FeR = crate::BitReader<Enum>;
impl Tc0mci1FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::AFallingEdgeOnMc,
            true => Enum::Falling,
        }
    }
    #[doc = "A falling edge on MCI1 does not affect counter 0."]
    #[inline(always)]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == Enum::AFallingEdgeOnMc
    }
    #[doc = "If MODE0 is 1, counter 0 advances on a falling edge on MCI1."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Enum::Falling
    }
}
#[doc = "Counter 0 rising edge mode, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: A rising edge on MCI0 does not affect counter 0."]
    ARisingEdgeOnMci = 0,
    #[doc = "1: If MODE0 is 1, counter 0 advances on a rising edge on MCI2."]
    Rising = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC0MCI2_RE` reader - Counter 0 rising edge mode, channel 2."]
pub type Tc0mci2ReR = crate::BitReader<Enum>;
impl Tc0mci2ReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::ARisingEdgeOnMci,
            true => Enum::Rising,
        }
    }
    #[doc = "A rising edge on MCI0 does not affect counter 0."]
    #[inline(always)]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == Enum::ARisingEdgeOnMci
    }
    #[doc = "If MODE0 is 1, counter 0 advances on a rising edge on MCI2."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Enum::Rising
    }
}
#[doc = "Counter 0 falling edge mode, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: A falling edge on MCI0 does not affect counter 0."]
    AFallingEdgeOnMc = 0,
    #[doc = "1: If MODE0 is 1, counter 0 advances on a falling edge on MCI2."]
    Fallling = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC0MCI2_FE` reader - Counter 0 falling edge mode, channel 2."]
pub type Tc0mci2FeR = crate::BitReader<Enum>;
impl Tc0mci2FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::AFallingEdgeOnMc,
            true => Enum::Fallling,
        }
    }
    #[doc = "A falling edge on MCI0 does not affect counter 0."]
    #[inline(always)]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == Enum::AFallingEdgeOnMc
    }
    #[doc = "If MODE0 is 1, counter 0 advances on a falling edge on MCI2."]
    #[inline(always)]
    pub fn is_fallling(&self) -> bool {
        *self == Enum::Fallling
    }
}
#[doc = "Counter 1 rising edge mode, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: A rising edge on MCI0 does not affect counter 1."]
    ARisingEdgeOnMci = 0,
    #[doc = "1: If MODE1 is 1, counter 1 advances on a rising edge on MCI0."]
    Rising = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC1MCI0_RE` reader - Counter 1 rising edge mode, channel 0."]
pub type Tc1mci0ReR = crate::BitReader<Enum>;
impl Tc1mci0ReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::ARisingEdgeOnMci,
            true => Enum::Rising,
        }
    }
    #[doc = "A rising edge on MCI0 does not affect counter 1."]
    #[inline(always)]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == Enum::ARisingEdgeOnMci
    }
    #[doc = "If MODE1 is 1, counter 1 advances on a rising edge on MCI0."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Enum::Rising
    }
}
#[doc = "Counter 1 falling edge mode, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: A falling edge on MCI0 does not affect counter 1."]
    AFallingEdgeOnMc = 0,
    #[doc = "1: If MODE1 is 1, counter 1 advances on a falling edge on MCI0."]
    Falling = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC1MCI0_FE` reader - Counter 1 falling edge mode, channel 0."]
pub type Tc1mci0FeR = crate::BitReader<Enum>;
impl Tc1mci0FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::AFallingEdgeOnMc,
            true => Enum::Falling,
        }
    }
    #[doc = "A falling edge on MCI0 does not affect counter 1."]
    #[inline(always)]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == Enum::AFallingEdgeOnMc
    }
    #[doc = "If MODE1 is 1, counter 1 advances on a falling edge on MCI0."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Enum::Falling
    }
}
#[doc = "Counter 1 rising edge mode, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: A rising edge on MCI1 does not affect counter 1."]
    ARisingEdgeOnMci = 0,
    #[doc = "1: If MODE1 is 1, counter 1 advances on a rising edge on MCI1."]
    Rising = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC1MCI1_RE` reader - Counter 1 rising edge mode, channel 1."]
pub type Tc1mci1ReR = crate::BitReader<Enum>;
impl Tc1mci1ReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::ARisingEdgeOnMci,
            true => Enum::Rising,
        }
    }
    #[doc = "A rising edge on MCI1 does not affect counter 1."]
    #[inline(always)]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == Enum::ARisingEdgeOnMci
    }
    #[doc = "If MODE1 is 1, counter 1 advances on a rising edge on MCI1."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Enum::Rising
    }
}
#[doc = "Counter 1 falling edge mode, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: A falling edge on MCI0 does not affect counter 1."]
    AFallingEdgeOnMc = 0,
    #[doc = "1: If MODE1 is 1, counter 1 advances on a falling edge on MCI1."]
    Falling = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC1MCI1_FE` reader - Counter 1 falling edge mode, channel 1."]
pub type Tc1mci1FeR = crate::BitReader<Enum>;
impl Tc1mci1FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::AFallingEdgeOnMc,
            true => Enum::Falling,
        }
    }
    #[doc = "A falling edge on MCI0 does not affect counter 1."]
    #[inline(always)]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == Enum::AFallingEdgeOnMc
    }
    #[doc = "If MODE1 is 1, counter 1 advances on a falling edge on MCI1."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Enum::Falling
    }
}
#[doc = "Counter 1 rising edge mode, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: A rising edge on MCI2 does not affect counter 1."]
    ARisingEdgeOnMci = 0,
    #[doc = "1: If MODE1 is 1, counter 1 advances on a rising edge on MCI2."]
    Rising = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC1MCI2_RE` reader - Counter 1 rising edge mode, channel 2."]
pub type Tc1mci2ReR = crate::BitReader<Enum>;
impl Tc1mci2ReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::ARisingEdgeOnMci,
            true => Enum::Rising,
        }
    }
    #[doc = "A rising edge on MCI2 does not affect counter 1."]
    #[inline(always)]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == Enum::ARisingEdgeOnMci
    }
    #[doc = "If MODE1 is 1, counter 1 advances on a rising edge on MCI2."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Enum::Rising
    }
}
#[doc = "Counter 1 falling edge mode, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: A falling edge on MCI2 does not affect counter 1."]
    AFallingEdgeOnMc = 0,
    #[doc = "1: If MODE1 is 1, counter 1 advances on a falling edge on MCI2."]
    Falling = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC1MCI2_FE` reader - Counter 1 falling edge mode, channel 2."]
pub type Tc1mci2FeR = crate::BitReader<Enum>;
impl Tc1mci2FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::AFallingEdgeOnMc,
            true => Enum::Falling,
        }
    }
    #[doc = "A falling edge on MCI2 does not affect counter 1."]
    #[inline(always)]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == Enum::AFallingEdgeOnMc
    }
    #[doc = "If MODE1 is 1, counter 1 advances on a falling edge on MCI2."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Enum::Falling
    }
}
#[doc = "Counter 2 rising edge mode, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: A rising edge on MCI0 does not affect counter 2."]
    ARisingEdgeOnMci = 0,
    #[doc = "1: If MODE2 is 1, counter 2 advances on a rising edge on MCI0."]
    Rising = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC2MCI0_RE` reader - Counter 2 rising edge mode, channel 0."]
pub type Tc2mci0ReR = crate::BitReader<Enum>;
impl Tc2mci0ReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::ARisingEdgeOnMci,
            true => Enum::Rising,
        }
    }
    #[doc = "A rising edge on MCI0 does not affect counter 2."]
    #[inline(always)]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == Enum::ARisingEdgeOnMci
    }
    #[doc = "If MODE2 is 1, counter 2 advances on a rising edge on MCI0."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Enum::Rising
    }
}
#[doc = "Counter 2 falling edge mode, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: A falling edge on MCI0 does not affect counter 2."]
    AFallingEdgeOnMc = 0,
    #[doc = "1: If MODE2 is 1, counter 2 advances on a falling edge on MCI0."]
    Falling = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC2MCI0_FE` reader - Counter 2 falling edge mode, channel 0."]
pub type Tc2mci0FeR = crate::BitReader<Enum>;
impl Tc2mci0FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::AFallingEdgeOnMc,
            true => Enum::Falling,
        }
    }
    #[doc = "A falling edge on MCI0 does not affect counter 2."]
    #[inline(always)]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == Enum::AFallingEdgeOnMc
    }
    #[doc = "If MODE2 is 1, counter 2 advances on a falling edge on MCI0."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Enum::Falling
    }
}
#[doc = "Counter 2 rising edge mode, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: A rising edge on MCI1 does not affect counter 2."]
    ARisingEdgeOnMci = 0,
    #[doc = "1: If MODE2 is 1, counter 2 advances on a rising edge on MCI1."]
    Rising = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC2MCI1_RE` reader - Counter 2 rising edge mode, channel 1."]
pub type Tc2mci1ReR = crate::BitReader<Enum>;
impl Tc2mci1ReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::ARisingEdgeOnMci,
            true => Enum::Rising,
        }
    }
    #[doc = "A rising edge on MCI1 does not affect counter 2."]
    #[inline(always)]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == Enum::ARisingEdgeOnMci
    }
    #[doc = "If MODE2 is 1, counter 2 advances on a rising edge on MCI1."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Enum::Rising
    }
}
#[doc = "Counter 2 falling edge mode, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: A falling edge on MCI1 does not affect counter 2."]
    AFallingEdgeOnMc = 0,
    #[doc = "1: If MODE2 is 1, counter 2 advances on a falling edge on MCI1."]
    Falling = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC2MCI1_FE` reader - Counter 2 falling edge mode, channel 1."]
pub type Tc2mci1FeR = crate::BitReader<Enum>;
impl Tc2mci1FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::AFallingEdgeOnMc,
            true => Enum::Falling,
        }
    }
    #[doc = "A falling edge on MCI1 does not affect counter 2."]
    #[inline(always)]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == Enum::AFallingEdgeOnMc
    }
    #[doc = "If MODE2 is 1, counter 2 advances on a falling edge on MCI1."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Enum::Falling
    }
}
#[doc = "Counter 2 rising edge mode, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: A rising edge on MCI2 does not affect counter 2."]
    ARisingEdgeOnMci = 0,
    #[doc = "1: If MODE2 is 1, counter 2 advances on a rising edge on MCI2."]
    Risiing = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC2MCI2_RE` reader - Counter 2 rising edge mode, channel 2."]
pub type Tc2mci2ReR = crate::BitReader<Enum>;
impl Tc2mci2ReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::ARisingEdgeOnMci,
            true => Enum::Risiing,
        }
    }
    #[doc = "A rising edge on MCI2 does not affect counter 2."]
    #[inline(always)]
    pub fn is_a_rising_edge_on_mci(&self) -> bool {
        *self == Enum::ARisingEdgeOnMci
    }
    #[doc = "If MODE2 is 1, counter 2 advances on a rising edge on MCI2."]
    #[inline(always)]
    pub fn is_risiing(&self) -> bool {
        *self == Enum::Risiing
    }
}
#[doc = "Counter 2 falling edge mode, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: A falling edge on MCI2 does not affect counter 2."]
    AFallingEdgeOnMc = 0,
    #[doc = "1: If MODE2 is 1, counter 2 advances on a falling edge on MCI2."]
    Falling = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC2MCI2_FE` reader - Counter 2 falling edge mode, channel 2."]
pub type Tc2mci2FeR = crate::BitReader<Enum>;
impl Tc2mci2FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::AFallingEdgeOnMc,
            true => Enum::Falling,
        }
    }
    #[doc = "A falling edge on MCI2 does not affect counter 2."]
    #[inline(always)]
    pub fn is_a_falling_edge_on_mc(&self) -> bool {
        *self == Enum::AFallingEdgeOnMc
    }
    #[doc = "If MODE2 is 1, counter 2 advances on a falling edge on MCI2."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Enum::Falling
    }
}
#[doc = "Channel 0 counter/timer mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Channel 0 is in timer mode."]
    Channel0IsInTime = 0,
    #[doc = "1: Channel 0 is in counter mode."]
    Channel0IsInCoun = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTR0` reader - Channel 0 counter/timer mode."]
pub type Cntr0R = crate::BitReader<Enum>;
impl Cntr0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Channel0IsInTime,
            true => Enum::Channel0IsInCoun,
        }
    }
    #[doc = "Channel 0 is in timer mode."]
    #[inline(always)]
    pub fn is_channel_0_is_in_time(&self) -> bool {
        *self == Enum::Channel0IsInTime
    }
    #[doc = "Channel 0 is in counter mode."]
    #[inline(always)]
    pub fn is_channel_0_is_in_coun(&self) -> bool {
        *self == Enum::Channel0IsInCoun
    }
}
#[doc = "Channel 1 counter/timer mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Channel 1 is in timer mode."]
    Channel1IsInTime = 0,
    #[doc = "1: Channel 1 is in counter mode."]
    Channel1IsInCoun = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTR1` reader - Channel 1 counter/timer mode."]
pub type Cntr1R = crate::BitReader<Enum>;
impl Cntr1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Channel1IsInTime,
            true => Enum::Channel1IsInCoun,
        }
    }
    #[doc = "Channel 1 is in timer mode."]
    #[inline(always)]
    pub fn is_channel_1_is_in_time(&self) -> bool {
        *self == Enum::Channel1IsInTime
    }
    #[doc = "Channel 1 is in counter mode."]
    #[inline(always)]
    pub fn is_channel_1_is_in_coun(&self) -> bool {
        *self == Enum::Channel1IsInCoun
    }
}
#[doc = "Channel 2 counter/timer mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Channel 2 is in timer mode."]
    Channel2IsInTime = 0,
    #[doc = "1: Channel 2 is in counter mode."]
    Channel2IsInCoun = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTR2` reader - Channel 2 counter/timer mode."]
pub type Cntr2R = crate::BitReader<Enum>;
impl Cntr2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Channel2IsInTime,
            true => Enum::Channel2IsInCoun,
        }
    }
    #[doc = "Channel 2 is in timer mode."]
    #[inline(always)]
    pub fn is_channel_2_is_in_time(&self) -> bool {
        *self == Enum::Channel2IsInTime
    }
    #[doc = "Channel 2 is in counter mode."]
    #[inline(always)]
    pub fn is_channel_2_is_in_coun(&self) -> bool {
        *self == Enum::Channel2IsInCoun
    }
}
impl R {
    #[doc = "Bit 0 - Counter 0 rising edge mode, channel 0."]
    #[inline(always)]
    pub fn tc0mci0_re(&self) -> Tc0mci0ReR {
        Tc0mci0ReR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Counter 0 falling edge mode, channel 0."]
    #[inline(always)]
    pub fn tc0mci0_fe(&self) -> Tc0mci0FeR {
        Tc0mci0FeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Counter 0 rising edge mode, channel 1."]
    #[inline(always)]
    pub fn tc0mci1_re(&self) -> Tc0mci1ReR {
        Tc0mci1ReR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Counter 0 falling edge mode, channel 1."]
    #[inline(always)]
    pub fn tc0mci1_fe(&self) -> Tc0mci1FeR {
        Tc0mci1FeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Counter 0 rising edge mode, channel 2."]
    #[inline(always)]
    pub fn tc0mci2_re(&self) -> Tc0mci2ReR {
        Tc0mci2ReR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Counter 0 falling edge mode, channel 2."]
    #[inline(always)]
    pub fn tc0mci2_fe(&self) -> Tc0mci2FeR {
        Tc0mci2FeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Counter 1 rising edge mode, channel 0."]
    #[inline(always)]
    pub fn tc1mci0_re(&self) -> Tc1mci0ReR {
        Tc1mci0ReR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Counter 1 falling edge mode, channel 0."]
    #[inline(always)]
    pub fn tc1mci0_fe(&self) -> Tc1mci0FeR {
        Tc1mci0FeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Counter 1 rising edge mode, channel 1."]
    #[inline(always)]
    pub fn tc1mci1_re(&self) -> Tc1mci1ReR {
        Tc1mci1ReR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Counter 1 falling edge mode, channel 1."]
    #[inline(always)]
    pub fn tc1mci1_fe(&self) -> Tc1mci1FeR {
        Tc1mci1FeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Counter 1 rising edge mode, channel 2."]
    #[inline(always)]
    pub fn tc1mci2_re(&self) -> Tc1mci2ReR {
        Tc1mci2ReR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Counter 1 falling edge mode, channel 2."]
    #[inline(always)]
    pub fn tc1mci2_fe(&self) -> Tc1mci2FeR {
        Tc1mci2FeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Counter 2 rising edge mode, channel 0."]
    #[inline(always)]
    pub fn tc2mci0_re(&self) -> Tc2mci0ReR {
        Tc2mci0ReR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Counter 2 falling edge mode, channel 0."]
    #[inline(always)]
    pub fn tc2mci0_fe(&self) -> Tc2mci0FeR {
        Tc2mci0FeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Counter 2 rising edge mode, channel 1."]
    #[inline(always)]
    pub fn tc2mci1_re(&self) -> Tc2mci1ReR {
        Tc2mci1ReR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Counter 2 falling edge mode, channel 1."]
    #[inline(always)]
    pub fn tc2mci1_fe(&self) -> Tc2mci1FeR {
        Tc2mci1FeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Counter 2 rising edge mode, channel 2."]
    #[inline(always)]
    pub fn tc2mci2_re(&self) -> Tc2mci2ReR {
        Tc2mci2ReR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Counter 2 falling edge mode, channel 2."]
    #[inline(always)]
    pub fn tc2mci2_fe(&self) -> Tc2mci2FeR {
        Tc2mci2FeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 29 - Channel 0 counter/timer mode."]
    #[inline(always)]
    pub fn cntr0(&self) -> Cntr0R {
        Cntr0R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Channel 1 counter/timer mode."]
    #[inline(always)]
    pub fn cntr1(&self) -> Cntr1R {
        Cntr1R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Channel 2 counter/timer mode."]
    #[inline(always)]
    pub fn cntr2(&self) -> Cntr2R {
        Cntr2R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Count Control read address\n\nYou can [`read`](crate::Reg::read) this register and get [`cntcon::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntconSpec;
impl crate::RegisterSpec for CntconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntcon::R`](R) reader structure"]
impl crate::Readable for CntconSpec {}
#[doc = "`reset()` method sets CNTCON to value 0"]
impl crate::Resettable for CntconSpec {}
