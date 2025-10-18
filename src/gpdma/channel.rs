#[doc = "Register `CHANNEL%s` reader"]
pub type R = crate::R<ChannelSpec>;
#[doc = "Register `CHANNEL%s` writer"]
pub type W = crate::W<ChannelSpec>;
#[doc = "Field `E` reader - Channel enable. Reading this bit indicates whether a channel is currently enabled or disabled: 0 = channel disabled. 1 = channel enabled. The Channel Enable bit status can also be found by reading the DMACEnbldChns Register. A channel is enabled by setting this bit. A channel can be disabled by clearing the Enable bit. This causes the current AHB transfer (if one is in progress) to complete and the channel is then disabled. Any data in the FIFO of the relevant channel is lost. Restarting the channel by setting the Channel Enable bit has unpredictable effects, the channel must be fully re-initialized. The channel is also disabled, and Channel Enable bit cleared, when the last LLI is reached, the DMA transfer is completed, or if a channel error is encountered. If a channel must be disabled without losing data in the FIFO, the Halt bit must be set so that further DMA requests are ignored. The Active bit must then be polled until it reaches 0, indicating that there is no data left in the FIFO. Finally, the Channel Enable bit can be cleared."]
pub type ER = crate::BitReader;
#[doc = "Field `E` writer - Channel enable. Reading this bit indicates whether a channel is currently enabled or disabled: 0 = channel disabled. 1 = channel enabled. The Channel Enable bit status can also be found by reading the DMACEnbldChns Register. A channel is enabled by setting this bit. A channel can be disabled by clearing the Enable bit. This causes the current AHB transfer (if one is in progress) to complete and the channel is then disabled. Any data in the FIFO of the relevant channel is lost. Restarting the channel by setting the Channel Enable bit has unpredictable effects, the channel must be fully re-initialized. The channel is also disabled, and Channel Enable bit cleared, when the last LLI is reached, the DMA transfer is completed, or if a channel error is encountered. If a channel must be disabled without losing data in the FIFO, the Halt bit must be set so that further DMA requests are ignored. The Active bit must then be polled until it reaches 0, indicating that there is no data left in the FIFO. Finally, the Channel Enable bit can be cleared."]
pub type EW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRCPERIPHERAL` reader - Source peripheral. This value selects the DMA source request peripheral. This field is ignored if the source of the transfer is from memory. See Table 672 for peripheral identification."]
pub type SrcperipheralR = crate::FieldReader;
#[doc = "Field `SRCPERIPHERAL` writer - Source peripheral. This value selects the DMA source request peripheral. This field is ignored if the source of the transfer is from memory. See Table 672 for peripheral identification."]
pub type SrcperipheralW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `DESTPERIPHERAL` reader - Destination peripheral. This value selects the DMA destination request peripheral. This field is ignored if the destination of the transfer is to memory. See Table 672 for peripheral identification."]
pub type DestperipheralR = crate::FieldReader;
#[doc = "Field `DESTPERIPHERAL` writer - Destination peripheral. This value selects the DMA destination request peripheral. This field is ignored if the destination of the transfer is to memory. See Table 672 for peripheral identification."]
pub type DestperipheralW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TRANSFERTYPE` reader - This value indicates the type of transfer and specifies the flow controller. The transfer type can be memory-to-memory, memory-to-peripheral, peripheral-to-memory, or peripheral-to-peripheral. Flow can be controlled by the DMA controller, the source peripheral, or the destination peripheral. Refer to Table 694 for the encoding of this field."]
pub type TransfertypeR = crate::FieldReader;
#[doc = "Field `TRANSFERTYPE` writer - This value indicates the type of transfer and specifies the flow controller. The transfer type can be memory-to-memory, memory-to-peripheral, peripheral-to-memory, or peripheral-to-peripheral. Flow can be controlled by the DMA controller, the source peripheral, or the destination peripheral. Refer to Table 694 for the encoding of this field."]
pub type TransfertypeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IE` reader - Interrupt error mask. When cleared, this bit masks out the error interrupt of the relevant channel."]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - Interrupt error mask. When cleared, this bit masks out the error interrupt of the relevant channel."]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITC` reader - Terminal count interrupt mask. When cleared, this bit masks out the terminal count interrupt of the relevant channel."]
pub type ItcR = crate::BitReader;
#[doc = "Field `ITC` writer - Terminal count interrupt mask. When cleared, this bit masks out the terminal count interrupt of the relevant channel."]
pub type ItcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L` reader - Lock. When set, this bit enables locked transfers. This information is not used in the LPC178x/177x."]
pub type LR = crate::BitReader;
#[doc = "Field `L` writer - Lock. When set, this bit enables locked transfers. This information is not used in the LPC178x/177x."]
pub type LW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `A` reader - Active: 0 = there is no data in the FIFO of the channel. 1 = the channel FIFO has data. This value can be used with the Halt and Channel Enable bits to cleanly disable a DMA channel. This is a read-only bit."]
pub type AR = crate::BitReader;
#[doc = "Field `A` writer - Active: 0 = there is no data in the FIFO of the channel. 1 = the channel FIFO has data. This value can be used with the Halt and Channel Enable bits to cleanly disable a DMA channel. This is a read-only bit."]
pub type AW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H` reader - Halt: 0 = enable DMA requests. 1 = ignore further source DMA requests. The contents of the channel FIFO are drained. This value can be used with the Active and Channel Enable bits to cleanly disable a DMA channel."]
pub type HR = crate::BitReader;
#[doc = "Field `H` writer - Halt: 0 = enable DMA requests. 1 = ignore further source DMA requests. The contents of the channel FIFO are drained. This value can be used with the Active and Channel Enable bits to cleanly disable a DMA channel."]
pub type HW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel enable. Reading this bit indicates whether a channel is currently enabled or disabled: 0 = channel disabled. 1 = channel enabled. The Channel Enable bit status can also be found by reading the DMACEnbldChns Register. A channel is enabled by setting this bit. A channel can be disabled by clearing the Enable bit. This causes the current AHB transfer (if one is in progress) to complete and the channel is then disabled. Any data in the FIFO of the relevant channel is lost. Restarting the channel by setting the Channel Enable bit has unpredictable effects, the channel must be fully re-initialized. The channel is also disabled, and Channel Enable bit cleared, when the last LLI is reached, the DMA transfer is completed, or if a channel error is encountered. If a channel must be disabled without losing data in the FIFO, the Halt bit must be set so that further DMA requests are ignored. The Active bit must then be polled until it reaches 0, indicating that there is no data left in the FIFO. Finally, the Channel Enable bit can be cleared."]
    #[inline(always)]
    pub fn e(&self) -> ER {
        ER::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:5 - Source peripheral. This value selects the DMA source request peripheral. This field is ignored if the source of the transfer is from memory. See Table 672 for peripheral identification."]
    #[inline(always)]
    pub fn srcperipheral(&self) -> SrcperipheralR {
        SrcperipheralR::new(((self.bits >> 1) & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - Destination peripheral. This value selects the DMA destination request peripheral. This field is ignored if the destination of the transfer is to memory. See Table 672 for peripheral identification."]
    #[inline(always)]
    pub fn destperipheral(&self) -> DestperipheralR {
        DestperipheralR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bits 11:13 - This value indicates the type of transfer and specifies the flow controller. The transfer type can be memory-to-memory, memory-to-peripheral, peripheral-to-memory, or peripheral-to-peripheral. Flow can be controlled by the DMA controller, the source peripheral, or the destination peripheral. Refer to Table 694 for the encoding of this field."]
    #[inline(always)]
    pub fn transfertype(&self) -> TransfertypeR {
        TransfertypeR::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - Interrupt error mask. When cleared, this bit masks out the error interrupt of the relevant channel."]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Terminal count interrupt mask. When cleared, this bit masks out the terminal count interrupt of the relevant channel."]
    #[inline(always)]
    pub fn itc(&self) -> ItcR {
        ItcR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Lock. When set, this bit enables locked transfers. This information is not used in the LPC178x/177x."]
    #[inline(always)]
    pub fn l(&self) -> LR {
        LR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Active: 0 = there is no data in the FIFO of the channel. 1 = the channel FIFO has data. This value can be used with the Halt and Channel Enable bits to cleanly disable a DMA channel. This is a read-only bit."]
    #[inline(always)]
    pub fn a(&self) -> AR {
        AR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Halt: 0 = enable DMA requests. 1 = ignore further source DMA requests. The contents of the channel FIFO are drained. This value can be used with the Active and Channel Enable bits to cleanly disable a DMA channel."]
    #[inline(always)]
    pub fn h(&self) -> HR {
        HR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable. Reading this bit indicates whether a channel is currently enabled or disabled: 0 = channel disabled. 1 = channel enabled. The Channel Enable bit status can also be found by reading the DMACEnbldChns Register. A channel is enabled by setting this bit. A channel can be disabled by clearing the Enable bit. This causes the current AHB transfer (if one is in progress) to complete and the channel is then disabled. Any data in the FIFO of the relevant channel is lost. Restarting the channel by setting the Channel Enable bit has unpredictable effects, the channel must be fully re-initialized. The channel is also disabled, and Channel Enable bit cleared, when the last LLI is reached, the DMA transfer is completed, or if a channel error is encountered. If a channel must be disabled without losing data in the FIFO, the Halt bit must be set so that further DMA requests are ignored. The Active bit must then be polled until it reaches 0, indicating that there is no data left in the FIFO. Finally, the Channel Enable bit can be cleared."]
    #[inline(always)]
    pub fn e(&mut self) -> EW<'_, ChannelSpec> {
        EW::new(self, 0)
    }
    #[doc = "Bits 1:5 - Source peripheral. This value selects the DMA source request peripheral. This field is ignored if the source of the transfer is from memory. See Table 672 for peripheral identification."]
    #[inline(always)]
    pub fn srcperipheral(&mut self) -> SrcperipheralW<'_, ChannelSpec> {
        SrcperipheralW::new(self, 1)
    }
    #[doc = "Bits 6:10 - Destination peripheral. This value selects the DMA destination request peripheral. This field is ignored if the destination of the transfer is to memory. See Table 672 for peripheral identification."]
    #[inline(always)]
    pub fn destperipheral(&mut self) -> DestperipheralW<'_, ChannelSpec> {
        DestperipheralW::new(self, 6)
    }
    #[doc = "Bits 11:13 - This value indicates the type of transfer and specifies the flow controller. The transfer type can be memory-to-memory, memory-to-peripheral, peripheral-to-memory, or peripheral-to-peripheral. Flow can be controlled by the DMA controller, the source peripheral, or the destination peripheral. Refer to Table 694 for the encoding of this field."]
    #[inline(always)]
    pub fn transfertype(&mut self) -> TransfertypeW<'_, ChannelSpec> {
        TransfertypeW::new(self, 11)
    }
    #[doc = "Bit 14 - Interrupt error mask. When cleared, this bit masks out the error interrupt of the relevant channel."]
    #[inline(always)]
    pub fn ie(&mut self) -> IeW<'_, ChannelSpec> {
        IeW::new(self, 14)
    }
    #[doc = "Bit 15 - Terminal count interrupt mask. When cleared, this bit masks out the terminal count interrupt of the relevant channel."]
    #[inline(always)]
    pub fn itc(&mut self) -> ItcW<'_, ChannelSpec> {
        ItcW::new(self, 15)
    }
    #[doc = "Bit 16 - Lock. When set, this bit enables locked transfers. This information is not used in the LPC178x/177x."]
    #[inline(always)]
    pub fn l(&mut self) -> LW<'_, ChannelSpec> {
        LW::new(self, 16)
    }
    #[doc = "Bit 17 - Active: 0 = there is no data in the FIFO of the channel. 1 = the channel FIFO has data. This value can be used with the Halt and Channel Enable bits to cleanly disable a DMA channel. This is a read-only bit."]
    #[inline(always)]
    pub fn a(&mut self) -> AW<'_, ChannelSpec> {
        AW::new(self, 17)
    }
    #[doc = "Bit 18 - Halt: 0 = enable DMA requests. 1 = ignore further source DMA requests. The contents of the channel FIFO are drained. This value can be used with the Active and Channel Enable bits to cleanly disable a DMA channel."]
    #[inline(always)]
    pub fn h(&mut self) -> HW<'_, ChannelSpec> {
        HW::new(self, 18)
    }
}
#[doc = "DMA Channel 0 Configuration Register\\[1\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`channel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`channel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChannelSpec;
impl crate::RegisterSpec for ChannelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`channel::R`](R) reader structure"]
impl crate::Readable for ChannelSpec {}
#[doc = "`write(|w| ..)` method takes [`channel::W`](W) writer structure"]
impl crate::Writable for ChannelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHANNEL%s to value 0"]
impl crate::Resettable for ChannelSpec {}
