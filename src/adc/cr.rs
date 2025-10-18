#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `SEL` reader - Selects which of the AD0\\[7:0\\] pins is (are) to be sampled and converted. For AD0, bit 0 selects Pin AD0\\[0\\], and bit 7 selects pin AD0\\[7\\]. In software-controlled mode, only one of these bits should be 1. In hardware scan mode, any value containing 1 to 8 ones is allowed. All zeroes is equivalent to 0x01."]
pub type SelR = crate::FieldReader;
#[doc = "Field `SEL` writer - Selects which of the AD0\\[7:0\\] pins is (are) to be sampled and converted. For AD0, bit 0 selects Pin AD0\\[0\\], and bit 7 selects pin AD0\\[7\\]. In software-controlled mode, only one of these bits should be 1. In hardware scan mode, any value containing 1 to 8 ones is allowed. All zeroes is equivalent to 0x01."]
pub type SelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLKDIV` reader - The APB clock (PCLK) is divided by (this value plus one) to produce the clock for the A/D converter, which should be less than or equal to 12.4 MHz. Typically, software should program the smallest value in this field that yields a clock of 12.4 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
pub type ClkdivR = crate::FieldReader;
#[doc = "Field `CLKDIV` writer - The APB clock (PCLK) is divided by (this value plus one) to produce the clock for the A/D converter, which should be less than or equal to 12.4 MHz. Typically, software should program the smallest value in this field that yields a clock of 12.4 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
pub type ClkdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Burst mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "1: The AD converter does repeated conversions at up to 400 kHz, scanning (if necessary) through the pins selected by bits set to ones in the SEL field. The first conversion after the start corresponds to the least-significant 1 in the SEL field, then higher numbered 1-bits (pins) if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion that's in progress when this bit is cleared will be completed. START bits must be 000 when BURST = 1 or conversions will not start."]
    Burst = 1,
    #[doc = "0: Conversions are software controlled and require 31 clocks."]
    Sw = 0,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BURST` reader - Burst mode"]
