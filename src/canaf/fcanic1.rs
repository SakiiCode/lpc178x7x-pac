#[doc = "Register `FCANIC1` reader"]
pub type R = crate::R<Fcanic1Spec>;
#[doc = "Register `FCANIC1` writer"]
pub type W = crate::W<Fcanic1Spec>;
#[doc = "Field `IntPnd32` reader - FullCan Interrupt Pending bit 32. 0 = FullCan Interrupt Pending bit 32. 1 = FullCan Interrupt Pending bit 33. ... 31 = FullCan Interrupt Pending bit 63."]
pub type IntPnd32R = crate::FieldReader<u32>;
#[doc = "Field `IntPnd32` writer - FullCan Interrupt Pending bit 32. 0 = FullCan Interrupt Pending bit 32. 1 = FullCan Interrupt Pending bit 33. ... 31 = FullCan Interrupt Pending bit 63."]
pub type IntPnd32W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - FullCan Interrupt Pending bit 32. 0 = FullCan Interrupt Pending bit 32. 1 = FullCan Interrupt Pending bit 33. ... 31 = FullCan Interrupt Pending bit 63."]
    #[inline(always)]
    pub fn int_pnd32(&self) -> IntPnd32R {
        IntPnd32R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - FullCan Interrupt Pending bit 32. 0 = FullCan Interrupt Pending bit 32. 1 = FullCan Interrupt Pending bit 33. ... 31 = FullCan Interrupt Pending bit 63."]
    #[inline(always)]
    pub fn int_pnd32(&mut self) -> IntPnd32W<'_, Fcanic1Spec> {
        IntPnd32W::new(self, 0)
    }
}
#[doc = "FullCAN interrupt and capture register1\n\nYou can [`read`](crate::Reg::read) this register and get [`fcanic1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcanic1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Fcanic1Spec;
impl crate::RegisterSpec for Fcanic1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcanic1::R`](R) reader structure"]
impl crate::Readable for Fcanic1Spec {}
#[doc = "`write(|w| ..)` method takes [`fcanic1::W`](W) writer structure"]
impl crate::Writable for Fcanic1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCANIC1 to value 0"]
impl crate::Resettable for Fcanic1Spec {}
