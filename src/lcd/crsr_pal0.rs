#[doc = "Register `CRSR_PAL0` reader"]
pub type R = crate::R<CrsrPal0Spec>;
#[doc = "Register `CRSR_PAL0` writer"]
pub type W = crate::W<CrsrPal0Spec>;
#[doc = "Field `RED` reader - Red color component"]
pub type RedR = crate::FieldReader;
#[doc = "Field `RED` writer - Red color component"]
pub type RedW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `GREEN` reader - Green color component"]
pub type GreenR = crate::FieldReader;
#[doc = "Field `GREEN` writer - Green color component"]
pub type GreenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BLUE` reader - Blue color component."]
pub type BlueR = crate::FieldReader;
#[doc = "Field `BLUE` writer - Blue color component."]
pub type BlueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Red color component"]
    #[inline(always)]
    pub fn red(&self) -> RedR {
        RedR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Green color component"]
    #[inline(always)]
    pub fn green(&self) -> GreenR {
        GreenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Blue color component."]
    #[inline(always)]
    pub fn blue(&self) -> BlueR {
        BlueR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Red color component"]
    #[inline(always)]
    pub fn red(&mut self) -> RedW<'_, CrsrPal0Spec> {
        RedW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Green color component"]
    #[inline(always)]
    pub fn green(&mut self) -> GreenW<'_, CrsrPal0Spec> {
        GreenW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Blue color component."]
    #[inline(always)]
    pub fn blue(&mut self) -> BlueW<'_, CrsrPal0Spec> {
        BlueW::new(self, 16)
    }
}
#[doc = "Cursor Palette register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`crsr_pal0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crsr_pal0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrsrPal0Spec;
impl crate::RegisterSpec for CrsrPal0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crsr_pal0::R`](R) reader structure"]
impl crate::Readable for CrsrPal0Spec {}
#[doc = "`write(|w| ..)` method takes [`crsr_pal0::W`](W) writer structure"]
impl crate::Writable for CrsrPal0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRSR_PAL0 to value 0"]
impl crate::Resettable for CrsrPal0Spec {}
