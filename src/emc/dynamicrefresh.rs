#[doc = "Register `DYNAMICREFRESH` reader"]
pub type R = crate::R<DynamicrefreshSpec>;
#[doc = "Register `DYNAMICREFRESH` writer"]
pub type W = crate::W<DynamicrefreshSpec>;
#[doc = "Field `REFRESH` reader - Refresh timer. Indicates the multiple of 16 CCLKs between SDRAM refresh cycles. 0x0 = Refresh disabled (POR reset value). 0x1 - 0x7FF = n x16 = 16n CCLKs between SDRAM refresh cycles. For example: 0x1 = 1 x 16 = 16 CCLKs between SDRAM refresh cycles. 0x8 = 8 x 16 = 128 CCLKs between SDRAM refresh cycles"]
pub type RefreshR = crate::FieldReader<u16>;
#[doc = "Field `REFRESH` writer - Refresh timer. Indicates the multiple of 16 CCLKs between SDRAM refresh cycles. 0x0 = Refresh disabled (POR reset value). 0x1 - 0x7FF = n x16 = 16n CCLKs between SDRAM refresh cycles. For example: 0x1 = 1 x 16 = 16 CCLKs between SDRAM refresh cycles. 0x8 = 8 x 16 = 128 CCLKs between SDRAM refresh cycles"]
pub type RefreshW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Refresh timer. Indicates the multiple of 16 CCLKs between SDRAM refresh cycles. 0x0 = Refresh disabled (POR reset value). 0x1 - 0x7FF = n x16 = 16n CCLKs between SDRAM refresh cycles. For example: 0x1 = 1 x 16 = 16 CCLKs between SDRAM refresh cycles. 0x8 = 8 x 16 = 128 CCLKs between SDRAM refresh cycles"]
    #[inline(always)]
    pub fn refresh(&self) -> RefreshR {
        RefreshR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Refresh timer. Indicates the multiple of 16 CCLKs between SDRAM refresh cycles. 0x0 = Refresh disabled (POR reset value). 0x1 - 0x7FF = n x16 = 16n CCLKs between SDRAM refresh cycles. For example: 0x1 = 1 x 16 = 16 CCLKs between SDRAM refresh cycles. 0x8 = 8 x 16 = 128 CCLKs between SDRAM refresh cycles"]
    #[inline(always)]
    pub fn refresh(&mut self) -> RefreshW<'_, DynamicrefreshSpec> {
        RefreshW::new(self, 0)
    }
}
#[doc = "Configures dynamic memory refresh.\n\nYou can [`read`](crate::Reg::read) this register and get [`dynamicrefresh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dynamicrefresh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DynamicrefreshSpec;
impl crate::RegisterSpec for DynamicrefreshSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dynamicrefresh::R`](R) reader structure"]
impl crate::Readable for DynamicrefreshSpec {}
#[doc = "`write(|w| ..)` method takes [`dynamicrefresh::W`](W) writer structure"]
impl crate::Writable for DynamicrefreshSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DYNAMICREFRESH to value 0"]
impl crate::Resettable for DynamicrefreshSpec {}
