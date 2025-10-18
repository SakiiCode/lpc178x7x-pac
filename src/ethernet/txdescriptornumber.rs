#[doc = "Register `TXDESCRIPTORNUMBER` reader"]
pub type R = crate::R<TxdescriptornumberSpec>;
#[doc = "Register `TXDESCRIPTORNUMBER` writer"]
pub type W = crate::W<TxdescriptornumberSpec>;
#[doc = "Field `TXDN` reader - TxDescriptorNumber. Number of descriptors in the descriptor array for which TxDescriptor is the base address. The register is minus one encoded."]
pub type TxdnR = crate::FieldReader<u16>;
#[doc = "Field `TXDN` writer - TxDescriptorNumber. Number of descriptors in the descriptor array for which TxDescriptor is the base address. The register is minus one encoded."]
pub type TxdnW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - TxDescriptorNumber. Number of descriptors in the descriptor array for which TxDescriptor is the base address. The register is minus one encoded."]
    #[inline(always)]
    pub fn txdn(&self) -> TxdnR {
        TxdnR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - TxDescriptorNumber. Number of descriptors in the descriptor array for which TxDescriptor is the base address. The register is minus one encoded."]
    #[inline(always)]
    pub fn txdn(&mut self) -> TxdnW<'_, TxdescriptornumberSpec> {
        TxdnW::new(self, 0)
    }
}
#[doc = "Transmit number of descriptors register.\n\nYou can [`read`](crate::Reg::read) this register and get [`txdescriptornumber::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdescriptornumber::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdescriptornumberSpec;
impl crate::RegisterSpec for TxdescriptornumberSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdescriptornumber::R`](R) reader structure"]
impl crate::Readable for TxdescriptornumberSpec {}
#[doc = "`write(|w| ..)` method takes [`txdescriptornumber::W`](W) writer structure"]
impl crate::Writable for TxdescriptornumberSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDESCRIPTORNUMBER to value 0"]
impl crate::Resettable for TxdescriptornumberSpec {}
