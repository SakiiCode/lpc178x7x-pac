#[doc = "Register `UPBASE` reader"]
pub type R = crate::R<UpbaseSpec>;
#[doc = "Register `UPBASE` writer"]
pub type W = crate::W<UpbaseSpec>;
#[doc = "Field `LCDUPBASE` reader - LCD upper panel base address. This is the start address of the upper panel frame data in memory and is doubleword aligned."]
pub type LcdupbaseR = crate::FieldReader<u32>;
#[doc = "Field `LCDUPBASE` writer - LCD upper panel base address. This is the start address of the upper panel frame data in memory and is doubleword aligned."]
pub type LcdupbaseW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 3:31 - LCD upper panel base address. This is the start address of the upper panel frame data in memory and is doubleword aligned."]
    #[inline(always)]
    pub fn lcdupbase(&self) -> LcdupbaseR {
        LcdupbaseR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:31 - LCD upper panel base address. This is the start address of the upper panel frame data in memory and is doubleword aligned."]
    #[inline(always)]
    pub fn lcdupbase(&mut self) -> LcdupbaseW<'_, UpbaseSpec> {
        LcdupbaseW::new(self, 3)
    }
}
#[doc = "Upper Panel Frame Base Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`upbase::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`upbase::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UpbaseSpec;
impl crate::RegisterSpec for UpbaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`upbase::R`](R) reader structure"]
impl crate::Readable for UpbaseSpec {}
#[doc = "`write(|w| ..)` method takes [`upbase::W`](W) writer structure"]
impl crate::Writable for UpbaseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UPBASE to value 0"]
impl crate::Resettable for UpbaseSpec {}
