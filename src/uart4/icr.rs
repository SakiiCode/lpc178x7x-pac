#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "IrDA mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irdaen {
    #[doc = "0: Disabled. IrDA mode on UART4 is disabled, UART4 acts as a standard UART."]
    DisabledIrdaMode_ = 0,
    #[doc = "1: Enabled. IrDA mode on UART4 is enabled."]
    EnabledIrdaModeO = 1,
}
impl From<Irdaen> for bool {
    #[inline(always)]
    fn from(variant: Irdaen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRDAEN` reader - IrDA mode"]
pub type IrdaenR = crate::BitReader<Irdaen>;
impl IrdaenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irdaen {
        match self.bits {
            false => Irdaen::DisabledIrdaMode_,
            true => Irdaen::EnabledIrdaModeO,
        }
    }
    #[doc = "Disabled. IrDA mode on UART4 is disabled, UART4 acts as a standard UART."]
    #[inline(always)]
    pub fn is_disabled_irda_mode_(&self) -> bool {
        *self == Irdaen::DisabledIrdaMode_
    }
    #[doc = "Enabled. IrDA mode on UART4 is enabled."]
    #[inline(always)]
    pub fn is_enabled_irda_mode_o(&self) -> bool {
        *self == Irdaen::EnabledIrdaModeO
    }
}
#[doc = "Field `IRDAEN` writer - IrDA mode"]
pub type IrdaenW<'a, REG> = crate::BitWriter<'a, REG, Irdaen>;
impl<'a, REG> IrdaenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled. IrDA mode on UART4 is disabled, UART4 acts as a standard UART."]
    #[inline(always)]
    pub fn disabled_irda_mode_(self) -> &'a mut crate::W<REG> {
        self.variant(Irdaen::DisabledIrdaMode_)
    }
    #[doc = "Enabled. IrDA mode on UART4 is enabled."]
    #[inline(always)]
    pub fn enabled_irda_mode_o(self) -> &'a mut crate::W<REG> {
        self.variant(Irdaen::EnabledIrdaModeO)
    }
}
#[doc = "Serial input direction.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Irdainv {
    #[doc = "0: Not inverted."]
    NotInverted_ = 0,
    #[doc = "1: Inverted. This has no effect on the serial output."]
    InvertedThisHasN = 1,
}
impl From<Irdainv> for bool {
    #[inline(always)]
    fn from(variant: Irdainv) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IRDAINV` reader - Serial input direction."]
pub type IrdainvR = crate::BitReader<Irdainv>;
impl IrdainvR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Irdainv {
        match self.bits {
            false => Irdainv::NotInverted_,
            true => Irdainv::InvertedThisHasN,
        }
    }
    #[doc = "Not inverted."]
    #[inline(always)]
    pub fn is_not_inverted_(&self) -> bool {
        *self == Irdainv::NotInverted_
    }
    #[doc = "Inverted. This has no effect on the serial output."]
    #[inline(always)]
    pub fn is_inverted_this_has_n(&self) -> bool {
        *self == Irdainv::InvertedThisHasN
    }
}
#[doc = "Field `IRDAINV` writer - Serial input direction."]
pub type IrdainvW<'a, REG> = crate::BitWriter<'a, REG, Irdainv>;
impl<'a, REG> IrdainvW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not inverted."]
    #[inline(always)]
    pub fn not_inverted_(self) -> &'a mut crate::W<REG> {
        self.variant(Irdainv::NotInverted_)
    }
    #[doc = "Inverted. This has no effect on the serial output."]
    #[inline(always)]
    pub fn inverted_this_has_n(self) -> &'a mut crate::W<REG> {
        self.variant(Irdainv::InvertedThisHasN)
    }
}
#[doc = "IrDA fixed pulse width mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fixpulseen {
    #[doc = "0: Disabled."]
    Disabled_ = 0,
    #[doc = "1: Enabled."]
    Enabled_ = 1,
}
impl From<Fixpulseen> for bool {
    #[inline(always)]
    fn from(variant: Fixpulseen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIXPULSEEN` reader - IrDA fixed pulse width mode."]
pub type FixpulseenR = crate::BitReader<Fixpulseen>;
impl FixpulseenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fixpulseen {
        match self.bits {
            false => Fixpulseen::Disabled_,
            true => Fixpulseen::Enabled_,
        }
    }
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn is_disabled_(&self) -> bool {
        *self == Fixpulseen::Disabled_
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn is_enabled_(&self) -> bool {
        *self == Fixpulseen::Enabled_
    }
}
#[doc = "Field `FIXPULSEEN` writer - IrDA fixed pulse width mode."]
pub type FixpulseenW<'a, REG> = crate::BitWriter<'a, REG, Fixpulseen>;
impl<'a, REG> FixpulseenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled."]
    #[inline(always)]
    pub fn disabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Fixpulseen::Disabled_)
    }
    #[doc = "Enabled."]
    #[inline(always)]
    pub fn enabled_(self) -> &'a mut crate::W<REG> {
        self.variant(Fixpulseen::Enabled_)
    }
}
#[doc = "Configures the pulse when FixPulseEn = 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pulsediv {
    #[doc = "0: 2xTPCLK"]
    _2xtpclk = 0,
    #[doc = "1: 4xTPCLK"]
    _4xtpclk = 1,
    #[doc = "2: 8xTPCLK"]
    _8xtpclk = 2,
    #[doc = "3: 16xTPCLK"]
    _16xtpclk = 3,
    #[doc = "4: 32xTPCLK"]
    _32xtpclk = 4,
    #[doc = "5: 64xTPCLK"]
    _64xtpclk = 5,
    #[doc = "6: 128xTPCLK"]
    _128xtpclk = 6,
    #[doc = "7: 256xTPCLK"]
    _256xtpclk = 7,
}
impl From<Pulsediv> for u8 {
    #[inline(always)]
    fn from(variant: Pulsediv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pulsediv {
    type Ux = u8;
}
impl crate::IsEnum for Pulsediv {}
#[doc = "Field `PULSEDIV` reader - Configures the pulse when FixPulseEn = 1."]
pub type PulsedivR = crate::FieldReader<Pulsediv>;
impl PulsedivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pulsediv {
        match self.bits {
            0 => Pulsediv::_2xtpclk,
            1 => Pulsediv::_4xtpclk,
            2 => Pulsediv::_8xtpclk,
            3 => Pulsediv::_16xtpclk,
            4 => Pulsediv::_32xtpclk,
            5 => Pulsediv::_64xtpclk,
            6 => Pulsediv::_128xtpclk,
            7 => Pulsediv::_256xtpclk,
            _ => unreachable!(),
        }
    }
    #[doc = "2xTPCLK"]
    #[inline(always)]
    pub fn is_2xtpclk(&self) -> bool {
        *self == Pulsediv::_2xtpclk
    }
    #[doc = "4xTPCLK"]
    #[inline(always)]
    pub fn is_4xtpclk(&self) -> bool {
        *self == Pulsediv::_4xtpclk
    }
    #[doc = "8xTPCLK"]
    #[inline(always)]
    pub fn is_8xtpclk(&self) -> bool {
        *self == Pulsediv::_8xtpclk
    }
    #[doc = "16xTPCLK"]
    #[inline(always)]
    pub fn is_16xtpclk(&self) -> bool {
        *self == Pulsediv::_16xtpclk
    }
    #[doc = "32xTPCLK"]
    #[inline(always)]
    pub fn is_32xtpclk(&self) -> bool {
        *self == Pulsediv::_32xtpclk
    }
    #[doc = "64xTPCLK"]
    #[inline(always)]
    pub fn is_64xtpclk(&self) -> bool {
        *self == Pulsediv::_64xtpclk
    }
    #[doc = "128xTPCLK"]
    #[inline(always)]
    pub fn is_128xtpclk(&self) -> bool {
        *self == Pulsediv::_128xtpclk
    }
    #[doc = "256xTPCLK"]
    #[inline(always)]
    pub fn is_256xtpclk(&self) -> bool {
        *self == Pulsediv::_256xtpclk
    }
}
#[doc = "Field `PULSEDIV` writer - Configures the pulse when FixPulseEn = 1."]
pub type PulsedivW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pulsediv, crate::Safe>;
impl<'a, REG> PulsedivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "2xTPCLK"]
    #[inline(always)]
    pub fn _2xtpclk(self) -> &'a mut crate::W<REG> {
        self.variant(Pulsediv::_2xtpclk)
    }
    #[doc = "4xTPCLK"]
    #[inline(always)]
    pub fn _4xtpclk(self) -> &'a mut crate::W<REG> {
        self.variant(Pulsediv::_4xtpclk)
    }
    #[doc = "8xTPCLK"]
    #[inline(always)]
    pub fn _8xtpclk(self) -> &'a mut crate::W<REG> {
        self.variant(Pulsediv::_8xtpclk)
    }
    #[doc = "16xTPCLK"]
    #[inline(always)]
    pub fn _16xtpclk(self) -> &'a mut crate::W<REG> {
        self.variant(Pulsediv::_16xtpclk)
    }
    #[doc = "32xTPCLK"]
    #[inline(always)]
    pub fn _32xtpclk(self) -> &'a mut crate::W<REG> {
        self.variant(Pulsediv::_32xtpclk)
    }
    #[doc = "64xTPCLK"]
    #[inline(always)]
    pub fn _64xtpclk(self) -> &'a mut crate::W<REG> {
        self.variant(Pulsediv::_64xtpclk)
    }
    #[doc = "128xTPCLK"]
    #[inline(always)]
    pub fn _128xtpclk(self) -> &'a mut crate::W<REG> {
        self.variant(Pulsediv::_128xtpclk)
    }
    #[doc = "256xTPCLK"]
    #[inline(always)]
    pub fn _256xtpclk(self) -> &'a mut crate::W<REG> {
        self.variant(Pulsediv::_256xtpclk)
    }
}
impl R {
    #[doc = "Bit 0 - IrDA mode"]
    #[inline(always)]
    pub fn irdaen(&self) -> IrdaenR {
        IrdaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Serial input direction."]
    #[inline(always)]
    pub fn irdainv(&self) -> IrdainvR {
        IrdainvR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IrDA fixed pulse width mode."]
    #[inline(always)]
    pub fn fixpulseen(&self) -> FixpulseenR {
        FixpulseenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Configures the pulse when FixPulseEn = 1."]
    #[inline(always)]
    pub fn pulsediv(&self) -> PulsedivR {
        PulsedivR::new(((self.bits >> 3) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - IrDA mode"]
    #[inline(always)]
    pub fn irdaen(&mut self) -> IrdaenW<'_, IcrSpec> {
        IrdaenW::new(self, 0)
    }
    #[doc = "Bit 1 - Serial input direction."]
    #[inline(always)]
    pub fn irdainv(&mut self) -> IrdainvW<'_, IcrSpec> {
        IrdainvW::new(self, 1)
    }
    #[doc = "Bit 2 - IrDA fixed pulse width mode."]
    #[inline(always)]
    pub fn fixpulseen(&mut self) -> FixpulseenW<'_, IcrSpec> {
        FixpulseenW::new(self, 2)
    }
    #[doc = "Bits 3:5 - Configures the pulse when FixPulseEn = 1."]
    #[inline(always)]
    pub fn pulsediv(&mut self) -> PulsedivW<'_, IcrSpec> {
        PulsedivW::new(self, 3)
    }
}
#[doc = "IrDA Control Register. Enables and configures the IrDA mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for IcrSpec {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}
