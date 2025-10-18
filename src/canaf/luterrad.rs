#[doc = "Register `LUTERRAD` reader"]
pub type R = crate::R<LuterradSpec>;
#[doc = "Field `LUTERRAD` reader - It the LUT Error bit (below) is 1, this read-only field contains the address in AF Lookup Table RAM, at which the Acceptance Filter encountered an error in the content of the tables."]
pub type LuterradR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 2:10 - It the LUT Error bit (below) is 1, this read-only field contains the address in AF Lookup Table RAM, at which the Acceptance Filter encountered an error in the content of the tables."]
    #[inline(always)]
    pub fn luterrad(&self) -> LuterradR {
        LuterradR::new(((self.bits >> 2) & 0x01ff) as u16)
    }
}
#[doc = "LUT Error Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`luterrad::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LuterradSpec;
impl crate::RegisterSpec for LuterradSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`luterrad::R`](R) reader structure"]
impl crate::Readable for LuterradSpec {}
#[doc = "`reset()` method sets LUTERRAD to value 0"]
impl crate::Resettable for LuterradSpec {}
