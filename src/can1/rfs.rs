#[doc = "Register `RFS` reader"]
pub type R = crate::R<RfsSpec>;
#[doc = "Register `RFS` writer"]
pub type W = crate::W<RfsSpec>;
#[doc = "Field `IDINDEX` reader - ID Index. If the BP bit (below) is 0, this value is the zero-based number of the Lookup Table RAM entry at which the Acceptance Filter matched the received Identifier. Disabled entries in the Standard tables are included in this numbering, but will not be matched. See Section 21.17 Examples of acceptance filter tables and ID index values on page 587 for examples of ID Index values."]
pub type IdindexR = crate::FieldReader<u16>;
#[doc = "Field `IDINDEX` writer - ID Index. If the BP bit (below) is 0, this value is the zero-based number of the Lookup Table RAM entry at which the Acceptance Filter matched the received Identifier. Disabled entries in the Standard tables are included in this numbering, but will not be matched. See Section 21.17 Examples of acceptance filter tables and ID index values on page 587 for examples of ID Index values."]
pub type IdindexW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `BP` reader - If this bit is 1, the current message was received in AF Bypass mode, and the ID Index field (above) is meaningless."]
pub type BpR = crate::BitReader;
#[doc = "Field `BP` writer - If this bit is 1, the current message was received in AF Bypass mode, and the ID Index field (above) is meaningless."]
pub type BpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLC` reader - The field contains the Data Length Code (DLC) field of the current received message. When RTR = 0, this is related to the number of data bytes available in the CANRDA and CANRDB registers as follows: 0000-0111 = 0 to 7 bytes1000-1111 = 8 bytes With RTR = 1, this value indicates the number of data bytes requested to be sent back, with the same encoding."]
pub type DlcR = crate::FieldReader;
#[doc = "Field `DLC` writer - The field contains the Data Length Code (DLC) field of the current received message. When RTR = 0, this is related to the number of data bytes available in the CANRDA and CANRDB registers as follows: 0000-0111 = 0 to 7 bytes1000-1111 = 8 bytes With RTR = 1, this value indicates the number of data bytes requested to be sent back, with the same encoding."]
pub type DlcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RTR` reader - This bit contains the Remote Transmission Request bit of the current received message. 0 indicates a Data Frame, in which (if DLC is non-zero) data can be read from the CANRDA and possibly the CANRDB registers. 1 indicates a Remote frame, in which case the DLC value identifies the number of data bytes requested to be sent using the same Identifier."]
pub type RtrR = crate::BitReader;
#[doc = "Field `RTR` writer - This bit contains the Remote Transmission Request bit of the current received message. 0 indicates a Data Frame, in which (if DLC is non-zero) data can be read from the CANRDA and possibly the CANRDB registers. 1 indicates a Remote frame, in which case the DLC value identifies the number of data bytes requested to be sent using the same Identifier."]
pub type RtrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FF` reader - A 0 in this bit indicates that the current received message included an 11-bit Identifier, while a 1 indicates a 29-bit Identifier. This affects the contents of the CANid register described below."]
pub type FfR = crate::BitReader;
#[doc = "Field `FF` writer - A 0 in this bit indicates that the current received message included an 11-bit Identifier, while a 1 indicates a 29-bit Identifier. This affects the contents of the CANid register described below."]
pub type FfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - ID Index. If the BP bit (below) is 0, this value is the zero-based number of the Lookup Table RAM entry at which the Acceptance Filter matched the received Identifier. Disabled entries in the Standard tables are included in this numbering, but will not be matched. See Section 21.17 Examples of acceptance filter tables and ID index values on page 587 for examples of ID Index values."]
    #[inline(always)]
    pub fn idindex(&self) -> IdindexR {
        IdindexR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - If this bit is 1, the current message was received in AF Bypass mode, and the ID Index field (above) is meaningless."]
    #[inline(always)]
    pub fn bp(&self) -> BpR {
        BpR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 16:19 - The field contains the Data Length Code (DLC) field of the current received message. When RTR = 0, this is related to the number of data bytes available in the CANRDA and CANRDB registers as follows: 0000-0111 = 0 to 7 bytes1000-1111 = 8 bytes With RTR = 1, this value indicates the number of data bytes requested to be sent back, with the same encoding."]
    #[inline(always)]
    pub fn dlc(&self) -> DlcR {
        DlcR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 30 - This bit contains the Remote Transmission Request bit of the current received message. 0 indicates a Data Frame, in which (if DLC is non-zero) data can be read from the CANRDA and possibly the CANRDB registers. 1 indicates a Remote frame, in which case the DLC value identifies the number of data bytes requested to be sent using the same Identifier."]
    #[inline(always)]
    pub fn rtr(&self) -> RtrR {
        RtrR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - A 0 in this bit indicates that the current received message included an 11-bit Identifier, while a 1 indicates a 29-bit Identifier. This affects the contents of the CANid register described below."]
    #[inline(always)]
    pub fn ff(&self) -> FfR {
        FfR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - ID Index. If the BP bit (below) is 0, this value is the zero-based number of the Lookup Table RAM entry at which the Acceptance Filter matched the received Identifier. Disabled entries in the Standard tables are included in this numbering, but will not be matched. See Section 21.17 Examples of acceptance filter tables and ID index values on page 587 for examples of ID Index values."]
    #[inline(always)]
    pub fn idindex(&mut self) -> IdindexW<'_, RfsSpec> {
        IdindexW::new(self, 0)
    }
    #[doc = "Bit 10 - If this bit is 1, the current message was received in AF Bypass mode, and the ID Index field (above) is meaningless."]
    #[inline(always)]
    pub fn bp(&mut self) -> BpW<'_, RfsSpec> {
        BpW::new(self, 10)
    }
    #[doc = "Bits 16:19 - The field contains the Data Length Code (DLC) field of the current received message. When RTR = 0, this is related to the number of data bytes available in the CANRDA and CANRDB registers as follows: 0000-0111 = 0 to 7 bytes1000-1111 = 8 bytes With RTR = 1, this value indicates the number of data bytes requested to be sent back, with the same encoding."]
    #[inline(always)]
    pub fn dlc(&mut self) -> DlcW<'_, RfsSpec> {
        DlcW::new(self, 16)
    }
    #[doc = "Bit 30 - This bit contains the Remote Transmission Request bit of the current received message. 0 indicates a Data Frame, in which (if DLC is non-zero) data can be read from the CANRDA and possibly the CANRDB registers. 1 indicates a Remote frame, in which case the DLC value identifies the number of data bytes requested to be sent using the same Identifier."]
    #[inline(always)]
    pub fn rtr(&mut self) -> RtrW<'_, RfsSpec> {
        RtrW::new(self, 30)
    }
    #[doc = "Bit 31 - A 0 in this bit indicates that the current received message included an 11-bit Identifier, while a 1 indicates a 29-bit Identifier. This affects the contents of the CANid register described below."]
    #[inline(always)]
    pub fn ff(&mut self) -> FfW<'_, RfsSpec> {
        FfW::new(self, 31)
    }
}
#[doc = "Receive frame status. Can only be written when RM in CANMOD is 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rfs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RfsSpec;
impl crate::RegisterSpec for RfsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rfs::R`](R) reader structure"]
impl crate::Readable for RfsSpec {}
#[doc = "`write(|w| ..)` method takes [`rfs::W`](W) writer structure"]
impl crate::Writable for RfsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RFS to value 0"]
impl crate::Resettable for RfsSpec {}
