#[doc = "Register `CNTCON` reader"]
pub type R = crate::R<CntconSpec>;
#[doc = "Counter 0 rising edge mode, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tc0mci0Re {
    #[doc = "0: A rising edge on MCI0 does not affect counter 0."]
    Disabled = 0,
    #[doc = "1: If MODE0 is 1, counter 0 advances on a rising edge on MCI0."]
    Rising = 1,
}
impl From<Tc0mci0Re> for bool {
    #[inline(always)]
    fn from(variant: Tc0mci0Re) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC0MCI0_RE` reader - Counter 0 rising edge mode, channel 0."]
pub type Tc0mci0ReR = crate::BitReader<Tc0mci0Re>;
impl Tc0mci0ReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tc0mci0Re {
        match self.bits {
            false => Tc0mci0Re::Disabled,
            true => Tc0mci0Re::Rising,
        }
    }
    #[doc = "A rising edge on MCI0 does not affect counter 0."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tc0mci0Re::Disabled
    }
    #[doc = "If MODE0 is 1, counter 0 advances on a rising edge on MCI0."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Tc0mci0Re::Rising
    }
}
#[doc = "Counter 0 falling edge mode, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tc0mci0Fe {
    #[doc = "0: A falling edge on MCI0 does not affect counter 0."]
    Disabled = 0,
    #[doc = "1: If MODE0 is 1, counter 0 advances on a falling edge on MCI0."]
    Falling = 1,
}
impl From<Tc0mci0Fe> for bool {
    #[inline(always)]
    fn from(variant: Tc0mci0Fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC0MCI0_FE` reader - Counter 0 falling edge mode, channel 0."]
pub type Tc0mci0FeR = crate::BitReader<Tc0mci0Fe>;
impl Tc0mci0FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tc0mci0Fe {
        match self.bits {
            false => Tc0mci0Fe::Disabled,
            true => Tc0mci0Fe::Falling,
        }
    }
    #[doc = "A falling edge on MCI0 does not affect counter 0."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tc0mci0Fe::Disabled
    }
    #[doc = "If MODE0 is 1, counter 0 advances on a falling edge on MCI0."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Tc0mci0Fe::Falling
    }
}
#[doc = "Counter 0 rising edge mode, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tc0mci1Re {
    #[doc = "0: A rising edge on MCI1 does not affect counter 0."]
    Disabled = 0,
    #[doc = "1: If MODE0 is 1, counter 0 advances on a rising edge on MCI1."]
    Rising = 1,
}
impl From<Tc0mci1Re> for bool {
    #[inline(always)]
    fn from(variant: Tc0mci1Re) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC0MCI1_RE` reader - Counter 0 rising edge mode, channel 1."]
pub type Tc0mci1ReR = crate::BitReader<Tc0mci1Re>;
impl Tc0mci1ReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tc0mci1Re {
        match self.bits {
            false => Tc0mci1Re::Disabled,
            true => Tc0mci1Re::Rising,
        }
    }
    #[doc = "A rising edge on MCI1 does not affect counter 0."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tc0mci1Re::Disabled
    }
    #[doc = "If MODE0 is 1, counter 0 advances on a rising edge on MCI1."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Tc0mci1Re::Rising
    }
}
#[doc = "Counter 0 falling edge mode, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tc0mci1Fe {
    #[doc = "0: A falling edge on MCI1 does not affect counter 0."]
    Disabled = 0,
    #[doc = "1: If MODE0 is 1, counter 0 advances on a falling edge on MCI1."]
    Falling = 1,
}
impl From<Tc0mci1Fe> for bool {
    #[inline(always)]
    fn from(variant: Tc0mci1Fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC0MCI1_FE` reader - Counter 0 falling edge mode, channel 1."]
pub type Tc0mci1FeR = crate::BitReader<Tc0mci1Fe>;
impl Tc0mci1FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tc0mci1Fe {
        match self.bits {
            false => Tc0mci1Fe::Disabled,
            true => Tc0mci1Fe::Falling,
        }
    }
    #[doc = "A falling edge on MCI1 does not affect counter 0."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tc0mci1Fe::Disabled
    }
    #[doc = "If MODE0 is 1, counter 0 advances on a falling edge on MCI1."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Tc0mci1Fe::Falling
    }
}
#[doc = "Counter 0 rising edge mode, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tc0mci2Re {
    #[doc = "0: A rising edge on MCI0 does not affect counter 0."]
    Disabled = 0,
    #[doc = "1: If MODE0 is 1, counter 0 advances on a rising edge on MCI2."]
    Rising = 1,
}
impl From<Tc0mci2Re> for bool {
    #[inline(always)]
    fn from(variant: Tc0mci2Re) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC0MCI2_RE` reader - Counter 0 rising edge mode, channel 2."]
pub type Tc0mci2ReR = crate::BitReader<Tc0mci2Re>;
impl Tc0mci2ReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tc0mci2Re {
        match self.bits {
            false => Tc0mci2Re::Disabled,
            true => Tc0mci2Re::Rising,
        }
    }
    #[doc = "A rising edge on MCI0 does not affect counter 0."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tc0mci2Re::Disabled
    }
    #[doc = "If MODE0 is 1, counter 0 advances on a rising edge on MCI2."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Tc0mci2Re::Rising
    }
}
#[doc = "Counter 0 falling edge mode, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tc0mci2Fe {
    #[doc = "0: A falling edge on MCI0 does not affect counter 0."]
    Disabled = 0,
    #[doc = "1: If MODE0 is 1, counter 0 advances on a falling edge on MCI2."]
    Fallling = 1,
}
impl From<Tc0mci2Fe> for bool {
    #[inline(always)]
    fn from(variant: Tc0mci2Fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC0MCI2_FE` reader - Counter 0 falling edge mode, channel 2."]
pub type Tc0mci2FeR = crate::BitReader<Tc0mci2Fe>;
impl Tc0mci2FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tc0mci2Fe {
        match self.bits {
            false => Tc0mci2Fe::Disabled,
            true => Tc0mci2Fe::Fallling,
        }
    }
    #[doc = "A falling edge on MCI0 does not affect counter 0."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tc0mci2Fe::Disabled
    }
    #[doc = "If MODE0 is 1, counter 0 advances on a falling edge on MCI2."]
    #[inline(always)]
    pub fn is_fallling(&self) -> bool {
        *self == Tc0mci2Fe::Fallling
    }
}
#[doc = "Counter 1 rising edge mode, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tc1mci0Re {
    #[doc = "0: A rising edge on MCI0 does not affect counter 1."]
    Disabled = 0,
    #[doc = "1: If MODE1 is 1, counter 1 advances on a rising edge on MCI0."]
    Rising = 1,
}
impl From<Tc1mci0Re> for bool {
    #[inline(always)]
    fn from(variant: Tc1mci0Re) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC1MCI0_RE` reader - Counter 1 rising edge mode, channel 0."]
pub type Tc1mci0ReR = crate::BitReader<Tc1mci0Re>;
impl Tc1mci0ReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tc1mci0Re {
        match self.bits {
            false => Tc1mci0Re::Disabled,
            true => Tc1mci0Re::Rising,
        }
    }
    #[doc = "A rising edge on MCI0 does not affect counter 1."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tc1mci0Re::Disabled
    }
    #[doc = "If MODE1 is 1, counter 1 advances on a rising edge on MCI0."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Tc1mci0Re::Rising
    }
}
#[doc = "Counter 1 falling edge mode, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tc1mci0Fe {
    #[doc = "0: A falling edge on MCI0 does not affect counter 1."]
    Disabled = 0,
    #[doc = "1: If MODE1 is 1, counter 1 advances on a falling edge on MCI0."]
    Falling = 1,
}
impl From<Tc1mci0Fe> for bool {
    #[inline(always)]
    fn from(variant: Tc1mci0Fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC1MCI0_FE` reader - Counter 1 falling edge mode, channel 0."]
pub type Tc1mci0FeR = crate::BitReader<Tc1mci0Fe>;
impl Tc1mci0FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tc1mci0Fe {
        match self.bits {
            false => Tc1mci0Fe::Disabled,
            true => Tc1mci0Fe::Falling,
        }
    }
    #[doc = "A falling edge on MCI0 does not affect counter 1."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tc1mci0Fe::Disabled
    }
    #[doc = "If MODE1 is 1, counter 1 advances on a falling edge on MCI0."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Tc1mci0Fe::Falling
    }
}
#[doc = "Counter 1 rising edge mode, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tc1mci1Re {
    #[doc = "0: A rising edge on MCI1 does not affect counter 1."]
    Disabled = 0,
    #[doc = "1: If MODE1 is 1, counter 1 advances on a rising edge on MCI1."]
    Rising = 1,
}
impl From<Tc1mci1Re> for bool {
    #[inline(always)]
    fn from(variant: Tc1mci1Re) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC1MCI1_RE` reader - Counter 1 rising edge mode, channel 1."]
pub type Tc1mci1ReR = crate::BitReader<Tc1mci1Re>;
impl Tc1mci1ReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tc1mci1Re {
        match self.bits {
            false => Tc1mci1Re::Disabled,
            true => Tc1mci1Re::Rising,
        }
    }
    #[doc = "A rising edge on MCI1 does not affect counter 1."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tc1mci1Re::Disabled
    }
    #[doc = "If MODE1 is 1, counter 1 advances on a rising edge on MCI1."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Tc1mci1Re::Rising
    }
}
#[doc = "Counter 1 falling edge mode, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tc1mci1Fe {
    #[doc = "0: A falling edge on MCI0 does not affect counter 1."]
    Disabled = 0,
    #[doc = "1: If MODE1 is 1, counter 1 advances on a falling edge on MCI1."]
    Falling = 1,
}
impl From<Tc1mci1Fe> for bool {
    #[inline(always)]
    fn from(variant: Tc1mci1Fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC1MCI1_FE` reader - Counter 1 falling edge mode, channel 1."]
pub type Tc1mci1FeR = crate::BitReader<Tc1mci1Fe>;
impl Tc1mci1FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tc1mci1Fe {
        match self.bits {
            false => Tc1mci1Fe::Disabled,
            true => Tc1mci1Fe::Falling,
        }
    }
    #[doc = "A falling edge on MCI0 does not affect counter 1."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tc1mci1Fe::Disabled
    }
    #[doc = "If MODE1 is 1, counter 1 advances on a falling edge on MCI1."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Tc1mci1Fe::Falling
    }
}
#[doc = "Counter 1 rising edge mode, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tc1mci2Re {
    #[doc = "0: A rising edge on MCI2 does not affect counter 1."]
    Disabled = 0,
    #[doc = "1: If MODE1 is 1, counter 1 advances on a rising edge on MCI2."]
    Rising = 1,
}
impl From<Tc1mci2Re> for bool {
    #[inline(always)]
    fn from(variant: Tc1mci2Re) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC1MCI2_RE` reader - Counter 1 rising edge mode, channel 2."]
pub type Tc1mci2ReR = crate::BitReader<Tc1mci2Re>;
impl Tc1mci2ReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tc1mci2Re {
        match self.bits {
            false => Tc1mci2Re::Disabled,
            true => Tc1mci2Re::Rising,
        }
    }
    #[doc = "A rising edge on MCI2 does not affect counter 1."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tc1mci2Re::Disabled
    }
    #[doc = "If MODE1 is 1, counter 1 advances on a rising edge on MCI2."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Tc1mci2Re::Rising
    }
}
#[doc = "Counter 1 falling edge mode, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tc1mci2Fe {
    #[doc = "0: A falling edge on MCI2 does not affect counter 1."]
    Disabled = 0,
    #[doc = "1: If MODE1 is 1, counter 1 advances on a falling edge on MCI2."]
    Falling = 1,
}
impl From<Tc1mci2Fe> for bool {
    #[inline(always)]
    fn from(variant: Tc1mci2Fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC1MCI2_FE` reader - Counter 1 falling edge mode, channel 2."]
pub type Tc1mci2FeR = crate::BitReader<Tc1mci2Fe>;
impl Tc1mci2FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tc1mci2Fe {
        match self.bits {
            false => Tc1mci2Fe::Disabled,
            true => Tc1mci2Fe::Falling,
        }
    }
    #[doc = "A falling edge on MCI2 does not affect counter 1."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tc1mci2Fe::Disabled
    }
    #[doc = "If MODE1 is 1, counter 1 advances on a falling edge on MCI2."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Tc1mci2Fe::Falling
    }
}
#[doc = "Counter 2 rising edge mode, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tc2mci0Re {
    #[doc = "0: A rising edge on MCI0 does not affect counter 2."]
    Disabled = 0,
    #[doc = "1: If MODE2 is 1, counter 2 advances on a rising edge on MCI0."]
    Rising = 1,
}
impl From<Tc2mci0Re> for bool {
    #[inline(always)]
    fn from(variant: Tc2mci0Re) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC2MCI0_RE` reader - Counter 2 rising edge mode, channel 0."]
pub type Tc2mci0ReR = crate::BitReader<Tc2mci0Re>;
impl Tc2mci0ReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tc2mci0Re {
        match self.bits {
            false => Tc2mci0Re::Disabled,
            true => Tc2mci0Re::Rising,
        }
    }
    #[doc = "A rising edge on MCI0 does not affect counter 2."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tc2mci0Re::Disabled
    }
    #[doc = "If MODE2 is 1, counter 2 advances on a rising edge on MCI0."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Tc2mci0Re::Rising
    }
}
#[doc = "Counter 2 falling edge mode, channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tc2mci0Fe {
    #[doc = "0: A falling edge on MCI0 does not affect counter 2."]
    Disabled = 0,
    #[doc = "1: If MODE2 is 1, counter 2 advances on a falling edge on MCI0."]
    Falling = 1,
}
impl From<Tc2mci0Fe> for bool {
    #[inline(always)]
    fn from(variant: Tc2mci0Fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC2MCI0_FE` reader - Counter 2 falling edge mode, channel 0."]
pub type Tc2mci0FeR = crate::BitReader<Tc2mci0Fe>;
impl Tc2mci0FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tc2mci0Fe {
        match self.bits {
            false => Tc2mci0Fe::Disabled,
            true => Tc2mci0Fe::Falling,
        }
    }
    #[doc = "A falling edge on MCI0 does not affect counter 2."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tc2mci0Fe::Disabled
    }
    #[doc = "If MODE2 is 1, counter 2 advances on a falling edge on MCI0."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Tc2mci0Fe::Falling
    }
}
#[doc = "Counter 2 rising edge mode, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tc2mci1Re {
    #[doc = "0: A rising edge on MCI1 does not affect counter 2."]
    Disabled = 0,
    #[doc = "1: If MODE2 is 1, counter 2 advances on a rising edge on MCI1."]
    Rising = 1,
}
impl From<Tc2mci1Re> for bool {
    #[inline(always)]
    fn from(variant: Tc2mci1Re) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC2MCI1_RE` reader - Counter 2 rising edge mode, channel 1."]
pub type Tc2mci1ReR = crate::BitReader<Tc2mci1Re>;
impl Tc2mci1ReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tc2mci1Re {
        match self.bits {
            false => Tc2mci1Re::Disabled,
            true => Tc2mci1Re::Rising,
        }
    }
    #[doc = "A rising edge on MCI1 does not affect counter 2."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tc2mci1Re::Disabled
    }
    #[doc = "If MODE2 is 1, counter 2 advances on a rising edge on MCI1."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Tc2mci1Re::Rising
    }
}
#[doc = "Counter 2 falling edge mode, channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tc2mci1Fe {
    #[doc = "0: A falling edge on MCI1 does not affect counter 2."]
    Disabled = 0,
    #[doc = "1: If MODE2 is 1, counter 2 advances on a falling edge on MCI1."]
    Falling = 1,
}
impl From<Tc2mci1Fe> for bool {
    #[inline(always)]
    fn from(variant: Tc2mci1Fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC2MCI1_FE` reader - Counter 2 falling edge mode, channel 1."]
pub type Tc2mci1FeR = crate::BitReader<Tc2mci1Fe>;
impl Tc2mci1FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tc2mci1Fe {
        match self.bits {
            false => Tc2mci1Fe::Disabled,
            true => Tc2mci1Fe::Falling,
        }
    }
    #[doc = "A falling edge on MCI1 does not affect counter 2."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tc2mci1Fe::Disabled
    }
    #[doc = "If MODE2 is 1, counter 2 advances on a falling edge on MCI1."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Tc2mci1Fe::Falling
    }
}
#[doc = "Counter 2 rising edge mode, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tc2mci2Re {
    #[doc = "0: A rising edge on MCI2 does not affect counter 2."]
    Disabled = 0,
    #[doc = "1: If MODE2 is 1, counter 2 advances on a rising edge on MCI2."]
    Risiing = 1,
}
impl From<Tc2mci2Re> for bool {
    #[inline(always)]
    fn from(variant: Tc2mci2Re) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC2MCI2_RE` reader - Counter 2 rising edge mode, channel 2."]
pub type Tc2mci2ReR = crate::BitReader<Tc2mci2Re>;
impl Tc2mci2ReR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tc2mci2Re {
        match self.bits {
            false => Tc2mci2Re::Disabled,
            true => Tc2mci2Re::Risiing,
        }
    }
    #[doc = "A rising edge on MCI2 does not affect counter 2."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tc2mci2Re::Disabled
    }
    #[doc = "If MODE2 is 1, counter 2 advances on a rising edge on MCI2."]
    #[inline(always)]
    pub fn is_risiing(&self) -> bool {
        *self == Tc2mci2Re::Risiing
    }
}
#[doc = "Counter 2 falling edge mode, channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tc2mci2Fe {
    #[doc = "0: A falling edge on MCI2 does not affect counter 2."]
    Disabled = 0,
    #[doc = "1: If MODE2 is 1, counter 2 advances on a falling edge on MCI2."]
    Falling = 1,
}
impl From<Tc2mci2Fe> for bool {
    #[inline(always)]
    fn from(variant: Tc2mci2Fe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TC2MCI2_FE` reader - Counter 2 falling edge mode, channel 2."]
pub type Tc2mci2FeR = crate::BitReader<Tc2mci2Fe>;
impl Tc2mci2FeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tc2mci2Fe {
        match self.bits {
            false => Tc2mci2Fe::Disabled,
            true => Tc2mci2Fe::Falling,
        }
    }
    #[doc = "A falling edge on MCI2 does not affect counter 2."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Tc2mci2Fe::Disabled
    }
    #[doc = "If MODE2 is 1, counter 2 advances on a falling edge on MCI2."]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Tc2mci2Fe::Falling
    }
}
#[doc = "Channel 0 counter/timer mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cntr0 {
    #[doc = "0: Channel 0 is in timer mode."]
    Timer = 0,
    #[doc = "1: Channel 0 is in counter mode."]
    Counter = 1,
}
impl From<Cntr0> for bool {
    #[inline(always)]
    fn from(variant: Cntr0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTR0` reader - Channel 0 counter/timer mode."]
pub type Cntr0R = crate::BitReader<Cntr0>;
impl Cntr0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cntr0 {
        match self.bits {
            false => Cntr0::Timer,
            true => Cntr0::Counter,
        }
    }
    #[doc = "Channel 0 is in timer mode."]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == Cntr0::Timer
    }
    #[doc = "Channel 0 is in counter mode."]
    #[inline(always)]
    pub fn is_counter(&self) -> bool {
        *self == Cntr0::Counter
    }
}
#[doc = "Channel 1 counter/timer mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cntr1 {
    #[doc = "0: Channel 1 is in timer mode."]
    Timer = 0,
    #[doc = "1: Channel 1 is in counter mode."]
    Counter = 1,
}
impl From<Cntr1> for bool {
    #[inline(always)]
    fn from(variant: Cntr1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTR1` reader - Channel 1 counter/timer mode."]
pub type Cntr1R = crate::BitReader<Cntr1>;
impl Cntr1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cntr1 {
        match self.bits {
            false => Cntr1::Timer,
            true => Cntr1::Counter,
        }
    }
    #[doc = "Channel 1 is in timer mode."]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == Cntr1::Timer
    }
    #[doc = "Channel 1 is in counter mode."]
    #[inline(always)]
    pub fn is_counter(&self) -> bool {
        *self == Cntr1::Counter
    }
}
#[doc = "Channel 2 counter/timer mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cntr2 {
    #[doc = "0: Channel 2 is in timer mode."]
    Timer = 0,
    #[doc = "1: Channel 2 is in counter mode."]
    Counter = 1,
}
impl From<Cntr2> for bool {
    #[inline(always)]
    fn from(variant: Cntr2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CNTR2` reader - Channel 2 counter/timer mode."]
pub type Cntr2R = crate::BitReader<Cntr2>;
impl Cntr2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cntr2 {
        match self.bits {
            false => Cntr2::Timer,
            true => Cntr2::Counter,
        }
    }
    #[doc = "Channel 2 is in timer mode."]
    #[inline(always)]
    pub fn is_timer(&self) -> bool {
        *self == Cntr2::Timer
    }
    #[doc = "Channel 2 is in counter mode."]
    #[inline(always)]
    pub fn is_counter(&self) -> bool {
        *self == Cntr2::Counter
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
