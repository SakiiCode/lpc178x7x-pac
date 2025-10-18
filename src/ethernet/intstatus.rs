#[doc = "Register `INTSTATUS` reader"]
pub type R = crate::R<IntstatusSpec>;
#[doc = "Field `RXOVERRUNINT` reader - Interrupt set on a fatal overrun error in the receive queue. The fatal interrupt should be resolved by a Rx soft-reset. The bit is not set when there is a nonfatal overrun error."]
pub type RxoverrunintR = crate::BitReader;
#[doc = "Field `RXERRORINT` reader - Interrupt trigger on receive errors: AlignmentError, RangeError, LengthError, SymbolError, CRCError or NoDescriptor or Overrun."]
pub type RxerrorintR = crate::BitReader;
#[doc = "Field `RXFINISHEDINT` reader - Interrupt triggered when all receive descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
pub type RxfinishedintR = crate::BitReader;
#[doc = "Field `RXDONEINT` reader - Interrupt triggered when a receive descriptor has been processed while the Interrupt bit in the Control field of the descriptor was set."]
pub type RxdoneintR = crate::BitReader;
#[doc = "Field `TXUNDERRUNINT` reader - Interrupt set on a fatal underrun error in the transmit queue. The fatal interrupt should be resolved by a Tx soft-reset. The bit is not set when there is a nonfatal underrun error."]
pub type TxunderrunintR = crate::BitReader;
#[doc = "Field `TXERRORINT` reader - Interrupt trigger on transmit errors: LateCollision, ExcessiveCollision and ExcessiveDefer, NoDescriptor or Underrun."]
pub type TxerrorintR = crate::BitReader;
#[doc = "Field `TXFINISHEDINT` reader - Interrupt triggered when all transmit descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
pub type TxfinishedintR = crate::BitReader;
#[doc = "Field `TXDONEINT` reader - Interrupt triggered when a descriptor has been transmitted while the Interrupt bit in the Control field of the descriptor was set."]
pub type TxdoneintR = crate::BitReader;
#[doc = "Field `SOFTINT` reader - Interrupt triggered by software writing a 1 to the SoftIntSet bit in the IntSet register."]
pub type SoftintR = crate::BitReader;
#[doc = "Field `WAKEUPINT` reader - Interrupt triggered by a Wake-up event detected by the receive filter."]
pub type WakeupintR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Interrupt set on a fatal overrun error in the receive queue. The fatal interrupt should be resolved by a Rx soft-reset. The bit is not set when there is a nonfatal overrun error."]
    #[inline(always)]
    pub fn rxoverrunint(&self) -> RxoverrunintR {
        RxoverrunintR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt trigger on receive errors: AlignmentError, RangeError, LengthError, SymbolError, CRCError or NoDescriptor or Overrun."]
    #[inline(always)]
    pub fn rxerrorint(&self) -> RxerrorintR {
        RxerrorintR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt triggered when all receive descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    pub fn rxfinishedint(&self) -> RxfinishedintR {
        RxfinishedintR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt triggered when a receive descriptor has been processed while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    pub fn rxdoneint(&self) -> RxdoneintR {
        RxdoneintR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt set on a fatal underrun error in the transmit queue. The fatal interrupt should be resolved by a Tx soft-reset. The bit is not set when there is a nonfatal underrun error."]
    #[inline(always)]
    pub fn txunderrunint(&self) -> TxunderrunintR {
        TxunderrunintR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt trigger on transmit errors: LateCollision, ExcessiveCollision and ExcessiveDefer, NoDescriptor or Underrun."]
    #[inline(always)]
    pub fn txerrorint(&self) -> TxerrorintR {
        TxerrorintR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt triggered when all transmit descriptors have been processed i.e. on the transition to the situation where ProduceIndex == ConsumeIndex."]
    #[inline(always)]
    pub fn txfinishedint(&self) -> TxfinishedintR {
        TxfinishedintR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt triggered when a descriptor has been transmitted while the Interrupt bit in the Control field of the descriptor was set."]
    #[inline(always)]
    pub fn txdoneint(&self) -> TxdoneintR {
        TxdoneintR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 12 - Interrupt triggered by software writing a 1 to the SoftIntSet bit in the IntSet register."]
    #[inline(always)]
    pub fn softint(&self) -> SoftintR {
        SoftintR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Interrupt triggered by a Wake-up event detected by the receive filter."]
    #[inline(always)]
    pub fn wakeupint(&self) -> WakeupintR {
        WakeupintR::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "Interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatus::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatusSpec;
impl crate::RegisterSpec for IntstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstatus::R`](R) reader structure"]
impl crate::Readable for IntstatusSpec {}
#[doc = "`reset()` method sets INTSTATUS to value 0"]
impl crate::Resettable for IntstatusSpec {}
