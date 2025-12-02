#[doc = "Register `CR0` reader"]
pub type R = crate::R<Cr0Spec>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<Cr0Spec>;
#[doc = "Data Size Select. This field controls the number of bits transferred in each frame. Values 0000-0010 are not supported and should not be used.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Dss {
    #[doc = "3: 4-bit transfer"]
    _4Bit = 3,
    #[doc = "4: 5-bit transfer"]
    _5Bit = 4,
    #[doc = "5: 6-bit transfer"]
    _6Bit = 5,
    #[doc = "6: 7-bit transfer"]
    _7Bit = 6,
    #[doc = "7: 8-bit transfer"]
    _8Bit = 7,
    #[doc = "8: 9-bit transfer"]
    _9Bit = 8,
    #[doc = "9: 10-bit transfer"]
    _10Bit = 9,
    #[doc = "10: 11-bit transfer"]
    _11Bit = 10,
    #[doc = "11: 12-bit transfer"]
    _12Bit = 11,
    #[doc = "12: 13-bit transfer"]
    _13Bit = 12,
    #[doc = "13: 14-bit transfer"]
    _14Bit = 13,
    #[doc = "14: 15-bit transfer"]
    _15Bit = 14,
    #[doc = "15: 16-bit transfer"]
    _16Bit = 15,
}
impl From<Dss> for u8 {
    #[inline(always)]
    fn from(variant: Dss) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Dss {
    type Ux = u8;
}
impl crate::IsEnum for Dss {}
#[doc = "Field `DSS` reader - Data Size Select. This field controls the number of bits transferred in each frame. Values 0000-0010 are not supported and should not be used."]
pub type DssR = crate::FieldReader<Dss>;
impl DssR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Dss> {
        match self.bits {
            3 => Some(Dss::_4Bit),
            4 => Some(Dss::_5Bit),
            5 => Some(Dss::_6Bit),
            6 => Some(Dss::_7Bit),
            7 => Some(Dss::_8Bit),
            8 => Some(Dss::_9Bit),
            9 => Some(Dss::_10Bit),
            10 => Some(Dss::_11Bit),
            11 => Some(Dss::_12Bit),
            12 => Some(Dss::_13Bit),
            13 => Some(Dss::_14Bit),
            14 => Some(Dss::_15Bit),
            15 => Some(Dss::_16Bit),
            _ => None,
        }
    }
    #[doc = "4-bit transfer"]
    #[inline(always)]
    pub fn is_4_bit(&self) -> bool {
        *self == Dss::_4Bit
    }
    #[doc = "5-bit transfer"]
    #[inline(always)]
    pub fn is_5_bit(&self) -> bool {
        *self == Dss::_5Bit
    }
    #[doc = "6-bit transfer"]
    #[inline(always)]
    pub fn is_6_bit(&self) -> bool {
        *self == Dss::_6Bit
    }
    #[doc = "7-bit transfer"]
    #[inline(always)]
    pub fn is_7_bit(&self) -> bool {
        *self == Dss::_7Bit
    }
    #[doc = "8-bit transfer"]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == Dss::_8Bit
    }
    #[doc = "9-bit transfer"]
    #[inline(always)]
    pub fn is_9_bit(&self) -> bool {
        *self == Dss::_9Bit
    }
    #[doc = "10-bit transfer"]
    #[inline(always)]
    pub fn is_10_bit(&self) -> bool {
        *self == Dss::_10Bit
    }
    #[doc = "11-bit transfer"]
    #[inline(always)]
    pub fn is_11_bit(&self) -> bool {
        *self == Dss::_11Bit
    }
    #[doc = "12-bit transfer"]
    #[inline(always)]
    pub fn is_12_bit(&self) -> bool {
        *self == Dss::_12Bit
    }
    #[doc = "13-bit transfer"]
    #[inline(always)]
    pub fn is_13_bit(&self) -> bool {
        *self == Dss::_13Bit
    }
    #[doc = "14-bit transfer"]
    #[inline(always)]
    pub fn is_14_bit(&self) -> bool {
        *self == Dss::_14Bit
    }
    #[doc = "15-bit transfer"]
    #[inline(always)]
    pub fn is_15_bit(&self) -> bool {
        *self == Dss::_15Bit
    }
    #[doc = "16-bit transfer"]
    #[inline(always)]
    pub fn is_16_bit(&self) -> bool {
        *self == Dss::_16Bit
    }
}
#[doc = "Field `DSS` writer - Data Size Select. This field controls the number of bits transferred in each frame. Values 0000-0010 are not supported and should not be used."]
pub type DssW<'a, REG> = crate::FieldWriter<'a, REG, 4, Dss>;
impl<'a, REG> DssW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "4-bit transfer"]
    #[inline(always)]
    pub fn _4_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_4Bit)
    }
    #[doc = "5-bit transfer"]
    #[inline(always)]
    pub fn _5_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_5Bit)
    }
    #[doc = "6-bit transfer"]
    #[inline(always)]
    pub fn _6_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_6Bit)
    }
    #[doc = "7-bit transfer"]
    #[inline(always)]
    pub fn _7_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_7Bit)
    }
    #[doc = "8-bit transfer"]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_8Bit)
    }
    #[doc = "9-bit transfer"]
    #[inline(always)]
    pub fn _9_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_9Bit)
    }
    #[doc = "10-bit transfer"]
    #[inline(always)]
    pub fn _10_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_10Bit)
    }
    #[doc = "11-bit transfer"]
    #[inline(always)]
    pub fn _11_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_11Bit)
    }
    #[doc = "12-bit transfer"]
    #[inline(always)]
    pub fn _12_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_12Bit)
    }
    #[doc = "13-bit transfer"]
    #[inline(always)]
    pub fn _13_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_13Bit)
    }
    #[doc = "14-bit transfer"]
    #[inline(always)]
    pub fn _14_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_14Bit)
    }
    #[doc = "15-bit transfer"]
    #[inline(always)]
    pub fn _15_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_15Bit)
    }
    #[doc = "16-bit transfer"]
    #[inline(always)]
    pub fn _16_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Dss::_16Bit)
    }
}
#[doc = "Frame Format.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Frf {
    #[doc = "0: SPI"]
    Spi = 0,
    #[doc = "1: TI"]
    Ti = 1,
    #[doc = "2: Microwire"]
    Microwire = 2,
}
impl From<Frf> for u8 {
    #[inline(always)]
    fn from(variant: Frf) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Frf {
    type Ux = u8;
}
impl crate::IsEnum for Frf {}
#[doc = "Field `FRF` reader - Frame Format."]
pub type FrfR = crate::FieldReader<Frf>;
impl FrfR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Frf {
        match self.bits {
            0 => Frf::Spi,
            1 => Frf::Ti,
            2 => Frf::Microwire,
            _ => unreachable!(),
        }
    }
    #[doc = "SPI"]
    #[inline(always)]
    pub fn is_spi(&self) -> bool {
        *self == Frf::Spi
    }
    #[doc = "TI"]
    #[inline(always)]
    pub fn is_ti(&self) -> bool {
        *self == Frf::Ti
    }
    #[doc = "Microwire"]
    #[inline(always)]
    pub fn is_microwire(&self) -> bool {
        *self == Frf::Microwire
    }
}
#[doc = "Field `FRF` writer - Frame Format."]
pub type FrfW<'a, REG> = crate::FieldWriter<'a, REG, 2, Frf>;
impl<'a, REG> FrfW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SPI"]
    #[inline(always)]
    pub fn spi(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::Spi)
    }
    #[doc = "TI"]
    #[inline(always)]
    pub fn ti(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::Ti)
    }
    #[doc = "Microwire"]
    #[inline(always)]
    pub fn microwire(self) -> &'a mut crate::W<REG> {
        self.variant(Frf::Microwire)
    }
}
#[doc = "Clock Out Polarity. This bit is only used in SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpol {
    #[doc = "0: SSP controller maintains the bus clock low between frames."]
    BusLow = 0,
    #[doc = "1: SSP controller maintains the bus clock high between frames."]
    BusHigh = 1,
}
impl From<Cpol> for bool {
    #[inline(always)]
    fn from(variant: Cpol) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPOL` reader - Clock Out Polarity. This bit is only used in SPI mode."]
pub type CpolR = crate::BitReader<Cpol>;
impl CpolR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpol {
        match self.bits {
            false => Cpol::BusLow,
            true => Cpol::BusHigh,
        }
    }
    #[doc = "SSP controller maintains the bus clock low between frames."]
    #[inline(always)]
    pub fn is_bus_low(&self) -> bool {
        *self == Cpol::BusLow
    }
    #[doc = "SSP controller maintains the bus clock high between frames."]
    #[inline(always)]
    pub fn is_bus_high(&self) -> bool {
        *self == Cpol::BusHigh
    }
}
#[doc = "Field `CPOL` writer - Clock Out Polarity. This bit is only used in SPI mode."]
pub type CpolW<'a, REG> = crate::BitWriter<'a, REG, Cpol>;
impl<'a, REG> CpolW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SSP controller maintains the bus clock low between frames."]
    #[inline(always)]
    pub fn bus_low(self) -> &'a mut crate::W<REG> {
        self.variant(Cpol::BusLow)
    }
    #[doc = "SSP controller maintains the bus clock high between frames."]
    #[inline(always)]
    pub fn bus_high(self) -> &'a mut crate::W<REG> {
        self.variant(Cpol::BusHigh)
    }
}
#[doc = "Clock Out Phase. This bit is only used in SPI mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cpha {
    #[doc = "0: SSP controller captures serial data on the first clock transition of the frame, that is, the transition away from the inter-frame state of the clock line."]
    FirstClock = 0,
    #[doc = "1: SSP controller captures serial data on the second clock transition of the frame, that is, the transition back to the inter-frame state of the clock line."]
    SecondClock = 1,
}
impl From<Cpha> for bool {
    #[inline(always)]
    fn from(variant: Cpha) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CPHA` reader - Clock Out Phase. This bit is only used in SPI mode."]
pub type CphaR = crate::BitReader<Cpha>;
impl CphaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cpha {
        match self.bits {
            false => Cpha::FirstClock,
            true => Cpha::SecondClock,
        }
    }
    #[doc = "SSP controller captures serial data on the first clock transition of the frame, that is, the transition away from the inter-frame state of the clock line."]
    #[inline(always)]
    pub fn is_first_clock(&self) -> bool {
        *self == Cpha::FirstClock
    }
    #[doc = "SSP controller captures serial data on the second clock transition of the frame, that is, the transition back to the inter-frame state of the clock line."]
    #[inline(always)]
    pub fn is_second_clock(&self) -> bool {
        *self == Cpha::SecondClock
    }
}
#[doc = "Field `CPHA` writer - Clock Out Phase. This bit is only used in SPI mode."]
pub type CphaW<'a, REG> = crate::BitWriter<'a, REG, Cpha>;
impl<'a, REG> CphaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "SSP controller captures serial data on the first clock transition of the frame, that is, the transition away from the inter-frame state of the clock line."]
    #[inline(always)]
    pub fn first_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Cpha::FirstClock)
    }
    #[doc = "SSP controller captures serial data on the second clock transition of the frame, that is, the transition back to the inter-frame state of the clock line."]
    #[inline(always)]
    pub fn second_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Cpha::SecondClock)
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
