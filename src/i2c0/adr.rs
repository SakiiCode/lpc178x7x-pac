#[doc = "Register `ADR%s` reader"]
pub type R = crate::R<AdrSpec>;
#[doc = "Register `ADR%s` writer"]
pub type W = crate::W<AdrSpec>;
#[doc = "Field `GC` reader - General Call enable bit."]
pub type GcR = crate::BitReader;
#[doc = "Field `GC` writer - General Call enable bit."]
pub type GcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Address` reader - The I2C device address for slave mode."]
pub type AddressR = crate::FieldReader;
#[doc = "Field `Address` writer - The I2C device address for slave mode."]
pub type AddressW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bit 0 - General Call enable bit."]
    #[inline(always)]
    pub fn gc(&self) -> GcR {
        GcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:7 - The I2C device address for slave mode."]
    #[inline(always)]
    pub fn address(&self) -> AddressR {
        AddressR::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - General Call enable bit."]
    #[inline(always)]
    pub fn gc(&mut self) -> GcW<'_, AdrSpec> {
        GcW::new(self, 0)
    }
    #[doc = "Bits 1:7 - The I2C device address for slave mode."]
    #[inline(always)]
    pub fn address(&mut self) -> AddressW<'_, AdrSpec> {
        AddressW::new(self, 1)
    }
}
#[doc = "I2C Slave Address Register. Contains the 7-bit slave address for operation of the I2C interface in slave mode, and is not used in master mode. The least significant bit determines whether a slave responds to the General Call address.\n\nYou can [`read`](crate::Reg::read) this register and get [`adr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AdrSpec;
impl crate::RegisterSpec for AdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`adr::R`](R) reader structure"]
impl crate::Readable for AdrSpec {}
#[doc = "`write(|w| ..)` method takes [`adr::W`](W) writer structure"]
impl crate::Writable for AdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ADR%s to value 0"]
impl crate::Resettable for AdrSpec {}
