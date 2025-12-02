#[doc = "Register `SYNCCTRL` reader"]
pub type R = crate::R<SyncctrlSpec>;
#[doc = "Register `SYNCCTRL` writer"]
pub type W = crate::W<SyncctrlSpec>;
#[doc = "Enables synchronous mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sync {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Sync> for bool {
    #[inline(always)]
    fn from(variant: Sync) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SYNC` reader - Enables synchronous mode."]
pub type SyncR = crate::BitReader<Sync>;
impl SyncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sync {
        match self.bits {
            false => Sync::Disabled,
            true => Sync::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sync::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Sync::Enabled
    }
}
#[doc = "Field `SYNC` writer - Enables synchronous mode."]
pub type SyncW<'a, REG> = crate::BitWriter<'a, REG, Sync>;
impl<'a, REG> SyncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sync::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sync::Enabled)
    }
}
#[doc = "Clock source select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Csrc {
    #[doc = "0: Synchronous slave mode (SCLK in)"]
    Slave = 0,
    #[doc = "1: Synchronous master mode (SCLK out)"]
    Master = 1,
}
impl From<Csrc> for bool {
    #[inline(always)]
    fn from(variant: Csrc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSRC` reader - Clock source select."]
pub type CsrcR = crate::BitReader<Csrc>;
impl CsrcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Csrc {
        match self.bits {
            false => Csrc::Slave,
            true => Csrc::Master,
        }
    }
    #[doc = "Synchronous slave mode (SCLK in)"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == Csrc::Slave
    }
    #[doc = "Synchronous master mode (SCLK out)"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == Csrc::Master
    }
}
#[doc = "Field `CSRC` writer - Clock source select."]
pub type CsrcW<'a, REG> = crate::BitWriter<'a, REG, Csrc>;
impl<'a, REG> CsrcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Synchronous slave mode (SCLK in)"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut crate::W<REG> {
        self.variant(Csrc::Slave)
    }
    #[doc = "Synchronous master mode (SCLK out)"]
    #[inline(always)]
    pub fn master(self) -> &'a mut crate::W<REG> {
        self.variant(Csrc::Master)
    }
}
#[doc = "Falling edge sampling.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fes {
    #[doc = "0: RxD is sampled on the rising edge of SCLK"]
    Rising = 0,
    #[doc = "1: RxD is sampled on the falling edge of SCLK"]
    Falling = 1,
}
impl From<Fes> for bool {
    #[inline(always)]
    fn from(variant: Fes) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FES` reader - Falling edge sampling."]
pub type FesR = crate::BitReader<Fes>;
impl FesR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fes {
        match self.bits {
            false => Fes::Rising,
            true => Fes::Falling,
        }
    }
    #[doc = "RxD is sampled on the rising edge of SCLK"]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Fes::Rising
    }
    #[doc = "RxD is sampled on the falling edge of SCLK"]
    #[inline(always)]
    pub fn is_falling(&self) -> bool {
        *self == Fes::Falling
    }
}
#[doc = "Field `FES` writer - Falling edge sampling."]
pub type FesW<'a, REG> = crate::BitWriter<'a, REG, Fes>;
impl<'a, REG> FesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RxD is sampled on the rising edge of SCLK"]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Fes::Rising)
    }
    #[doc = "RxD is sampled on the falling edge of SCLK"]
    #[inline(always)]
    pub fn falling(self) -> &'a mut crate::W<REG> {
        self.variant(Fes::Falling)
    }
}
#[doc = "Transmit synchronization bypass in synchronous slave mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tsbypass {
    #[doc = "0: The input clock is synchronized prior to being used in clock edge detection logic."]
    NoBypass = 0,
    #[doc = "1: The input clock is not synchronized prior to being used in clock edge detection logic. This allows for a high er input clock rate at the expense of potential metastability."]
    Bypass = 1,
}
impl From<Tsbypass> for bool {
    #[inline(always)]
    fn from(variant: Tsbypass) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TSBYPASS` reader - Transmit synchronization bypass in synchronous slave mode."]
pub type TsbypassR = crate::BitReader<Tsbypass>;
impl TsbypassR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Tsbypass {
        match self.bits {
            false => Tsbypass::NoBypass,
            true => Tsbypass::Bypass,
        }
    }
    #[doc = "The input clock is synchronized prior to being used in clock edge detection logic."]
    #[inline(always)]
    pub fn is_no_bypass(&self) -> bool {
        *self == Tsbypass::NoBypass
    }
    #[doc = "The input clock is not synchronized prior to being used in clock edge detection logic. This allows for a high er input clock rate at the expense of potential metastability."]
    #[inline(always)]
    pub fn is_bypass(&self) -> bool {
        *self == Tsbypass::Bypass
    }
}
#[doc = "Field `TSBYPASS` writer - Transmit synchronization bypass in synchronous slave mode."]
pub type TsbypassW<'a, REG> = crate::BitWriter<'a, REG, Tsbypass>;
impl<'a, REG> TsbypassW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The input clock is synchronized prior to being used in clock edge detection logic."]
    #[inline(always)]
    pub fn no_bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Tsbypass::NoBypass)
    }
    #[doc = "The input clock is not synchronized prior to being used in clock edge detection logic. This allows for a high er input clock rate at the expense of potential metastability."]
    #[inline(always)]
    pub fn bypass(self) -> &'a mut crate::W<REG> {
        self.variant(Tsbypass::Bypass)
    }
}
#[doc = "Continuous master clock enable (used only when CSRC is 1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cscen {
    #[doc = "0: SCLK cycles only when characters are being sent on TxD"]
    SendOnly = 0,
    #[doc = "1: SCLK runs continuously (characters can be received on RxD independently from transmission on TxD)"]
    Continuous = 1,
}
impl From<Cscen> for bool {
    #[inline(always)]
    fn from(variant: Cscen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CSCEN` reader - Continuous master clock enable (used only when CSRC is 1)"]
pub type CscenR = crate::BitReader<Cscen>;
impl CscenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cscen {
        match self.bits {
            false => Cscen::SendOnly,
            true => Cscen::Continuous,
        }
    }
    #[doc = "SCLK cycles only when characters are being sent on TxD"]
    #[inline(always)]
    pub fn is_send_only(&self) -> bool {
        *self == Cscen::SendOnly
    }
    #[doc = "SCLK runs continuously (characters can be received on RxD independently from transmission on TxD)"]
    #[inline(always)]
    pub fn is_continuous(&self) -> bool {
        *self == Cscen::Continuous
    }
}
#[doc = "Field `CSCEN` writer - Continuous master clock enable (used only when CSRC is 1)"]
pub type CscenW<'a, REG> = crate::BitWriter<'a, REG, Cscen>;
impl<'a, REG> CscenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SCLK cycles only when characters are being sent on TxD"]
    #[inline(always)]
    pub fn send_only(self) -> &'a mut crate::W<REG> {
        self.variant(Cscen::SendOnly)
    }
    #[doc = "SCLK runs continuously (characters can be received on RxD independently from transmission on TxD)"]
    #[inline(always)]
    pub fn continuous(self) -> &'a mut crate::W<REG> {
        self.variant(Cscen::Continuous)
    }
}
#[doc = "Start/stop bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sssdis {
    #[doc = "0: Send start and stop bits as in other modes."]
    Send = 0,
    #[doc = "1: Do not send start/stop bits."]
    NoSend = 1,
}
impl From<Sssdis> for bool {
    #[inline(always)]
    fn from(variant: Sssdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SSSDIS` reader - Start/stop bits"]
pub type SssdisR = crate::BitReader<Sssdis>;
impl SssdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sssdis {
        match self.bits {
            false => Sssdis::Send,
            true => Sssdis::NoSend,
        }
    }
    #[doc = "Send start and stop bits as in other modes."]
    #[inline(always)]
    pub fn is_send(&self) -> bool {
        *self == Sssdis::Send
    }
    #[doc = "Do not send start/stop bits."]
    #[inline(always)]
    pub fn is_no_send(&self) -> bool {
        *self == Sssdis::NoSend
    }
}
#[doc = "Field `SSSDIS` writer - Start/stop bits"]
pub type SssdisW<'a, REG> = crate::BitWriter<'a, REG, Sssdis>;
impl<'a, REG> SssdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Send start and stop bits as in other modes."]
    #[inline(always)]
    pub fn send(self) -> &'a mut crate::W<REG> {
        self.variant(Sssdis::Send)
    }
    #[doc = "Do not send start/stop bits."]
    #[inline(always)]
    pub fn no_send(self) -> &'a mut crate::W<REG> {
        self.variant(Sssdis::NoSend)
    }
}
#[doc = "Continuous clock clear\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ccclr {
    #[doc = "0: CSCEN is under software control."]
    Disabled = 0,
    #[doc = "1: Hardware clears CSCEN after each character is received."]
    Enabled = 1,
}
impl From<Ccclr> for bool {
    #[inline(always)]
    fn from(variant: Ccclr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCCLR` reader - Continuous clock clear"]
pub type CcclrR = crate::BitReader<Ccclr>;
impl CcclrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ccclr {
        match self.bits {
            false => Ccclr::Disabled,
            true => Ccclr::Enabled,
        }
    }
    #[doc = "CSCEN is under software control."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ccclr::Disabled
    }
    #[doc = "Hardware clears CSCEN after each character is received."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ccclr::Enabled
    }
}
#[doc = "Field `CCCLR` writer - Continuous clock clear"]
pub type CcclrW<'a, REG> = crate::BitWriter<'a, REG, Ccclr>;
impl<'a, REG> CcclrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "CSCEN is under software control."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ccclr::Disabled)
    }
    #[doc = "Hardware clears CSCEN after each character is received."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ccclr::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enables synchronous mode."]
    #[inline(always)]
    pub fn sync(&self) -> SyncR {
        SyncR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock source select."]
    #[inline(always)]
    pub fn csrc(&self) -> CsrcR {
        CsrcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Falling edge sampling."]
    #[inline(always)]
    pub fn fes(&self) -> FesR {
        FesR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmit synchronization bypass in synchronous slave mode."]
    #[inline(always)]
    pub fn tsbypass(&self) -> TsbypassR {
        TsbypassR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Continuous master clock enable (used only when CSRC is 1)"]
    #[inline(always)]
    pub fn cscen(&self) -> CscenR {
        CscenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Start/stop bits"]
    #[inline(always)]
    pub fn sssdis(&self) -> SssdisR {
        SssdisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Continuous clock clear"]
    #[inline(always)]
    pub fn ccclr(&self) -> CcclrR {
        CcclrR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables synchronous mode."]
    #[inline(always)]
    pub fn sync(&mut self) -> SyncW<'_, SyncctrlSpec> {
        SyncW::new(self, 0)
    }
    #[doc = "Bit 1 - Clock source select."]
    #[inline(always)]
    pub fn csrc(&mut self) -> CsrcW<'_, SyncctrlSpec> {
        CsrcW::new(self, 1)
    }
    #[doc = "Bit 2 - Falling edge sampling."]
    #[inline(always)]
    pub fn fes(&mut self) -> FesW<'_, SyncctrlSpec> {
        FesW::new(self, 2)
    }
    #[doc = "Bit 3 - Transmit synchronization bypass in synchronous slave mode."]
    #[inline(always)]
    pub fn tsbypass(&mut self) -> TsbypassW<'_, SyncctrlSpec> {
        TsbypassW::new(self, 3)
    }
    #[doc = "Bit 4 - Continuous master clock enable (used only when CSRC is 1)"]
    #[inline(always)]
    pub fn cscen(&mut self) -> CscenW<'_, SyncctrlSpec> {
        CscenW::new(self, 4)
    }
    #[doc = "Bit 5 - Start/stop bits"]
    #[inline(always)]
    pub fn sssdis(&mut self) -> SssdisW<'_, SyncctrlSpec> {
        SssdisW::new(self, 5)
    }
    #[doc = "Bit 6 - Continuous clock clear"]
    #[inline(always)]
    pub fn ccclr(&mut self) -> CcclrW<'_, SyncctrlSpec> {
        CcclrW::new(self, 6)
    }
}
#[doc = "Synchronous mode control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`syncctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syncctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyncctrlSpec;
impl crate::RegisterSpec for SyncctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syncctrl::R`](R) reader structure"]
impl crate::Readable for SyncctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`syncctrl::W`](W) writer structure"]
impl crate::Writable for SyncctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYNCCTRL to value 0"]
impl crate::Resettable for SyncctrlSpec {}
