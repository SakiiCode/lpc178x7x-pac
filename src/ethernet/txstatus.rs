#[doc = "Register `TXSTATUS` reader"]
pub type R = crate::R<TxstatusSpec>;
#[doc = "Register `TXSTATUS` writer"]
pub type W = crate::W<TxstatusSpec>;
#[doc = "Field `TXSTAT` reader - TxStatus. MSBs of transmit status base address."]
pub type TxstatR = crate::FieldReader<u32>;
#[doc = "Field `TXSTAT` writer - TxStatus. MSBs of transmit status base address."]
pub type TxstatW<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 2:31 - TxStatus. MSBs of transmit status base address."]
    #[inline(always)]
    pub fn txstat(&self) -> TxstatR {
        TxstatR::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 2:31 - TxStatus. MSBs of transmit status base address."]
    #[inline(always)]
    pub fn txstat(&mut self) -> TxstatW<'_, TxstatusSpec> {
        TxstatW::new(self, 2)
    }
}
#[doc = "Transmit status base address register.\n\nYou can [`read`](crate::Reg::read) this register and get [`txstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxstatusSpec;
impl crate::RegisterSpec for TxstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txstatus::R`](R) reader structure"]
impl crate::Readable for TxstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`txstatus::W`](W) writer structure"]
impl crate::Writable for TxstatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TXSTATUS to value 0"]
impl crate::Resettable for TxstatusSpec {}
