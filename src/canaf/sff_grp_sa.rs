#[doc = "Register `SFF_GRP_SA` reader"]
pub type R = crate::R<SffGrpSaSpec>;
#[doc = "Register `SFF_GRP_SA` writer"]
pub type W = crate::W<SffGrpSaSpec>;
#[doc = "Field `SFF_GRP_SA` reader - The start address of the table of grouped Standard Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the EFF_sa register described below. The largest value that should be written to this register is 0x800, when only the Standard Individual table is used, and the last word (address 0x7FC) in AF Lookup Table RAM is used. For compatibility with possible future devices, please write zeroes in bits 31:12 and 1:0 of this register."]
pub type SffGrpSaR = crate::FieldReader<u16>;
#[doc = "Field `SFF_GRP_SA` writer - The start address of the table of grouped Standard Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the EFF_sa register described below. The largest value that should be written to this register is 0x800, when only the Standard Individual table is used, and the last word (address 0x7FC) in AF Lookup Table RAM is used. For compatibility with possible future devices, please write zeroes in bits 31:12 and 1:0 of this register."]
pub type SffGrpSaW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 2:11 - The start address of the table of grouped Standard Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the EFF_sa register described below. The largest value that should be written to this register is 0x800, when only the Standard Individual table is used, and the last word (address 0x7FC) in AF Lookup Table RAM is used. For compatibility with possible future devices, please write zeroes in bits 31:12 and 1:0 of this register."]
    #[inline(always)]
    pub fn sff_grp_sa(&self) -> SffGrpSaR {
        SffGrpSaR::new(((self.bits >> 2) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:11 - The start address of the table of grouped Standard Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the EFF_sa register described below. The largest value that should be written to this register is 0x800, when only the Standard Individual table is used, and the last word (address 0x7FC) in AF Lookup Table RAM is used. For compatibility with possible future devices, please write zeroes in bits 31:12 and 1:0 of this register."]
    #[inline(always)]
    pub fn sff_grp_sa(&mut self) -> SffGrpSaW<'_, SffGrpSaSpec> {
        SffGrpSaW::new(self, 2)
    }
}
#[doc = "Standard Frame Group Start Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sff_grp_sa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sff_grp_sa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SffGrpSaSpec;
impl crate::RegisterSpec for SffGrpSaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sff_grp_sa::R`](R) reader structure"]
impl crate::Readable for SffGrpSaSpec {}
#[doc = "`write(|w| ..)` method takes [`sff_grp_sa::W`](W) writer structure"]
impl crate::Writable for SffGrpSaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SFF_GRP_SA to value 0"]
impl crate::Resettable for SffGrpSaSpec {}
