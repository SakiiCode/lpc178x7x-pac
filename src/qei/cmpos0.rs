#[doc = "Register `CMPOS0` reader"]
pub type R = crate::R<Cmpos0Spec>;
#[doc = "Register `CMPOS0` writer"]
pub type W = crate::W<Cmpos0Spec>;
#[doc = "Field `PCMP0` reader - Position compare value 0."]
pub type Pcmp0R = crate::FieldReader<u32>;
#[doc = "Field `PCMP0` writer - Position compare value 0."]
pub type Pcmp0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Position compare value 0."]
    #[inline(always)]
    pub fn pcmp0(&self) -> Pcmp0R {
        Pcmp0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Position compare value 0."]
    #[inline(always)]
    pub fn pcmp0(&mut self) -> Pcmp0W<'_, Cmpos0Spec> {
        Pcmp0W::new(self, 0)
    }
}
#[doc = "Position compare register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpos0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpos0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cmpos0Spec;
impl crate::RegisterSpec for Cmpos0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpos0::R`](R) reader structure"]
impl crate::Readable for Cmpos0Spec {}
#[doc = "`write(|w| ..)` method takes [`cmpos0::W`](W) writer structure"]
impl crate::Writable for Cmpos0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMPOS0 to value 0xffff_ffff"]
impl crate::Resettable for Cmpos0Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
