#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `TFE` reader - Transmit FIFO Empty. This bit is 1 is the Transmit FIFO is empty, 0 if not."]
pub type TfeR = crate::BitReader;
#[doc = "Field `TNF` reader - Transmit FIFO Not Full. This bit is 0 if the Tx FIFO is full, 1 if not."]
pub type TnfR = crate::BitReader;
#[doc = "Field `RNE` reader - Receive FIFO Not Empty. This bit is 0 if the Receive FIFO is empty, 1 if not."]
pub type RneR = crate::BitReader;
#[doc = "Field `RFF` reader - Receive FIFO Full. This bit is 1 if the Receive FIFO is full, 0 if not."]
pub type RffR = crate::BitReader;
#[doc = "Field `BSY` reader - Busy. This bit is 0 if the SSPn controller is idle, or 1 if it is currently sending/receiving a frame and/or the Tx FIFO is not empty."]
pub type BsyR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Transmit FIFO Empty. This bit is 1 is the Transmit FIFO is empty, 0 if not."]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit FIFO Not Full. This bit is 0 if the Tx FIFO is full, 1 if not."]
    #[inline(always)]
    pub fn tnf(&self) -> TnfR {
        TnfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive FIFO Not Empty. This bit is 0 if the Receive FIFO is empty, 1 if not."]
    #[inline(always)]
    pub fn rne(&self) -> RneR {
        RneR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Receive FIFO Full. This bit is 1 if the Receive FIFO is full, 0 if not."]
    #[inline(always)]
    pub fn rff(&self) -> RffR {
        RffR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Busy. This bit is 0 if the SSPn controller is idle, or 1 if it is currently sending/receiving a frame and/or the Tx FIFO is not empty."]
    #[inline(always)]
    pub fn bsy(&self) -> BsyR {
        BsyR::new(((self.bits >> 4) & 1) != 0)
    }
}
#[doc = "Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0x03"]
impl crate::Resettable for SrSpec {
    const RESET_VALUE: u32 = 0x03;
}
