#[doc = "Register `RXDESCRIPTOR` reader"]
pub type R = crate::R<RxdescriptorSpec>;
#[doc = "Register `RXDESCRIPTOR` writer"]
pub type W = crate::W<RxdescriptorSpec>;
#[doc = "Field `RXDESCRIPTOR` reader - MSBs of receive descriptor base address."]
pub type RxdescriptorR = crate::FieldReader<u32>;
#[doc = "Field `RXDESCRIPTOR` writer - MSBs of receive descriptor base address."]
pub type RxdescriptorW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - MSBs of receive descriptor base address."]
    #[inline(always)]
    pub fn rxdescriptor(&self) -> RxdescriptorR {
        RxdescriptorR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - MSBs of receive descriptor base address."]
    #[inline(always)]
    pub fn rxdescriptor(&mut self) -> RxdescriptorW<'_, RxdescriptorSpec> {
        RxdescriptorW::new(self, 2)
    }
}
#[doc = "Receive descriptor base address register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdescriptor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdescriptor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdescriptorSpec;
impl crate::RegisterSpec for RxdescriptorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdescriptor::R`](R) reader structure"]
impl crate::Readable for RxdescriptorSpec {}
#[doc = "`write(|w| ..)` method takes [`rxdescriptor::W`](W) writer structure"]
impl crate::Writable for RxdescriptorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXDESCRIPTOR to value 0"]
impl crate::Resettable for RxdescriptorSpec {}
