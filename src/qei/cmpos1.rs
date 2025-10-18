#[doc = "Register `CMPOS1` reader"]
pub type R = crate::R<Cmpos1Spec>;
#[doc = "Register `CMPOS1` writer"]
pub type W = crate::W<Cmpos1Spec>;
#[doc = "Field `PCMP1` reader - Position compare value 1."]
pub type Pcmp1R = crate::FieldReader<u32>;
#[doc = "Field `PCMP1` writer - Position compare value 1."]
pub type Pcmp1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Position compare value 1."]
    #[inline(always)]
    pub fn pcmp1(&self) -> Pcmp1R {
        Pcmp1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Position compare value 1."]
    #[inline(always)]
    pub fn pcmp1(&mut self) -> Pcmp1W<'_, Cmpos1Spec> {
        Pcmp1W::new(self, 0)
    }
}
#[doc = "Position compare register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpos1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpos1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmpos1Spec;
impl crate::RegisterSpec for Cmpos1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpos1::R`](R) reader structure"]
impl crate::Readable for Cmpos1Spec {}
#[doc = "`write(|w| ..)` method takes [`cmpos1::W`](W) writer structure"]
impl crate::Writable for Cmpos1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMPOS1 to value 0xffff_ffff"]
impl crate::Resettable for Cmpos1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
