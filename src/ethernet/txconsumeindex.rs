#[doc = "Register `TXCONSUMEINDEX` reader"]
pub type R = crate::R<TxconsumeindexSpec>;
#[doc = "Field `TXCI` reader - TxConsumeIndex. Index of the descriptor that is going to be transmitted next by the transmit datapath."]
pub type TxciR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - TxConsumeIndex. Index of the descriptor that is going to be transmitted next by the transmit datapath."]
    #[inline(always)]
    pub fn txci(&self) -> TxciR {
        TxciR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Transmit consume index register.\n\nYou can [`read`](crate::Reg::read) this register and get [`txconsumeindex::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxconsumeindexSpec;
impl crate::RegisterSpec for TxconsumeindexSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txconsumeindex::R`](R) reader structure"]
impl crate::Readable for TxconsumeindexSpec {}
#[doc = "`reset()` method sets TXCONSUMEINDEX to value 0"]
impl crate::Resettable for TxconsumeindexSpec {}
