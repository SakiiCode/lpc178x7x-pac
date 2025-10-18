#[doc = "Register `TXDESCRIPTOR` reader"]
pub type R = crate::R<TxdescriptorSpec>;
#[doc = "Register `TXDESCRIPTOR` writer"]
pub type W = crate::W<TxdescriptorSpec>;
#[doc = "Field `TXD` reader - TxDescriptor. MSBs of transmit descriptor base address."]
pub type TxdR = crate::FieldReader<u32>;
#[doc = "Field `TXD` writer - TxDescriptor. MSBs of transmit descriptor base address."]
pub type TxdW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - TxDescriptor. MSBs of transmit descriptor base address."]
    #[inline(always)]
    pub fn txd(&self) -> TxdR {
        TxdR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - TxDescriptor. MSBs of transmit descriptor base address."]
    #[inline(always)]
    pub fn txd(&mut self) -> TxdW<'_, TxdescriptorSpec> {
        TxdW::new(self, 2)
    }
}
#[doc = "Transmit descriptor base address register.\n\nYou can [`read`](crate::Reg::read) this register and get [`txdescriptor::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdescriptor::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdescriptorSpec;
impl crate::RegisterSpec for TxdescriptorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdescriptor::R`](R) reader structure"]
impl crate::Readable for TxdescriptorSpec {}
#[doc = "`write(|w| ..)` method takes [`txdescriptor::W`](W) writer structure"]
impl crate::Writable for TxdescriptorSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXDESCRIPTOR to value 0"]
impl crate::Resettable for TxdescriptorSpec {}
