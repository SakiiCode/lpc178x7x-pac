#[doc = "Register `CRSR_CFG` reader"]
pub type R = crate::R<CrsrCfgSpec>;
#[doc = "Register `CRSR_CFG` writer"]
pub type W = crate::W<CrsrCfgSpec>;
#[doc = "Field `CRSRSIZE` reader - Cursor size selection. 0 = 32x32 pixel cursor. Allows for 4 defined cursors. 1 = 64x64 pixel cursor."]
pub type CrsrsizeR = crate::BitReader;
#[doc = "Field `CRSRSIZE` writer - Cursor size selection. 0 = 32x32 pixel cursor. Allows for 4 defined cursors. 1 = 64x64 pixel cursor."]
pub type CrsrsizeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRAMESYNC` reader - Cursor frame synchronization type. 0 = Cursor coordinates are asynchronous. 1 = Cursor coordinates are synchronized to the frame synchronization pulse."]
pub type FramesyncR = crate::BitReader;
#[doc = "Field `FRAMESYNC` writer - Cursor frame synchronization type. 0 = Cursor coordinates are asynchronous. 1 = Cursor coordinates are synchronized to the frame synchronization pulse."]
pub type FramesyncW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Cursor size selection. 0 = 32x32 pixel cursor. Allows for 4 defined cursors. 1 = 64x64 pixel cursor."]
    #[inline(always)]
    pub fn crsrsize(&self) -> CrsrsizeR {
        CrsrsizeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Cursor frame synchronization type. 0 = Cursor coordinates are asynchronous. 1 = Cursor coordinates are synchronized to the frame synchronization pulse."]
    #[inline(always)]
    pub fn framesync(&self) -> FramesyncR {
        FramesyncR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cursor size selection. 0 = 32x32 pixel cursor. Allows for 4 defined cursors. 1 = 64x64 pixel cursor."]
    #[inline(always)]
    pub fn crsrsize(&mut self) -> CrsrsizeW<'_, CrsrCfgSpec> {
        CrsrsizeW::new(self, 0)
    }
    #[doc = "Bit 1 - Cursor frame synchronization type. 0 = Cursor coordinates are asynchronous. 1 = Cursor coordinates are synchronized to the frame synchronization pulse."]
    #[inline(always)]
    pub fn framesync(&mut self) -> FramesyncW<'_, CrsrCfgSpec> {
        FramesyncW::new(self, 1)
    }
}
#[doc = "Cursor Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`crsr_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crsr_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrsrCfgSpec;
impl crate::RegisterSpec for CrsrCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crsr_cfg::R`](R) reader structure"]
impl crate::Readable for CrsrCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`crsr_cfg::W`](W) writer structure"]
impl crate::Writable for CrsrCfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRSR_CFG to value 0"]
impl crate::Resettable for CrsrCfgSpec {}
