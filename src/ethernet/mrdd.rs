#[doc = "Register `MRDD` reader"]
pub type R = crate::R<MrddSpec>;
#[doc = "Field `READDATA` reader - READ DATA. Following an MII Mgmt Read Cycle, the 16-bit data can be read from this location."]
pub type ReaddataR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - READ DATA. Following an MII Mgmt Read Cycle, the 16-bit data can be read from this location."]
    #[inline(always)]
    pub fn readdata(&self) -> ReaddataR {
        ReaddataR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "MII Mgmt Read Data register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mrdd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MrddSpec;
impl crate::RegisterSpec for MrddSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mrdd::R`](R) reader structure"]
impl crate::Readable for MrddSpec {}
#[doc = "`reset()` method sets MRDD to value 0"]
impl crate::Resettable for MrddSpec {}
