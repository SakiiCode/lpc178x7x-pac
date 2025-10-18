#[doc = "Register `CNTVAL` reader"]
pub type R = crate::R<CntvalSpec>;
#[doc = "Register `CNTVAL` writer"]
pub type W = crate::W<CntvalSpec>;
#[doc = "Field `VALUE` reader - 16-bit reload value for the DAC interrupt/DMA timer."]
pub type ValueR = crate::FieldReader<u16>;
#[doc = "Field `VALUE` writer - 16-bit reload value for the DAC interrupt/DMA timer."]
pub type ValueW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 16-bit reload value for the DAC interrupt/DMA timer."]
    #[inline(always)]
    pub fn value(&self) -> ValueR {
        ValueR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - 16-bit reload value for the DAC interrupt/DMA timer."]
    #[inline(always)]
    pub fn value(&mut self) -> ValueW<'_, CntvalSpec> {
        ValueW::new(self, 0)
    }
}
#[doc = "DAC Counter Value register. This register contains the reload value for the DAC DMA/Interrupt timer.\n\nYou can [`read`](crate::Reg::read) this register and get [`cntval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CntvalSpec;
impl crate::RegisterSpec for CntvalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntval::R`](R) reader structure"]
impl crate::Readable for CntvalSpec {}
#[doc = "`write(|w| ..)` method takes [`cntval::W`](W) writer structure"]
impl crate::Writable for CntvalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNTVAL to value 0"]
impl crate::Resettable for CntvalSpec {}
