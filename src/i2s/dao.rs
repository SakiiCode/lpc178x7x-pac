#[doc = "Register `DAO` reader"]
pub type R = crate::R<DaoSpec>;
#[doc = "Register `DAO` writer"]
pub type W = crate::W<DaoSpec>;
#[doc = "Selects the number of bytes in data as follows:\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wordwidth {
    #[doc = "0: 8-bit data"]
    _8Bit = 0,
    #[doc = "1: 16-bit data"]
    _16Bit = 1,
    #[doc = "3: 32-bit data"]
    _32Bit = 3,
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
            0 => Some(Wordwidth::_8Bit),
            1 => Some(Wordwidth::_16Bit),
            3 => Some(Wordwidth::_32Bit),
            _ => None,
        }
    }
    #[doc = "8-bit data"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == Wordwidth::_8Bit
    }
    #[doc = "16-bit data"]
    #[inline(always)]
    pub fn is_16_bit(&self) -> bool {
        *self == Wordwidth::_16Bit
    }
    #[doc = "32-bit data"]
    #[inline(always)]
    pub fn is_32_bit(&self) -> bool {
        *self == Wordwidth::_32Bit
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
    pub fn _8_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Wordwidth::_8Bit)
    }
    #[doc = "16-bit data"]
    #[inline(always)]
    pub fn _16_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Wordwidth::_16Bit)
    }
    #[doc = "32-bit data"]
    #[inline(always)]
    pub fn _32_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Wordwidth::_32Bit)
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
#[doc = "Field `RESET` reader - When 1, asynchronously resets the transmit channel and FIFO."]
pub type ResetR = crate::BitReader;
#[doc = "Field `RESET` writer - When 1, asynchronously resets the transmit channel and FIFO."]
pub type ResetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WS_SEL` reader - When 0, the interface is in master mode. When 1, the interface is in slave mode. See Section 34.7.2 for a summary of useful combinations for this bit with TXMODE."]
pub type WsSelR = crate::BitReader;
#[doc = "Field `WS_SEL` writer - When 0, the interface is in master mode. When 1, the interface is in slave mode. See Section 34.7.2 for a summary of useful combinations for this bit with TXMODE."]
pub type WsSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WS_HALFPERIOD` reader - Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31."]
pub type WsHalfperiodR = crate::FieldReader<u16>;
#[doc = "Field `WS_HALFPERIOD` writer - Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31."]
pub type WsHalfperiodW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `MUTE` reader - When 1, the transmit channel sends only zeroes."]
pub type MuteR = crate::BitReader;
#[doc = "Field `MUTE` writer - When 1, the transmit channel sends only zeroes."]
pub type MuteW<'a, REG> = crate::BitWriter<'a, REG>;
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
    #[doc = "Bit 4 - When 1, asynchronously resets the transmit channel and FIFO."]
    #[inline(always)]
    pub fn reset(&self) -> ResetR {
        ResetR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When 0, the interface is in master mode. When 1, the interface is in slave mode. See Section 34.7.2 for a summary of useful combinations for this bit with TXMODE."]
    #[inline(always)]
    pub fn ws_sel(&self) -> WsSelR {
        WsSelR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:14 - Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31."]
    #[inline(always)]
    pub fn ws_halfperiod(&self) -> WsHalfperiodR {
        WsHalfperiodR::new(((self.bits >> 6) & 0x01ff) as u16)
    }
    #[doc = "Bit 15 - When 1, the transmit channel sends only zeroes."]
    #[inline(always)]
    pub fn mute(&self) -> MuteR {
        MuteR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Selects the number of bytes in data as follows:"]
    #[inline(always)]
    pub fn wordwidth(&mut self) -> WordwidthW<'_, DaoSpec> {
        WordwidthW::new(self, 0)
    }
    #[doc = "Bit 2 - When 1, data is of monaural format. When 0, the data is in stereo format."]
    #[inline(always)]
    pub fn mono(&mut self) -> MonoW<'_, DaoSpec> {
        MonoW::new(self, 2)
    }
    #[doc = "Bit 3 - When 1, disables accesses on FIFOs, places the transmit channel in mute mode."]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<'_, DaoSpec> {
        StopW::new(self, 3)
    }
    #[doc = "Bit 4 - When 1, asynchronously resets the transmit channel and FIFO."]
    #[inline(always)]
    pub fn reset(&mut self) -> ResetW<'_, DaoSpec> {
        ResetW::new(self, 4)
    }
    #[doc = "Bit 5 - When 0, the interface is in master mode. When 1, the interface is in slave mode. See Section 34.7.2 for a summary of useful combinations for this bit with TXMODE."]
    #[inline(always)]
    pub fn ws_sel(&mut self) -> WsSelW<'_, DaoSpec> {
        WsSelW::new(self, 5)
    }
    #[doc = "Bits 6:14 - Word select half period minus 1, i.e. WS 64clk period -> ws_halfperiod = 31."]
    #[inline(always)]
    pub fn ws_halfperiod(&mut self) -> WsHalfperiodW<'_, DaoSpec> {
        WsHalfperiodW::new(self, 6)
    }
    #[doc = "Bit 15 - When 1, the transmit channel sends only zeroes."]
    #[inline(always)]
    pub fn mute(&mut self) -> MuteW<'_, DaoSpec> {
        MuteW::new(self, 15)
    }
}
#[doc = "I2S Digital Audio Output Register. Contains control bits for the I2S transmit channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`dao::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dao::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DaoSpec;
impl crate::RegisterSpec for DaoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dao::R`](R) reader structure"]
impl crate::Readable for DaoSpec {}
#[doc = "`write(|w| ..)` method takes [`dao::W`](W) writer structure"]
impl crate::Writable for DaoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DAO to value 0x87e1"]
impl crate::Resettable for DaoSpec {
    const RESET_VALUE: u32 = 0x87e1;
}
