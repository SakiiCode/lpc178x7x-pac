#[doc = "Register `P2_10` reader"]
pub type R = crate::R<P2_10Spec>;
#[doc = "Register `P2_10` writer"]
pub type W = crate::W<P2_10Spec>;
#[doc = "Selects pin function for pin P2\\[10\\]"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Func {
    #[doc = "0: General purpose digital input/output pin. This pin includes a 5 ns input glitch filter.A LOW on this pin while RESET is LOW forces the on-chip boot loader to take over control of the part after a reset and go into ISP mode."]
    P2_10 = 0,
    #[doc = "1: External interrupt 0 input."]
    Eint0 = 1,
    #[doc = "2: Non-maskable interrupt input."]
    Nmi = 2,
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
#[doc = "Field `FUNC` reader - Selects pin function for pin P2\\[10\\]"]
pub type FuncR = crate::FieldReader<Func>;
impl FuncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Func> {
        match self.bits {
            0 => Some(Func::P2_10),
            1 => Some(Func::Eint0),
            2 => Some(Func::Nmi),
            _ => None,
        }
    }
    #[doc = "General purpose digital input/output pin. This pin includes a 5 ns input glitch filter.A LOW on this pin while RESET is LOW forces the on-chip boot loader to take over control of the part after a reset and go into ISP mode."]
    #[inline(always)]
    pub fn is_p2_10(&self) -> bool {
        *self == Func::P2_10
    }
    #[doc = "External interrupt 0 input."]
    #[inline(always)]
    pub fn is_eint0(&self) -> bool {
        *self == Func::Eint0
    }
    #[doc = "Non-maskable interrupt input."]
    #[inline(always)]
    pub fn is_nmi(&self) -> bool {
        *self == Func::Nmi
    }
}
#[doc = "Field `FUNC` writer - Selects pin function for pin P2\\[10\\]"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 3, Func>;
impl<'a, REG> FuncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "General purpose digital input/output pin. This pin includes a 5 ns input glitch filter.A LOW on this pin while RESET is LOW forces the on-chip boot loader to take over control of the part after a reset and go into ISP mode."]
    #[inline(always)]
    pub fn p2_10(self) -> &'a mut crate::W<REG> {
        self.variant(Func::P2_10)
    }
    #[doc = "External interrupt 0 input."]
    #[inline(always)]
    pub fn eint0(self) -> &'a mut crate::W<REG> {
        self.variant(Func::Eint0)
    }
    #[doc = "Non-maskable interrupt input."]
    #[inline(always)]
    pub fn nmi(self) -> &'a mut crate::W<REG> {
        self.variant(Func::Nmi)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P2\\[10\\]"]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P2\\[10\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FuncW<'_, P2_10Spec> {
        FuncW::new(self, 0)
    }
}
#[doc = "I/O configuration register for pin P2\\[10\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_10::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2_10Spec;
impl crate::RegisterSpec for P2_10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p2_10::R`](R) reader structure"]
impl crate::Readable for P2_10Spec {}
#[doc = "`write(|w| ..)` method takes [`p2_10::W`](W) writer structure"]
impl crate::Writable for P2_10Spec {
    type Safety = crate::Unsafe;
}
