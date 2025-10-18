#[doc = "Register `INTEN_CLR` writer"]
pub type W = crate::W<IntenClrSpec>;
#[doc = "Field `ILIM0_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type Ilim0ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMAT0_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type Imat0ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICAP0_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type Icap0ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILIM1_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type Ilim1ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMAT1_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type Imat1ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICAP1_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type Icap1ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILIM2_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type Ilim2ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMAT2_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type Imat2ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICAP2_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type Icap2ClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORT_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type AbortClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn ilim0_clr(&mut self) -> Ilim0ClrW<'_, IntenClrSpec> {
        Ilim0ClrW::new(self, 0)
    }
    #[doc = "Bit 1 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn imat0_clr(&mut self) -> Imat0ClrW<'_, IntenClrSpec> {
        Imat0ClrW::new(self, 1)
    }
    #[doc = "Bit 2 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn icap0_clr(&mut self) -> Icap0ClrW<'_, IntenClrSpec> {
        Icap0ClrW::new(self, 2)
    }
    #[doc = "Bit 4 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn ilim1_clr(&mut self) -> Ilim1ClrW<'_, IntenClrSpec> {
        Ilim1ClrW::new(self, 4)
    }
    #[doc = "Bit 5 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn imat1_clr(&mut self) -> Imat1ClrW<'_, IntenClrSpec> {
        Imat1ClrW::new(self, 5)
    }
    #[doc = "Bit 6 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn icap1_clr(&mut self) -> Icap1ClrW<'_, IntenClrSpec> {
        Icap1ClrW::new(self, 6)
    }
    #[doc = "Bit 8 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn ilim2_clr(&mut self) -> Ilim2ClrW<'_, IntenClrSpec> {
        Ilim2ClrW::new(self, 8)
    }
    #[doc = "Bit 9 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn imat2_clr(&mut self) -> Imat2ClrW<'_, IntenClrSpec> {
        Imat2ClrW::new(self, 9)
    }
    #[doc = "Bit 10 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn icap2_clr(&mut self) -> Icap2ClrW<'_, IntenClrSpec> {
        Icap2ClrW::new(self, 10)
    }
    #[doc = "Bit 15 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn abort_clr(&mut self) -> AbortClrW<'_, IntenClrSpec> {
        AbortClrW::new(self, 15)
    }
}
#[doc = "Interrupt Enable clear address\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenClrSpec;
impl crate::RegisterSpec for IntenClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`inten_clr::W`](W) writer structure"]
impl crate::Writable for IntenClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTEN_CLR to value 0"]
impl crate::Resettable for IntenClrSpec {}
