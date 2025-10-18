#[doc = "Register `SCLH` reader"]
pub type R = crate::R<SclhSpec>;
#[doc = "Register `SCLH` writer"]
pub type W = crate::W<SclhSpec>;
#[doc = "Field `SCLH` reader - Count for SCL HIGH time period selection."]
pub type SclhR = crate::FieldReader<u16>;
#[doc = "Field `SCLH` writer - Count for SCL HIGH time period selection."]
pub type SclhW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Count for SCL HIGH time period selection."]
    #[inline(always)]
    pub fn sclh(&self) -> SclhR {
        SclhR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Count for SCL HIGH time period selection."]
    #[inline(always)]
    pub fn sclh(&mut self) -> SclhW<'_, SclhSpec> {
        SclhW::new(self, 0)
    }
}
#[doc = "SCH Duty Cycle Register High Half Word. Determines the high time of the I2C clock.\n\nYou can [`read`](crate::Reg::read) this register and get [`sclh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sclh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SclhSpec;
impl crate::RegisterSpec for SclhSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sclh::R`](R) reader structure"]
impl crate::Readable for SclhSpec {}
#[doc = "`write(|w| ..)` method takes [`sclh::W`](W) writer structure"]
impl crate::Writable for SclhSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCLH to value 0x04"]
impl crate::Resettable for SclhSpec {
    const RESET_VALUE: u32 = 0x04;
}
