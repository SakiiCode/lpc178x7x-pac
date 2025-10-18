#[doc = "Register `CRSR_INTMSK` reader"]
pub type R = crate::R<CrsrIntmskSpec>;
#[doc = "Register `CRSR_INTMSK` writer"]
pub type W = crate::W<CrsrIntmskSpec>;
#[doc = "Field `CRSRIM` reader - Cursor interrupt mask. When clear, the cursor never interrupts the processor. When set, the cursor interrupts the processor immediately after reading of the last word of cursor image."]
pub type CrsrimR = crate::BitReader;
#[doc = "Field `CRSRIM` writer - Cursor interrupt mask. When clear, the cursor never interrupts the processor. When set, the cursor interrupts the processor immediately after reading of the last word of cursor image."]
pub type CrsrimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Cursor interrupt mask. When clear, the cursor never interrupts the processor. When set, the cursor interrupts the processor immediately after reading of the last word of cursor image."]
    #[inline(always)]
    pub fn crsrim(&self) -> CrsrimR {
        CrsrimR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cursor interrupt mask. When clear, the cursor never interrupts the processor. When set, the cursor interrupts the processor immediately after reading of the last word of cursor image."]
    #[inline(always)]
    pub fn crsrim(&mut self) -> CrsrimW<'_, CrsrIntmskSpec> {
        CrsrimW::new(self, 0)
    }
}
#[doc = "Cursor Interrupt Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`crsr_intmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crsr_intmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrsrIntmskSpec;
impl crate::RegisterSpec for CrsrIntmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crsr_intmsk::R`](R) reader structure"]
impl crate::Readable for CrsrIntmskSpec {}
#[doc = "`write(|w| ..)` method takes [`crsr_intmsk::W`](W) writer structure"]
impl crate::Writable for CrsrIntmskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRSR_INTMSK to value 0"]
impl crate::Resettable for CrsrIntmskSpec {}