pub type BurstR = crate::BitReader<Enum>;
impl BurstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            true => Enum::Burst,
            false => Enum::Sw,
        }
    }
    #[doc = "The AD converter does repeated conversions at up to 400 kHz, scanning (if necessary) through the pins selected by bits set to ones in the SEL field. The first conversion after the start corresponds to the least-significant 1 in the SEL field, then higher numbered 1-bits (pins) if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion that's in progress when this bit is cleared will be completed. START bits must be 000 when BURST = 1 or conversions will not start."]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        *self == Enum::Burst
    }
    #[doc = "Conversions are software controlled and require 31 clocks."]
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        *self == Enum::Sw
    }
}
#[doc = "Field `BURST` writer - Burst mode"]
pub type BurstW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> BurstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The AD converter does repeated conversions at up to 400 kHz, scanning (if necessary) through the pins selected by bits set to ones in the SEL field. The first conversion after the start corresponds to the least-significant 1 in the SEL field, then higher numbered 1-bits (pins) if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion that's in progress when this bit is cleared will be completed. START bits must be 000 when BURST = 1 or conversions will not start."]
    #[inline(always)]
    pub fn burst(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Burst)
    }
    #[doc = "Conversions are software controlled and require 31 clocks."]
    #[inline(always)]
    pub fn sw(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Sw)
    }
}
#[doc = "Power down mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "1: The A/D converter is operational."]
    Powered = 1,
    #[doc = "0: The A/D converter is in power-down mode."]
    Powerdown = 0,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDN` reader - Power down mode"]
pub type PdnR = crate::BitReader<Enum>;
impl PdnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            true => Enum::Powered,
            false => Enum::Powerdown,
        }
    }
    #[doc = "The A/D converter is operational."]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == Enum::Powered
    }
    #[doc = "The A/D converter is in power-down mode."]
    #[inline(always)]
    pub fn is_powerdown(&self) -> bool {
        *self == Enum::Powerdown
    }
}
#[doc = "Field `PDN` writer - Power down mode"]
pub type PdnW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> PdnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The A/D converter is operational."]
    #[inline(always)]
    pub fn powered(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Powered)
    }
    #[doc = "The A/D converter is in power-down mode."]
    #[inline(always)]
    pub fn powerdown(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Powerdown)
    }
}
#[doc = "When the BURST bit is 0, these bits control whether and when an A/D conversion is started:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Enum {
    #[doc = "0: No start (this value should be used when clearing PDN to 0)."]
    NoStartThisValue = 0,
    #[doc = "1: Start conversion now."]
    StartConversionNow = 1,
    #[doc = "2: Start conversion when the edge selected by bit 27 occurs on the P2\\[10\\] pin."]
    P2_10 = 2,
    #[doc = "3: Start conversion when the edge selected by bit 27 occurs on the P1\\[27\\] pin."]
    P1_27 = 3,
    #[doc = "4: Start conversion when the edge selected by bit 27 occurs on MAT0.1. Note that this does not require that the MAT0.1 function appear on a device pin."]
    Mat0_1 = 4,
    #[doc = "5: Start conversion when the edge selected by bit 27 occurs on MAT0.3. Note that it is not possible to cause the MAT0.3 function to appear on a device pin."]
    Mat0_3 = 5,
    #[doc = "6: Start conversion when the edge selected by bit 27 occurs on MAT1.0. Note that this does not require that the MAT1.0 function appear on a device pin."]
    Mat1_0 = 6,
    #[doc = "7: Start conversion when the edge selected by bit 27 occurs on MAT1.1. Note that this does not require that the MAT1.1 function appear on a device pin."]
    Mat1_1 = 7,
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
#[doc = "Field `START` reader - When the BURST bit is 0, these bits control whether and when an A/D conversion is started:"]
pub type StartR = crate::FieldReader<Enum>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            0 => Enum::NoStartThisValue,
            1 => Enum::StartConversionNow,
            2 => Enum::P2_10,
            3 => Enum::P1_27,
            4 => Enum::Mat0_1,
            5 => Enum::Mat0_3,
            6 => Enum::Mat1_0,
            7 => Enum::Mat1_1,
            _ => unreachable!(),
        }
    }
    #[doc = "No start (this value should be used when clearing PDN to 0)."]
    #[inline(always)]
    pub fn is_no_start_this_value(&self) -> bool {
        *self == Enum::NoStartThisValue
    }
    #[doc = "Start conversion now."]
    #[inline(always)]
    pub fn is_start_conversion_now(&self) -> bool {
        *self == Enum::StartConversionNow
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on the P2\\[10\\] pin."]
    #[inline(always)]
    pub fn is_p2_10(&self) -> bool {
        *self == Enum::P2_10
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on the P1\\[27\\] pin."]
    #[inline(always)]
    pub fn is_p1_27(&self) -> bool {
        *self == Enum::P1_27
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT0.1. Note that this does not require that the MAT0.1 function appear on a device pin."]
    #[inline(always)]
    pub fn is_mat0_1(&self) -> bool {
        *self == Enum::Mat0_1
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT0.3. Note that it is not possible to cause the MAT0.3 function to appear on a device pin."]
    #[inline(always)]
    pub fn is_mat0_3(&self) -> bool {
        *self == Enum::Mat0_3
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT1.0. Note that this does not require that the MAT1.0 function appear on a device pin."]
    #[inline(always)]
    pub fn is_mat1_0(&self) -> bool {
        *self == Enum::Mat1_0
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT1.1. Note that this does not require that the MAT1.1 function appear on a device pin."]
    #[inline(always)]
    pub fn is_mat1_1(&self) -> bool {
        *self == Enum::Mat1_1
    }
}
#[doc = "Field `START` writer - When the BURST bit is 0, these bits control whether and when an A/D conversion is started:"]
pub type StartW<'a, REG> = crate::FieldWriter<'a, REG, 3, Enum, crate::Safe>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No start (this value should be used when clearing PDN to 0)."]
    #[inline(always)]
    pub fn no_start_this_value(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::NoStartThisValue)
    }
    #[doc = "Start conversion now."]
    #[inline(always)]
    pub fn start_conversion_now(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::StartConversionNow)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on the P2\\[10\\] pin."]
    #[inline(always)]
    pub fn p2_10(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::P2_10)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on the P1\\[27\\] pin."]
    #[inline(always)]
    pub fn p1_27(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::P1_27)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT0.1. Note that this does not require that the MAT0.1 function appear on a device pin."]
    #[inline(always)]
    pub fn mat0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Mat0_1)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT0.3. Note that it is not possible to cause the MAT0.3 function to appear on a device pin."]
    #[inline(always)]
    pub fn mat0_3(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Mat0_3)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT1.0. Note that this does not require that the MAT1.0 function appear on a device pin."]
    #[inline(always)]
    pub fn mat1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Mat1_0)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT1.1. Note that this does not require that the MAT1.1 function appear on a device pin."]
    #[inline(always)]
    pub fn mat1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Mat1_1)
    }
}
#[doc = "This bit is significant only when the START field contains 010-111. In these cases:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "1: Start conversion on a falling edge on the selected CAP/MAT signal."]
    Fallling = 1,
    #[doc = "0: Start conversion on a rising edge on the selected CAP/MAT signal."]
    Rising = 0,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDGE` reader - This bit is significant only when the START field contains 010-111. In these cases:"]
