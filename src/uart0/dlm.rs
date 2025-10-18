#[doc = "Register `DLM` reader"]
pub type R = crate::R<DlmSpec>;
#[doc = "Register `DLM` writer"]
pub type W = crate::W<DlmSpec>;
#[doc = "Field `DLMSB` reader - The UARTn Divisor Latch MSB Register, along with the U0DLL register, determines the baud rate of the UARTn."]
pub type DlmsbR = crate::FieldReader;
#[doc = "Field `DLMSB` writer - The UARTn Divisor Latch MSB Register, along with the U0DLL register, determines the baud rate of the UARTn."]
pub type DlmsbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The UARTn Divisor Latch MSB Register, along with the U0DLL register, determines the baud rate of the UARTn."]
    #[inline(always)]
    pub fn dlmsb(&self) -> DlmsbR {
        DlmsbR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - The UARTn Divisor Latch MSB Register, along with the U0DLL register, determines the baud rate of the UARTn."]
    #[inline(always)]
    pub fn dlmsb(&mut self) -> DlmsbW<'_, DlmSpec> {
        DlmsbW::new(self, 0)
    }
}
#[doc = "Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider (DLAB =1).\n\nYou can [`read`](crate::Reg::read) this register and get [`dlm::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlm::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DlmSpec;
impl crate::RegisterSpec for DlmSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dlm::R`](R) reader structure"]
impl crate::Readable for DlmSpec {}
#[doc = "`write(|w| ..)` method takes [`dlm::W`](W) writer structure"]
impl crate::Writable for DlmSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DLM to value 0"]
impl crate::Resettable for DlmSpec {}
