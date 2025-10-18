#[doc = "Register `MATRIXARB` reader"]
pub type R = crate::R<MatrixarbSpec>;
#[doc = "Register `MATRIXARB` writer"]
pub type W = crate::W<MatrixarbSpec>;
#[doc = "Field `PRI_ICODE` reader - I-Code bus priority. Should be lower than PRI_DCODE for proper operation."]
pub type PriIcodeR = crate::FieldReader;
#[doc = "Field `PRI_ICODE` writer - I-Code bus priority. Should be lower than PRI_DCODE for proper operation."]
pub type PriIcodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRI_DCODE` reader - D-Code bus priority."]
pub type PriDcodeR = crate::FieldReader;
#[doc = "Field `PRI_DCODE` writer - D-Code bus priority."]
pub type PriDcodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRI_SYS` reader - System bus priority."]
pub type PriSysR = crate::FieldReader;
#[doc = "Field `PRI_SYS` writer - System bus priority."]
pub type PriSysW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRI_GPDMA` reader - General Purpose DMA controller priority."]
pub type PriGpdmaR = crate::FieldReader;
#[doc = "Field `PRI_GPDMA` writer - General Purpose DMA controller priority."]
pub type PriGpdmaW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRI_ETH` reader - Ethernet DMA priority."]
pub type PriEthR = crate::FieldReader;
#[doc = "Field `PRI_ETH` writer - Ethernet DMA priority."]
pub type PriEthW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRI_LCD` reader - LCD DMA priority."]
pub type PriLcdR = crate::FieldReader;
#[doc = "Field `PRI_LCD` writer - LCD DMA priority."]
pub type PriLcdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRI_USB` reader - USB DMA priority."]
pub type PriUsbR = crate::FieldReader;
#[doc = "Field `PRI_USB` writer - USB DMA priority."]
pub type PriUsbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ROM_LAT` reader - ROM latency select. Should always be 0."]
pub type RomLatR = crate::BitReader;
#[doc = "Field `ROM_LAT` writer - ROM latency select. Should always be 0."]
pub type RomLatW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - I-Code bus priority. Should be lower than PRI_DCODE for proper operation."]
    #[inline(always)]
    pub fn pri_icode(&self) -> PriIcodeR {
        PriIcodeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - D-Code bus priority."]
    #[inline(always)]
    pub fn pri_dcode(&self) -> PriDcodeR {
        PriDcodeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - System bus priority."]
    #[inline(always)]
    pub fn pri_sys(&self) -> PriSysR {
        PriSysR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - General Purpose DMA controller priority."]
    #[inline(always)]
    pub fn pri_gpdma(&self) -> PriGpdmaR {
        PriGpdmaR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Ethernet DMA priority."]
    #[inline(always)]
    pub fn pri_eth(&self) -> PriEthR {
        PriEthR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - LCD DMA priority."]
    #[inline(always)]
    pub fn pri_lcd(&self) -> PriLcdR {
        PriLcdR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - USB DMA priority."]
    #[inline(always)]
    pub fn pri_usb(&self) -> PriUsbR {
        PriUsbR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 16 - ROM latency select. Should always be 0."]
    #[inline(always)]
    pub fn rom_lat(&self) -> RomLatR {
        RomLatR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - I-Code bus priority. Should be lower than PRI_DCODE for proper operation."]
    #[inline(always)]
    pub fn pri_icode(&mut self) -> PriIcodeW<'_, MatrixarbSpec> {
        PriIcodeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - D-Code bus priority."]
    #[inline(always)]
    pub fn pri_dcode(&mut self) -> PriDcodeW<'_, MatrixarbSpec> {
        PriDcodeW::new(self, 2)
    }
    #[doc = "Bits 4:5 - System bus priority."]
    #[inline(always)]
    pub fn pri_sys(&mut self) -> PriSysW<'_, MatrixarbSpec> {
        PriSysW::new(self, 4)
    }
    #[doc = "Bits 6:7 - General Purpose DMA controller priority."]
    #[inline(always)]
    pub fn pri_gpdma(&mut self) -> PriGpdmaW<'_, MatrixarbSpec> {
        PriGpdmaW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Ethernet DMA priority."]
    #[inline(always)]
    pub fn pri_eth(&mut self) -> PriEthW<'_, MatrixarbSpec> {
        PriEthW::new(self, 8)
    }
    #[doc = "Bits 10:11 - LCD DMA priority."]
    #[inline(always)]
    pub fn pri_lcd(&mut self) -> PriLcdW<'_, MatrixarbSpec> {
        PriLcdW::new(self, 10)
    }
    #[doc = "Bits 12:13 - USB DMA priority."]
    #[inline(always)]
    pub fn pri_usb(&mut self) -> PriUsbW<'_, MatrixarbSpec> {
        PriUsbW::new(self, 12)
    }
    #[doc = "Bit 16 - ROM latency select. Should always be 0."]
    #[inline(always)]
    pub fn rom_lat(&mut self) -> RomLatW<'_, MatrixarbSpec> {
        RomLatW::new(self, 16)
    }
}
#[doc = "Matrix arbitration register\n\nYou can [`read`](crate::Reg::read) this register and get [`matrixarb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrixarb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MatrixarbSpec;
impl crate::RegisterSpec for MatrixarbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`matrixarb::R`](R) reader structure"]
impl crate::Readable for MatrixarbSpec {}
#[doc = "`write(|w| ..)` method takes [`matrixarb::W`](W) writer structure"]
impl crate::Writable for MatrixarbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MATRIXARB to value 0"]
impl crate::Resettable for MatrixarbSpec {}
