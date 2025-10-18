#[doc = "Register `RXDESCRIPTORNUMBER` reader"]
pub type R = crate::R<RxdescriptornumberSpec>;
#[doc = "Register `RXDESCRIPTORNUMBER` writer"]
pub type W = crate::W<RxdescriptornumberSpec>;
#[doc = "Field `RXDESCRIPTORN` reader - RxDescriptorNumber. Number of descriptors in the descriptor array for which RxDescriptor is the base address. The number of descriptors is minus one encoded."]
pub type RxdescriptornR = crate::FieldReader<u16>;
#[doc = "Field `RXDESCRIPTORN` writer - RxDescriptorNumber. Number of descriptors in the descriptor array for which RxDescriptor is the base address. The number of descriptors is minus one encoded."]
pub type RxdescriptornW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RxDescriptorNumber. Number of descriptors in the descriptor array for which RxDescriptor is the base address. The number of descriptors is minus one encoded."]
    #[inline(always)]
    pub fn rxdescriptorn(&self) -> RxdescriptornR {
        RxdescriptornR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RxDescriptorNumber. Number of descriptors in the descriptor array for which RxDescriptor is the base address. The number of descriptors is minus one encoded."]
    #[inline(always)]
    pub fn rxdescriptorn(&mut self) -> RxdescriptornW<'_, RxdescriptornumberSpec> {
        RxdescriptornW::new(self, 0)
    }
}
#[doc = "Receive number of descriptors register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdescriptornumber::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdescriptornumber::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxdescriptornumberSpec;
impl crate::RegisterSpec for RxdescriptornumberSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxdescriptornumber::R`](R) reader structure"]
impl crate::Readable for RxdescriptornumberSpec {}
#[doc = "`write(|w| ..)` method takes [`rxdescriptornumber::W`](W) writer structure"]
impl crate::Writable for RxdescriptornumberSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RXDESCRIPTORNUMBER to value 0"]
impl crate::Resettable for RxdescriptornumberSpec {}
