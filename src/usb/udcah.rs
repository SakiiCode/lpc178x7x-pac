#[doc = "Register `UDCAH` reader"]
pub type R = crate::R<UdcahSpec>;
#[doc = "Register `UDCAH` writer"]
pub type W = crate::W<UdcahSpec>;
#[doc = "Field `UDCA_ADDR` reader - Start address of the UDCA."]
pub type UdcaAddrR = crate::FieldReader<u32>;
#[doc = "Field `UDCA_ADDR` writer - Start address of the UDCA."]
pub type UdcaAddrW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 7:31 - Start address of the UDCA."]
    #[inline(always)]
    pub fn udca_addr(&self) -> UdcaAddrR {
        UdcaAddrR::new((self.bits >> 7) & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 7:31 - Start address of the UDCA."]
    #[inline(always)]
    pub fn udca_addr(&mut self) -> UdcaAddrW<'_, UdcahSpec> {
        UdcaAddrW::new(self, 7)
    }
}
#[doc = "USB UDCA Head\n\nYou can [`read`](crate::Reg::read) this register and get [`udcah::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`udcah::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UdcahSpec;
impl crate::RegisterSpec for UdcahSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`udcah::R`](R) reader structure"]
impl crate::Readable for UdcahSpec {}
#[doc = "`write(|w| ..)` method takes [`udcah::W`](W) writer structure"]
impl crate::Writable for UdcahSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UDCAH to value 0"]
impl crate::Resettable for UdcahSpec {}
