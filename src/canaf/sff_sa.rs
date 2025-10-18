#[doc = "Register `SFF_SA` reader"]
pub type R = crate::R<SffSaSpec>;
#[doc = "Register `SFF_SA` writer"]
pub type W = crate::W<SffSaSpec>;
#[doc = "Field `SFF_SA` reader - The start address of the table of individual Standard Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the SFF_GRP_sa register described below. For compatibility with possible future devices, write zeroes in bits 31:11 and 1:0 of this register. If the eFCAN bit in the AFMR is 1, this value also indicates the size of the table of Standard IDs which the Acceptance Filter will search and (if found) automatically store received messages in Acceptance Filter RAM."]
pub type SffSaR = crate::FieldReader<u16>;
#[doc = "Field `SFF_SA` writer - The start address of the table of individual Standard Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the SFF_GRP_sa register described below. For compatibility with possible future devices, write zeroes in bits 31:11 and 1:0 of this register. If the eFCAN bit in the AFMR is 1, this value also indicates the size of the table of Standard IDs which the Acceptance Filter will search and (if found) automatically store received messages in Acceptance Filter RAM."]
pub type SffSaW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 2:10 - The start address of the table of individual Standard Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the SFF_GRP_sa register described below. For compatibility with possible future devices, write zeroes in bits 31:11 and 1:0 of this register. If the eFCAN bit in the AFMR is 1, this value also indicates the size of the table of Standard IDs which the Acceptance Filter will search and (if found) automatically store received messages in Acceptance Filter RAM."]
    #[inline(always)]
    pub fn sff_sa(&self) -> SffSaR {
        SffSaR::new(((self.bits >> 2) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:10 - The start address of the table of individual Standard Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the SFF_GRP_sa register described below. For compatibility with possible future devices, write zeroes in bits 31:11 and 1:0 of this register. If the eFCAN bit in the AFMR is 1, this value also indicates the size of the table of Standard IDs which the Acceptance Filter will search and (if found) automatically store received messages in Acceptance Filter RAM."]
    #[inline(always)]
    pub fn sff_sa(&mut self) -> SffSaW<'_, SffSaSpec> {
        SffSaW::new(self, 2)
    }
}
#[doc = "Standard Frame Individual Start Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sff_sa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sff_sa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SffSaSpec;
impl crate::RegisterSpec for SffSaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sff_sa::R`](R) reader structure"]
impl crate::Readable for SffSaSpec {}
#[doc = "`write(|w| ..)` method takes [`sff_sa::W`](W) writer structure"]
impl crate::Writable for SffSaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFF_SA to value 0"]
impl crate::Resettable for SffSaSpec {}
