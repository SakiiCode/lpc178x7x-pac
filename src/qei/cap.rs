#[doc = "Register `CAP` reader"]
pub type R = crate::R<CapSpec>;
#[doc = "Field `VELCAP` reader - Last velocity capture."]
pub type VelcapR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Last velocity capture."]
    #[inline(always)]
    pub fn velcap(&self) -> VelcapR {
        VelcapR::new(self.bits)
    }
}
#[doc = "Velocity capture register\n\nYou can [`read`](crate::Reg::read) this register and get [`cap::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CapSpec;
impl crate::RegisterSpec for CapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap::R`](R) reader structure"]
impl crate::Readable for CapSpec {}
#[doc = "`reset()` method sets CAP to value 0xffff_ffff"]
impl crate::Resettable for CapSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
