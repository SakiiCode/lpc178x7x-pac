#[doc = "Register `PLL%sCON` reader"]
pub type R = crate::R<PllconSpec>;
#[doc = "Register `PLL%sCON` writer"]
pub type W = crate::W<PllconSpec>;
#[doc = "Field `PLLE` reader - PLL Enable. When one, and after a valid PLL feed, this bit will activate the related PLL and allow it to lock to the requested frequency. See PLLSTAT register, Table 12."]
pub type PlleR = crate::BitReader;
#[doc = "Field `PLLE` writer - PLL Enable. When one, and after a valid PLL feed, this bit will activate the related PLL and allow it to lock to the requested frequency. See PLLSTAT register, Table 12."]
pub type PlleW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PLL Enable. When one, and after a valid PLL feed, this bit will activate the related PLL and allow it to lock to the requested frequency. See PLLSTAT register, Table 12."]
    #[inline(always)]
    pub fn plle(&self) -> PlleR {
        PlleR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL Enable. When one, and after a valid PLL feed, this bit will activate the related PLL and allow it to lock to the requested frequency. See PLLSTAT register, Table 12."]
    #[inline(always)]
    pub fn plle(&mut self) -> PlleW<'_, PllconSpec> {
        PlleW::new(self, 0)
    }
}
#[doc = "PLL0 Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pllcon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllconSpec;
impl crate::RegisterSpec for PllconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllcon::R`](R) reader structure"]
impl crate::Readable for PllconSpec {}
#[doc = "`write(|w| ..)` method takes [`pllcon::W`](W) writer structure"]
impl crate::Writable for PllconSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL%sCON to value 0"]
impl crate::Resettable for PllconSpec {}
