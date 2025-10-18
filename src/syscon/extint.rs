#[doc = "Register `EXTINT` reader"]
pub type R = crate::R<ExtintSpec>;
#[doc = "Register `EXTINT` writer"]
pub type W = crate::W<ExtintSpec>;
#[doc = "Field `EINT0` reader - In level-sensitive mode, this bit is set if the EINT0 function is selected for its pin, and the pin is in its active state. In edge-sensitive mode, this bit is set if the EINT0 function is selected for its pin, and the selected edge occurs on the pin. This bit is cleared by writing a one to it, except in level sensitive mode when the pin is in its active state.\\[1\\]"]
pub type Eint0R = crate::BitReader;
#[doc = "Field `EINT0` writer - In level-sensitive mode, this bit is set if the EINT0 function is selected for its pin, and the pin is in its active state. In edge-sensitive mode, this bit is set if the EINT0 function is selected for its pin, and the selected edge occurs on the pin. This bit is cleared by writing a one to it, except in level sensitive mode when the pin is in its active state.\\[1\\]"]
pub type Eint0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EINT1` reader - In level-sensitive mode, this bit is set if the EINT1 function is selected for its pin, and the pin is in its active state. In edge-sensitive mode, this bit is set if the EINT1 function is selected for its pin, and the selected edge occurs on the pin. This bit is cleared by writing a one to it, except in level sensitive mode when the pin is in its active state.\\[1\\]"]
pub type Eint1R = crate::BitReader;
#[doc = "Field `EINT1` writer - In level-sensitive mode, this bit is set if the EINT1 function is selected for its pin, and the pin is in its active state. In edge-sensitive mode, this bit is set if the EINT1 function is selected for its pin, and the selected edge occurs on the pin. This bit is cleared by writing a one to it, except in level sensitive mode when the pin is in its active state.\\[1\\]"]
pub type Eint1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EINT2` reader - In level-sensitive mode, this bit is set if the EINT2 function is selected for its pin, and the pin is in its active state. In edge-sensitive mode, this bit is set if the EINT2 function is selected for its pin, and the selected edge occurs on the pin. This bit is cleared by writing a one to it, except in level sensitive mode when the pin is in its active state.\\[1\\]"]
pub type Eint2R = crate::BitReader;
#[doc = "Field `EINT2` writer - In level-sensitive mode, this bit is set if the EINT2 function is selected for its pin, and the pin is in its active state. In edge-sensitive mode, this bit is set if the EINT2 function is selected for its pin, and the selected edge occurs on the pin. This bit is cleared by writing a one to it, except in level sensitive mode when the pin is in its active state.\\[1\\]"]
pub type Eint2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EINT3` reader - In level-sensitive mode, this bit is set if the EINT3 function is selected for its pin, and the pin is in its active state. In edge-sensitive mode, this bit is set if the EINT3 function is selected for its pin, and the selected edge occurs on the pin. This bit is cleared by writing a one to it, except in level sensitive mode when the pin is in its active state.\\[1\\]"]
pub type Eint3R = crate::BitReader;
#[doc = "Field `EINT3` writer - In level-sensitive mode, this bit is set if the EINT3 function is selected for its pin, and the pin is in its active state. In edge-sensitive mode, this bit is set if the EINT3 function is selected for its pin, and the selected edge occurs on the pin. This bit is cleared by writing a one to it, except in level sensitive mode when the pin is in its active state.\\[1\\]"]
pub type Eint3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - In level-sensitive mode, this bit is set if the EINT0 function is selected for its pin, and the pin is in its active state. In edge-sensitive mode, this bit is set if the EINT0 function is selected for its pin, and the selected edge occurs on the pin. This bit is cleared by writing a one to it, except in level sensitive mode when the pin is in its active state.\\[1\\]"]
    #[inline(always)]
    pub fn eint0(&self) -> Eint0R {
        Eint0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - In level-sensitive mode, this bit is set if the EINT1 function is selected for its pin, and the pin is in its active state. In edge-sensitive mode, this bit is set if the EINT1 function is selected for its pin, and the selected edge occurs on the pin. This bit is cleared by writing a one to it, except in level sensitive mode when the pin is in its active state.\\[1\\]"]
    #[inline(always)]
    pub fn eint1(&self) -> Eint1R {
        Eint1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - In level-sensitive mode, this bit is set if the EINT2 function is selected for its pin, and the pin is in its active state. In edge-sensitive mode, this bit is set if the EINT2 function is selected for its pin, and the selected edge occurs on the pin. This bit is cleared by writing a one to it, except in level sensitive mode when the pin is in its active state.\\[1\\]"]
    #[inline(always)]
    pub fn eint2(&self) -> Eint2R {
        Eint2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - In level-sensitive mode, this bit is set if the EINT3 function is selected for its pin, and the pin is in its active state. In edge-sensitive mode, this bit is set if the EINT3 function is selected for its pin, and the selected edge occurs on the pin. This bit is cleared by writing a one to it, except in level sensitive mode when the pin is in its active state.\\[1\\]"]
    #[inline(always)]
    pub fn eint3(&self) -> Eint3R {
        Eint3R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - In level-sensitive mode, this bit is set if the EINT0 function is selected for its pin, and the pin is in its active state. In edge-sensitive mode, this bit is set if the EINT0 function is selected for its pin, and the selected edge occurs on the pin. This bit is cleared by writing a one to it, except in level sensitive mode when the pin is in its active state.\\[1\\]"]
    #[inline(always)]
    pub fn eint0(&mut self) -> Eint0W<'_, ExtintSpec> {
        Eint0W::new(self, 0)
    }
    #[doc = "Bit 1 - In level-sensitive mode, this bit is set if the EINT1 function is selected for its pin, and the pin is in its active state. In edge-sensitive mode, this bit is set if the EINT1 function is selected for its pin, and the selected edge occurs on the pin. This bit is cleared by writing a one to it, except in level sensitive mode when the pin is in its active state.\\[1\\]"]
    #[inline(always)]
    pub fn eint1(&mut self) -> Eint1W<'_, ExtintSpec> {
        Eint1W::new(self, 1)
    }
    #[doc = "Bit 2 - In level-sensitive mode, this bit is set if the EINT2 function is selected for its pin, and the pin is in its active state. In edge-sensitive mode, this bit is set if the EINT2 function is selected for its pin, and the selected edge occurs on the pin. This bit is cleared by writing a one to it, except in level sensitive mode when the pin is in its active state.\\[1\\]"]
    #[inline(always)]
    pub fn eint2(&mut self) -> Eint2W<'_, ExtintSpec> {
        Eint2W::new(self, 2)
    }
    #[doc = "Bit 3 - In level-sensitive mode, this bit is set if the EINT3 function is selected for its pin, and the pin is in its active state. In edge-sensitive mode, this bit is set if the EINT3 function is selected for its pin, and the selected edge occurs on the pin. This bit is cleared by writing a one to it, except in level sensitive mode when the pin is in its active state.\\[1\\]"]
    #[inline(always)]
    pub fn eint3(&mut self) -> Eint3W<'_, ExtintSpec> {
        Eint3W::new(self, 3)
    }
}
#[doc = "External Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extint::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extint::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ExtintSpec;
impl crate::RegisterSpec for ExtintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`extint::R`](R) reader structure"]
impl crate::Readable for ExtintSpec {}
#[doc = "`write(|w| ..)` method takes [`extint::W`](W) writer structure"]
impl crate::Writable for ExtintSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXTINT to value 0"]
impl crate::Resettable for ExtintSpec {}