pub type EdgeR = crate::BitReader<Enum>;
impl EdgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            true => Enum::Fallling,
            false => Enum::Rising,
        }
    }
    #[doc = "Start conversion on a falling edge on the selected CAP/MAT signal."]
    #[inline(always)]
    pub fn is_fallling(&self) -> bool {
        *self == Enum::Fallling
    }
    #[doc = "Start conversion on a rising edge on the selected CAP/MAT signal."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Enum::Rising
    }
}
#[doc = "Field `EDGE` writer - This bit is significant only when the START field contains 010-111. In these cases:"]
pub type EdgeW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> EdgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start conversion on a falling edge on the selected CAP/MAT signal."]
    #[inline(always)]
    pub fn fallling(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Fallling)
    }
    #[doc = "Start conversion on a rising edge on the selected CAP/MAT signal."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Rising)
    }
}
impl R {
    #[doc = "Bits 0:7 - Selects which of the AD0\\[7:0\\] pins is (are) to be sampled and converted. For AD0, bit 0 selects Pin AD0\\[0\\], and bit 7 selects pin AD0\\[7\\]. In software-controlled mode, only one of these bits should be 1. In hardware scan mode, any value containing 1 to 8 ones is allowed. All zeroes is equivalent to 0x01."]
    #[inline(always)]
    pub fn sel(&self) -> SelR {
        SelR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The APB clock (PCLK) is divided by (this value plus one) to produce the clock for the A/D converter, which should be less than or equal to 12.4 MHz. Typically, software should program the smallest value in this field that yields a clock of 12.4 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
    #[inline(always)]
    pub fn clkdiv(&self) -> ClkdivR {
        ClkdivR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16 - Burst mode"]
    #[inline(always)]
    pub fn burst(&self) -> BurstR {
        BurstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - Power down mode"]
    #[inline(always)]
    pub fn pdn(&self) -> PdnR {
        PdnR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:26 - When the BURST bit is 0, these bits control whether and when an A/D conversion is started:"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - This bit is significant only when the START field contains 010-111. In these cases:"]
    #[inline(always)]
    pub fn edge(&self) -> EdgeR {
        EdgeR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Selects which of the AD0\\[7:0\\] pins is (are) to be sampled and converted. For AD0, bit 0 selects Pin AD0\\[0\\], and bit 7 selects pin AD0\\[7\\]. In software-controlled mode, only one of these bits should be 1. In hardware scan mode, any value containing 1 to 8 ones is allowed. All zeroes is equivalent to 0x01."]
    #[inline(always)]
    pub fn sel(&mut self) -> SelW<'_, CrSpec> {
        SelW::new(self, 0)
    }
    #[doc = "Bits 8:15 - The APB clock (PCLK) is divided by (this value plus one) to produce the clock for the A/D converter, which should be less than or equal to 12.4 MHz. Typically, software should program the smallest value in this field that yields a clock of 12.4 MHz or slightly less, but in certain cases (such as a high-impedance analog source) a slower clock may be desirable."]
    #[inline(always)]
    pub fn clkdiv(&mut self) -> ClkdivW<'_, CrSpec> {
        ClkdivW::new(self, 8)
    }
    #[doc = "Bit 16 - Burst mode"]
    #[inline(always)]
    pub fn burst(&mut self) -> BurstW<'_, CrSpec> {
        BurstW::new(self, 16)
    }
    #[doc = "Bit 21 - Power down mode"]
    #[inline(always)]
    pub fn pdn(&mut self) -> PdnW<'_, CrSpec> {
        PdnW::new(self, 21)
    }
    #[doc = "Bits 24:26 - When the BURST bit is 0, these bits control whether and when an A/D conversion is started:"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, CrSpec> {
        StartW::new(self, 24)
    }
    #[doc = "Bit 27 - This bit is significant only when the START field contains 010-111. In these cases:"]
    #[inline(always)]
    pub fn edge(&mut self) -> EdgeW<'_, CrSpec> {
        EdgeW::new(self, 27)
    }
}
#[doc = "A/D Control Register. The ADCR register must be written to select the operating mode before A/D conversion can occur.\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
