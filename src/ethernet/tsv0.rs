#[doc = "Register `TSV0` reader"]
pub type R = crate::R<Tsv0Spec>;
#[doc = "Field `CRCERR` reader - CRC error. The attached CRC in the packet did not match the internally generated CRC."]
pub type CrcerrR = crate::BitReader;
#[doc = "Field `LCE` reader - Length check error. Indicates the frame length field does not match the actual number of data items and is not a type field."]
pub type LceR = crate::BitReader;
#[doc = "Field `LOR` reader - Length out of range. Indicates that frame type/length field was larger than 1500 bytes. The EMAC doesn't distinguish the frame type and frame length, so, e.g. when the IP(0x8000) or ARP(0x0806) packets are received, it compares the frame type with the max length and gives the \"Length out of range\" error. In fact, this bit is not an error indication, but simply a statement by the chip regarding the status of the received frame."]
pub type LorR = crate::BitReader;
#[doc = "Field `DONE` reader - Transmission of packet was completed."]
pub type DoneR = crate::BitReader;
#[doc = "Field `MULTICAST` reader - Packet's destination was a multicast address."]
pub type MulticastR = crate::BitReader;
#[doc = "Field `BROADCAST` reader - Packet's destination was a broadcast address."]
pub type BroadcastR = crate::BitReader;
#[doc = "Field `PACKETDEFER` reader - Packet was deferred for at least one attempt, but less than an excessive defer."]
pub type PacketdeferR = crate::BitReader;
#[doc = "Field `EXDF` reader - Excessive Defer. Packet was deferred in excess of 6071 nibble times in 100 Mbps or 24287 bit times in 10 Mbps mode."]
pub type ExdfR = crate::BitReader;
#[doc = "Field `EXCOL` reader - Excessive Collision. Packet was aborted due to exceeding of maximum allowed number of collisions."]
pub type ExcolR = crate::BitReader;
#[doc = "Field `LCOL` reader - Late Collision. Collision occurred beyond collision window, 512 bit times."]
pub type LcolR = crate::BitReader;
#[doc = "Field `GIANT` reader - Byte count in frame was greater than can be represented in the transmit byte count field in TSV1."]
pub type GiantR = crate::BitReader;
#[doc = "Field `UNDERRUN` reader - Host side caused buffer underrun."]
pub type UnderrunR = crate::BitReader;
#[doc = "Field `TOTALBYTES` reader - The total number of bytes transferred including collided attempts."]
pub type TotalbytesR = crate::FieldReader<u16>;
#[doc = "Field `CONTROLFRAME` reader - The frame was a control frame."]
pub type ControlframeR = crate::BitReader;
#[doc = "Field `PAUSE` reader - The frame was a control frame with a valid PAUSE opcode."]
pub type PauseR = crate::BitReader;
#[doc = "Field `BACKPRESSURE` reader - Carrier-sense method backpressure was previously applied."]
pub type BackpressureR = crate::BitReader;
#[doc = "Field `VLAN` reader - Frame's length/type field contained 0x8100 which is the VLAN protocol identifier."]
pub type VlanR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - CRC error. The attached CRC in the packet did not match the internally generated CRC."]
    #[inline(always)]
    pub fn crcerr(&self) -> CrcerrR {
        CrcerrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Length check error. Indicates the frame length field does not match the actual number of data items and is not a type field."]
    #[inline(always)]
    pub fn lce(&self) -> LceR {
        LceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Length out of range. Indicates that frame type/length field was larger than 1500 bytes. The EMAC doesn't distinguish the frame type and frame length, so, e.g. when the IP(0x8000) or ARP(0x0806) packets are received, it compares the frame type with the max length and gives the \"Length out of range\" error. In fact, this bit is not an error indication, but simply a statement by the chip regarding the status of the received frame."]
    #[inline(always)]
    pub fn lor(&self) -> LorR {
        LorR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmission of packet was completed."]
    #[inline(always)]
    pub fn done(&self) -> DoneR {
        DoneR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Packet's destination was a multicast address."]
    #[inline(always)]
    pub fn multicast(&self) -> MulticastR {
        MulticastR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Packet's destination was a broadcast address."]
    #[inline(always)]
    pub fn broadcast(&self) -> BroadcastR {
        BroadcastR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Packet was deferred for at least one attempt, but less than an excessive defer."]
    #[inline(always)]
    pub fn packetdefer(&self) -> PacketdeferR {
        PacketdeferR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Excessive Defer. Packet was deferred in excess of 6071 nibble times in 100 Mbps or 24287 bit times in 10 Mbps mode."]
    #[inline(always)]
    pub fn exdf(&self) -> ExdfR {
        ExdfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Excessive Collision. Packet was aborted due to exceeding of maximum allowed number of collisions."]
    #[inline(always)]
    pub fn excol(&self) -> ExcolR {
        ExcolR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Late Collision. Collision occurred beyond collision window, 512 bit times."]
    #[inline(always)]
    pub fn lcol(&self) -> LcolR {
        LcolR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Byte count in frame was greater than can be represented in the transmit byte count field in TSV1."]
    #[inline(always)]
    pub fn giant(&self) -> GiantR {
        GiantR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Host side caused buffer underrun."]
    #[inline(always)]
    pub fn underrun(&self) -> UnderrunR {
        UnderrunR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:27 - The total number of bytes transferred including collided attempts."]
    #[inline(always)]
    pub fn totalbytes(&self) -> TotalbytesR {
        TotalbytesR::new(((self.bits >> 12) & 0xffff) as u16)
    }
    #[doc = "Bit 28 - The frame was a control frame."]
    #[inline(always)]
    pub fn controlframe(&self) -> ControlframeR {
        ControlframeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The frame was a control frame with a valid PAUSE opcode."]
    #[inline(always)]
    pub fn pause(&self) -> PauseR {
        PauseR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Carrier-sense method backpressure was previously applied."]
    #[inline(always)]
    pub fn backpressure(&self) -> BackpressureR {
        BackpressureR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Frame's length/type field contained 0x8100 which is the VLAN protocol identifier."]
    #[inline(always)]
    pub fn vlan(&self) -> VlanR {
        VlanR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "Transmit status vector 0 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`tsv0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tsv0Spec;
impl crate::RegisterSpec for Tsv0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsv0::R`](R) reader structure"]
impl crate::Readable for Tsv0Spec {}
#[doc = "`reset()` method sets TSV0 to value 0"]
impl crate::Resettable for Tsv0Spec {}
