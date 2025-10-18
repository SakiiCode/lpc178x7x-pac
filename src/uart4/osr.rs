#[doc = "Register `OSR` reader"]
pub type R = crate::R<OsrSpec>;
#[doc = "Register `OSR` writer"]
pub type W = crate::W<OsrSpec>;
#[doc = "Field `OSFRAC` reader - Fractional part of the oversampling ratio, in units of 1/8th of an input clock period. (001 = 0.125, ..., 111 = 0.875)"]
pub type OsfracR = crate::FieldReader;
#[doc = "Field `OSFRAC` writer - Fractional part of the oversampling ratio, in units of 1/8th of an input clock period. (001 = 0.125, ..., 111 = 0.875)"]
pub type OsfracW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OSINT` reader - Integer part of the oversampling ratio, minus 1. The reset values equate to the normal operating mode of 16 input clocks per bit time."]
pub type OsintR = crate::FieldReader;
#[doc = "Field `OSINT` writer - Integer part of the oversampling ratio, minus 1. The reset values equate to the normal operating mode of 16 input clocks per bit time."]
pub type OsintW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FDINT` reader - In smartcard mode, these bits act as a more-significant extension of the OSint field, allowing an oversampling ratio up to 2048 as required by ISO7816-3. In smartcard mode, bits 14:4 should initially be set to 371, yielding an oversampling ratio of 372."]
pub type FdintR = crate::FieldReader;
#[doc = "Field `FDINT` writer - In smartcard mode, these bits act as a more-significant extension of the OSint field, allowing an oversampling ratio up to 2048 as required by ISO7816-3. In smartcard mode, bits 14:4 should initially be set to 371, yielding an oversampling ratio of 372."]
pub type FdintW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 1:3 - Fractional part of the oversampling ratio, in units of 1/8th of an input clock period. (001 = 0.125, ..., 111 = 0.875)"]
    #[inline(always)]
    pub fn osfrac(&self) -> OsfracR {
        OsfracR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:7 - Integer part of the oversampling ratio, minus 1. The reset values equate to the normal operating mode of 16 input clocks per bit time."]
    #[inline(always)]
    pub fn osint(&self) -> OsintR {
        OsintR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - In smartcard mode, these bits act as a more-significant extension of the OSint field, allowing an oversampling ratio up to 2048 as required by ISO7816-3. In smartcard mode, bits 14:4 should initially be set to 371, yielding an oversampling ratio of 372."]
    #[inline(always)]
    pub fn fdint(&self) -> FdintR {
        FdintR::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:3 - Fractional part of the oversampling ratio, in units of 1/8th of an input clock period. (001 = 0.125, ..., 111 = 0.875)"]
    #[inline(always)]
    pub fn osfrac(&mut self) -> OsfracW<'_, OsrSpec> {
        OsfracW::new(self, 1)
    }
    #[doc = "Bits 4:7 - Integer part of the oversampling ratio, minus 1. The reset values equate to the normal operating mode of 16 input clocks per bit time."]
    #[inline(always)]
    pub fn osint(&mut self) -> OsintW<'_, OsrSpec> {
        OsintW::new(self, 4)
    }
    #[doc = "Bits 8:14 - In smartcard mode, these bits act as a more-significant extension of the OSint field, allowing an oversampling ratio up to 2048 as required by ISO7816-3. In smartcard mode, bits 14:4 should initially be set to 371, yielding an oversampling ratio of 372."]
    #[inline(always)]
    pub fn fdint(&mut self) -> FdintW<'_, OsrSpec> {
        FdintW::new(self, 8)
    }
}
#[doc = "Oversampling register. Controls the degree of oversampling during each bit time.\n\nYou can [`read`](crate::Reg::read) this register and get [`osr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OsrSpec;
impl crate::RegisterSpec for OsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`osr::R`](R) reader structure"]
impl crate::Readable for OsrSpec {}
#[doc = "`write(|w| ..)` method takes [`osr::W`](W) writer structure"]
impl crate::Writable for OsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OSR to value 0xf0"]
impl crate::Resettable for OsrSpec {
    const RESET_VALUE: u32 = 0xf0;
}
