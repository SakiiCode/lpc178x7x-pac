#[doc = "Register `INXCMP1` reader"]
pub type R = crate::R<Inxcmp1Spec>;
#[doc = "Register `INXCMP1` writer"]
pub type W = crate::W<Inxcmp1Spec>;
#[doc = "Field `ICMP1` reader - Index compare value 1."]
pub type Icmp1R = crate::FieldReader<u32>;
#[doc = "Field `ICMP1` writer - Index compare value 1."]
pub type Icmp1W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Index compare value 1."]
    #[inline(always)]
    pub fn icmp1(&self) -> Icmp1R {
        Icmp1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Index compare value 1."]
    #[inline(always)]
    pub fn icmp1(&mut self) -> Icmp1W<'_, Inxcmp1Spec> {
        Icmp1W::new(self, 0)
    }
}
#[doc = "Index compare register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`inxcmp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inxcmp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Inxcmp1Spec;
impl crate::RegisterSpec for Inxcmp1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inxcmp1::R`](R) reader structure"]
impl crate::Readable for Inxcmp1Spec {}
#[doc = "`write(|w| ..)` method takes [`inxcmp1::W`](W) writer structure"]
impl crate::Writable for Inxcmp1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INXCMP1 to value 0xffff_ffff"]
impl crate::Resettable for Inxcmp1Spec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
