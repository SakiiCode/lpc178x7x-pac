#[doc = "Register `P0_29` reader"]
pub type R = crate::R<P0_29Spec>;
#[doc = "Register `P0_29` writer"]
pub type W = crate::W<P0_29Spec>;
#[doc = "Selects pin function for pin P0\\[29\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Func {
    #[doc = "0: General purpose digital input/output pin."]
    P0_29 = 0,
    #[doc = "1: USB port 1 bidirectional D+ line."]
    UsbDp1 = 1,
    #[doc = "2: External interrupt 0 input."]
    Eint0 = 2,
}
impl From<Func> for u8 {
    #[inline(always)]
    fn from(variant: Func) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Func {
    type Ux = u8;
}
impl crate::IsEnum for Func {}
#[doc = "Field `FUNC` reader - Selects pin function for pin P0\\[29\\]"]
pub type FuncR = crate::FieldReader<Func>;
impl FuncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Func> {
        match self.bits {
            0 => Some(Func::P0_29),
            1 => Some(Func::UsbDp1),
            2 => Some(Func::Eint0),
            _ => None,
        }
    }
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn is_p0_29(&self) -> bool {
        *self == Func::P0_29
    }
    #[doc = "USB port 1 bidirectional D+ line."]
    #[inline(always)]
    pub fn is_usb_dp1(&self) -> bool {
        *self == Func::UsbDp1
    }
    #[doc = "External interrupt 0 input."]
    #[inline(always)]
    pub fn is_eint0(&self) -> bool {
        *self == Func::Eint0
    }
}
#[doc = "Field `FUNC` writer - Selects pin function for pin P0\\[29\\]"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 3, Func>;
impl<'a, REG> FuncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "General purpose digital input/output pin."]
    #[inline(always)]
    pub fn p0_29(self) -> &'a mut crate::W<REG> {
        self.variant(Func::P0_29)
    }
    #[doc = "USB port 1 bidirectional D+ line."]
    #[inline(always)]
    pub fn usb_dp1(self) -> &'a mut crate::W<REG> {
        self.variant(Func::UsbDp1)
    }
    #[doc = "External interrupt 0 input."]
    #[inline(always)]
    pub fn eint0(self) -> &'a mut crate::W<REG> {
        self.variant(Func::Eint0)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[29\\]"]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P0\\[29\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FuncW<'_, P0_29Spec> {
        FuncW::new(self, 0)
    }
}
#[doc = "I/O configuration register for pin P0\\[29\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p0_29::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0_29::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P0_29Spec;
impl crate::RegisterSpec for P0_29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p0_29::R`](R) reader structure"]
impl crate::Readable for P0_29Spec {}
#[doc = "`write(|w| ..)` method takes [`p0_29::W`](W) writer structure"]
impl crate::Writable for P0_29Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets P0_29 to value 0"]
impl crate::Resettable for P0_29Spec {}
