#[doc = "Register `CRSR_XY` reader"]
pub type R = crate::R<CrsrXySpec>;
#[doc = "Register `CRSR_XY` writer"]
pub type W = crate::W<CrsrXySpec>;
#[doc = "Field `CRSRX` reader - X ordinate of the cursor origin measured in pixels. When 0, the left edge of the cursor is at the left of the display."]
pub type CrsrxR = crate::FieldReader<u16>;
#[doc = "Field `CRSRX` writer - X ordinate of the cursor origin measured in pixels. When 0, the left edge of the cursor is at the left of the display."]
pub type CrsrxW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CRSRY` reader - Y ordinate of the cursor origin measured in pixels. When 0, the top edge of the cursor is at the top of the display."]
pub type CrsryR = crate::FieldReader<u16>;
#[doc = "Field `CRSRY` writer - Y ordinate of the cursor origin measured in pixels. When 0, the top edge of the cursor is at the top of the display."]
pub type CrsryW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - X ordinate of the cursor origin measured in pixels. When 0, the left edge of the cursor is at the left of the display."]
    #[inline(always)]
    pub fn crsrx(&self) -> CrsrxR {
        CrsrxR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Y ordinate of the cursor origin measured in pixels. When 0, the top edge of the cursor is at the top of the display."]
    #[inline(always)]
    pub fn crsry(&self) -> CrsryR {
        CrsryR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - X ordinate of the cursor origin measured in pixels. When 0, the left edge of the cursor is at the left of the display."]
    #[inline(always)]
    pub fn crsrx(&mut self) -> CrsrxW<'_, CrsrXySpec> {
        CrsrxW::new(self, 0)
    }
    #[doc = "Bits 16:25 - Y ordinate of the cursor origin measured in pixels. When 0, the top edge of the cursor is at the top of the display."]
    #[inline(always)]
    pub fn crsry(&mut self) -> CrsryW<'_, CrsrXySpec> {
        CrsryW::new(self, 16)
    }
}
#[doc = "Cursor XY Position register\n\nYou can [`read`](crate::Reg::read) this register and get [`crsr_xy::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crsr_xy::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrsrXySpec;
impl crate::RegisterSpec for CrsrXySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crsr_xy::R`](R) reader structure"]
impl crate::Readable for CrsrXySpec {}
#[doc = "`write(|w| ..)` method takes [`crsr_xy::W`](W) writer structure"]
impl crate::Writable for CrsrXySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRSR_XY to value 0"]
impl crate::Resettable for CrsrXySpec {}
