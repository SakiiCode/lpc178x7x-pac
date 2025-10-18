#[doc = "Register `MWTD` writer"]
pub type W = crate::W<MwtdSpec>;
#[doc = "Field `WRITEDATA` writer - WRITE DATA. When written, an MII Mgmt write cycle is performed using the 16-bit data and the pre-configured PHY and Register addresses from the MII Mgmt Address register (MADR)."]
pub type WritedataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl W {
    #[doc = "Bits 0:15 - WRITE DATA. When written, an MII Mgmt write cycle is performed using the 16-bit data and the pre-configured PHY and Register addresses from the MII Mgmt Address register (MADR)."]
    #[inline(always)]
    pub fn writedata(&mut self) -> WritedataW<'_, MwtdSpec> {
        WritedataW::new(self, 0)
    }
}
#[doc = "MII Mgmt Write Data register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwtd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MwtdSpec;
impl crate::RegisterSpec for MwtdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mwtd::W`](W) writer structure"]
impl crate::Writable for MwtdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MWTD to value 0"]
impl crate::Resettable for MwtdSpec {}
