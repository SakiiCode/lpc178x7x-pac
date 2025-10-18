#[doc = "Register `RAWINTERRSTAT` reader"]
pub type R = crate::R<RawinterrstatSpec>;
#[doc = "Field `RAWINTERRSTAT0` reader - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type Rawinterrstat0R = crate::BitReader;
#[doc = "Field `RAWINTERRSTAT1` reader - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type Rawinterrstat1R = crate::BitReader;
#[doc = "Field `RAWINTERRSTAT2` reader - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type Rawinterrstat2R = crate::BitReader;
#[doc = "Field `RAWINTERRSTAT3` reader - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type Rawinterrstat3R = crate::BitReader;
#[doc = "Field `RAWINTERRSTAT4` reader - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type Rawinterrstat4R = crate::BitReader;
#[doc = "Field `RAWINTERRSTAT5` reader - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type Rawinterrstat5R = crate::BitReader;
#[doc = "Field `RAWINTERRSTAT6` reader - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type Rawinterrstat6R = crate::BitReader;
#[doc = "Field `RAWINTERRSTAT7` reader - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
pub type Rawinterrstat7R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat0(&self) -> Rawinterrstat0R {
        Rawinterrstat0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat1(&self) -> Rawinterrstat1R {
        Rawinterrstat1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat2(&self) -> Rawinterrstat2R {
        Rawinterrstat2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat3(&self) -> Rawinterrstat3R {
        Rawinterrstat3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat4(&self) -> Rawinterrstat4R {
        Rawinterrstat4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat5(&self) -> Rawinterrstat5R {
        Rawinterrstat5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat6(&self) -> Rawinterrstat6R {
        Rawinterrstat6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Status of the error interrupt for DMA channels prior to masking. Each bit represents one channel: 0 - the corresponding channel has no active error interrupt request. 1 - the corresponding channel does have an active error interrupt request."]
    #[inline(always)]
    pub fn rawinterrstat7(&self) -> Rawinterrstat7R {
        Rawinterrstat7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "DMA Raw Error Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rawinterrstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawinterrstatSpec;
impl crate::RegisterSpec for RawinterrstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rawinterrstat::R`](R) reader structure"]
impl crate::Readable for RawinterrstatSpec {}
#[doc = "`reset()` method sets RAWINTERRSTAT to value 0"]
impl crate::Resettable for RawinterrstatSpec {}
