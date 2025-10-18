#[doc = "Register `CRSR_CTRL` reader"]
pub type R = crate::R<CrsrCtrlSpec>;
#[doc = "Register `CRSR_CTRL` writer"]
pub type W = crate::W<CrsrCtrlSpec>;
#[doc = "Field `CRSRON` reader - Cursor enable. 0 = Cursor is not displayed. 1 = Cursor is displayed."]
pub type CrsronR = crate::BitReader;
#[doc = "Field `CRSRON` writer - Cursor enable. 0 = Cursor is not displayed. 1 = Cursor is displayed."]
pub type CrsronW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRSRNUM1_0` reader - Cursor image number. If the selected cursor size is 6x64, this field has no effect. If the selected cursor size is 32x32: 00 = Cursor0. 01 = Cursor1. 10 = Cursor2. 11 = Cursor3."]
pub type Crsrnum1_0R = crate::FieldReader;
#[doc = "Field `CRSRNUM1_0` writer - Cursor image number. If the selected cursor size is 6x64, this field has no effect. If the selected cursor size is 32x32: 00 = Cursor0. 01 = Cursor1. 10 = Cursor2. 11 = Cursor3."]
pub type Crsrnum1_0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Cursor enable. 0 = Cursor is not displayed. 1 = Cursor is displayed."]
    #[inline(always)]
    pub fn crsron(&self) -> CrsronR {
        CrsronR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Cursor image number. If the selected cursor size is 6x64, this field has no effect. If the selected cursor size is 32x32: 00 = Cursor0. 01 = Cursor1. 10 = Cursor2. 11 = Cursor3."]
    #[inline(always)]
    pub fn crsrnum1_0(&self) -> Crsrnum1_0R {
        Crsrnum1_0R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Cursor enable. 0 = Cursor is not displayed. 1 = Cursor is displayed."]
    #[inline(always)]
    pub fn crsron(&mut self) -> CrsronW<'_, CrsrCtrlSpec> {
        CrsronW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Cursor image number. If the selected cursor size is 6x64, this field has no effect. If the selected cursor size is 32x32: 00 = Cursor0. 01 = Cursor1. 10 = Cursor2. 11 = Cursor3."]
    #[inline(always)]
    pub fn crsrnum1_0(&mut self) -> Crsrnum1_0W<'_, CrsrCtrlSpec> {
        Crsrnum1_0W::new(self, 4)
    }
}
#[doc = "Cursor Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`crsr_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crsr_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrsrCtrlSpec;
impl crate::RegisterSpec for CrsrCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crsr_ctrl::R`](R) reader structure"]
impl crate::Readable for CrsrCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`crsr_ctrl::W`](W) writer structure"]
impl crate::Writable for CrsrCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRSR_CTRL to value 0"]
impl crate::Resettable for CrsrCtrlSpec {}
