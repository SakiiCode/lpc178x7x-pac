#[doc = "Register `EFF_GRP_SA` reader"]
pub type R = crate::R<EffGrpSaSpec>;
#[doc = "Register `EFF_GRP_SA` writer"]
pub type W = crate::W<EffGrpSaSpec>;
#[doc = "Field `EFF_GRP_SA` reader - The start address of the table of grouped Extended Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the ENDofTable register described below. The largest value that should be written to this register is 0x800, when this table is empty and the last word (address 0x7FC) in AF Lookup Table RAM is used. For compatibility with possible future devices, please write zeroes in bits 31:12 and 1:0 of this register."]
pub type EffGrpSaR = crate::FieldReader<u16>;
#[doc = "Field `EFF_GRP_SA` writer - The start address of the table of grouped Extended Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the ENDofTable register described below. The largest value that should be written to this register is 0x800, when this table is empty and the last word (address 0x7FC) in AF Lookup Table RAM is used. For compatibility with possible future devices, please write zeroes in bits 31:12 and 1:0 of this register."]
pub type EffGrpSaW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 2:11 - The start address of the table of grouped Extended Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the ENDofTable register described below. The largest value that should be written to this register is 0x800, when this table is empty and the last word (address 0x7FC) in AF Lookup Table RAM is used. For compatibility with possible future devices, please write zeroes in bits 31:12 and 1:0 of this register."]
    #[inline(always)]
    pub fn eff_grp_sa(&self) -> EffGrpSaR {
        EffGrpSaR::new(((self.bits >> 2) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:11 - The start address of the table of grouped Extended Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the ENDofTable register described below. The largest value that should be written to this register is 0x800, when this table is empty and the last word (address 0x7FC) in AF Lookup Table RAM is used. For compatibility with possible future devices, please write zeroes in bits 31:12 and 1:0 of this register."]
    #[inline(always)]
    pub fn eff_grp_sa(&mut self) -> EffGrpSaW<'_, EffGrpSaSpec> {
        EffGrpSaW::new(self, 2)
    }
}
#[doc = "Extended Frame Group Start Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eff_grp_sa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eff_grp_sa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EffGrpSaSpec;
impl crate::RegisterSpec for EffGrpSaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eff_grp_sa::R`](R) reader structure"]
impl crate::Readable for EffGrpSaSpec {}
#[doc = "`write(|w| ..)` method takes [`eff_grp_sa::W`](W) writer structure"]
impl crate::Writable for EffGrpSaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EFF_GRP_SA to value 0"]
impl crate::Resettable for EffGrpSaSpec {}
