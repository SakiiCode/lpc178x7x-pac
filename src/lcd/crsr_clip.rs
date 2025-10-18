#[doc = "Register `CRSR_CLIP` reader"]
pub type R = crate::R<CrsrClipSpec>;
#[doc = "Register `CRSR_CLIP` writer"]
pub type W = crate::W<CrsrClipSpec>;
#[doc = "Field `CRSRCLIPX` reader - Cursor clip position for X direction. Distance from the left edge of the cursor image to the first displayed pixel in the cursor. When 0, the first pixel of the cursor line is displayed."]
pub type CrsrclipxR = crate::FieldReader;
#[doc = "Field `CRSRCLIPX` writer - Cursor clip position for X direction. Distance from the left edge of the cursor image to the first displayed pixel in the cursor. When 0, the first pixel of the cursor line is displayed."]
pub type CrsrclipxW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CRSRCLIPY` reader - Cursor clip position for Y direction. Distance from the top of the cursor image to the first displayed pixel in the cursor. When 0, the first displayed pixel is from the top line of the cursor image."]
pub type CrsrclipyR = crate::FieldReader;
#[doc = "Field `CRSRCLIPY` writer - Cursor clip position for Y direction. Distance from the top of the cursor image to the first displayed pixel in the cursor. When 0, the first displayed pixel is from the top line of the cursor image."]
pub type CrsrclipyW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Cursor clip position for X direction. Distance from the left edge of the cursor image to the first displayed pixel in the cursor. When 0, the first pixel of the cursor line is displayed."]
    #[inline(always)]
    pub fn crsrclipx(&self) -> CrsrclipxR {
        CrsrclipxR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:13 - Cursor clip position for Y direction. Distance from the top of the cursor image to the first displayed pixel in the cursor. When 0, the first displayed pixel is from the top line of the cursor image."]
    #[inline(always)]
    pub fn crsrclipy(&self) -> CrsrclipyR {
        CrsrclipyR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Cursor clip position for X direction. Distance from the left edge of the cursor image to the first displayed pixel in the cursor. When 0, the first pixel of the cursor line is displayed."]
    #[inline(always)]
    pub fn crsrclipx(&mut self) -> CrsrclipxW<'_, CrsrClipSpec> {
        CrsrclipxW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Cursor clip position for Y direction. Distance from the top of the cursor image to the first displayed pixel in the cursor. When 0, the first displayed pixel is from the top line of the cursor image."]
    #[inline(always)]
    pub fn crsrclipy(&mut self) -> CrsrclipyW<'_, CrsrClipSpec> {
        CrsrclipyW::new(self, 8)
    }
}
#[doc = "Cursor Clip Position register\n\nYou can [`read`](crate::Reg::read) this register and get [`crsr_clip::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crsr_clip::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrsrClipSpec;
impl crate::RegisterSpec for CrsrClipSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crsr_clip::R`](R) reader structure"]
impl crate::Readable for CrsrClipSpec {}
#[doc = "`write(|w| ..)` method takes [`crsr_clip::W`](W) writer structure"]
impl crate::Writable for CrsrClipSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRSR_CLIP to value 0"]
impl crate::Resettable for CrsrClipSpec {}
