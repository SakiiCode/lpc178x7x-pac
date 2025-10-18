#[doc = "Register `EFF_SA` reader"]
pub type R = crate::R<EffSaSpec>;
#[doc = "Register `EFF_SA` writer"]
pub type W = crate::W<EffSaSpec>;
#[doc = "Field `EFF_SA` reader - The start address of the table of individual Extended Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the EFF_GRP_sa register described below. The largest value that should be written to this register is 0x800, when both Extended Tables are empty and the last word (address 0x7FC) in AF Lookup Table RAM is used. For compatibility with possible future devices, please write zeroes in bits 31:11 and 1:0 of this register."]
pub type EffSaR = crate::FieldReader<u16>;
#[doc = "Field `EFF_SA` writer - The start address of the table of individual Extended Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the EFF_GRP_sa register described below. The largest value that should be written to this register is 0x800, when both Extended Tables are empty and the last word (address 0x7FC) in AF Lookup Table RAM is used. For compatibility with possible future devices, please write zeroes in bits 31:11 and 1:0 of this register."]
pub type EffSaW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 2:10 - The start address of the table of individual Extended Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the EFF_GRP_sa register described below. The largest value that should be written to this register is 0x800, when both Extended Tables are empty and the last word (address 0x7FC) in AF Lookup Table RAM is used. For compatibility with possible future devices, please write zeroes in bits 31:11 and 1:0 of this register."]
    #[inline(always)]
    pub fn eff_sa(&self) -> EffSaR {
        EffSaR::new(((self.bits >> 2) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 2:10 - The start address of the table of individual Extended Identifiers in AF Lookup RAM. If the table is empty, write the same value in this register and the EFF_GRP_sa register described below. The largest value that should be written to this register is 0x800, when both Extended Tables are empty and the last word (address 0x7FC) in AF Lookup Table RAM is used. For compatibility with possible future devices, please write zeroes in bits 31:11 and 1:0 of this register."]
    #[inline(always)]
    pub fn eff_sa(&mut self) -> EffSaW<'_, EffSaSpec> {
        EffSaW::new(self, 2)
    }
}
#[doc = "Extended Frame Start Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eff_sa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eff_sa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EffSaSpec;
impl crate::RegisterSpec for EffSaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eff_sa::R`](R) reader structure"]
impl crate::Readable for EffSaSpec {}
#[doc = "`write(|w| ..)` method takes [`eff_sa::W`](W) writer structure"]
impl crate::Writable for EffSaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EFF_SA to value 0"]
impl crate::Resettable for EffSaSpec {}
