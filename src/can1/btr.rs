#[doc = "Register `BTR` reader"]
pub type R = crate::R<BtrSpec>;
#[doc = "Register `BTR` writer"]
pub type W = crate::W<BtrSpec>;
#[doc = "Field `BRP` reader - Baud Rate Prescaler. The APB clock is divided by (this value plus one) to produce the CAN clock."]
pub type BrpR = crate::FieldReader<u16>;
#[doc = "Field `BRP` writer - Baud Rate Prescaler. The APB clock is divided by (this value plus one) to produce the CAN clock."]
pub type BrpW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SJW` reader - The Synchronization Jump Width is (this value plus one) CAN clocks."]
pub type SjwR = crate::FieldReader;
#[doc = "Field `SJW` writer - The Synchronization Jump Width is (this value plus one) CAN clocks."]
pub type SjwW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TESG1` reader - The delay from the nominal Sync point to the sample point is (this value plus one) CAN clocks."]
pub type Tesg1R = crate::FieldReader;
#[doc = "Field `TESG1` writer - The delay from the nominal Sync point to the sample point is (this value plus one) CAN clocks."]
pub type Tesg1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `TESG2` reader - The delay from the sample point to the next nominal sync point is (this value plus one) CAN clocks. The nominal CAN bit time is (this value plus the value in TSEG1 plus 3) CAN clocks."]
pub type Tesg2R = crate::FieldReader;
#[doc = "Field `TESG2` writer - The delay from the sample point to the next nominal sync point is (this value plus one) CAN clocks. The nominal CAN bit time is (this value plus the value in TSEG1 plus 3) CAN clocks."]
pub type Tesg2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Sampling\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sam {
    #[doc = "0: The bus is sampled once (recommended for high speed buses)"]
    TheBusIsSampledO = 0,
    #[doc = "1: The bus is sampled 3 times (recommended for low to medium speed buses to filter spikes on the bus-line)"]
    TheBusIsSampled3 = 1,
}
impl From<Sam> for bool {
    #[inline(always)]
    fn from(variant: Sam) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SAM` reader - Sampling"]
pub type SamR = crate::BitReader<Sam>;
impl SamR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Sam {
        match self.bits {
            false => Sam::TheBusIsSampledO,
            true => Sam::TheBusIsSampled3,
        }
    }
    #[doc = "The bus is sampled once (recommended for high speed buses)"]
    #[inline(always)]
    pub fn is_the_bus_is_sampled_o(&self) -> bool {
        *self == Sam::TheBusIsSampledO
    }
    #[doc = "The bus is sampled 3 times (recommended for low to medium speed buses to filter spikes on the bus-line)"]
    #[inline(always)]
    pub fn is_the_bus_is_sampled_3(&self) -> bool {
        *self == Sam::TheBusIsSampled3
    }
}
#[doc = "Field `SAM` writer - Sampling"]
pub type SamW<'a, REG> = crate::BitWriter<'a, REG, Sam>;
impl<'a, REG> SamW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The bus is sampled once (recommended for high speed buses)"]
    #[inline(always)]
    pub fn the_bus_is_sampled_o(self) -> &'a mut crate::W<REG> {
        self.variant(Sam::TheBusIsSampledO)
    }
    #[doc = "The bus is sampled 3 times (recommended for low to medium speed buses to filter spikes on the bus-line)"]
    #[inline(always)]
    pub fn the_bus_is_sampled_3(self) -> &'a mut crate::W<REG> {
        self.variant(Sam::TheBusIsSampled3)
    }
}
impl R {
    #[doc = "Bits 0:9 - Baud Rate Prescaler. The APB clock is divided by (this value plus one) to produce the CAN clock."]
    #[inline(always)]
    pub fn brp(&self) -> BrpR {
        BrpR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 14:15 - The Synchronization Jump Width is (this value plus one) CAN clocks."]
    #[inline(always)]
    pub fn sjw(&self) -> SjwR {
        SjwR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:19 - The delay from the nominal Sync point to the sample point is (this value plus one) CAN clocks."]
    #[inline(always)]
    pub fn tesg1(&self) -> Tesg1R {
        Tesg1R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:22 - The delay from the sample point to the next nominal sync point is (this value plus one) CAN clocks. The nominal CAN bit time is (this value plus the value in TSEG1 plus 3) CAN clocks."]
    #[inline(always)]
    pub fn tesg2(&self) -> Tesg2R {
        Tesg2R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - Sampling"]
    #[inline(always)]
    pub fn sam(&self) -> SamR {
        SamR::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Baud Rate Prescaler. The APB clock is divided by (this value plus one) to produce the CAN clock."]
    #[inline(always)]
    pub fn brp(&mut self) -> BrpW<'_, BtrSpec> {
        BrpW::new(self, 0)
    }
    #[doc = "Bits 14:15 - The Synchronization Jump Width is (this value plus one) CAN clocks."]
    #[inline(always)]
    pub fn sjw(&mut self) -> SjwW<'_, BtrSpec> {
        SjwW::new(self, 14)
    }
    #[doc = "Bits 16:19 - The delay from the nominal Sync point to the sample point is (this value plus one) CAN clocks."]
    #[inline(always)]
    pub fn tesg1(&mut self) -> Tesg1W<'_, BtrSpec> {
        Tesg1W::new(self, 16)
    }
    #[doc = "Bits 20:22 - The delay from the sample point to the next nominal sync point is (this value plus one) CAN clocks. The nominal CAN bit time is (this value plus the value in TSEG1 plus 3) CAN clocks."]
    #[inline(always)]
    pub fn tesg2(&mut self) -> Tesg2W<'_, BtrSpec> {
        Tesg2W::new(self, 20)
    }
    #[doc = "Bit 23 - Sampling"]
    #[inline(always)]
    pub fn sam(&mut self) -> SamW<'_, BtrSpec> {
        SamW::new(self, 23)
    }
}
#[doc = "Bus Timing. Can only be written when RM in CANMOD is 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`btr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BtrSpec;
impl crate::RegisterSpec for BtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`btr::R`](R) reader structure"]
impl crate::Readable for BtrSpec {}
#[doc = "`write(|w| ..)` method takes [`btr::W`](W) writer structure"]
impl crate::Writable for BtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BTR to value 0x001c_0000"]
impl crate::Resettable for BtrSpec {
    const RESET_VALUE: u32 = 0x001c_0000;
}
