#[doc = "Register `TC[%s]` reader"]
pub type R = crate::R<TcSpec>;
#[doc = "Register `TC[%s]` writer"]
pub type W = crate::W<TcSpec>;
#[doc = "Field `MCTC` reader - Timer/Counter value."]
pub type MctcR = crate::FieldReader<u32>;
#[doc = "Field `MCTC` writer - Timer/Counter value."]
pub type MctcW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Timer/Counter value."]
    #[inline(always)]
    pub fn mctc(&self) -> MctcR {
        MctcR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer/Counter value."]
    #[inline(always)]
    pub fn mctc(&mut self) -> MctcW<'_, TcSpec> {
        MctcW::new(self, 0)
    }
}
#[doc = "Timer Counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`tc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TcSpec;
impl crate::RegisterSpec for TcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tc::R`](R) reader structure"]
impl crate::Readable for TcSpec {}
#[doc = "`write(|w| ..)` method takes [`tc::W`](W) writer structure"]
impl crate::Writable for TcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TC[%s] to value 0"]
impl crate::Resettable for TcSpec {}
