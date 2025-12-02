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
pub enum Burst {
    #[doc = "1: The AD converter does repeated conversions at up to 400 kHz, scanning (if necessary) through the pins selected by bits set to ones in the SEL field. The first conversion after the start corresponds to the least-significant 1 in the SEL field, then higher numbered 1-bits (pins) if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion that's in progress when this bit is cleared will be completed. START bits must be 000 when BURST = 1 or conversions will not start."]
    Burst = 1,
    #[doc = "0: Conversions are software controlled and require 31 clocks."]
    Sw = 0,
}
impl From<Burst> for bool {
    #[inline(always)]
    fn from(variant: Burst) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BURST` reader - Burst mode"]
pub type BurstR = crate::BitReader<Burst>;
impl BurstR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Burst {
        match self.bits {
            true => Burst::Burst,
            false => Burst::Sw,
        }
    }
    #[doc = "The AD converter does repeated conversions at up to 400 kHz, scanning (if necessary) through the pins selected by bits set to ones in the SEL field. The first conversion after the start corresponds to the least-significant 1 in the SEL field, then higher numbered 1-bits (pins) if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion that's in progress when this bit is cleared will be completed. START bits must be 000 when BURST = 1 or conversions will not start."]
    #[inline(always)]
    pub fn is_burst(&self) -> bool {
        *self == Burst::Burst
    }
    #[doc = "Conversions are software controlled and require 31 clocks."]
    #[inline(always)]
    pub fn is_sw(&self) -> bool {
        *self == Burst::Sw
    }
}
#[doc = "Field `BURST` writer - Burst mode"]
pub type BurstW<'a, REG> = crate::BitWriter<'a, REG, Burst>;
impl<'a, REG> BurstW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The AD converter does repeated conversions at up to 400 kHz, scanning (if necessary) through the pins selected by bits set to ones in the SEL field. The first conversion after the start corresponds to the least-significant 1 in the SEL field, then higher numbered 1-bits (pins) if applicable. Repeated conversions can be terminated by clearing this bit, but the conversion that's in progress when this bit is cleared will be completed. START bits must be 000 when BURST = 1 or conversions will not start."]
    #[inline(always)]
    pub fn burst(self) -> &'a mut crate::W<REG> {
        self.variant(Burst::Burst)
    }
    #[doc = "Conversions are software controlled and require 31 clocks."]
    #[inline(always)]
    pub fn sw(self) -> &'a mut crate::W<REG> {
        self.variant(Burst::Sw)
    }
}
#[doc = "Power down mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pdn {
    #[doc = "1: The A/D converter is operational."]
    Powered = 1,
    #[doc = "0: The A/D converter is in power-down mode."]
    Powerdown = 0,
}
impl From<Pdn> for bool {
    #[inline(always)]
    fn from(variant: Pdn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PDN` reader - Power down mode"]
pub type PdnR = crate::BitReader<Pdn>;
impl PdnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pdn {
        match self.bits {
            true => Pdn::Powered,
            false => Pdn::Powerdown,
        }
    }
    #[doc = "The A/D converter is operational."]
    #[inline(always)]
    pub fn is_powered(&self) -> bool {
        *self == Pdn::Powered
    }
    #[doc = "The A/D converter is in power-down mode."]
    #[inline(always)]
    pub fn is_powerdown(&self) -> bool {
        *self == Pdn::Powerdown
    }
}
#[doc = "Field `PDN` writer - Power down mode"]
pub type PdnW<'a, REG> = crate::BitWriter<'a, REG, Pdn>;
impl<'a, REG> PdnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The A/D converter is operational."]
    #[inline(always)]
    pub fn powered(self) -> &'a mut crate::W<REG> {
        self.variant(Pdn::Powered)
    }
    #[doc = "The A/D converter is in power-down mode."]
    #[inline(always)]
    pub fn powerdown(self) -> &'a mut crate::W<REG> {
        self.variant(Pdn::Powerdown)
    }
}
#[doc = "When the BURST bit is 0, these bits control whether and when an A/D conversion is started:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Start {
    #[doc = "0: No start (this value should be used when clearing PDN to 0)."]
    NoStart = 0,
    #[doc = "1: Start conversion now."]
    StartNow = 1,
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
impl From<Start> for u8 {
    #[inline(always)]
    fn from(variant: Start) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Start {
    type Ux = u8;
}
impl crate::IsEnum for Start {}
#[doc = "Field `START` reader - When the BURST bit is 0, these bits control whether and when an A/D conversion is started:"]
pub type StartR = crate::FieldReader<Start>;
impl StartR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Start {
        match self.bits {
            0 => Start::NoStart,
            1 => Start::StartNow,
            2 => Start::P2_10,
            3 => Start::P1_27,
            4 => Start::Mat0_1,
            5 => Start::Mat0_3,
            6 => Start::Mat1_0,
            7 => Start::Mat1_1,
            _ => unreachable!(),
        }
    }
    #[doc = "No start (this value should be used when clearing PDN to 0)."]
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        *self == Start::NoStart
    }
    #[doc = "Start conversion now."]
    #[inline(always)]
    pub fn is_start_now(&self) -> bool {
        *self == Start::StartNow
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on the P2\\[10\\] pin."]
    #[inline(always)]
    pub fn is_p2_10(&self) -> bool {
        *self == Start::P2_10
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on the P1\\[27\\] pin."]
    #[inline(always)]
    pub fn is_p1_27(&self) -> bool {
        *self == Start::P1_27
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT0.1. Note that this does not require that the MAT0.1 function appear on a device pin."]
    #[inline(always)]
    pub fn is_mat0_1(&self) -> bool {
        *self == Start::Mat0_1
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT0.3. Note that it is not possible to cause the MAT0.3 function to appear on a device pin."]
    #[inline(always)]
    pub fn is_mat0_3(&self) -> bool {
        *self == Start::Mat0_3
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT1.0. Note that this does not require that the MAT1.0 function appear on a device pin."]
    #[inline(always)]
    pub fn is_mat1_0(&self) -> bool {
        *self == Start::Mat1_0
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT1.1. Note that this does not require that the MAT1.1 function appear on a device pin."]
    #[inline(always)]
    pub fn is_mat1_1(&self) -> bool {
        *self == Start::Mat1_1
    }
}
#[doc = "Field `START` writer - When the BURST bit is 0, these bits control whether and when an A/D conversion is started:"]
pub type StartW<'a, REG> = crate::FieldWriter<'a, REG, 3, Start, crate::Safe>;
impl<'a, REG> StartW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No start (this value should be used when clearing PDN to 0)."]
    #[inline(always)]
    pub fn no_start(self) -> &'a mut crate::W<REG> {
        self.variant(Start::NoStart)
    }
    #[doc = "Start conversion now."]
    #[inline(always)]
    pub fn start_now(self) -> &'a mut crate::W<REG> {
        self.variant(Start::StartNow)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on the P2\\[10\\] pin."]
    #[inline(always)]
    pub fn p2_10(self) -> &'a mut crate::W<REG> {
        self.variant(Start::P2_10)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on the P1\\[27\\] pin."]
    #[inline(always)]
    pub fn p1_27(self) -> &'a mut crate::W<REG> {
        self.variant(Start::P1_27)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT0.1. Note that this does not require that the MAT0.1 function appear on a device pin."]
    #[inline(always)]
    pub fn mat0_1(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Mat0_1)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT0.3. Note that it is not possible to cause the MAT0.3 function to appear on a device pin."]
    #[inline(always)]
    pub fn mat0_3(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Mat0_3)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT1.0. Note that this does not require that the MAT1.0 function appear on a device pin."]
    #[inline(always)]
    pub fn mat1_0(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Mat1_0)
    }
    #[doc = "Start conversion when the edge selected by bit 27 occurs on MAT1.1. Note that this does not require that the MAT1.1 function appear on a device pin."]
    #[inline(always)]
    pub fn mat1_1(self) -> &'a mut crate::W<REG> {
        self.variant(Start::Mat1_1)
    }
}
#[doc = "This bit is significant only when the START field contains 010-111. In these cases:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Edge {
    #[doc = "1: Start conversion on a falling edge on the selected CAP/MAT signal."]
    Fallling = 1,
    #[doc = "0: Start conversion on a rising edge on the selected CAP/MAT signal."]
    Rising = 0,
}
impl From<Edge> for bool {
    #[inline(always)]
    fn from(variant: Edge) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EDGE` reader - This bit is significant only when the START field contains 010-111. In these cases:"]
pub type EdgeR = crate::BitReader<Edge>;
impl EdgeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Edge {
        match self.bits {
            true => Edge::Fallling,
            false => Edge::Rising,
        }
    }
    #[doc = "Start conversion on a falling edge on the selected CAP/MAT signal."]
    #[inline(always)]
    pub fn is_fallling(&self) -> bool {
        *self == Edge::Fallling
    }
    #[doc = "Start conversion on a rising edge on the selected CAP/MAT signal."]
    #[inline(always)]
    pub fn is_rising(&self) -> bool {
        *self == Edge::Rising
    }
}
#[doc = "Field `EDGE` writer - This bit is significant only when the START field contains 010-111. In these cases:"]
pub type EdgeW<'a, REG> = crate::BitWriter<'a, REG, Edge>;
impl<'a, REG> EdgeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Start conversion on a falling edge on the selected CAP/MAT signal."]
    #[inline(always)]
    pub fn fallling(self) -> &'a mut crate::W<REG> {
        self.variant(Edge::Fallling)
    }
    #[doc = "Start conversion on a rising edge on the selected CAP/MAT signal."]
    #[inline(always)]
    pub fn rising(self) -> &'a mut crate::W<REG> {
        self.variant(Edge::Rising)
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
