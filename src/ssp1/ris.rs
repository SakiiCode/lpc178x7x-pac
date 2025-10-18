#[doc = "Register `RIS` reader"]
pub type R = crate::R<RisSpec>;
#[doc = "Field `RORRIS` reader - This bit is 1 if another frame was completely received while the RxFIFO was full. The ARM spec implies that the preceding frame data is overwritten by the new frame data when this occurs."]
pub type RorrisR = crate::BitReader;
#[doc = "Field `RTRIS` reader - This bit is 1 if the Rx FIFO is not empty, and has not been read for a time-out period. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR X \\[SCR+1\\])."]
pub type RtrisR = crate::BitReader;
#[doc = "Field `RXRIS` reader - This bit is 1 if the Rx FIFO is at least half full."]
pub type RxrisR = crate::BitReader;
#[doc = "Field `TXRIS` reader - This bit is 1 if the Tx FIFO is at least half empty."]
pub type TxrisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit is 1 if another frame was completely received while the RxFIFO was full. The ARM spec implies that the preceding frame data is overwritten by the new frame data when this occurs."]
    #[inline(always)]
    pub fn rorris(&self) -> RorrisR {
        RorrisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit is 1 if the Rx FIFO is not empty, and has not been read for a time-out period. The time-out period is the same for master and slave modes and is determined by the SSP bit rate: 32 bits at PCLK / (CPSDVSR X \\[SCR+1\\])."]
    #[inline(always)]
    pub fn rtris(&self) -> RtrisR {
        RtrisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit is 1 if the Rx FIFO is at least half full."]
    #[inline(always)]
    pub fn rxris(&self) -> RxrisR {
        RxrisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is 1 if the Tx FIFO is at least half empty."]
    #[inline(always)]
    pub fn txris(&self) -> TxrisR {
        TxrisR::new(((self.bits >> 3) & 1) != 0)
    }
}
#[doc = "Raw Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ris::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RisSpec;
impl crate::RegisterSpec for RisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ris::R`](R) reader structure"]
impl crate::Readable for RisSpec {}
#[doc = "`reset()` method sets RIS to value 0x08"]
impl crate::Resettable for RisSpec {
    const RESET_VALUE: u32 = 0x08;
}
