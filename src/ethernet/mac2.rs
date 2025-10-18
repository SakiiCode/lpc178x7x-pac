#[doc = "Register `MAC2` reader"]
pub type R = crate::R<Mac2Spec>;
#[doc = "Register `MAC2` writer"]
pub type W = crate::W<Mac2Spec>;
#[doc = "Field `FULLDUPLEX` reader - When enabled (set to 1), the MAC operates in Full-Duplex mode. When disabled, the MAC operates in Half-Duplex mode."]
pub type FullduplexR = crate::BitReader;
#[doc = "Field `FULLDUPLEX` writer - When enabled (set to 1), the MAC operates in Full-Duplex mode. When disabled, the MAC operates in Half-Duplex mode."]
pub type FullduplexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLC` reader - FRAMELENGTH CHECKING. When enabled (set to 1), both transmit and receive frame lengths are compared to the Length/Type field. If the Length/Type field represents a length then the check is performed. Mismatches are reported in the StatusInfo word for each received frame."]
pub type FlcR = crate::BitReader;
#[doc = "Field `FLC` writer - FRAMELENGTH CHECKING. When enabled (set to 1), both transmit and receive frame lengths are compared to the Length/Type field. If the Length/Type field represents a length then the check is performed. Mismatches are reported in the StatusInfo word for each received frame."]
pub type FlcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HFEN` reader - HUGE FRAME ENABLEWhen enabled (set to 1), frames of any length are transmitted and received."]
pub type HfenR = crate::BitReader;
#[doc = "Field `HFEN` writer - HUGE FRAME ENABLEWhen enabled (set to 1), frames of any length are transmitted and received."]
pub type HfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DELAYEDCRC` reader - DELAYED CRC. This bit determines the number of bytes, if any, of proprietary header information that exist on the front of IEEE 802.3 frames. When 1, four bytes of header (ignored by the CRC function) are added. When 0, there is no proprietary header."]
pub type DelayedcrcR = crate::BitReader;
#[doc = "Field `DELAYEDCRC` writer - DELAYED CRC. This bit determines the number of bytes, if any, of proprietary header information that exist on the front of IEEE 802.3 frames. When 1, four bytes of header (ignored by the CRC function) are added. When 0, there is no proprietary header."]
pub type DelayedcrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEN` reader - CRC ENABLESet this bit to append a CRC to every frame whether padding was required or not. Must be set if PAD/CRC ENABLE is set. Clear this bit if frames presented to the MAC contain a CRC."]
pub type CrcenR = crate::BitReader;
#[doc = "Field `CRCEN` writer - CRC ENABLESet this bit to append a CRC to every frame whether padding was required or not. Must be set if PAD/CRC ENABLE is set. Clear this bit if frames presented to the MAC contain a CRC."]
pub type CrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PADCRCEN` reader - PAD CRC ENABLE. Set this bit to have the MAC pad all short frames. Clear this bit if frames presented to the MAC have a valid length. This bit is used in conjunction with AUTO PAD ENABLE and VLAN PAD ENABLE. See Table 153 - Pad Operation for details on the pad function."]
pub type PadcrcenR = crate::BitReader;
#[doc = "Field `PADCRCEN` writer - PAD CRC ENABLE. Set this bit to have the MAC pad all short frames. Clear this bit if frames presented to the MAC have a valid length. This bit is used in conjunction with AUTO PAD ENABLE and VLAN PAD ENABLE. See Table 153 - Pad Operation for details on the pad function."]
pub type PadcrcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VLANPADEN` reader - VLAN PAD ENABLE. Set this bit to cause the MAC to pad all short frames to 64 bytes and append a valid CRC. Consult Table 153 - Pad Operation for more information on the various padding features. Note: This bit is ignored if PAD / CRC ENABLE is cleared."]
pub type VlanpadenR = crate::BitReader;
#[doc = "Field `VLANPADEN` writer - VLAN PAD ENABLE. Set this bit to cause the MAC to pad all short frames to 64 bytes and append a valid CRC. Consult Table 153 - Pad Operation for more information on the various padding features. Note: This bit is ignored if PAD / CRC ENABLE is cleared."]
pub type VlanpadenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTODETPADEN` reader - AUTODETECTPAD ENABLE. Set this bit to cause the MAC to automatically detect the type of frame, either tagged or un-tagged, by comparing the two octets following the source address with 0x8100 (VLAN Protocol ID) and pad accordingly. Table 153 - Pad Operation provides a description of the pad function based on the configuration of this register. Note: This bit is ignored if PAD / CRC ENABLE is cleared."]
pub type AutodetpadenR = crate::BitReader;
#[doc = "Field `AUTODETPADEN` writer - AUTODETECTPAD ENABLE. Set this bit to cause the MAC to automatically detect the type of frame, either tagged or un-tagged, by comparing the two octets following the source address with 0x8100 (VLAN Protocol ID) and pad accordingly. Table 153 - Pad Operation provides a description of the pad function based on the configuration of this register. Note: This bit is ignored if PAD / CRC ENABLE is cleared."]
pub type AutodetpadenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PPENF` reader - PURE PREAMBLE ENFORCEMEN. When enabled (set to 1), the MAC will verify the content of the preamble to ensure it contains 0x55 and is error-free. A packet with an incorrect preamble is discarded. When disabled, no preamble checking is performed."]
pub type PpenfR = crate::BitReader;
#[doc = "Field `PPENF` writer - PURE PREAMBLE ENFORCEMEN. When enabled (set to 1), the MAC will verify the content of the preamble to ensure it contains 0x55 and is error-free. A packet with an incorrect preamble is discarded. When disabled, no preamble checking is performed."]
pub type PpenfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPENF` reader - LONG PREAMBLE ENFORCEMENT. When enabled (set to 1), the MAC only allows receive packets which contain preamble fields less than 12 bytes in length. When disabled, the MAC allows any length preamble as per the Standard."]
pub type LpenfR = crate::BitReader;
#[doc = "Field `LPENF` writer - LONG PREAMBLE ENFORCEMENT. When enabled (set to 1), the MAC only allows receive packets which contain preamble fields less than 12 bytes in length. When disabled, the MAC allows any length preamble as per the Standard."]
pub type LpenfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOBACKOFF` reader - When enabled (set to 1), the MAC will immediately retransmit following a collision rather than using the Binary Exponential Backoff algorithm as specified in the Standard."]
pub type NobackoffR = crate::BitReader;
#[doc = "Field `NOBACKOFF` writer - When enabled (set to 1), the MAC will immediately retransmit following a collision rather than using the Binary Exponential Backoff algorithm as specified in the Standard."]
pub type NobackoffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BP_NOBACKOFF` reader - BACK PRESSURE / NO BACKOFF. When enabled (set to 1), after the MAC incidentally causes a collision during back pressure, it will immediately retransmit without backoff, reducing the chance of further collisions and ensuring transmit packets get sent."]
pub type BpNobackoffR = crate::BitReader;
#[doc = "Field `BP_NOBACKOFF` writer - BACK PRESSURE / NO BACKOFF. When enabled (set to 1), after the MAC incidentally causes a collision during back pressure, it will immediately retransmit without backoff, reducing the chance of further collisions and ensuring transmit packets get sent."]
pub type BpNobackoffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXCESSDEFER` reader - When enabled (set to 1) the MAC will defer to carrier indefinitely as per the Standard. When disabled, the MAC will abort when the excessive deferral limit is reached."]
pub type ExcessdeferR = crate::BitReader;
#[doc = "Field `EXCESSDEFER` writer - When enabled (set to 1) the MAC will defer to carrier indefinitely as per the Standard. When disabled, the MAC will abort when the excessive deferral limit is reached."]
pub type ExcessdeferW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When enabled (set to 1), the MAC operates in Full-Duplex mode. When disabled, the MAC operates in Half-Duplex mode."]
    #[inline(always)]
    pub fn fullduplex(&self) -> FullduplexR {
        FullduplexR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FRAMELENGTH CHECKING. When enabled (set to 1), both transmit and receive frame lengths are compared to the Length/Type field. If the Length/Type field represents a length then the check is performed. Mismatches are reported in the StatusInfo word for each received frame."]
    #[inline(always)]
    pub fn flc(&self) -> FlcR {
        FlcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HUGE FRAME ENABLEWhen enabled (set to 1), frames of any length are transmitted and received."]
    #[inline(always)]
    pub fn hfen(&self) -> HfenR {
        HfenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DELAYED CRC. This bit determines the number of bytes, if any, of proprietary header information that exist on the front of IEEE 802.3 frames. When 1, four bytes of header (ignored by the CRC function) are added. When 0, there is no proprietary header."]
    #[inline(always)]
    pub fn delayedcrc(&self) -> DelayedcrcR {
        DelayedcrcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC ENABLESet this bit to append a CRC to every frame whether padding was required or not. Must be set if PAD/CRC ENABLE is set. Clear this bit if frames presented to the MAC contain a CRC."]
    #[inline(always)]
    pub fn crcen(&self) -> CrcenR {
        CrcenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PAD CRC ENABLE. Set this bit to have the MAC pad all short frames. Clear this bit if frames presented to the MAC have a valid length. This bit is used in conjunction with AUTO PAD ENABLE and VLAN PAD ENABLE. See Table 153 - Pad Operation for details on the pad function."]
    #[inline(always)]
    pub fn padcrcen(&self) -> PadcrcenR {
        PadcrcenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - VLAN PAD ENABLE. Set this bit to cause the MAC to pad all short frames to 64 bytes and append a valid CRC. Consult Table 153 - Pad Operation for more information on the various padding features. Note: This bit is ignored if PAD / CRC ENABLE is cleared."]
    #[inline(always)]
    pub fn vlanpaden(&self) -> VlanpadenR {
        VlanpadenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AUTODETECTPAD ENABLE. Set this bit to cause the MAC to automatically detect the type of frame, either tagged or un-tagged, by comparing the two octets following the source address with 0x8100 (VLAN Protocol ID) and pad accordingly. Table 153 - Pad Operation provides a description of the pad function based on the configuration of this register. Note: This bit is ignored if PAD / CRC ENABLE is cleared."]
    #[inline(always)]
    pub fn autodetpaden(&self) -> AutodetpadenR {
        AutodetpadenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PURE PREAMBLE ENFORCEMEN. When enabled (set to 1), the MAC will verify the content of the preamble to ensure it contains 0x55 and is error-free. A packet with an incorrect preamble is discarded. When disabled, no preamble checking is performed."]
    #[inline(always)]
    pub fn ppenf(&self) -> PpenfR {
        PpenfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LONG PREAMBLE ENFORCEMENT. When enabled (set to 1), the MAC only allows receive packets which contain preamble fields less than 12 bytes in length. When disabled, the MAC allows any length preamble as per the Standard."]
    #[inline(always)]
    pub fn lpenf(&self) -> LpenfR {
        LpenfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - When enabled (set to 1), the MAC will immediately retransmit following a collision rather than using the Binary Exponential Backoff algorithm as specified in the Standard."]
    #[inline(always)]
    pub fn nobackoff(&self) -> NobackoffR {
        NobackoffR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - BACK PRESSURE / NO BACKOFF. When enabled (set to 1), after the MAC incidentally causes a collision during back pressure, it will immediately retransmit without backoff, reducing the chance of further collisions and ensuring transmit packets get sent."]
    #[inline(always)]
    pub fn bp_nobackoff(&self) -> BpNobackoffR {
        BpNobackoffR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - When enabled (set to 1) the MAC will defer to carrier indefinitely as per the Standard. When disabled, the MAC will abort when the excessive deferral limit is reached."]
    #[inline(always)]
    pub fn excessdefer(&self) -> ExcessdeferR {
        ExcessdeferR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When enabled (set to 1), the MAC operates in Full-Duplex mode. When disabled, the MAC operates in Half-Duplex mode."]
    #[inline(always)]
    pub fn fullduplex(&mut self) -> FullduplexW<'_, Mac2Spec> {
        FullduplexW::new(self, 0)
    }
    #[doc = "Bit 1 - FRAMELENGTH CHECKING. When enabled (set to 1), both transmit and receive frame lengths are compared to the Length/Type field. If the Length/Type field represents a length then the check is performed. Mismatches are reported in the StatusInfo word for each received frame."]
    #[inline(always)]
    pub fn flc(&mut self) -> FlcW<'_, Mac2Spec> {
        FlcW::new(self, 1)
    }
    #[doc = "Bit 2 - HUGE FRAME ENABLEWhen enabled (set to 1), frames of any length are transmitted and received."]
    #[inline(always)]
    pub fn hfen(&mut self) -> HfenW<'_, Mac2Spec> {
        HfenW::new(self, 2)
    }
    #[doc = "Bit 3 - DELAYED CRC. This bit determines the number of bytes, if any, of proprietary header information that exist on the front of IEEE 802.3 frames. When 1, four bytes of header (ignored by the CRC function) are added. When 0, there is no proprietary header."]
    #[inline(always)]
    pub fn delayedcrc(&mut self) -> DelayedcrcW<'_, Mac2Spec> {
        DelayedcrcW::new(self, 3)
    }
    #[doc = "Bit 4 - CRC ENABLESet this bit to append a CRC to every frame whether padding was required or not. Must be set if PAD/CRC ENABLE is set. Clear this bit if frames presented to the MAC contain a CRC."]
    #[inline(always)]
    pub fn crcen(&mut self) -> CrcenW<'_, Mac2Spec> {
        CrcenW::new(self, 4)
    }
    #[doc = "Bit 5 - PAD CRC ENABLE. Set this bit to have the MAC pad all short frames. Clear this bit if frames presented to the MAC have a valid length. This bit is used in conjunction with AUTO PAD ENABLE and VLAN PAD ENABLE. See Table 153 - Pad Operation for details on the pad function."]
    #[inline(always)]
    pub fn padcrcen(&mut self) -> PadcrcenW<'_, Mac2Spec> {
        PadcrcenW::new(self, 5)
    }
    #[doc = "Bit 6 - VLAN PAD ENABLE. Set this bit to cause the MAC to pad all short frames to 64 bytes and append a valid CRC. Consult Table 153 - Pad Operation for more information on the various padding features. Note: This bit is ignored if PAD / CRC ENABLE is cleared."]
    #[inline(always)]
    pub fn vlanpaden(&mut self) -> VlanpadenW<'_, Mac2Spec> {
        VlanpadenW::new(self, 6)
    }
    #[doc = "Bit 7 - AUTODETECTPAD ENABLE. Set this bit to cause the MAC to automatically detect the type of frame, either tagged or un-tagged, by comparing the two octets following the source address with 0x8100 (VLAN Protocol ID) and pad accordingly. Table 153 - Pad Operation provides a description of the pad function based on the configuration of this register. Note: This bit is ignored if PAD / CRC ENABLE is cleared."]
    #[inline(always)]
    pub fn autodetpaden(&mut self) -> AutodetpadenW<'_, Mac2Spec> {
        AutodetpadenW::new(self, 7)
    }
    #[doc = "Bit 8 - PURE PREAMBLE ENFORCEMEN. When enabled (set to 1), the MAC will verify the content of the preamble to ensure it contains 0x55 and is error-free. A packet with an incorrect preamble is discarded. When disabled, no preamble checking is performed."]
    #[inline(always)]
    pub fn ppenf(&mut self) -> PpenfW<'_, Mac2Spec> {
        PpenfW::new(self, 8)
    }
    #[doc = "Bit 9 - LONG PREAMBLE ENFORCEMENT. When enabled (set to 1), the MAC only allows receive packets which contain preamble fields less than 12 bytes in length. When disabled, the MAC allows any length preamble as per the Standard."]
    #[inline(always)]
    pub fn lpenf(&mut self) -> LpenfW<'_, Mac2Spec> {
        LpenfW::new(self, 9)
    }
    #[doc = "Bit 12 - When enabled (set to 1), the MAC will immediately retransmit following a collision rather than using the Binary Exponential Backoff algorithm as specified in the Standard."]
    #[inline(always)]
    pub fn nobackoff(&mut self) -> NobackoffW<'_, Mac2Spec> {
        NobackoffW::new(self, 12)
    }
    #[doc = "Bit 13 - BACK PRESSURE / NO BACKOFF. When enabled (set to 1), after the MAC incidentally causes a collision during back pressure, it will immediately retransmit without backoff, reducing the chance of further collisions and ensuring transmit packets get sent."]
    #[inline(always)]
    pub fn bp_nobackoff(&mut self) -> BpNobackoffW<'_, Mac2Spec> {
        BpNobackoffW::new(self, 13)
    }
    #[doc = "Bit 14 - When enabled (set to 1) the MAC will defer to carrier indefinitely as per the Standard. When disabled, the MAC will abort when the excessive deferral limit is reached."]
    #[inline(always)]
    pub fn excessdefer(&mut self) -> ExcessdeferW<'_, Mac2Spec> {
        ExcessdeferW::new(self, 14)
    }
}
#[doc = "MAC configuration register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`mac2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mac2Spec;
impl crate::RegisterSpec for Mac2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac2::R`](R) reader structure"]
impl crate::Readable for Mac2Spec {}
#[doc = "`write(|w| ..)` method takes [`mac2::W`](W) writer structure"]
impl crate::Writable for Mac2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAC2 to value 0"]
impl crate::Resettable for Mac2Spec {}
