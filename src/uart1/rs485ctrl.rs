#[doc = "Register `RS485CTRL` reader"]
pub type R = crate::R<Rs485ctrlSpec>;
#[doc = "Register `RS485CTRL` writer"]
pub type W = crate::W<Rs485ctrlSpec>;
#[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) mode select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disabled."]
    Disabled_ = 0,
    #[doc = "1: Enabled. In this mode, an address is detected when a received byte causes the UART to set the parity error and generate an interrupt."]
    EnabledInThisMod = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMMEN` reader - RS-485/EIA-485 Normal Multidrop Mode (NMM) mode select."]
pub type NmmenR = crate::BitReader<Enum>;
impl NmmenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disabled_,
            true => Enum::EnabledInThisMod,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == Enum::Disabled_
    }
    #[doc = "Enabled. In this mode, an address is detected when a received byte causes the UART to set the parity error and generate an interrupt."]
    #[inline(always)]
    pub fn is_enabled_in_this_mod(&self) -> bool {
        *self == Enum::EnabledInThisMod
    }
}
#[doc = "Field `NMMEN` writer - RS-485/EIA-485 Normal Multidrop Mode (NMM) mode select."]
pub type NmmenW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> NmmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled_)
    }
    #[doc = "Enabled. In this mode, an address is detected when a received byte causes the UART to set the parity error and generate an interrupt."]
    #[inline(always)]
    pub fn enabled_in_this_mod(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::EnabledInThisMod)
    }
}
#[doc = "Receive enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Enabled."]
    Enabled_ = 0,
    #[doc = "1: Disabled."]
    Disabled_ = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDIS` reader - Receive enable."]
pub type RxdisR = crate::BitReader<Enum>;
impl RxdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Enabled_,
            true => Enum::Disabled_,
        }
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled_(&self) -> bool {
        *self == Enum::Enabled_
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == Enum::Disabled_
    }
}
#[doc = "Field `RXDIS` writer - Receive enable."]
pub type RxdisW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> RxdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Enabled_)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled_)
    }
}
#[doc = "Auto Address Detect (AAD) enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disabled."]
    Disabled_ = 0,
    #[doc = "1: Enabled."]
    Enabled_ = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AADEN` reader - Auto Address Detect (AAD) enable."]
pub type AadenR = crate::BitReader<Enum>;
impl AadenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Disabled_,
            true => Enum::Enabled_,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == Enum::Disabled_
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled_(&self) -> bool {
        *self == Enum::Enabled_
    }
}
#[doc = "Field `AADEN` writer - Auto Address Detect (AAD) enable."]
pub type AadenW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> AadenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Disabled_)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Enabled_)
    }
}
#[doc = "Direction control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: RTS. If direction control is enabled (bit DCTRL = 1), pin RTS is used for direction control."]
    RtsIfDirectionCo = 0,
    #[doc = "1: DTR. If direction control is enabled (bit DCTRL = 1), pin DTR is used for direction control."]
    DtrIfDirectionCo = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEL` reader - Direction control."]
