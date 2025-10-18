#[doc = "Register `DAI` reader"]
pub type R = crate::R<DaiSpec>;
#[doc = "Register `DAI` writer"]
pub type W = crate::W<DaiSpec>;
#[doc = "Selects the number of bytes in data as follows:\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wordwidth {
    #[doc = "0: 8-bit data"]
    _8BitData = 0,
    #[doc = "1: 16-bit data"]
    _16BitData = 1,
    #[doc = "3: 32-bit data"]
    _32BitData = 3,
}
impl From<Wordwidth> for u8 {
    #[inline(always)]
    fn from(variant: Wordwidth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wordwidth {
    type Ux = u8;
}
impl crate::IsEnum for Wordwidth {}
#[doc = "Field `WORDWIDTH` reader - Selects the number of bytes in data as follows:"]
pub type WordwidthR = crate::FieldReader<Wordwidth>;
impl WordwidthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wordwidth> {
        match self.bits {
            0 => Some(Wordwidth::_8BitData),
            1 => Some(Wordwidth::_16BitData),
            3 => Some(Wordwidth::_32BitData),
            _ => None,
        }
    }
    #[doc = "8-bit data"]
    #[inline(always)]
    pub fn is_8_bit_data(&self) -> bool {
        *self == Wordwidth::_8BitData
    }
    #[doc = "16-bit data"]
    #[inline(always)]
    pub fn is_16_bit_data(&self) -> bool {
        *self == Wordwidth::_16BitData
    }
    #[doc = "32-bit data"]
    #[inline(always)]
    pub fn is_32_bit_data(&self) -> bool {
        *self == Wordwidth::_32BitData
    }
}
#[doc = "Field `WORDWIDTH` writer - Selects the number of bytes in data as follows:"]
pub type WordwidthW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wordwidth>;
impl<'a, REG> WordwidthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8-bit data"]
    #[inline(always)]
    pub fn _8_bit_data(self) -> &'a mut crate::W<REG> {
        self.variant(Wordwidth::_8BitData)
    }
    #[doc = "16-bit data"]
    #[inline(always)]
    pub fn _16_bit_data(self) -> &'a mut crate::W<REG> {
        self.variant(Wordwidth::_16BitData)
    }
    #[doc = "32-bit data"]
    #[inline(always)]
    pub fn _32_bit_data(self) -> &'a mut crate::W<REG> {
        self.variant(Wordwidth::_32BitData)
    }
}
#[doc = "Field `MONO` reader - When 1, data is of monaural format. When 0, the data is in stereo format."]
pub type MonoR = crate::BitReader;
#[doc = "Field `MONO` writer - When 1, data is of monaural format. When 0, the data is in stereo format."]
pub type MonoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOP` reader - When 1, disables accesses on FIFOs, places the transmit channel in mute mode."]
pub type StopR = crate::BitReader;
#[doc = "Field `STOP` writer - When 1, disables accesses on FIFOs, places the transmit channel in mute mode."]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET` reader - When 1, asynchronously reset the transmit channel and FIFO."]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - When 1, asynchronously reset the transmit channel and FIFO."]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WS_SEL` reader - When 0, the interface is in master mode. When 1, the interface is in slave mode. See Section 34.7.2 for a summary of useful combinations for this bit with RXMODE."]
pub type WsSelR = crate::BitReader;
#[doc = "Field `WS_SEL` writer - When 0, the interface is in master mode. When 1, the interface is in slave mode. See Section 34.7.2 for a summary of useful combinations for this bit with RXMODE."]
pub type WsSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WS_HALFPERIOD` reader - Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31."]
pub type WsHalfperiodR = crate::FieldReader<u16>;
#[doc = "Field `WS_HALFPERIOD` writer - Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31."]
pub type WsHalfperiodW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:1 - Selects the number of bytes in data as follows:"]
    #[inline(always)]
    pub fn wordwidth(&self) -> WordwidthR {
        WordwidthR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - When 1, data is of monaural format. When 0, the data is in stereo format."]
    #[inline(always)]
    pub fn mono(&self) -> MonoR {
        MonoR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When 1, disables accesses on FIFOs, places the transmit channel in mute mode."]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When 1, asynchronously reset the transmit channel and FIFO."]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When 0, the interface is in master mode. When 1, the interface is in slave mode. See Section 34.7.2 for a summary of useful combinations for this bit with RXMODE."]
    #[inline(always)]
    pub fn ws_sel(&self) -> WsSelR {
        WsSelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:14 - Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31."]
    #[inline(always)]
    pub fn ws_halfperiod(&self) -> WsHalfperiodR {
        WsHalfperiodR::new(((self.bits >> 6) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects the number of bytes in data as follows:"]
    #[inline(always)]
    pub fn wordwidth(&mut self) -> WordwidthW<'_, DaiSpec> {
        WordwidthW::new(self, 0)
    }
    #[doc = "Bit 2 - When 1, data is of monaural format. When 0, the data is in stereo format."]
    #[inline(always)]
    pub fn mono(&mut self) -> MonoW<'_, DaiSpec> {
        MonoW::new(self, 2)
    }
    #[doc = "Bit 3 - When 1, disables accesses on FIFOs, places the transmit channel in mute mode."]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, DaiSpec> {
        StopW::new(self, 3)
    }
    #[doc = "Bit 4 - When 1, asynchronously reset the transmit channel and FIFO."]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, DaiSpec> {
        ResetW::new(self, 4)
    }
    #[doc = "Bit 5 - When 0, the interface is in master mode. When 1, the interface is in slave mode. See Section 34.7.2 for a summary of useful combinations for this bit with RXMODE."]
    #[inline(always)]
    pub fn ws_sel(&mut self) -> WsSelW<'_, DaiSpec> {
        WsSelW::new(self, 5)
    }
    #[doc = "Bits 6:14 - Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31."]
    #[inline(always)]
    pub fn ws_halfperiod(&mut self) -> WsHalfperiodW<'_, DaiSpec> {
        WsHalfperiodW::new(self, 6)
    }
}
#[doc = "I2S Digital Audio Input Register. Contains control bits for the I2S receive channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`dai::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dai::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaiSpec;
impl crate::RegisterSpec for DaiSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dai::R`](R) reader structure"]
impl crate::Readable for DaiSpec {}
#[doc = "`write(|w| ..)` method takes [`dai::W`](W) writer structure"]
impl crate::Writable for DaiSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAI to value 0x07e1"]
impl crate::Resettable for DaiSpec {
    const RESET_VALUE: u32 = 0x07e1;
}
