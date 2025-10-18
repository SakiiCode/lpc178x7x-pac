#[doc = "Register `STATUS` reader"]
pub type R = crate::R<StatusSpec>;
#[doc = "Busy. This bit is used to ensure that the memory controller enters the low-power or disabled mode cleanly by determining if the memory controller is busy or not.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: EMC is idle (warm reset value)."]
    Idle = 0,
    #[doc = "1: EMC is busy performing memory transactions, commands, auto-refresh cycles, or is in self-refresh mode (POR reset value)."]
    Busy = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B` reader - Busy. This bit is used to ensure that the memory controller enters the low-power or disabled mode cleanly by determining if the memory controller is busy or not."]
pub type BR = crate::BitReader<Enum>;
impl BR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Idle,
            true => Enum::Busy,
        }
    }
    #[doc = "EMC is idle (warm reset value)."]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == Enum::Idle
    }
    #[doc = "EMC is busy performing memory transactions, commands, auto-refresh cycles, or is in self-refresh mode (POR reset value)."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Enum::Busy
    }
}
#[doc = "Write buffer status.This bit enables the EMC to enter low-power mode or disabled mode cleanly.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Write buffers empty (POR reset value)"]
    Empty = 0,
    #[doc = "1: Write buffers contain data."]
    Data = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `S` reader - Write buffer status.This bit enables the EMC to enter low-power mode or disabled mode cleanly."]
pub type SR = crate::BitReader<Enum>;
impl SR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Empty,
            true => Enum::Data,
        }
    }
    #[doc = "Write buffers empty (POR reset value)"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Enum::Empty
    }
    #[doc = "Write buffers contain data."]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == Enum::Data
    }
}
#[doc = "Self-refresh acknowledge. This bit indicates the operating mode of the EMC.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Normal mode"]
    Normal = 0,
    #[doc = "1: Self-refresh mode (POR reset value)."]
    Selfrefresh = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SA` reader - Self-refresh acknowledge. This bit indicates the operating mode of the EMC."]
pub type SaR = crate::BitReader<Enum>;
impl SaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Normal,
            true => Enum::Selfrefresh,
        }
    }
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == Enum::Normal
    }
    #[doc = "Self-refresh mode (POR reset value)."]
    #[inline(always)]
    pub fn is_selfrefresh(&self) -> bool {
        *self == Enum::Selfrefresh
    }
}
impl R {
    #[doc = "Bit 0 - Busy. This bit is used to ensure that the memory controller enters the low-power or disabled mode cleanly by determining if the memory controller is busy or not."]
    #[inline(always)]
    pub fn b(&self) -> BR {
        BR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write buffer status.This bit enables the EMC to enter low-power mode or disabled mode cleanly."]
    #[inline(always)]
    pub fn s(&self) -> SR {
        SR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Self-refresh acknowledge. This bit indicates the operating mode of the EMC."]
    #[inline(always)]
    pub fn sa(&self) -> SaR {
        SaR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "Provides EMC status information.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatusSpec;
impl crate::RegisterSpec for StatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for StatusSpec {}
#[doc = "`reset()` method sets STATUS to value 0x05"]
impl crate::Resettable for StatusSpec {
    const RESET_VALUE: u32 = 0x05;
}
