#[doc = "Register `PLL%sSTAT` reader"]
pub type R = crate::R<PllstatSpec>;
#[doc = "Field `MSEL` reader - Read-back for the PLL Multiplier value. This is the value currently used by the related PLL."]
pub type MselR = crate::FieldReader;
#[doc = "Field `PSEL` reader - Read-back for the PLL Divider value. This is the value currently used by the related PLL."]
pub type PselR = crate::FieldReader;
#[doc = "Field `PLLE_STAT` reader - Read-back for the PLL Enable bit. When one, the related PLL is currently activated. When zero, the related PLL is turned off. This bit is automatically cleared when Power-down mode is activated."]
pub type PlleStatR = crate::BitReader;
#[doc = "Field `PLOCK` reader - Reflects the PLL Lock status. When zero, the related PLL is not locked. When one, the related PLL is locked onto the requested frequency."]
pub type PlockR = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - Read-back for the PLL Multiplier value. This is the value currently used by the related PLL."]
    #[inline(always)]
    pub fn msel(&self) -> MselR {
        MselR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:6 - Read-back for the PLL Divider value. This is the value currently used by the related PLL."]
    #[inline(always)]
    pub fn psel(&self) -> PselR {
        PselR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - Read-back for the PLL Enable bit. When one, the related PLL is currently activated. When zero, the related PLL is turned off. This bit is automatically cleared when Power-down mode is activated."]
    #[inline(always)]
    pub fn plle_stat(&self) -> PlleStatR {
        PlleStatR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - Reflects the PLL Lock status. When zero, the related PLL is not locked. When one, the related PLL is locked onto the requested frequency."]
    #[inline(always)]
    pub fn plock(&self) -> PlockR {
        PlockR::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "PLL0 Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`pllstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllstatSpec;
impl crate::RegisterSpec for PllstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllstat::R`](R) reader structure"]
impl crate::Readable for PllstatSpec {}
#[doc = "`reset()` method sets PLL%sSTAT to value 0"]
impl crate::Resettable for PllstatSpec {}
