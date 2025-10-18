#[doc = "Register `ENBLDCHNS` reader"]
pub type R = crate::R<EnbldchnsSpec>;
#[doc = "Field `ENABLEDCHANNELS0` reader - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
pub type Enabledchannels0R = crate::BitReader;
#[doc = "Field `ENABLEDCHANNELS1` reader - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
pub type Enabledchannels1R = crate::BitReader;
#[doc = "Field `ENABLEDCHANNELS2` reader - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
pub type Enabledchannels2R = crate::BitReader;
#[doc = "Field `ENABLEDCHANNELS3` reader - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
pub type Enabledchannels3R = crate::BitReader;
#[doc = "Field `ENABLEDCHANNELS4` reader - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
pub type Enabledchannels4R = crate::BitReader;
#[doc = "Field `ENABLEDCHANNELS5` reader - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
pub type Enabledchannels5R = crate::BitReader;
#[doc = "Field `ENABLEDCHANNELS6` reader - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
pub type Enabledchannels6R = crate::BitReader;
#[doc = "Field `ENABLEDCHANNELS7` reader - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
pub type Enabledchannels7R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels0(&self) -> Enabledchannels0R {
        Enabledchannels0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels1(&self) -> Enabledchannels1R {
        Enabledchannels1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels2(&self) -> Enabledchannels2R {
        Enabledchannels2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels3(&self) -> Enabledchannels3R {
        Enabledchannels3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels4(&self) -> Enabledchannels4R {
        Enabledchannels4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels5(&self) -> Enabledchannels5R {
        Enabledchannels5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels6(&self) -> Enabledchannels6R {
        Enabledchannels6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable status for DMA channels. Each bit represents one channel: 0 - DMA channel is disabled. 1 - DMA channel is enabled."]
    #[inline(always)]
    pub fn enabledchannels7(&self) -> Enabledchannels7R {
        Enabledchannels7R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "DMA Enabled Channel Register\n\nYou can [`read`](crate::Reg::read) this register and get [`enbldchns::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnbldchnsSpec;
impl crate::RegisterSpec for EnbldchnsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enbldchns::R`](R) reader structure"]
impl crate::Readable for EnbldchnsSpec {}
#[doc = "`reset()` method sets ENBLDCHNS to value 0"]
impl crate::Resettable for EnbldchnsSpec {}
