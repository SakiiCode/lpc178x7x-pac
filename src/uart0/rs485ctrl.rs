#[doc = "Register `RS485CTRL` reader"]
pub type R = crate::R<Rs485ctrlSpec>;
#[doc = "Register `RS485CTRL` writer"]
pub type W = crate::W<Rs485ctrlSpec>;
#[doc = "NMM enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nmmen {
    #[doc = "0: RS-485/EIA-485 Normal Multidrop Mode (NMM) is disabled."]
    Disabled = 0,
    #[doc = "1: RS-485/EIA-485 Normal Multidrop Mode (NMM) is enabled. In this mode, an address is detected when a received byte has the parity bit = 1, generating a received data interrupt. See Section 18.6.16 RS-485/EIA-485 modes of operation."]
    Enabled = 1,
}
impl From<Nmmen> for bool {
    #[inline(always)]
    fn from(variant: Nmmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMMEN` reader - NMM enable."]
pub type NmmenR = crate::BitReader<Nmmen>;
impl NmmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Nmmen {
        match self.bits {
            false => Nmmen::Disabled,
            true => Nmmen::Enabled,
        }
    }
    #[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Nmmen::Disabled
    }
    #[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is enabled. In this mode, an address is detected when a received byte has the parity bit = 1, generating a received data interrupt. See Section 18.6.16 RS-485/EIA-485 modes of operation."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Nmmen::Enabled
    }
}
#[doc = "Field `NMMEN` writer - NMM enable."]
pub type NmmenW<'a, REG> = crate::BitWriter<'a, REG, Nmmen>;
impl<'a, REG> NmmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Nmmen::Disabled)
    }
    #[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) is enabled. In this mode, an address is detected when a received byte has the parity bit = 1, generating a received data interrupt. See Section 18.6.16 RS-485/EIA-485 modes of operation."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Nmmen::Enabled)
    }
}
#[doc = "Receiver enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdis {
    #[doc = "0: The receiver is enabled."]
    Enabled = 0,
    #[doc = "1: The receiver is disabled."]
    Disabled = 1,
}
impl From<Rxdis> for bool {
    #[inline(always)]
    fn from(variant: Rxdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDIS` reader - Receiver enable."]
pub type RxdisR = crate::BitReader<Rxdis>;
impl RxdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxdis {
        match self.bits {
            false => Rxdis::Enabled,
            true => Rxdis::Disabled,
        }
    }
    #[doc = "The receiver is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxdis::Enabled
    }
    #[doc = "The receiver is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxdis::Disabled
    }
}
#[doc = "Field `RXDIS` writer - Receiver enable."]
pub type RxdisW<'a, REG> = crate::BitWriter<'a, REG, Rxdis>;
impl<'a, REG> RxdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The receiver is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdis::Enabled)
    }
    #[doc = "The receiver is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdis::Disabled)
    }
}
#[doc = "AAD enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aaden {
    #[doc = "0: Auto Address Detect (AAD) is disabled."]
    Disabled = 0,
    #[doc = "1: Auto Address Detect (AAD) is enabled."]
    Enabled = 1,
}
impl From<Aaden> for bool {
    #[inline(always)]
    fn from(variant: Aaden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AADEN` reader - AAD enable."]
pub type AadenR = crate::BitReader<Aaden>;
impl AadenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Aaden {
        match self.bits {
            false => Aaden::Disabled,
            true => Aaden::Enabled,
        }
    }
    #[doc = "Auto Address Detect (AAD) is disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Aaden::Disabled
    }
    #[doc = "Auto Address Detect (AAD) is enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Aaden::Enabled
    }
}
#[doc = "Field `AADEN` writer - AAD enable."]
pub type AadenW<'a, REG> = crate::BitWriter<'a, REG, Aaden>;
impl<'a, REG> AadenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Auto Address Detect (AAD) is disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Aaden::Disabled)
    }
    #[doc = "Auto Address Detect (AAD) is enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Aaden::Enabled)
    }
}
#[doc = "Direction control enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dctrl {
    #[doc = "0: Disable Auto Direction Control."]
    DisableAutoDirecti = 0,
    #[doc = "1: Enable Auto Direction Control."]
    EnableAutoDirectio = 1,
}
impl From<Dctrl> for bool {
    #[inline(always)]
    fn from(variant: Dctrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCTRL` reader - Direction control enable."]
pub type DctrlR = crate::BitReader<Dctrl>;
impl DctrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dctrl {
        match self.bits {
            false => Dctrl::DisableAutoDirecti,
            true => Dctrl::EnableAutoDirectio,
        }
    }
    #[doc = "Disable Auto Direction Control."]
    #[inline(always)]
    pub fn is_disable_auto_directi(&self) -> bool {
        *self == Dctrl::DisableAutoDirecti
    }
    #[doc = "Enable Auto Direction Control."]
    #[inline(always)]
    pub fn is_enable_auto_directio(&self) -> bool {
        *self == Dctrl::EnableAutoDirectio
    }
}
#[doc = "Field `DCTRL` writer - Direction control enable."]
pub type DctrlW<'a, REG> = crate::BitWriter<'a, REG, Dctrl>;
impl<'a, REG> DctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Auto Direction Control."]
    #[inline(always)]
    pub fn disable_auto_directi(self) -> &'a mut crate::W<REG> {
        self.variant(Dctrl::DisableAutoDirecti)
    }
    #[doc = "Enable Auto Direction Control."]
    #[inline(always)]
    pub fn enable_auto_directio(self) -> &'a mut crate::W<REG> {
        self.variant(Dctrl::EnableAutoDirectio)
    }
}
#[doc = "Direction control pin polarity. This bit reverses the polarity of the direction control signal on the Un_OE pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oinv {
    #[doc = "0: The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    Dirlow = 0,
    #[doc = "1: The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    Dirhigh = 1,
}
impl From<Oinv> for bool {
    #[inline(always)]
    fn from(variant: Oinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OINV` reader - Direction control pin polarity. This bit reverses the polarity of the direction control signal on the Un_OE pin."]
pub type OinvR = crate::BitReader<Oinv>;
impl OinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oinv {
        match self.bits {
            false => Oinv::Dirlow,
            true => Oinv::Dirhigh,
        }
    }
    #[doc = "The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    #[inline(always)]
    pub fn is_dirlow(&self) -> bool {
        *self == Oinv::Dirlow
    }
    #[doc = "The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    #[inline(always)]
    pub fn is_dirhigh(&self) -> bool {
        *self == Oinv::Dirhigh
    }
}
#[doc = "Field `OINV` writer - Direction control pin polarity. This bit reverses the polarity of the direction control signal on the Un_OE pin."]
pub type OinvW<'a, REG> = crate::BitWriter<'a, REG, Oinv>;
impl<'a, REG> OinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    #[inline(always)]
    pub fn dirlow(self) -> &'a mut crate::W<REG> {
        self.variant(Oinv::Dirlow)
    }
    #[doc = "The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    #[inline(always)]
    pub fn dirhigh(self) -> &'a mut crate::W<REG> {
        self.variant(Oinv::Dirhigh)
    }
}
impl R {
    #[doc = "Bit 0 - NMM enable."]
    #[inline(always)]
    pub fn nmmen(&self) -> NmmenR {
        NmmenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receiver enable."]
    #[inline(always)]
    pub fn rxdis(&self) -> RxdisR {
        RxdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AAD enable."]
    #[inline(always)]
    pub fn aaden(&self) -> AadenR {
        AadenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Direction control enable."]
    #[inline(always)]
    pub fn dctrl(&self) -> DctrlR {
        DctrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Direction control pin polarity. This bit reverses the polarity of the direction control signal on the Un_OE pin."]
    #[inline(always)]
    pub fn oinv(&self) -> OinvR {
        OinvR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - NMM enable."]
    #[inline(always)]
    pub fn nmmen(&mut self) -> NmmenW<'_, Rs485ctrlSpec> {
        NmmenW::new(self, 0)
    }
    #[doc = "Bit 1 - Receiver enable."]
    #[inline(always)]
    pub fn rxdis(&mut self) -> RxdisW<'_, Rs485ctrlSpec> {
        RxdisW::new(self, 1)
    }
    #[doc = "Bit 2 - AAD enable."]
    #[inline(always)]
    pub fn aaden(&mut self) -> AadenW<'_, Rs485ctrlSpec> {
        AadenW::new(self, 2)
    }
    #[doc = "Bit 4 - Direction control enable."]
    #[inline(always)]
    pub fn dctrl(&mut self) -> DctrlW<'_, Rs485ctrlSpec> {
        DctrlW::new(self, 4)
    }
    #[doc = "Bit 5 - Direction control pin polarity. This bit reverses the polarity of the direction control signal on the Un_OE pin."]
    #[inline(always)]
    pub fn oinv(&mut self) -> OinvW<'_, Rs485ctrlSpec> {
        OinvW::new(self, 5)
    }
}
#[doc = "RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`rs485ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rs485ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rs485ctrlSpec;
impl crate::RegisterSpec for Rs485ctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rs485ctrl::R`](R) reader structure"]
impl crate::Readable for Rs485ctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rs485ctrl::W`](W) writer structure"]
impl crate::Writable for Rs485ctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RS485CTRL to value 0"]
impl crate::Resettable for Rs485ctrlSpec {}
