#[doc = "Register `DATACTRL` reader"]
pub type R = crate::R<DatactrlSpec>;
#[doc = "Register `DATACTRL` writer"]
pub type W = crate::W<DatactrlSpec>;
#[doc = "Field `ENABLE` reader - Data transfer enable."]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Data transfer enable."]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Data transfer direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: From controller to card."]
    FromControllerToC = 0,
    #[doc = "1: From card to controller."]
    FromCardToControl = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DIRECTION` reader - Data transfer direction"]
pub type DirectionR = crate::BitReader<Enum>;
impl DirectionR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::FromControllerToC,
            true => Enum::FromCardToControl,
        }
    }
    #[doc = "From controller to card."]
    #[inline(always)]
    pub fn is_from_controller_to_c(&self) -> bool {
        *self == Enum::FromControllerToC
    }
    #[doc = "From card to controller."]
    #[inline(always)]
    pub fn is_from_card_to_control(&self) -> bool {
        *self == Enum::FromCardToControl
    }
}
#[doc = "Field `DIRECTION` writer - Data transfer direction"]
pub type DirectionW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> DirectionW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "From controller to card."]
    #[inline(always)]
    pub fn from_controller_to_c(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::FromControllerToC)
    }
    #[doc = "From card to controller."]
    #[inline(always)]
    pub fn from_card_to_control(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::FromCardToControl)
    }
}
#[doc = "Data transfer mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Block data transfer."]
    BlockDataTransfer_ = 0,
    #[doc = "1: Stream data transfer."]
    StreamDataTransfer = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Data transfer mode"]
pub type ModeR = crate::BitReader<Enum>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::BlockDataTransfer_,
            true => Enum::StreamDataTransfer,
        }
    }
    #[doc = "Block data transfer."]
    #[inline(always)]
    pub fn is_block_data_transfer_(&self) -> bool {
        *self == Enum::BlockDataTransfer_
    }
    #[doc = "Stream data transfer."]
    #[inline(always)]
    pub fn is_stream_data_transfer(&self) -> bool {
        *self == Enum::StreamDataTransfer
    }
}
#[doc = "Field `MODE` writer - Data transfer mode"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Block data transfer."]
    #[inline(always)]
    pub fn block_data_transfer_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::BlockDataTransfer_)
    }
    #[doc = "Stream data transfer."]
    #[inline(always)]
    pub fn stream_data_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::StreamDataTransfer)
    }
}
#[doc = "Enable DMA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: DMA disabled."]
    DmaDisabled_ = 0,
    #[doc = "1: DMA enabled."]
    DmaEnabled_ = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DMAENABLE` reader - Enable DMA"]
pub type DmaenableR = crate::BitReader<Enum>;
impl DmaenableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::DmaDisabled_,
            true => Enum::DmaEnabled_,
        }
    }
    #[doc = "DMA disabled."]
    #[inline(always)]
    pub fn is_dma_disabled_(&self) -> bool {
        *self == Enum::DmaDisabled_
    }
    #[doc = "DMA enabled."]
    #[inline(always)]
    pub fn is_dma_enabled_(&self) -> bool {
        *self == Enum::DmaEnabled_
    }
}
#[doc = "Field `DMAENABLE` writer - Enable DMA"]
pub type DmaenableW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> DmaenableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "DMA disabled."]
    #[inline(always)]
    pub fn dma_disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::DmaDisabled_)
    }
    #[doc = "DMA enabled."]
    #[inline(always)]
    pub fn dma_enabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::DmaEnabled_)
    }
}
#[doc = "Field `BLOCKSIZE` reader - Data block length"]
pub type BlocksizeR = crate::FieldReader;
#[doc = "Field `BLOCKSIZE` writer - Data block length"]
pub type BlocksizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Data transfer enable."]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data transfer direction"]
    #[inline(always)]
    pub fn direction(&self) -> DirectionR {
        DirectionR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Data transfer mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable DMA"]
    #[inline(always)]
    pub fn dmaenable(&self) -> DmaenableR {
        DmaenableR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Data block length"]
    #[inline(always)]
    pub fn blocksize(&self) -> BlocksizeR {
        BlocksizeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Data transfer enable."]
    #[inline(always)]
    pub fn enable(&mut self) -> EnableW<'_, DatactrlSpec> {
        EnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Data transfer direction"]
    #[inline(always)]
    pub fn direction(&mut self) -> DirectionW<'_, DatactrlSpec> {
        DirectionW::new(self, 1)
    }
    #[doc = "Bit 2 - Data transfer mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, DatactrlSpec> {
        ModeW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable DMA"]
    #[inline(always)]
    pub fn dmaenable(&mut self) -> DmaenableW<'_, DatactrlSpec> {
        DmaenableW::new(self, 3)
    }
    #[doc = "Bits 4:7 - Data block length"]
    #[inline(always)]
    pub fn blocksize(&mut self) -> BlocksizeW<'_, DatactrlSpec> {
        BlocksizeW::new(self, 4)
    }
}
#[doc = "Data control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`datactrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datactrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DatactrlSpec;
impl crate::RegisterSpec for DatactrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`datactrl::R`](R) reader structure"]
impl crate::Readable for DatactrlSpec {}
#[doc = "`write(|w| ..)` method takes [`datactrl::W`](W) writer structure"]
impl crate::Writable for DatactrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATACTRL to value 0"]
impl crate::Resettable for DatactrlSpec {}
