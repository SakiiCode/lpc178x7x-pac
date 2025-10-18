#[doc = "Register `LE` reader"]
pub type R = crate::R<LeSpec>;
#[doc = "Register `LE` writer"]
pub type W = crate::W<LeSpec>;
#[doc = "Field `LED` reader - Line-end delay. Controls Line-end signal delay from the rising-edge of the last panel clock, LCD_DCLK. Program with the number of LCDCLK clock periods minus 1."]
pub type LedR = crate::FieldReader;
#[doc = "Field `LED` writer - Line-end delay. Controls Line-end signal delay from the rising-edge of the last panel clock, LCD_DCLK. Program with the number of LCDCLK clock periods minus 1."]
pub type LedW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `LEE` reader - LCD Line end enable. 0 = LCD_LE disabled (held LOW). 1 = LCD_LE signal active."]
pub type LeeR = crate::BitReader;
#[doc = "Field `LEE` writer - LCD Line end enable. 0 = LCD_LE disabled (held LOW). 1 = LCD_LE signal active."]
pub type LeeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Line-end delay. Controls Line-end signal delay from the rising-edge of the last panel clock, LCD_DCLK. Program with the number of LCDCLK clock periods minus 1."]
    #[inline(always)]
    pub fn led(&self) -> LedR {
        LedR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 16 - LCD Line end enable. 0 = LCD_LE disabled (held LOW). 1 = LCD_LE signal active."]
    #[inline(always)]
    pub fn lee(&self) -> LeeR {
        LeeR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Line-end delay. Controls Line-end signal delay from the rising-edge of the last panel clock, LCD_DCLK. Program with the number of LCDCLK clock periods minus 1."]
    #[inline(always)]
    pub fn led(&mut self) -> LedW<'_, LeSpec> {
        LedW::new(self, 0)
    }
    #[doc = "Bit 16 - LCD Line end enable. 0 = LCD_LE disabled (held LOW). 1 = LCD_LE signal active."]
    #[inline(always)]
    pub fn lee(&mut self) -> LeeW<'_, LeSpec> {
        LeeW::new(self, 16)
    }
}
#[doc = "Line End Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`le::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`le::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LeSpec;
impl crate::RegisterSpec for LeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`le::R`](R) reader structure"]
impl crate::Readable for LeSpec {}
#[doc = "`write(|w| ..)` method takes [`le::W`](W) writer structure"]
impl crate::Writable for LeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LE to value 0"]
impl crate::Resettable for LeSpec {}
