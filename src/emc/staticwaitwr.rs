#[doc = "Register `STATICWAITWR%s` reader"]
pub type R = crate::R<StaticwaitwrSpec>;
#[doc = "Register `STATICWAITWR%s` writer"]
pub type W = crate::W<StaticwaitwrSpec>;
#[doc = "Field `WAITWR` reader - Write wait states. SRAM wait state time for write accesses after the first read: 0x0 - 0x1E = (n + 2) CCLK cycle write access time. The wait state time for write accesses after the first read is WAITWR (n + 2) x tCCLK. 0x1F = 33 CCLK cycle write access time (POR reset value)."]
pub type WaitwrR = crate::FieldReader;
#[doc = "Field `WAITWR` writer - Write wait states. SRAM wait state time for write accesses after the first read: 0x0 - 0x1E = (n + 2) CCLK cycle write access time. The wait state time for write accesses after the first read is WAITWR (n + 2) x tCCLK. 0x1F = 33 CCLK cycle write access time (POR reset value)."]
pub type WaitwrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Write wait states. SRAM wait state time for write accesses after the first read: 0x0 - 0x1E = (n + 2) CCLK cycle write access time. The wait state time for write accesses after the first read is WAITWR (n + 2) x tCCLK. 0x1F = 33 CCLK cycle write access time (POR reset value)."]
    #[inline(always)]
    pub fn waitwr(&self) -> WaitwrR {
        WaitwrR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Write wait states. SRAM wait state time for write accesses after the first read: 0x0 - 0x1E = (n + 2) CCLK cycle write access time. The wait state time for write accesses after the first read is WAITWR (n + 2) x tCCLK. 0x1F = 33 CCLK cycle write access time (POR reset value)."]
    #[inline(always)]
    pub fn waitwr(&mut self) -> WaitwrW<'_, StaticwaitwrSpec> {
        WaitwrW::new(self, 0)
    }
}
#[doc = "Delay from EMC_CS0 to a write access.\n\nYou can [`read`](crate::Reg::read) this register and get [`staticwaitwr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`staticwaitwr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StaticwaitwrSpec;
impl crate::RegisterSpec for StaticwaitwrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`staticwaitwr::R`](R) reader structure"]
impl crate::Readable for StaticwaitwrSpec {}
#[doc = "`write(|w| ..)` method takes [`staticwaitwr::W`](W) writer structure"]
impl crate::Writable for StaticwaitwrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATICWAITWR%s to value 0x1f"]
impl crate::Resettable for StaticwaitwrSpec {
    const RESET_VALUE: u32 = 0x1f;
}
