#[doc = "Register `CR0` reader"]
pub type R = crate::R<Cr0Spec>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<Cr0Spec>;
#[doc = "Data Size Select. This field controls the number of bits transferred in each frame. Values 0000-0010 are not supported and should not be used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Enum {
    #[doc = "3: 4-bit transfer"]
    _4BitTransfer = 3,
    #[doc = "4: 5-bit transfer"]
    _5BitTransfer = 4,
    #[doc = "5: 6-bit transfer"]
    _6BitTransfer = 5,
    #[doc = "6: 7-bit transfer"]
    _7BitTransfer = 6,
    #[doc = "7: 8-bit transfer"]
    _8BitTransfer = 7,
    #[doc = "8: 9-bit transfer"]
    _9BitTransfer = 8,
    #[doc = "9: 10-bit transfer"]
    _10BitTransfer = 9,
    #[doc = "10: 11-bit transfer"]
    _11BitTransfer = 10,
    #[doc = "11: 12-bit transfer"]
    _12BitTransfer = 11,
    #[doc = "12: 13-bit transfer"]
    _13BitTransfer = 12,
    #[doc = "13: 14-bit transfer"]
    _14BitTransfer = 13,
    #[doc = "14: 15-bit transfer"]
    _15BitTransfer = 14,
    #[doc = "15: 16-bit transfer"]
    _16BitTransfer = 15,
}
impl From<Enum> for u8 {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Enum {
    type Ux = u8;
}
impl crate::IsEnum for Enum {}
#[doc = "Field `DSS` reader - Data Size Select. This field controls the number of bits transferred in each frame. Values 0000-0010 are not supported and should not be used."]
pub type DssR = crate::FieldReader<Enum>;
impl DssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Enum> {
        match self.bits {
            3 => Some(Enum::_4BitTransfer),
            4 => Some(Enum::_5BitTransfer),
            5 => Some(Enum::_6BitTransfer),
            6 => Some(Enum::_7BitTransfer),
            7 => Some(Enum::_8BitTransfer),
            8 => Some(Enum::_9BitTransfer),
            9 => Some(Enum::_10BitTransfer),
            10 => Some(Enum::_11BitTransfer),
            11 => Some(Enum::_12BitTransfer),
            12 => Some(Enum::_13BitTransfer),
            13 => Some(Enum::_14BitTransfer),
            14 => Some(Enum::_15BitTransfer),
            15 => Some(Enum::_16BitTransfer),
            _ => None,
        }
    }
    #[doc = "4-bit transfer"]
    #[inline(always)]
    pub fn is_4_bit_transfer(&self) -> bool {
        *self == Enum::_4BitTransfer
    }
    #[doc = "5-bit transfer"]
    #[inline(always)]
    pub fn is_5_bit_transfer(&self) -> bool {
        *self == Enum::_5BitTransfer
    }
    #[doc = "6-bit transfer"]
    #[inline(always)]
    pub fn is_6_bit_transfer(&self) -> bool {
        *self == Enum::_6BitTransfer
    }
    #[doc = "7-bit transfer"]
    #[inline(always)]
    pub fn is_7_bit_transfer(&self) -> bool {
        *self == Enum::_7BitTransfer
    }
    #[doc = "8-bit transfer"]
    #[inline(always)]
    pub fn is_8_bit_transfer(&self) -> bool {
        *self == Enum::_8BitTransfer
    }
    #[doc = "9-bit transfer"]
    #[inline(always)]
    pub fn is_9_bit_transfer(&self) -> bool {
        *self == Enum::_9BitTransfer
    }
    #[doc = "10-bit transfer"]
    #[inline(always)]
    pub fn is_10_bit_transfer(&self) -> bool {
        *self == Enum::_10BitTransfer
    }
    #[doc = "11-bit transfer"]
    #[inline(always)]
    pub fn is_11_bit_transfer(&self) -> bool {
        *self == Enum::_11BitTransfer
    }
    #[doc = "12-bit transfer"]
    #[inline(always)]
    pub fn is_12_bit_transfer(&self) -> bool {
        *self == Enum::_12BitTransfer
    }
    #[doc = "13-bit transfer"]
    #[inline(always)]
    pub fn is_13_bit_transfer(&self) -> bool {
        *self == Enum::_13BitTransfer
    }
    #[doc = "14-bit transfer"]
    #[inline(always)]
    pub fn is_14_bit_transfer(&self) -> bool {
        *self == Enum::_14BitTransfer
    }
    #[doc = "15-bit transfer"]
    #[inline(always)]
    pub fn is_15_bit_transfer(&self) -> bool {
        *self == Enum::_15BitTransfer
    }
    #[doc = "16-bit transfer"]
    #[inline(always)]
    pub fn is_16_bit_transfer(&self) -> bool {
        *self == Enum::_16BitTransfer
    }
}
#[doc = "Field `DSS` writer - Data Size Select. This field controls the number of bits transferred in each frame. Values 0000-0010 are not supported and should not be used."]
pub type DssW<'a, REG> = crate::FieldWriter<'a, REG, 4, Enum>;
impl<'a, REG> DssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4-bit transfer"]
    #[inline(always)]
    pub fn _4_bit_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::_4BitTransfer)
    }
    #[doc = "5-bit transfer"]
    #[inline(always)]
    pub fn _5_bit_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::_5BitTransfer)
    }
    #[doc = "6-bit transfer"]
    #[inline(always)]
    pub fn _6_bit_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::_6BitTransfer)
    }
    #[doc = "7-bit transfer"]
    #[inline(always)]
    pub fn _7_bit_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::_7BitTransfer)
    }
    #[doc = "8-bit transfer"]
    #[inline(always)]
    pub fn _8_bit_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::_8BitTransfer)
    }
    #[doc = "9-bit transfer"]
    #[inline(always)]
    pub fn _9_bit_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::_9BitTransfer)
    }
    #[doc = "10-bit transfer"]
    #[inline(always)]
    pub fn _10_bit_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::_10BitTransfer)
    }
    #[doc = "11-bit transfer"]
    #[inline(always)]
    pub fn _11_bit_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::_11BitTransfer)
    }
    #[doc = "12-bit transfer"]
    #[inline(always)]
    pub fn _12_bit_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::_12BitTransfer)
    }
    #[doc = "13-bit transfer"]
    #[inline(always)]
    pub fn _13_bit_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::_13BitTransfer)
    }
    #[doc = "14-bit transfer"]
    #[inline(always)]
    pub fn _14_bit_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::_14BitTransfer)
    }
    #[doc = "15-bit transfer"]
    #[inline(always)]
    pub fn _15_bit_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::_15BitTransfer)
    }
    #[doc = "16-bit transfer"]
    #[inline(always)]
    pub fn _16_bit_transfer(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::_16BitTransfer)
    }
}
#[doc = "Frame Format.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Enum {
    #[doc = "0: SPI"]
    Spi = 0,
    #[doc = "1: TI"]
    Ti = 1,
    #[doc = "2: Microwire"]
    Microwire = 2,
    #[doc = "3: This combination is not supported and should not be used."]
    ThisCombinationIs_ = 3,
}
impl From<Enum> for u8 {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Enum {
    type Ux = u8;
}
impl crate::IsEnum for Enum {}
#[doc = "Field `FRF` reader - Frame Format."]
pub type FrfR = crate::FieldReader<Enum>;
impl FrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            0 => Enum::Spi,
            1 => Enum::Ti,
            2 => Enum::Microwire,
            3 => Enum::ThisCombinationIs_,
            _ => unreachable!(),
        }
    }
    #[doc = "SPI"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == Enum::Spi
    }
    #[doc = "TI"]
    #[inline(always)]
    pub fn is_ti(&self) -> bool {
        *self == Enum::Ti
    }
    #[doc = "Microwire"]
    #[inline(always)]
    pub fn is_microwire(&self) -> bool {
        *self == Enum::Microwire
    }
    #[doc = "This combination is not supported and should not be used."]
    #[inline(always)]
    pub fn is_this_combination_is_(&self) -> bool {
        *self == Enum::ThisCombinationIs_
    }
}
#[doc = "Field `FRF` writer - Frame Format."]
pub type FrfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Enum, crate::Safe>;
impl<'a, REG> FrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SPI"]
    #[inline(always)]
    pub fn spi(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Spi)
    }
    #[doc = "TI"]
    #[inline(always)]
    pub fn ti(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Ti)
    }
    #[doc = "Microwire"]
    #[inline(always)]
    pub fn microwire(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Microwire)
    }
    #[doc = "This combination is not supported and should not be used."]
    #[inline(always)]
    pub fn this_combination_is_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::ThisCombinationIs_)
    }
}
#[doc = "Clock Out Polarity. This bit is only used in SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: SSP controller maintains the bus clock low between frames."]
    BusLow = 0,
    #[doc = "1: SSP controller maintains the bus clock high between frames."]
    BusHigh = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Clock Out Polarity. This bit is only used in SPI mode."]
