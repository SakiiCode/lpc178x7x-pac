#[doc = "Register `MAXPSIZE` reader"]
pub type R = crate::R<MaxpsizeSpec>;
#[doc = "Register `MAXPSIZE` writer"]
pub type W = crate::W<MaxpsizeSpec>;
#[doc = "Field `MPS` reader - The maximum packet size value."]
pub type MpsR = crate::FieldReader<u16>;
#[doc = "Field `MPS` writer - The maximum packet size value."]
pub type MpsW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - The maximum packet size value."]
    #[inline(always)]
    pub fn mps(&self) -> MpsR {
        MpsR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - The maximum packet size value."]
    #[inline(always)]
    pub fn mps(&mut self) -> MpsW<'_, MaxpsizeSpec> {
        MpsW::new(self, 0)
    }
}
#[doc = "USB MaxPacketSize\n\nYou can [`read`](crate::Reg::read) this register and get [`maxpsize::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxpsize::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxpsizeSpec;
impl crate::RegisterSpec for MaxpsizeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maxpsize::R`](R) reader structure"]
impl crate::Readable for MaxpsizeSpec {}
#[doc = "`write(|w| ..)` method takes [`maxpsize::W`](W) writer structure"]
impl crate::Writable for MaxpsizeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAXPSIZE to value 0x08"]
impl crate::Resettable for MaxpsizeSpec {
    const RESET_VALUE: u32 = 0x08;
}
