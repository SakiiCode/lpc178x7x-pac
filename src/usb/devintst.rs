#[doc = "Register `DEVINTST` reader"]
pub type R = crate::R<DevintstSpec>;
#[doc = "Field `FRAME` reader - The frame interrupt occurs every 1 ms. This is used in isochronous packet transfers."]
pub type FrameR = crate::BitReader;
#[doc = "Field `EP_FAST` reader - Fast endpoint interrupt. If an Endpoint Interrupt Priority register (USBEpIntPri) bit is set, the corresponding endpoint interrupt will be routed to this bit."]
pub type EpFastR = crate::BitReader;
#[doc = "Field `EP_SLOW` reader - Slow endpoints interrupt. If an Endpoint Interrupt Priority Register (USBEpIntPri) bit is not set, the corresponding endpoint interrupt will be routed to this bit."]
pub type EpSlowR = crate::BitReader;
#[doc = "Field `DEV_STAT` reader - Set when USB Bus reset, USB suspend change or Connect change event occurs. Refer to Section 13.12.6 Set Device Status (Command: 0xFE, Data: write 1 byte) on page 366."]
pub type DevStatR = crate::BitReader;
#[doc = "Field `CCEMPTY` reader - The command code register (USBCmdCode) is empty (New command can be written)."]
pub type CcemptyR = crate::BitReader;
#[doc = "Field `CDFULL` reader - Command data register (USBCmdData) is full (Data can be read now)."]
pub type CdfullR = crate::BitReader;
#[doc = "Field `RxENDPKT` reader - The current packet in the endpoint buffer is transferred to the CPU."]
pub type RxEndpktR = crate::BitReader;
#[doc = "Field `TxENDPKT` reader - The number of data bytes transferred to the endpoint buffer equals the number of bytes programmed in the TxPacket length register (USBTxPLen)."]
pub type TxEndpktR = crate::BitReader;
#[doc = "Field `EP_RLZED` reader - Endpoints realized. Set when Realize Endpoint register (USBReEp) or MaxPacketSize register (USBMaxPSize) is updated and the corresponding operation is completed."]
pub type EpRlzedR = crate::BitReader;
#[doc = "Field `ERR_INT` reader - Error Interrupt. Any bus error interrupt from the USB device. Refer to Section 13.12.9 Read Error Status (Command: 0xFB, Data: read 1 byte) on page 368"]
pub type ErrIntR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The frame interrupt occurs every 1 ms. This is used in isochronous packet transfers."]
    #[inline(always)]
    pub fn frame(&self) -> FrameR {
        FrameR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fast endpoint interrupt. If an Endpoint Interrupt Priority register (USBEpIntPri) bit is set, the corresponding endpoint interrupt will be routed to this bit."]
    #[inline(always)]
    pub fn ep_fast(&self) -> EpFastR {
        EpFastR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Slow endpoints interrupt. If an Endpoint Interrupt Priority Register (USBEpIntPri) bit is not set, the corresponding endpoint interrupt will be routed to this bit."]
    #[inline(always)]
    pub fn ep_slow(&self) -> EpSlowR {
        EpSlowR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set when USB Bus reset, USB suspend change or Connect change event occurs. Refer to Section 13.12.6 Set Device Status (Command: 0xFE, Data: write 1 byte) on page 366."]
    #[inline(always)]
    pub fn dev_stat(&self) -> DevStatR {
        DevStatR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The command code register (USBCmdCode) is empty (New command can be written)."]
    #[inline(always)]
    pub fn ccempty(&self) -> CcemptyR {
        CcemptyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Command data register (USBCmdData) is full (Data can be read now)."]
    #[inline(always)]
    pub fn cdfull(&self) -> CdfullR {
        CdfullR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The current packet in the endpoint buffer is transferred to the CPU."]
    #[inline(always)]
    pub fn rx_endpkt(&self) -> RxEndpktR {
        RxEndpktR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The number of data bytes transferred to the endpoint buffer equals the number of bytes programmed in the TxPacket length register (USBTxPLen)."]
    #[inline(always)]
    pub fn tx_endpkt(&self) -> TxEndpktR {
        TxEndpktR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Endpoints realized. Set when Realize Endpoint register (USBReEp) or MaxPacketSize register (USBMaxPSize) is updated and the corresponding operation is completed."]
    #[inline(always)]
    pub fn ep_rlzed(&self) -> EpRlzedR {
        EpRlzedR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Error Interrupt. Any bus error interrupt from the USB device. Refer to Section 13.12.9 Read Error Status (Command: 0xFB, Data: read 1 byte) on page 368"]
    #[inline(always)]
    pub fn err_int(&self) -> ErrIntR {
        ErrIntR::new(((self.bits >> 9) & 1) != 0)
    }
}
#[doc = "USB Device Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`devintst::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DevintstSpec;
impl crate::RegisterSpec for DevintstSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`devintst::R`](R) reader structure"]
impl crate::Readable for DevintstSpec {}
#[doc = "`reset()` method sets DEVINTST to value 0x10"]
impl crate::Resettable for DevintstSpec {
    const RESET_VALUE: u32 = 0x10;
}
