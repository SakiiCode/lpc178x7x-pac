#[doc = "Register `DMAINTST` reader"]
pub type R = crate::R<DmaintstSpec>;
#[doc = "End of Transfer Interrupt bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Eot {
    #[doc = "0: All bits in the USBEoTIntSt register are 0."]
    None = 0,
    #[doc = "1: At least one bit in the USBEoTIntSt is set."]
    Some = 1,
}
impl From<Eot> for bool {
    #[inline(always)]
    fn from(variant: Eot) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EOT` reader - End of Transfer Interrupt bit."]
pub type EotR = crate::BitReader<Eot>;
impl EotR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Eot {
        match self.bits {
            false => Eot::None,
            true => Eot::Some,
        }
    }
    #[doc = "All bits in the USBEoTIntSt register are 0."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Eot::None
    }
    #[doc = "At least one bit in the USBEoTIntSt is set."]
    #[inline(always)]
    pub fn is_some(&self) -> bool {
        *self == Eot::Some
    }
}
#[doc = "New DD Request Interrupt bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nddr {
    #[doc = "0: All bits in the USBNDDRIntSt register are 0."]
    None = 0,
    #[doc = "1: At least one bit in the USBNDDRIntSt is set."]
    Some = 1,
}
impl From<Nddr> for bool {
    #[inline(always)]
    fn from(variant: Nddr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NDDR` reader - New DD Request Interrupt bit."]
pub type NddrR = crate::BitReader<Nddr>;
impl NddrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nddr {
        match self.bits {
            false => Nddr::None,
            true => Nddr::Some,
        }
    }
    #[doc = "All bits in the USBNDDRIntSt register are 0."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Nddr::None
    }
    #[doc = "At least one bit in the USBNDDRIntSt is set."]
    #[inline(always)]
    pub fn is_some(&self) -> bool {
        *self == Nddr::Some
    }
}
#[doc = "System Error Interrupt bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Err {
    #[doc = "0: All bits in the USBSysErrIntSt register are 0."]
    None = 0,
    #[doc = "1: At least one bit in the USBSysErrIntSt is set."]
    Some = 1,
}
impl From<Err> for bool {
    #[inline(always)]
    fn from(variant: Err) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERR` reader - System Error Interrupt bit."]
pub type ErrR = crate::BitReader<Err>;
impl ErrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Err {
        match self.bits {
            false => Err::None,
            true => Err::Some,
        }
    }
    #[doc = "All bits in the USBSysErrIntSt register are 0."]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Err::None
    }
    #[doc = "At least one bit in the USBSysErrIntSt is set."]
    #[inline(always)]
    pub fn is_some(&self) -> bool {
        *self == Err::Some
    }
}
impl R {
    #[doc = "Bit 0 - End of Transfer Interrupt bit."]
    #[inline(always)]
    pub fn eot(&self) -> EotR {
        EotR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - New DD Request Interrupt bit."]
    #[inline(always)]
    pub fn nddr(&self) -> NddrR {
        NddrR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System Error Interrupt bit."]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "USB DMA Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`dmaintst::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmaintstSpec;
impl crate::RegisterSpec for DmaintstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmaintst::R`](R) reader structure"]
impl crate::Readable for DmaintstSpec {}
#[doc = "`reset()` method sets DMAINTST to value 0"]
impl crate::Resettable for DmaintstSpec {}
