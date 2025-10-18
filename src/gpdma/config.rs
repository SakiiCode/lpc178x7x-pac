#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Field `E` reader - DMA Controller enable: 0 = disabled (default). Disabling the DMA Controller reduces power consumption. 1 = enabled."]
pub type ER = crate::BitReader;
#[doc = "Field `E` writer - DMA Controller enable: 0 = disabled (default). Disabling the DMA Controller reduces power consumption. 1 = enabled."]
pub type EW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M` reader - AHB Master endianness configuration: 0 = little-endian mode (default). 1 = big-endian mode."]
pub type MR = crate::BitReader;
#[doc = "Field `M` writer - AHB Master endianness configuration: 0 = little-endian mode (default). 1 = big-endian mode."]
pub type MW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DMA Controller enable: 0 = disabled (default). Disabling the DMA Controller reduces power consumption. 1 = enabled."]
    #[inline(always)]
    pub fn e(&self) -> ER {
        ER::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AHB Master endianness configuration: 0 = little-endian mode (default). 1 = big-endian mode."]
    #[inline(always)]
    pub fn m(&self) -> MR {
        MR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DMA Controller enable: 0 = disabled (default). Disabling the DMA Controller reduces power consumption. 1 = enabled."]
    #[inline(always)]
    pub fn e(&mut self) -> EW<'_, ConfigSpec> {
        EW::new(self, 0)
    }
    #[doc = "Bit 1 - AHB Master endianness configuration: 0 = little-endian mode (default). 1 = big-endian mode."]
    #[inline(always)]
    pub fn m(&mut self) -> MW<'_, ConfigSpec> {
        MW::new(self, 1)
    }
}
#[doc = "DMA Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for ConfigSpec {}
