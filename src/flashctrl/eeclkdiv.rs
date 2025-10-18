#[doc = "Register `EECLKDIV` reader"]
pub type R = crate::R<EeclkdivSpec>;
#[doc = "Register `EECLKDIV` writer"]
pub type W = crate::W<EeclkdivSpec>;
#[doc = "Field `CLKDIV` reader - Division factor (minus 1 encoded)."]
pub type ClkdivR = crate::FieldReader<u16>;
#[doc = "Field `CLKDIV` writer - Division factor (minus 1 encoded)."]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Division factor (minus 1 encoded)."]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Division factor (minus 1 encoded)."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> ClkdivW<'_, EeclkdivSpec> {
        ClkdivW::new(self, 0)
    }
}
#[doc = "EEPROM clock divider register\n\nYou can [`read`](crate::Reg::read) this register and get [`eeclkdiv::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eeclkdiv::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EeclkdivSpec;
impl crate::RegisterSpec for EeclkdivSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eeclkdiv::R`](R) reader structure"]
impl crate::Readable for EeclkdivSpec {}
#[doc = "`write(|w| ..)` method takes [`eeclkdiv::W`](W) writer structure"]
impl crate::Writable for EeclkdivSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EECLKDIV to value 0"]
impl crate::Resettable for EeclkdivSpec {}
