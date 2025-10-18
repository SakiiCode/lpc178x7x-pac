#[doc = "Register `LPBASE` reader"]
pub type R = crate::R<LpbaseSpec>;
#[doc = "Register `LPBASE` writer"]
pub type W = crate::W<LpbaseSpec>;
#[doc = "Field `LCDLPBASE` reader - LCD lower panel base address. This is the start address of the lower panel frame data in memory and is doubleword aligned."]
pub type LcdlpbaseR = crate::FieldReader<u32>;
#[doc = "Field `LCDLPBASE` writer - LCD lower panel base address. This is the start address of the lower panel frame data in memory and is doubleword aligned."]
pub type LcdlpbaseW<'a, REG> = crate::FieldWriter<'a, REG, 29, u32>;
impl R {
    #[doc = "Bits 3:31 - LCD lower panel base address. This is the start address of the lower panel frame data in memory and is doubleword aligned."]
    #[inline(always)]
    pub fn lcdlpbase(&self) -> LcdlpbaseR {
        LcdlpbaseR::new((self.bits >> 3) & 0x1fff_ffff)
    }
}
impl W {
    #[doc = "Bits 3:31 - LCD lower panel base address. This is the start address of the lower panel frame data in memory and is doubleword aligned."]
    #[inline(always)]
    pub fn lcdlpbase(&mut self) -> LcdlpbaseW<'_, LpbaseSpec> {
        LcdlpbaseW::new(self, 3)
    }
}
#[doc = "Lower Panel Frame Base Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpbase::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpbase::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LpbaseSpec;
impl crate::RegisterSpec for LpbaseSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpbase::R`](R) reader structure"]
impl crate::Readable for LpbaseSpec {}
#[doc = "`write(|w| ..)` method takes [`lpbase::W`](W) writer structure"]
impl crate::Writable for LpbaseSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPBASE to value 0"]
impl crate::Resettable for LpbaseSpec {}
