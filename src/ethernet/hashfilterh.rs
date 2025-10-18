#[doc = "Register `HASHFILTERH` reader"]
pub type R = crate::R<HashfilterhSpec>;
#[doc = "Register `HASHFILTERH` writer"]
pub type W = crate::W<HashfilterhSpec>;
#[doc = "Field `HFH` reader - Bits 63:32 of the imperfect filter hash table for receive filtering."]
pub type HfhR = crate::FieldReader<u32>;
#[doc = "Field `HFH` writer - Bits 63:32 of the imperfect filter hash table for receive filtering."]
pub type HfhW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Bits 63:32 of the imperfect filter hash table for receive filtering."]
    #[inline(always)]
    pub fn hfh(&self) -> HfhR {
        HfhR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Bits 63:32 of the imperfect filter hash table for receive filtering."]
    #[inline(always)]
    pub fn hfh(&mut self) -> HfhW<'_, HashfilterhSpec> {
        HfhW::new(self, 0)
    }
}
#[doc = "Hash filter table MSBs register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hashfilterh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashfilterh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HashfilterhSpec;
impl crate::RegisterSpec for HashfilterhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hashfilterh::R`](R) reader structure"]
impl crate::Readable for HashfilterhSpec {}
#[doc = "`write(|w| ..)` method takes [`hashfilterh::W`](W) writer structure"]
impl crate::Writable for HashfilterhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HASHFILTERH to value 0"]
impl crate::Resettable for HashfilterhSpec {}
