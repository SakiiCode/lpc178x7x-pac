#[doc = "Register `INTF_CLR` writer"]
pub type W = crate::W<IntfClrSpec>;
#[doc = "Field `ILIM0_F_CLR` writer - Writing a one clears the corresponding bit in the INTF register, thus clearing the corresponding interrupt request."]
pub type Ilim0FClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMAT0_F_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type Imat0FClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICAP0_F_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type Icap0FClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILIM1_F_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type Ilim1FClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMAT1_F_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type Imat1FClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICAP1_F_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type Icap1FClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ILIM2_F_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type Ilim2FClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMAT2_F_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type Imat2FClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ICAP2_F_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type Icap2FClrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORT_F_CLR` writer - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
pub type AbortFClrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Writing a one clears the corresponding bit in the INTF register, thus clearing the corresponding interrupt request."]
    #[inline(always)]
    pub fn ilim0_f_clr(&mut self) -> Ilim0FClrW<'_, IntfClrSpec> {
        Ilim0FClrW::new(self, 0)
    }
    #[doc = "Bit 1 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn imat0_f_clr(&mut self) -> Imat0FClrW<'_, IntfClrSpec> {
        Imat0FClrW::new(self, 1)
    }
    #[doc = "Bit 2 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn icap0_f_clr(&mut self) -> Icap0FClrW<'_, IntfClrSpec> {
        Icap0FClrW::new(self, 2)
    }
    #[doc = "Bit 4 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn ilim1_f_clr(&mut self) -> Ilim1FClrW<'_, IntfClrSpec> {
        Ilim1FClrW::new(self, 4)
    }
    #[doc = "Bit 5 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn imat1_f_clr(&mut self) -> Imat1FClrW<'_, IntfClrSpec> {
        Imat1FClrW::new(self, 5)
    }
    #[doc = "Bit 6 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn icap1_f_clr(&mut self) -> Icap1FClrW<'_, IntfClrSpec> {
        Icap1FClrW::new(self, 6)
    }
    #[doc = "Bit 8 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn ilim2_f_clr(&mut self) -> Ilim2FClrW<'_, IntfClrSpec> {
        Ilim2FClrW::new(self, 8)
    }
    #[doc = "Bit 9 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn imat2_f_clr(&mut self) -> Imat2FClrW<'_, IntfClrSpec> {
        Imat2FClrW::new(self, 9)
    }
    #[doc = "Bit 10 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn icap2_f_clr(&mut self) -> Icap2FClrW<'_, IntfClrSpec> {
        Icap2FClrW::new(self, 10)
    }
    #[doc = "Bit 15 - Writing a one clears the corresponding bit in INTEN, thus disabling the interrupt."]
    #[inline(always)]
    pub fn abort_f_clr(&mut self) -> AbortFClrW<'_, IntfClrSpec> {
        AbortFClrW::new(self, 15)
    }
}
#[doc = "Interrupt flags clear address\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfClrSpec;
impl crate::RegisterSpec for IntfClrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intf_clr::W`](W) writer structure"]
impl crate::Writable for IntfClrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTF_CLR to value 0"]
impl crate::Resettable for IntfClrSpec {}
