#[doc = "Register `COMMAND` reader"]
pub type R = crate::R<CommandSpec>;
#[doc = "Register `COMMAND` writer"]
pub type W = crate::W<CommandSpec>;
#[doc = "Field `RXENABLE` reader - Enable receive."]
pub type RxenableR = crate::BitReader;
#[doc = "Field `RXENABLE` writer - Enable receive."]
pub type RxenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXENABLE` reader - Enable transmit."]
pub type TxenableR = crate::BitReader;
#[doc = "Field `TXENABLE` writer - Enable transmit."]
pub type TxenableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGRESET` reader - When a 1 is written, all datapaths and the host registers are reset. The MAC needs to be reset separately."]
pub type RegresetR = crate::BitReader;
#[doc = "Field `REGRESET` writer - When a 1 is written, all datapaths and the host registers are reset. The MAC needs to be reset separately."]
pub type RegresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXRESET` reader - When a 1 is written, the transmit datapath is reset."]
pub type TxresetR = crate::BitReader;
#[doc = "Field `TXRESET` writer - When a 1 is written, the transmit datapath is reset."]
pub type TxresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXRESET` reader - When a 1 is written, the receive datapath is reset."]
pub type RxresetR = crate::BitReader;
#[doc = "Field `RXRESET` writer - When a 1 is written, the receive datapath is reset."]
pub type RxresetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PASSRUNTFRAME` reader - When set to 1 , passes runt frames s1maller than 64 bytes to memory unless they have a CRC error. If 0 runt frames are filtered out."]
pub type PassruntframeR = crate::BitReader;
#[doc = "Field `PASSRUNTFRAME` writer - When set to 1 , passes runt frames s1maller than 64 bytes to memory unless they have a CRC error. If 0 runt frames are filtered out."]
pub type PassruntframeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PASSRXFILTER` reader - When set to 1 , disables receive filtering i.e. all frames received are written to memory."]
pub type PassrxfilterR = crate::BitReader;
#[doc = "Field `PASSRXFILTER` writer - When set to 1 , disables receive filtering i.e. all frames received are written to memory."]
pub type PassrxfilterW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFLOWCONTROL` reader - Enable IEEE 802.3 / clause 31 flow control sending pause frames in full duplex and continuous preamble in half duplex."]
pub type TxflowcontrolR = crate::BitReader;
#[doc = "Field `TXFLOWCONTROL` writer - Enable IEEE 802.3 / clause 31 flow control sending pause frames in full duplex and continuous preamble in half duplex."]
pub type TxflowcontrolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMII` reader - When set to 1 , RMII mode is selected; if 0, MII mode is selected."]
pub type RmiiR = crate::BitReader;
#[doc = "Field `RMII` writer - When set to 1 , RMII mode is selected; if 0, MII mode is selected."]
pub type RmiiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FULLDUPLEX` reader - When set to 1 , indicates full duplex operation."]
pub type FullduplexR = crate::BitReader;
#[doc = "Field `FULLDUPLEX` writer - When set to 1 , indicates full duplex operation."]
pub type FullduplexW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable receive."]
    #[inline(always)]
    pub fn rxenable(&self) -> RxenableR {
        RxenableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable transmit."]
    #[inline(always)]
    pub fn txenable(&self) -> TxenableR {
        TxenableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - When a 1 is written, all datapaths and the host registers are reset. The MAC needs to be reset separately."]
    #[inline(always)]
    pub fn regreset(&self) -> RegresetR {
        RegresetR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When a 1 is written, the transmit datapath is reset."]
    #[inline(always)]
    pub fn txreset(&self) -> TxresetR {
        TxresetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When a 1 is written, the receive datapath is reset."]
    #[inline(always)]
    pub fn rxreset(&self) -> RxresetR {
        RxresetR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When set to 1 , passes runt frames s1maller than 64 bytes to memory unless they have a CRC error. If 0 runt frames are filtered out."]
    #[inline(always)]
    pub fn passruntframe(&self) -> PassruntframeR {
        PassruntframeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When set to 1 , disables receive filtering i.e. all frames received are written to memory."]
    #[inline(always)]
    pub fn passrxfilter(&self) -> PassrxfilterR {
        PassrxfilterR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable IEEE 802.3 / clause 31 flow control sending pause frames in full duplex and continuous preamble in half duplex."]
    #[inline(always)]
    pub fn txflowcontrol(&self) -> TxflowcontrolR {
        TxflowcontrolR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - When set to 1 , RMII mode is selected; if 0, MII mode is selected."]
    #[inline(always)]
    pub fn rmii(&self) -> RmiiR {
        RmiiR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - When set to 1 , indicates full duplex operation."]
    #[inline(always)]
    pub fn fullduplex(&self) -> FullduplexR {
        FullduplexR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable receive."]
    #[inline(always)]
    pub fn rxenable(&mut self) -> RxenableW<'_, CommandSpec> {
        RxenableW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable transmit."]
    #[inline(always)]
    pub fn txenable(&mut self) -> TxenableW<'_, CommandSpec> {
        TxenableW::new(self, 1)
    }
    #[doc = "Bit 3 - When a 1 is written, all datapaths and the host registers are reset. The MAC needs to be reset separately."]
    #[inline(always)]
    pub fn regreset(&mut self) -> RegresetW<'_, CommandSpec> {
        RegresetW::new(self, 3)
    }
    #[doc = "Bit 4 - When a 1 is written, the transmit datapath is reset."]
    #[inline(always)]
    pub fn txreset(&mut self) -> TxresetW<'_, CommandSpec> {
        TxresetW::new(self, 4)
    }
    #[doc = "Bit 5 - When a 1 is written, the receive datapath is reset."]
    #[inline(always)]
    pub fn rxreset(&mut self) -> RxresetW<'_, CommandSpec> {
        RxresetW::new(self, 5)
    }
    #[doc = "Bit 6 - When set to 1 , passes runt frames s1maller than 64 bytes to memory unless they have a CRC error. If 0 runt frames are filtered out."]
    #[inline(always)]
    pub fn passruntframe(&mut self) -> PassruntframeW<'_, CommandSpec> {
        PassruntframeW::new(self, 6)
    }
    #[doc = "Bit 7 - When set to 1 , disables receive filtering i.e. all frames received are written to memory."]
    #[inline(always)]
    pub fn passrxfilter(&mut self) -> PassrxfilterW<'_, CommandSpec> {
        PassrxfilterW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable IEEE 802.3 / clause 31 flow control sending pause frames in full duplex and continuous preamble in half duplex."]
    #[inline(always)]
    pub fn txflowcontrol(&mut self) -> TxflowcontrolW<'_, CommandSpec> {
        TxflowcontrolW::new(self, 8)
    }
    #[doc = "Bit 9 - When set to 1 , RMII mode is selected; if 0, MII mode is selected."]
    #[inline(always)]
    pub fn rmii(&mut self) -> RmiiW<'_, CommandSpec> {
        RmiiW::new(self, 9)
    }
    #[doc = "Bit 10 - When set to 1 , indicates full duplex operation."]
    #[inline(always)]
    pub fn fullduplex(&mut self) -> FullduplexW<'_, CommandSpec> {
        FullduplexW::new(self, 10)
    }
}
#[doc = "Command register.\n\nYou can [`read`](crate::Reg::read) this register and get [`command::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CommandSpec;
impl crate::RegisterSpec for CommandSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`command::R`](R) reader structure"]
impl crate::Readable for CommandSpec {}
#[doc = "`write(|w| ..)` method takes [`command::W`](W) writer structure"]
impl crate::Writable for CommandSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets COMMAND to value 0"]
impl crate::Resettable for CommandSpec {}
