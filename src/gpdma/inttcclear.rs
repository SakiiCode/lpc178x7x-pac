#[doc = "Register `INTTCCLEAR` writer"]
pub type W = crate::W<InttcclearSpec>;
#[doc = "Field `INTTCCLEAR0` writer - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
pub type Inttcclear0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTTCCLEAR1` writer - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
pub type Inttcclear1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTTCCLEAR2` writer - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
pub type Inttcclear2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTTCCLEAR3` writer - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
pub type Inttcclear3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTTCCLEAR4` writer - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
pub type Inttcclear4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTTCCLEAR5` writer - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
pub type Inttcclear5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTTCCLEAR6` writer - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
pub type Inttcclear6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTTCCLEAR7` writer - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
pub type Inttcclear7W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
    #[inline(always)]
    pub fn inttcclear0(&mut self) -> Inttcclear0W<'_, InttcclearSpec> {
        Inttcclear0W::new(self, 0)
    }
    #[doc = "Bit 1 - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
    #[inline(always)]
    pub fn inttcclear1(&mut self) -> Inttcclear1W<'_, InttcclearSpec> {
        Inttcclear1W::new(self, 1)
    }
    #[doc = "Bit 2 - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
    #[inline(always)]
    pub fn inttcclear2(&mut self) -> Inttcclear2W<'_, InttcclearSpec> {
        Inttcclear2W::new(self, 2)
    }
    #[doc = "Bit 3 - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
    #[inline(always)]
    pub fn inttcclear3(&mut self) -> Inttcclear3W<'_, InttcclearSpec> {
        Inttcclear3W::new(self, 3)
    }
    #[doc = "Bit 4 - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
    #[inline(always)]
    pub fn inttcclear4(&mut self) -> Inttcclear4W<'_, InttcclearSpec> {
        Inttcclear4W::new(self, 4)
    }
    #[doc = "Bit 5 - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
    #[inline(always)]
    pub fn inttcclear5(&mut self) -> Inttcclear5W<'_, InttcclearSpec> {
        Inttcclear5W::new(self, 5)
    }
    #[doc = "Bit 6 - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
    #[inline(always)]
    pub fn inttcclear6(&mut self) -> Inttcclear6W<'_, InttcclearSpec> {
        Inttcclear6W::new(self, 6)
    }
    #[doc = "Bit 7 - Allows clearing the Terminal count interrupt request (IntTCStat) for DMA channels. Each bit represents one channel: 0 - writing 0 has no effect. 1 - clears the corresponding channel terminal count interrupt."]
    #[inline(always)]
    pub fn inttcclear7(&mut self) -> Inttcclear7W<'_, InttcclearSpec> {
        Inttcclear7W::new(self, 7)
    }
}
#[doc = "DMA Interrupt Terminal Count Request Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inttcclear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InttcclearSpec;
impl crate::RegisterSpec for InttcclearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`inttcclear::W`](W) writer structure"]
impl crate::Writable for InttcclearSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTTCCLEAR to value 0"]
impl crate::Resettable for InttcclearSpec {}