pub type SelR = crate::BitReader<Enum>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::RtsIfDirectionCo,
            true => Enum::DtrIfDirectionCo,
        }
    }
    #[doc = "RTS. If direction control is enabled (bit DCTRL = 1), pin RTS is used for direction control."]
    #[inline(always)]
    pub fn is_rts_if_direction_co(&self) -> bool {
        *self == Enum::RtsIfDirectionCo
    }
    #[doc = "DTR. If direction control is enabled (bit DCTRL = 1), pin DTR is used for direction control."]
    #[inline(always)]
    pub fn is_dtr_if_direction_co(&self) -> bool {
        *self == Enum::DtrIfDirectionCo
    }
}
#[doc = "Field `SEL` writer - Direction control."]
pub type SelW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTS. If direction control is enabled (bit DCTRL = 1), pin RTS is used for direction control."]
    #[inline(always)]
    pub fn rts_if_direction_co(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::RtsIfDirectionCo)
    }
    #[doc = "DTR. If direction control is enabled (bit DCTRL = 1), pin DTR is used for direction control."]
    #[inline(always)]
    pub fn dtr_if_direction_co(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::DtrIfDirectionCo)
    }
}
#[doc = "Direction control enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Disable Auto Direction Control."]
    DisableAutoDirecti = 0,
    #[doc = "1: Enable Auto Direction Control."]
    EnableAutoDirectio = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCTRL` reader - Direction control enable."]
pub type DctrlR = crate::BitReader<Enum>;
impl DctrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::DisableAutoDirecti,
            true => Enum::EnableAutoDirectio,
        }
    }
    #[doc = "Disable Auto Direction Control."]
    #[inline(always)]
    pub fn is_disable_auto_directi(&self) -> bool {
        *self == Enum::DisableAutoDirecti
    }
    #[doc = "Enable Auto Direction Control."]
    #[inline(always)]
    pub fn is_enable_auto_directio(&self) -> bool {
        *self == Enum::EnableAutoDirectio
    }
}
#[doc = "Field `DCTRL` writer - Direction control enable."]
pub type DctrlW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> DctrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable Auto Direction Control."]
    #[inline(always)]
    pub fn disable_auto_directi(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::DisableAutoDirecti)
    }
    #[doc = "Enable Auto Direction Control."]
    #[inline(always)]
    pub fn enable_auto_directio(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::EnableAutoDirectio)
    }
}
#[doc = "Polarity. This bit reverses the polarity of the direction control signal on the RTS (or DTR) pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: LOW. The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    LowTheDirectionC = 0,
    #[doc = "1: HIGH. The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    HighTheDirection_ = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OINV` reader - Polarity. This bit reverses the polarity of the direction control signal on the RTS (or DTR) pin."]
pub type OinvR = crate::BitReader<Enum>;
impl OinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::LowTheDirectionC,
            true => Enum::HighTheDirection_,
        }
    }
    #[doc = "LOW. The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    #[inline(always)]
    pub fn is_low_the_direction_c(&self) -> bool {
        *self == Enum::LowTheDirectionC
    }
    #[doc = "HIGH. The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    #[inline(always)]
    pub fn is_high_the_direction_(&self) -> bool {
        *self == Enum::HighTheDirection_
    }
}
#[doc = "Field `OINV` writer - Polarity. This bit reverses the polarity of the direction control signal on the RTS (or DTR) pin."]
pub type OinvW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> OinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LOW. The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    #[inline(always)]
    pub fn low_the_direction_c(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::LowTheDirectionC)
    }
    #[doc = "HIGH. The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    #[inline(always)]
    pub fn high_the_direction_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::HighTheDirection_)
    }
}
impl R {
    #[doc = "Bit 0 - RS-485/EIA-485 Normal Multidrop Mode (NMM) mode select."]
    #[inline(always)]
    pub fn nmmen(&self) -> NmmenR {
        NmmenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive enable."]
    #[inline(always)]
    pub fn rxdis(&self) -> RxdisR {
        RxdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Auto Address Detect (AAD) enable."]
    #[inline(always)]
    pub fn aaden(&self) -> AadenR {
        AadenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Direction control."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Direction control enable."]
    #[inline(always)]
    pub fn dctrl(&self) -> DctrlR {
        DctrlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Polarity. This bit reverses the polarity of the direction control signal on the RTS (or DTR) pin."]
    #[inline(always)]
    pub fn oinv(&self) -> OinvR {
        OinvR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RS-485/EIA-485 Normal Multidrop Mode (NMM) mode select."]
    #[inline(always)]
    pub fn nmmen(&mut self) -> NmmenW<'_, Rs485ctrlSpec> {
        NmmenW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive enable."]
    #[inline(always)]
    pub fn rxdis(&mut self) -> RxdisW<'_, Rs485ctrlSpec> {
        RxdisW::new(self, 1)
    }
    #[doc = "Bit 2 - Auto Address Detect (AAD) enable."]
    #[inline(always)]
    pub fn aaden(&mut self) -> AadenW<'_, Rs485ctrlSpec> {
        AadenW::new(self, 2)
    }
    #[doc = "Bit 3 - Direction control."]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<'_, Rs485ctrlSpec> {
        SelW::new(self, 3)
    }
    #[doc = "Bit 4 - Direction control enable."]
    #[inline(always)]
    pub fn dctrl(&mut self) -> DctrlW<'_, Rs485ctrlSpec> {
        DctrlW::new(self, 4)
    }
    #[doc = "Bit 5 - Polarity. This bit reverses the polarity of the direction control signal on the RTS (or DTR) pin."]
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
