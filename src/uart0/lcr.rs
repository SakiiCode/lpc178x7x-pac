#[doc = "Register `LCR` reader"]
pub type R = crate::R<LcrSpec>;
#[doc = "Register `LCR` writer"]
pub type W = crate::W<LcrSpec>;
#[doc = "Word Length Select.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wls {
    #[doc = "0: 5-bit character length"]
    _5Bit = 0,
    #[doc = "1: 6-bit character length"]
    _6Bit = 1,
    #[doc = "2: 7-bit character length"]
    _7Bit = 2,
    #[doc = "3: 8-bit character length"]
    _8Bit = 3,
}
impl From<Wls> for u8 {
    #[inline(always)]
    fn from(variant: Wls) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wls {
    type Ux = u8;
}
impl crate::IsEnum for Wls {}
#[doc = "Field `WLS` reader - Word Length Select."]
pub type WlsR = crate::FieldReader<Wls>;
impl WlsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wls {
        match self.bits {
            0 => Wls::_5Bit,
            1 => Wls::_6Bit,
            2 => Wls::_7Bit,
            3 => Wls::_8Bit,
            _ => unreachable!(),
        }
    }
    #[doc = "5-bit character length"]
    #[inline(always)]
    pub fn is_5_bit(&self) -> bool {
        *self == Wls::_5Bit
    }
    #[doc = "6-bit character length"]
    #[inline(always)]
    pub fn is_6_bit(&self) -> bool {
        *self == Wls::_6Bit
    }
    #[doc = "7-bit character length"]
    #[inline(always)]
    pub fn is_7_bit(&self) -> bool {
        *self == Wls::_7Bit
    }
    #[doc = "8-bit character length"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == Wls::_8Bit
    }
}
#[doc = "Field `WLS` writer - Word Length Select."]
pub type WlsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Wls, crate::Safe>;
impl<'a, REG> WlsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "5-bit character length"]
    #[inline(always)]
    pub fn _5_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Wls::_5Bit)
    }
    #[doc = "6-bit character length"]
    #[inline(always)]
    pub fn _6_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Wls::_6Bit)
    }
    #[doc = "7-bit character length"]
    #[inline(always)]
    pub fn _7_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Wls::_7Bit)
    }
    #[doc = "8-bit character length"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Wls::_8Bit)
    }
}
#[doc = "Stop Bit Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sbs {
    #[doc = "0: 1 stop bit."]
    _1Bit = 0,
    #[doc = "1: 2 stop bits (1.5 if UnLCR\\[1:0\\]=00)."]
    _2Bits = 1,
}
impl From<Sbs> for bool {
    #[inline(always)]
    fn from(variant: Sbs) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SBS` reader - Stop Bit Select"]
pub type SbsR = crate::BitReader<Sbs>;
impl SbsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sbs {
        match self.bits {
            false => Sbs::_1Bit,
            true => Sbs::_2Bits,
        }
    }
    #[doc = "1 stop bit."]
    #[inline(always)]
    pub fn is_1_bit(&self) -> bool {
        *self == Sbs::_1Bit
    }
    #[doc = "2 stop bits (1.5 if UnLCR\\[1:0\\]=00)."]
    #[inline(always)]
    pub fn is_2_bits(&self) -> bool {
        *self == Sbs::_2Bits
    }
}
#[doc = "Field `SBS` writer - Stop Bit Select"]
pub type SbsW<'a, REG> = crate::BitWriter<'a, REG, Sbs>;
impl<'a, REG> SbsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1 stop bit."]
    #[inline(always)]
    pub fn _1_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Sbs::_1Bit)
    }
    #[doc = "2 stop bits (1.5 if UnLCR\\[1:0\\]=00)."]
    #[inline(always)]
    pub fn _2_bits(self) -> &'a mut crate::W<REG> {
        self.variant(Sbs::_2Bits)
    }
}
#[doc = "Parity Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pe {
    #[doc = "0: Disable parity generation and checking."]
    Disabled = 0,
    #[doc = "1: Enable parity generation and checking."]
    Enabled = 1,
}
impl From<Pe> for bool {
    #[inline(always)]
    fn from(variant: Pe) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PE` reader - Parity Enable."]
pub type PeR = crate::BitReader<Pe>;
impl PeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pe {
        match self.bits {
            false => Pe::Disabled,
            true => Pe::Enabled,
        }
    }
    #[doc = "Disable parity generation and checking."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pe::Disabled
    }
    #[doc = "Enable parity generation and checking."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pe::Enabled
    }
}
#[doc = "Field `PE` writer - Parity Enable."]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG, Pe>;
impl<'a, REG> PeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable parity generation and checking."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::Disabled)
    }
    #[doc = "Enable parity generation and checking."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pe::Enabled)
    }
}
#[doc = "Parity Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ps {
    #[doc = "0: Odd parity. Number of 1s in the transmitted character and the attached parity bit will be odd."]
    Odd = 0,
    #[doc = "1: Even Parity. Number of 1s in the transmitted character and the attached parity bit will be even."]
    Even = 1,
    #[doc = "2: Forced 1 stick parity."]
    Forced1Stick = 2,
    #[doc = "3: Forced 0 stick parity."]
    Forced0Stick = 3,
}
impl From<Ps> for u8 {
    #[inline(always)]
    fn from(variant: Ps) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ps {
    type Ux = u8;
}
impl crate::IsEnum for Ps {}
#[doc = "Field `PS` reader - Parity Select"]
pub type PsR = crate::FieldReader<Ps>;
impl PsR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ps {
        match self.bits {
            0 => Ps::Odd,
            1 => Ps::Even,
            2 => Ps::Forced1Stick,
            3 => Ps::Forced0Stick,
            _ => unreachable!(),
        }
    }
    #[doc = "Odd parity. Number of 1s in the transmitted character and the attached parity bit will be odd."]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == Ps::Odd
    }
    #[doc = "Even Parity. Number of 1s in the transmitted character and the attached parity bit will be even."]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == Ps::Even
    }
    #[doc = "Forced 1 stick parity."]
    #[inline(always)]
    pub fn is_forced_1_stick(&self) -> bool {
        *self == Ps::Forced1Stick
    }
    #[doc = "Forced 0 stick parity."]
    #[inline(always)]
    pub fn is_forced_0_stick(&self) -> bool {
        *self == Ps::Forced0Stick
    }
}
#[doc = "Field `PS` writer - Parity Select"]
pub type PsW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ps, crate::Safe>;
impl<'a, REG> PsW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Odd parity. Number of 1s in the transmitted character and the attached parity bit will be odd."]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::Odd)
    }
    #[doc = "Even Parity. Number of 1s in the transmitted character and the attached parity bit will be even."]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::Even)
    }
    #[doc = "Forced 1 stick parity."]
    #[inline(always)]
    pub fn forced_1_stick(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::Forced1Stick)
    }
    #[doc = "Forced 0 stick parity."]
    #[inline(always)]
    pub fn forced_0_stick(self) -> &'a mut crate::W<REG> {
        self.variant(Ps::Forced0Stick)
    }
}
#[doc = "Break Control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bc {
    #[doc = "0: Disable break transmission."]
    Disabled = 0,
    #[doc = "1: Enable break transmission. Output pin UARTn TXD is forced to logic 0 when UnLCR\\[6\\] is active high."]
    Enabled = 1,
}
impl From<Bc> for bool {
    #[inline(always)]
    fn from(variant: Bc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BC` reader - Break Control"]
pub type BcR = crate::BitReader<Bc>;
impl BcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bc {
        match self.bits {
            false => Bc::Disabled,
            true => Bc::Enabled,
        }
    }
    #[doc = "Disable break transmission."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Bc::Disabled
    }
    #[doc = "Enable break transmission. Output pin UARTn TXD is forced to logic 0 when UnLCR\\[6\\] is active high."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Bc::Enabled
    }
}
#[doc = "Field `BC` writer - Break Control"]
pub type BcW<'a, REG> = crate::BitWriter<'a, REG, Bc>;
impl<'a, REG> BcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable break transmission."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Bc::Disabled)
    }
    #[doc = "Enable break transmission. Output pin UARTn TXD is forced to logic 0 when UnLCR\\[6\\] is active high."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Bc::Enabled)
    }
}
#[doc = "Divisor Latch Access Bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dlab {
    #[doc = "0: Disable access to Divisor Latches."]
    Disabled = 0,
    #[doc = "1: Enable access to Divisor Latches."]
    Enabled = 1,
}
impl From<Dlab> for bool {
    #[inline(always)]
    fn from(variant: Dlab) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DLAB` reader - Divisor Latch Access Bit"]
pub type DlabR = crate::BitReader<Dlab>;
impl DlabR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dlab {
        match self.bits {
            false => Dlab::Disabled,
            true => Dlab::Enabled,
        }
    }
    #[doc = "Disable access to Divisor Latches."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Dlab::Disabled
    }
    #[doc = "Enable access to Divisor Latches."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Dlab::Enabled
    }
}
#[doc = "Field `DLAB` writer - Divisor Latch Access Bit"]
pub type DlabW<'a, REG> = crate::BitWriter<'a, REG, Dlab>;
impl<'a, REG> DlabW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable access to Divisor Latches."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dlab::Disabled)
    }
    #[doc = "Enable access to Divisor Latches."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Dlab::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:1 - Word Length Select."]
    #[inline(always)]
    pub fn wls(&self) -> WlsR {
        WlsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Stop Bit Select"]
    #[inline(always)]
    pub fn sbs(&self) -> SbsR {
        SbsR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Parity Enable."]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Parity Select"]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Break Control"]
    #[inline(always)]
    pub fn bc(&self) -> BcR {
        BcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit"]
    #[inline(always)]
    pub fn dlab(&self) -> DlabR {
        DlabR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Word Length Select."]
    #[inline(always)]
    pub fn wls(&mut self) -> WlsW<'_, LcrSpec> {
        WlsW::new(self, 0)
    }
    #[doc = "Bit 2 - Stop Bit Select"]
    #[inline(always)]
    pub fn sbs(&mut self) -> SbsW<'_, LcrSpec> {
        SbsW::new(self, 2)
    }
    #[doc = "Bit 3 - Parity Enable."]
    #[inline(always)]
    pub fn pe(&mut self) -> PeW<'_, LcrSpec> {
        PeW::new(self, 3)
    }
    #[doc = "Bits 4:5 - Parity Select"]
    #[inline(always)]
    pub fn ps(&mut self) -> PsW<'_, LcrSpec> {
        PsW::new(self, 4)
    }
    #[doc = "Bit 6 - Break Control"]
    #[inline(always)]
    pub fn bc(&mut self) -> BcW<'_, LcrSpec> {
        BcW::new(self, 6)
    }
    #[doc = "Bit 7 - Divisor Latch Access Bit"]
    #[inline(always)]
    pub fn dlab(&mut self) -> DlabW<'_, LcrSpec> {
        DlabW::new(self, 7)
    }
}
#[doc = "Line Control Register. Contains controls for frame formatting and break generation.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LcrSpec;
impl crate::RegisterSpec for LcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lcr::R`](R) reader structure"]
impl crate::Readable for LcrSpec {}
#[doc = "`write(|w| ..)` method takes [`lcr::W`](W) writer structure"]
impl crate::Writable for LcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LCR to value 0"]
impl crate::Resettable for LcrSpec {}
