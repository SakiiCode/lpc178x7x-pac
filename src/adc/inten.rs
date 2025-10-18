#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adinten0 {
    #[doc = "0: Completion of a conversion on ADC channel 0 will not generate an interrupt."]
    Disable = 0,
    #[doc = "1: Completion of a conversion on ADC channel 0 will generate an interrupt."]
    Enable = 1,
}
impl From<Adinten0> for bool {
    #[inline(always)]
    fn from(variant: Adinten0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADINTEN0` reader - Interrupt enable"]
pub type Adinten0R = crate::BitReader<Adinten0>;
impl Adinten0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adinten0 {
        match self.bits {
            false => Adinten0::Disable,
            true => Adinten0::Enable,
        }
    }
    #[doc = "Completion of a conversion on ADC channel 0 will not generate an interrupt."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Adinten0::Disable
    }
    #[doc = "Completion of a conversion on ADC channel 0 will generate an interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Adinten0::Enable
    }
}
#[doc = "Field `ADINTEN0` writer - Interrupt enable"]
pub type Adinten0W<'a, REG> = crate::BitWriter<'a, REG, Adinten0>;
impl<'a, REG> Adinten0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Completion of a conversion on ADC channel 0 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Adinten0::Disable)
    }
    #[doc = "Completion of a conversion on ADC channel 0 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Adinten0::Enable)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adinten1 {
    #[doc = "0: Completion of a conversion on ADC channel 1 will not generate an interrupt."]
    Disable = 0,
    #[doc = "1: Completion of a conversion on ADC channel 1 will generate an interrupt."]
    Enable = 1,
}
impl From<Adinten1> for bool {
    #[inline(always)]
    fn from(variant: Adinten1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADINTEN1` reader - Interrupt enable"]
pub type Adinten1R = crate::BitReader<Adinten1>;
impl Adinten1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adinten1 {
        match self.bits {
            false => Adinten1::Disable,
            true => Adinten1::Enable,
        }
    }
    #[doc = "Completion of a conversion on ADC channel 1 will not generate an interrupt."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Adinten1::Disable
    }
    #[doc = "Completion of a conversion on ADC channel 1 will generate an interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Adinten1::Enable
    }
}
#[doc = "Field `ADINTEN1` writer - Interrupt enable"]
pub type Adinten1W<'a, REG> = crate::BitWriter<'a, REG, Adinten1>;
impl<'a, REG> Adinten1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Completion of a conversion on ADC channel 1 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Adinten1::Disable)
    }
    #[doc = "Completion of a conversion on ADC channel 1 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Adinten1::Enable)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adinten2 {
    #[doc = "0: Completion of a conversion on ADC channel 2 will not generate an interrupt."]
    Disable = 0,
    #[doc = "1: Completion of a conversion on ADC channel 2 will generate an interrupt."]
    Enable = 1,
}
impl From<Adinten2> for bool {
    #[inline(always)]
    fn from(variant: Adinten2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADINTEN2` reader - Interrupt enable"]
pub type Adinten2R = crate::BitReader<Adinten2>;
impl Adinten2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adinten2 {
        match self.bits {
            false => Adinten2::Disable,
            true => Adinten2::Enable,
        }
    }
    #[doc = "Completion of a conversion on ADC channel 2 will not generate an interrupt."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Adinten2::Disable
    }
    #[doc = "Completion of a conversion on ADC channel 2 will generate an interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Adinten2::Enable
    }
}
#[doc = "Field `ADINTEN2` writer - Interrupt enable"]
pub type Adinten2W<'a, REG> = crate::BitWriter<'a, REG, Adinten2>;
impl<'a, REG> Adinten2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Completion of a conversion on ADC channel 2 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Adinten2::Disable)
    }
    #[doc = "Completion of a conversion on ADC channel 2 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Adinten2::Enable)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adinten3 {
    #[doc = "0: Completion of a conversion on ADC channel 3 will not generate an interrupt."]
    Disable = 0,
    #[doc = "1: Completion of a conversion on ADC channel 3 will generate an interrupt."]
    Enable = 1,
}
impl From<Adinten3> for bool {
    #[inline(always)]
    fn from(variant: Adinten3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADINTEN3` reader - Interrupt enable"]
pub type Adinten3R = crate::BitReader<Adinten3>;
impl Adinten3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adinten3 {
        match self.bits {
            false => Adinten3::Disable,
            true => Adinten3::Enable,
        }
    }
    #[doc = "Completion of a conversion on ADC channel 3 will not generate an interrupt."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Adinten3::Disable
    }
    #[doc = "Completion of a conversion on ADC channel 3 will generate an interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Adinten3::Enable
    }
}
#[doc = "Field `ADINTEN3` writer - Interrupt enable"]
pub type Adinten3W<'a, REG> = crate::BitWriter<'a, REG, Adinten3>;
impl<'a, REG> Adinten3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Completion of a conversion on ADC channel 3 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Adinten3::Disable)
    }
    #[doc = "Completion of a conversion on ADC channel 3 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Adinten3::Enable)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adinten4 {
    #[doc = "0: Completion of a conversion on ADC channel 4 will not generate an interrupt."]
    Disable = 0,
    #[doc = "1: Completion of a conversion on ADC channel 4 will generate an interrupt."]
    Enable = 1,
}
impl From<Adinten4> for bool {
    #[inline(always)]
    fn from(variant: Adinten4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADINTEN4` reader - Interrupt enable"]
pub type Adinten4R = crate::BitReader<Adinten4>;
impl Adinten4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adinten4 {
        match self.bits {
            false => Adinten4::Disable,
            true => Adinten4::Enable,
        }
    }
    #[doc = "Completion of a conversion on ADC channel 4 will not generate an interrupt."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Adinten4::Disable
    }
    #[doc = "Completion of a conversion on ADC channel 4 will generate an interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Adinten4::Enable
    }
}
#[doc = "Field `ADINTEN4` writer - Interrupt enable"]
pub type Adinten4W<'a, REG> = crate::BitWriter<'a, REG, Adinten4>;
impl<'a, REG> Adinten4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Completion of a conversion on ADC channel 4 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Adinten4::Disable)
    }
    #[doc = "Completion of a conversion on ADC channel 4 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Adinten4::Enable)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adinten5 {
    #[doc = "0: Completion of a conversion on ADC channel 5 will not generate an interrupt."]
    Disable = 0,
    #[doc = "1: Completion of a conversion on ADC channel 5 will generate an interrupt."]
    Enable = 1,
}
impl From<Adinten5> for bool {
    #[inline(always)]
    fn from(variant: Adinten5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADINTEN5` reader - Interrupt enable"]
pub type Adinten5R = crate::BitReader<Adinten5>;
impl Adinten5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adinten5 {
        match self.bits {
            false => Adinten5::Disable,
            true => Adinten5::Enable,
        }
    }
    #[doc = "Completion of a conversion on ADC channel 5 will not generate an interrupt."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Adinten5::Disable
    }
    #[doc = "Completion of a conversion on ADC channel 5 will generate an interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Adinten5::Enable
    }
}
#[doc = "Field `ADINTEN5` writer - Interrupt enable"]
pub type Adinten5W<'a, REG> = crate::BitWriter<'a, REG, Adinten5>;
impl<'a, REG> Adinten5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Completion of a conversion on ADC channel 5 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Adinten5::Disable)
    }
    #[doc = "Completion of a conversion on ADC channel 5 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Adinten5::Enable)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adinten6 {
    #[doc = "0: Completion of a conversion on ADC channel 6 will not generate an interrupt."]
    Disable = 0,
    #[doc = "1: Completion of a conversion on ADC channel 6 will generate an interrupt."]
    Enable = 1,
}
impl From<Adinten6> for bool {
    #[inline(always)]
    fn from(variant: Adinten6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADINTEN6` reader - Interrupt enable"]
pub type Adinten6R = crate::BitReader<Adinten6>;
impl Adinten6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adinten6 {
        match self.bits {
            false => Adinten6::Disable,
            true => Adinten6::Enable,
        }
    }
    #[doc = "Completion of a conversion on ADC channel 6 will not generate an interrupt."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Adinten6::Disable
    }
    #[doc = "Completion of a conversion on ADC channel 6 will generate an interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Adinten6::Enable
    }
}
#[doc = "Field `ADINTEN6` writer - Interrupt enable"]
pub type Adinten6W<'a, REG> = crate::BitWriter<'a, REG, Adinten6>;
impl<'a, REG> Adinten6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Completion of a conversion on ADC channel 6 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Adinten6::Disable)
    }
    #[doc = "Completion of a conversion on ADC channel 6 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Adinten6::Enable)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adinten7 {
    #[doc = "0: Completion of a conversion on ADC channel 7 will not generate an interrupt."]
    Disable = 0,
    #[doc = "1: Completion of a conversion on ADC channel 7 will generate an interrupt."]
    Enable = 1,
}
impl From<Adinten7> for bool {
    #[inline(always)]
    fn from(variant: Adinten7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADINTEN7` reader - Interrupt enable"]
pub type Adinten7R = crate::BitReader<Adinten7>;
impl Adinten7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adinten7 {
        match self.bits {
            false => Adinten7::Disable,
            true => Adinten7::Enable,
        }
    }
    #[doc = "Completion of a conversion on ADC channel 7 will not generate an interrupt."]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Adinten7::Disable
    }
    #[doc = "Completion of a conversion on ADC channel 7 will generate an interrupt."]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Adinten7::Enable
    }
}
#[doc = "Field `ADINTEN7` writer - Interrupt enable"]
pub type Adinten7W<'a, REG> = crate::BitWriter<'a, REG, Adinten7>;
impl<'a, REG> Adinten7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Completion of a conversion on ADC channel 7 will not generate an interrupt."]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Adinten7::Disable)
    }
    #[doc = "Completion of a conversion on ADC channel 7 will generate an interrupt."]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Adinten7::Enable)
    }
}
#[doc = "Interrupt enable\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Adginten {
    #[doc = "0: Only the individual ADC channels enabled by ADINTEN7:0 will generate interrupts."]
    Channels = 0,
    #[doc = "1: The global DONE flag in ADDR is enabled to generate an interrupt in addition to any individual ADC channels that are enabled to generate interrupts."]
    Global = 1,
}
impl From<Adginten> for bool {
    #[inline(always)]
    fn from(variant: Adginten) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ADGINTEN` reader - Interrupt enable"]
pub type AdgintenR = crate::BitReader<Adginten>;
impl AdgintenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Adginten {
        match self.bits {
            false => Adginten::Channels,
            true => Adginten::Global,
        }
    }
    #[doc = "Only the individual ADC channels enabled by ADINTEN7:0 will generate interrupts."]
    #[inline(always)]
    pub fn is_channels(&self) -> bool {
        *self == Adginten::Channels
    }
    #[doc = "The global DONE flag in ADDR is enabled to generate an interrupt in addition to any individual ADC channels that are enabled to generate interrupts."]
    #[inline(always)]
    pub fn is_global(&self) -> bool {
        *self == Adginten::Global
    }
}
#[doc = "Field `ADGINTEN` writer - Interrupt enable"]
pub type AdgintenW<'a, REG> = crate::BitWriter<'a, REG, Adginten>;
impl<'a, REG> AdgintenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Only the individual ADC channels enabled by ADINTEN7:0 will generate interrupts."]
    #[inline(always)]
    pub fn channels(self) -> &'a mut crate::W<REG> {
        self.variant(Adginten::Channels)
    }
    #[doc = "The global DONE flag in ADDR is enabled to generate an interrupt in addition to any individual ADC channels that are enabled to generate interrupts."]
    #[inline(always)]
    pub fn global(self) -> &'a mut crate::W<REG> {
        self.variant(Adginten::Global)
    }
}
impl R {
    #[doc = "Bit 0 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten0(&self) -> Adinten0R {
        Adinten0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten1(&self) -> Adinten1R {
        Adinten1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten2(&self) -> Adinten2R {
        Adinten2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten3(&self) -> Adinten3R {
        Adinten3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten4(&self) -> Adinten4R {
        Adinten4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten5(&self) -> Adinten5R {
        Adinten5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten6(&self) -> Adinten6R {
        Adinten6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten7(&self) -> Adinten7R {
        Adinten7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Interrupt enable"]
    #[inline(always)]
    pub fn adginten(&self) -> AdgintenR {
        AdgintenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten0(&mut self) -> Adinten0W<'_, IntenSpec> {
        Adinten0W::new(self, 0)
    }
    #[doc = "Bit 1 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten1(&mut self) -> Adinten1W<'_, IntenSpec> {
        Adinten1W::new(self, 1)
    }
    #[doc = "Bit 2 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten2(&mut self) -> Adinten2W<'_, IntenSpec> {
        Adinten2W::new(self, 2)
    }
    #[doc = "Bit 3 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten3(&mut self) -> Adinten3W<'_, IntenSpec> {
        Adinten3W::new(self, 3)
    }
    #[doc = "Bit 4 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten4(&mut self) -> Adinten4W<'_, IntenSpec> {
        Adinten4W::new(self, 4)
    }
    #[doc = "Bit 5 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten5(&mut self) -> Adinten5W<'_, IntenSpec> {
        Adinten5W::new(self, 5)
    }
    #[doc = "Bit 6 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten6(&mut self) -> Adinten6W<'_, IntenSpec> {
        Adinten6W::new(self, 6)
    }
    #[doc = "Bit 7 - Interrupt enable"]
    #[inline(always)]
    pub fn adinten7(&mut self) -> Adinten7W<'_, IntenSpec> {
        Adinten7W::new(self, 7)
    }
    #[doc = "Bit 8 - Interrupt enable"]
    #[inline(always)]
    pub fn adginten(&mut self) -> AdgintenW<'_, IntenSpec> {
        AdgintenW::new(self, 8)
    }
}
#[doc = "A/D Interrupt Enable Register. This register contains enable bits that allow the DONE flag of each A/D channel to be included or excluded from contributing to the generation of an A/D interrupt.\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTEN to value 0x0100"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0x0100;
}
