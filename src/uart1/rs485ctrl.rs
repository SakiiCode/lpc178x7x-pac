#[doc = "Register `RS485CTRL` reader"]
pub type R = crate::R<Rs485ctrlSpec>;
#[doc = "Register `RS485CTRL` writer"]
pub type W = crate::W<Rs485ctrlSpec>;
#[doc = "RS-485/EIA-485 Normal Multidrop Mode (NMM) mode select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Nmmen {
    #[doc = "0: Disabled."]
    Disabled = 0,
    #[doc = "1: Enabled. In this mode, an address is detected when a received byte causes the UART to set the parity error and generate an interrupt."]
    Enabled = 1,
}
impl From<Nmmen> for bool {
    #[inline(always)]
    fn from(variant: Nmmen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NMMEN` reader - RS-485/EIA-485 Normal Multidrop Mode (NMM) mode select."]
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
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Nmmen::Disabled
    }
    #[doc = "Enabled. In this mode, an address is detected when a received byte causes the UART to set the parity error and generate an interrupt."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Nmmen::Enabled
    }
}
#[doc = "Field `NMMEN` writer - RS-485/EIA-485 Normal Multidrop Mode (NMM) mode select."]
pub type NmmenW<'a, REG> = crate::BitWriter<'a, REG, Nmmen>;
impl<'a, REG> NmmenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Nmmen::Disabled)
    }
    #[doc = "Enabled. In this mode, an address is detected when a received byte causes the UART to set the parity error and generate an interrupt."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Nmmen::Enabled)
    }
}
#[doc = "Receive enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdis {
    #[doc = "0: Enabled."]
    Enabled = 0,
    #[doc = "1: Disabled."]
    Disabled = 1,
}
impl From<Rxdis> for bool {
    #[inline(always)]
    fn from(variant: Rxdis) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDIS` reader - Receive enable."]
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
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxdis::Enabled
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxdis::Disabled
    }
}
#[doc = "Field `RXDIS` writer - Receive enable."]
pub type RxdisW<'a, REG> = crate::BitWriter<'a, REG, Rxdis>;
impl<'a, REG> RxdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdis::Enabled)
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxdis::Disabled)
    }
}
#[doc = "Auto Address Detect (AAD) enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Aaden {
    #[doc = "0: Disabled."]
    Disabled = 0,
    #[doc = "1: Enabled."]
    Enabled = 1,
}
impl From<Aaden> for bool {
    #[inline(always)]
    fn from(variant: Aaden) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AADEN` reader - Auto Address Detect (AAD) enable."]
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
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Aaden::Disabled
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Aaden::Enabled
    }
}
#[doc = "Field `AADEN` writer - Auto Address Detect (AAD) enable."]
pub type AadenW<'a, REG> = crate::BitWriter<'a, REG, Aaden>;
impl<'a, REG> AadenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Aaden::Disabled)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Aaden::Enabled)
    }
}
#[doc = "Direction control.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sel {
    #[doc = "0: RTS. If direction control is enabled (bit DCTRL = 1), pin RTS is used for direction control."]
    Rts = 0,
    #[doc = "1: DTR. If direction control is enabled (bit DCTRL = 1), pin DTR is used for direction control."]
    Dtr = 1,
}
impl From<Sel> for bool {
    #[inline(always)]
    fn from(variant: Sel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SEL` reader - Direction control."]
pub type SelR = crate::BitReader<Sel>;
impl SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sel {
        match self.bits {
            false => Sel::Rts,
            true => Sel::Dtr,
        }
    }
    #[doc = "RTS. If direction control is enabled (bit DCTRL = 1), pin RTS is used for direction control."]
    #[inline(always)]
    pub fn is_rts(&self) -> bool {
        *self == Sel::Rts
    }
    #[doc = "DTR. If direction control is enabled (bit DCTRL = 1), pin DTR is used for direction control."]
    #[inline(always)]
    pub fn is_dtr(&self) -> bool {
        *self == Sel::Dtr
    }
}
#[doc = "Field `SEL` writer - Direction control."]
pub type SelW<'a, REG> = crate::BitWriter<'a, REG, Sel>;
impl<'a, REG> SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "RTS. If direction control is enabled (bit DCTRL = 1), pin RTS is used for direction control."]
    #[inline(always)]
    pub fn rts(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Rts)
    }
    #[doc = "DTR. If direction control is enabled (bit DCTRL = 1), pin DTR is used for direction control."]
    #[inline(always)]
    pub fn dtr(self) -> &'a mut crate::W<REG> {
        self.variant(Sel::Dtr)
    }
}
#[doc = "Direction control enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dctrl {
    #[doc = "0: Disable Auto Direction Control."]
    Disabled = 0,
    #[doc = "1: Enable Auto Direction Control."]
    Enabled = 1,
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
            false => Dctrl::Disabled,
            true => Dctrl::Enabled,
        }
    }
    #[doc = "Disable Auto Direction Control."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dctrl::Disabled
    }
    #[doc = "Enable Auto Direction Control."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dctrl::Enabled
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
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dctrl::Disabled)
    }
    #[doc = "Enable Auto Direction Control."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dctrl::Enabled)
    }
}
#[doc = "Polarity. This bit reverses the polarity of the direction control signal on the RTS (or DTR) pin.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Oinv {
    #[doc = "0: LOW. The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    ActiveLow = 0,
    #[doc = "1: HIGH. The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    ActiveHigh = 1,
}
impl From<Oinv> for bool {
    #[inline(always)]
    fn from(variant: Oinv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OINV` reader - Polarity. This bit reverses the polarity of the direction control signal on the RTS (or DTR) pin."]
pub type OinvR = crate::BitReader<Oinv>;
impl OinvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Oinv {
        match self.bits {
            false => Oinv::ActiveLow,
            true => Oinv::ActiveHigh,
        }
    }
    #[doc = "LOW. The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == Oinv::ActiveLow
    }
    #[doc = "HIGH. The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == Oinv::ActiveHigh
    }
}
#[doc = "Field `OINV` writer - Polarity. This bit reverses the polarity of the direction control signal on the RTS (or DTR) pin."]
pub type OinvW<'a, REG> = crate::BitWriter<'a, REG, Oinv>;
impl<'a, REG> OinvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "LOW. The direction control pin will be driven to logic 0 when the transmitter has data to be sent. It will be driven to logic 1 after the last bit of data has been transmitted."]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(Oinv::ActiveLow)
    }
    #[doc = "HIGH. The direction control pin will be driven to logic 1 when the transmitter has data to be sent. It will be driven to logic 0 after the last bit of data has been transmitted."]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(Oinv::ActiveHigh)
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