pub type CpolR = crate::BitReader<Enum>;
impl CpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::BusLow,
            true => Enum::BusHigh,
        }
    }
    #[doc = "SSP controller maintains the bus clock low between frames."]
    #[inline(always)]
    pub fn is_bus_low(&self) -> bool {
        *self == Enum::BusLow
    }
    #[doc = "SSP controller maintains the bus clock high between frames."]
    #[inline(always)]
    pub fn is_bus_high(&self) -> bool {
        *self == Enum::BusHigh
    }
}
#[doc = "Field `CPOL` writer - Clock Out Polarity. This bit is only used in SPI mode."]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> CpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SSP controller maintains the bus clock low between frames."]
    #[inline(always)]
    pub fn bus_low(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::BusLow)
    }
    #[doc = "SSP controller maintains the bus clock high between frames."]
    #[inline(always)]
    pub fn bus_high(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::BusHigh)
    }
}
#[doc = "Clock Out Phase. This bit is only used in SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: SSP controller captures serial data on the first clock transition of the frame, that is, the transition away from the inter-frame state of the clock line."]
    FirstClock = 0,
    #[doc = "1: SSP controller captures serial data on the second clock transition of the frame, that is, the transition back to the inter-frame state of the clock line."]
    SecondClock = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA` reader - Clock Out Phase. This bit is only used in SPI mode."]
pub type CphaR = crate::BitReader<Enum>;
impl CphaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::FirstClock,
            true => Enum::SecondClock,
        }
    }
    #[doc = "SSP controller captures serial data on the first clock transition of the frame, that is, the transition away from the inter-frame state of the clock line."]
    #[inline(always)]
    pub fn is_first_clock(&self) -> bool {
        *self == Enum::FirstClock
    }
    #[doc = "SSP controller captures serial data on the second clock transition of the frame, that is, the transition back to the inter-frame state of the clock line."]
    #[inline(always)]
    pub fn is_second_clock(&self) -> bool {
        *self == Enum::SecondClock
    }
}
#[doc = "Field `CPHA` writer - Clock Out Phase. This bit is only used in SPI mode."]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> CphaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SSP controller captures serial data on the first clock transition of the frame, that is, the transition away from the inter-frame state of the clock line."]
    #[inline(always)]
    pub fn first_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::FirstClock)
    }
    #[doc = "SSP controller captures serial data on the second clock transition of the frame, that is, the transition back to the inter-frame state of the clock line."]
    #[inline(always)]
    pub fn second_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::SecondClock)
    }
}
#[doc = "Field `SCR` reader - Serial Clock Rate. The number of prescaler-output clocks per bit on the bus, minus one. Given that CPSDVSR is the prescale divider, and the APB clock PCLK clocks the prescaler, the bit frequency is PCLK / (CPSDVSR X \\[SCR+1\\])."]
pub type ScrR = crate::FieldReader;
#[doc = "Field `SCR` writer - Serial Clock Rate. The number of prescaler-output clocks per bit on the bus, minus one. Given that CPSDVSR is the prescale divider, and the APB clock PCLK clocks the prescaler, the bit frequency is PCLK / (CPSDVSR X \\[SCR+1\\])."]
pub type ScrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - Data Size Select. This field controls the number of bits transferred in each frame. Values 0000-0010 are not supported and should not be used."]
    #[inline(always)]
    pub fn dss(&self) -> DssR {
        DssR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Frame Format."]
    #[inline(always)]
    pub fn frf(&self) -> FrfR {
        FrfR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Clock Out Polarity. This bit is only used in SPI mode."]
    #[inline(always)]
    pub fn cpol(&self) -> CpolR {
        CpolR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clock Out Phase. This bit is only used in SPI mode."]
    #[inline(always)]
    pub fn cpha(&self) -> CphaR {
        CphaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Serial Clock Rate. The number of prescaler-output clocks per bit on the bus, minus one. Given that CPSDVSR is the prescale divider, and the APB clock PCLK clocks the prescaler, the bit frequency is PCLK / (CPSDVSR X \\[SCR+1\\])."]
    #[inline(always)]
    pub fn scr(&self) -> ScrR {
        ScrR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Data Size Select. This field controls the number of bits transferred in each frame. Values 0000-0010 are not supported and should not be used."]
    #[inline(always)]
    pub fn dss(&mut self) -> DssW<'_, Cr0Spec> {
        DssW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Frame Format."]
    #[inline(always)]
    pub fn frf(&mut self) -> FrfW<'_, Cr0Spec> {
        FrfW::new(self, 4)
    }
    #[doc = "Bit 6 - Clock Out Polarity. This bit is only used in SPI mode."]
    #[inline(always)]
    pub fn cpol(&mut self) -> CpolW<'_, Cr0Spec> {
        CpolW::new(self, 6)
    }
    #[doc = "Bit 7 - Clock Out Phase. This bit is only used in SPI mode."]
    #[inline(always)]
    pub fn cpha(&mut self) -> CphaW<'_, Cr0Spec> {
        CphaW::new(self, 7)
    }
    #[doc = "Bits 8:15 - Serial Clock Rate. The number of prescaler-output clocks per bit on the bus, minus one. Given that CPSDVSR is the prescale divider, and the APB clock PCLK clocks the prescaler, the bit frequency is PCLK / (CPSDVSR X \\[SCR+1\\])."]
    #[inline(always)]
    pub fn scr(&mut self) -> ScrW<'_, Cr0Spec> {
        ScrW::new(self, 8)
    }
}
#[doc = "Control Register 0. Selects the serial clock rate, bus type, and data size.\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr0Spec;
impl crate::RegisterSpec for Cr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::Readable for Cr0Spec {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::Writable for Cr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for Cr0Spec {}
