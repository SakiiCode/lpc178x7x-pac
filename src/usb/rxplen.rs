#[doc = "Register `RXPLEN` reader"]
pub type R = crate::R<RxplenSpec>;
#[doc = "Field `PKT_LNGTH` reader - The remaining number of bytes to be read from the currently selected endpoint's buffer. When this field decrements to 0, the RxENDPKT bit will be set in USBDevIntSt."]
pub type PktLngthR = crate::FieldReader<u16>;
#[doc = "Data valid. This bit is useful for isochronous endpoints. Non-isochronous endpoints do not raise an interrupt when an erroneous data packet is received. But invalid data packet can be produced with a bus reset. For isochronous endpoints, data transfer will happen even if an erroneous packet is received. In this case DV bit will not be set for the packet.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dv {
    #[doc = "0: Data is invalid."]
    Invalid = 0,
    #[doc = "1: Data is valid."]
    Valid = 1,
}
impl From<Dv> for bool {
    #[inline(always)]
    fn from(variant: Dv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DV` reader - Data valid. This bit is useful for isochronous endpoints. Non-isochronous endpoints do not raise an interrupt when an erroneous data packet is received. But invalid data packet can be produced with a bus reset. For isochronous endpoints, data transfer will happen even if an erroneous packet is received. In this case DV bit will not be set for the packet."]
pub type DvR = crate::BitReader<Dv>;
impl DvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dv {
        match self.bits {
            false => Dv::Invalid,
            true => Dv::Valid,
        }
    }
    #[doc = "Data is invalid."]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == Dv::Invalid
    }
    #[doc = "Data is valid."]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == Dv::Valid
    }
}
#[doc = "Field `PKT_RDY` reader - The PKT_LNGTH field is valid and the packet is ready for reading."]
pub type PktRdyR = crate::BitReader;
impl R {
    #[doc = "Bits 0:9 - The remaining number of bytes to be read from the currently selected endpoint's buffer. When this field decrements to 0, the RxENDPKT bit will be set in USBDevIntSt."]
    #[inline(always)]
    pub fn pkt_lngth(&self) -> PktLngthR {
        PktLngthR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Data valid. This bit is useful for isochronous endpoints. Non-isochronous endpoints do not raise an interrupt when an erroneous data packet is received. But invalid data packet can be produced with a bus reset. For isochronous endpoints, data transfer will happen even if an erroneous packet is received. In this case DV bit will not be set for the packet."]
    #[inline(always)]
    pub fn dv(&self) -> DvR {
        DvR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The PKT_LNGTH field is valid and the packet is ready for reading."]
    #[inline(always)]
    pub fn pkt_rdy(&self) -> PktRdyR {
        PktRdyR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "USB Receive Packet Length\n\nYou can [`read`](crate::Reg::read) this register and get [`rxplen::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxplenSpec;
impl crate::RegisterSpec for RxplenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxplen::R`](R) reader structure"]
impl crate::Readable for RxplenSpec {}
#[doc = "`reset()` method sets RXPLEN to value 0"]
impl crate::Resettable for RxplenSpec {}
