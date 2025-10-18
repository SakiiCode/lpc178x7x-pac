#[doc = "Register `DESTADDR%s` reader"]
pub type R = crate::R<DestaddrSpec>;
#[doc = "Register `DESTADDR%s` writer"]
pub type W = crate::W<DestaddrSpec>;
#[doc = "Field `DESTADDR` reader - DMA Destination address. Reading this register will return the current destination address."]
pub type DestaddrR = crate::FieldReader<u32>;
#[doc = "Field `DESTADDR` writer - DMA Destination address. Reading this register will return the current destination address."]
pub type DestaddrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - DMA Destination address. Reading this register will return the current destination address."]
    #[inline(always)]
    pub fn destaddr(&self) -> DestaddrR {
        DestaddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - DMA Destination address. Reading this register will return the current destination address."]
    #[inline(always)]
    pub fn destaddr(&mut self) -> DestaddrW<'_, DestaddrSpec> {
        DestaddrW::new(self, 0)
    }
}
#[doc = "DMA Channel 0 Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`destaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`destaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DestaddrSpec;
impl crate::RegisterSpec for DestaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`destaddr::R`](R) reader structure"]
impl crate::Readable for DestaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`destaddr::W`](W) writer structure"]
impl crate::Writable for DestaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DESTADDR%s to value 0"]
impl crate::Resettable for DestaddrSpec {}
