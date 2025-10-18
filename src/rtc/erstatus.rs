#[doc = "Register `ERSTATUS` reader"]
pub type R = crate::R<ErstatusSpec>;
#[doc = "Register `ERSTATUS` writer"]
pub type W = crate::W<ErstatusSpec>;
#[doc = "Event flag for channel 0 (RTC_EV0 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev0 {
    #[doc = "0: No event change on channel 0."]
    NoEventChangeOnC = 0,
    #[doc = "1: At least one event has occurred on channel 0."]
    AtLeastOneEventH = 1,
}
impl From<Ev0> for bool {
    #[inline(always)]
    fn from(variant: Ev0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV0` reader - Event flag for channel 0 (RTC_EV0 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub type Ev0R = crate::BitReader<Ev0>;
impl Ev0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev0 {
        match self.bits {
            false => Ev0::NoEventChangeOnC,
            true => Ev0::AtLeastOneEventH,
        }
    }
    #[doc = "No event change on channel 0."]
    #[inline(always)]
    pub fn is_no_event_change_on_c(&self) -> bool {
        *self == Ev0::NoEventChangeOnC
    }
    #[doc = "At least one event has occurred on channel 0."]
    #[inline(always)]
    pub fn is_at_least_one_event_h(&self) -> bool {
        *self == Ev0::AtLeastOneEventH
    }
}
#[doc = "Field `EV0` writer - Event flag for channel 0 (RTC_EV0 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub type Ev0W<'a, REG> = crate::BitWriter<'a, REG, Ev0>;
impl<'a, REG> Ev0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No event change on channel 0."]
    #[inline(always)]
    pub fn no_event_change_on_c(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0::NoEventChangeOnC)
    }
    #[doc = "At least one event has occurred on channel 0."]
    #[inline(always)]
    pub fn at_least_one_event_h(self) -> &'a mut crate::W<REG> {
        self.variant(Ev0::AtLeastOneEventH)
    }
}
#[doc = "Event flag for channel 1 (RTC_EV1 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev1 {
    #[doc = "0: No event change on channel 1."]
    NoEventChangeOnC = 0,
    #[doc = "1: At least one event has occurred on channel 1."]
    AtLeastOneEventH = 1,
}
impl From<Ev1> for bool {
    #[inline(always)]
    fn from(variant: Ev1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV1` reader - Event flag for channel 1 (RTC_EV1 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub type Ev1R = crate::BitReader<Ev1>;
impl Ev1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev1 {
        match self.bits {
            false => Ev1::NoEventChangeOnC,
            true => Ev1::AtLeastOneEventH,
        }
    }
    #[doc = "No event change on channel 1."]
    #[inline(always)]
    pub fn is_no_event_change_on_c(&self) -> bool {
        *self == Ev1::NoEventChangeOnC
    }
    #[doc = "At least one event has occurred on channel 1."]
    #[inline(always)]
    pub fn is_at_least_one_event_h(&self) -> bool {
        *self == Ev1::AtLeastOneEventH
    }
}
#[doc = "Field `EV1` writer - Event flag for channel 1 (RTC_EV1 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub type Ev1W<'a, REG> = crate::BitWriter<'a, REG, Ev1>;
impl<'a, REG> Ev1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No event change on channel 1."]
    #[inline(always)]
    pub fn no_event_change_on_c(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1::NoEventChangeOnC)
    }
    #[doc = "At least one event has occurred on channel 1."]
    #[inline(always)]
    pub fn at_least_one_event_h(self) -> &'a mut crate::W<REG> {
        self.variant(Ev1::AtLeastOneEventH)
    }
}
#[doc = "Event flag for channel 2 (RTC_EV2 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ev2 {
    #[doc = "0: No event change on channel 2."]
    NoEventChangeOnC = 0,
    #[doc = "1: At least one event has occurred on channel 2."]
    AtLeastOneEventH = 1,
}
impl From<Ev2> for bool {
    #[inline(always)]
    fn from(variant: Ev2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV2` reader - Event flag for channel 2 (RTC_EV2 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub type Ev2R = crate::BitReader<Ev2>;
impl Ev2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ev2 {
        match self.bits {
            false => Ev2::NoEventChangeOnC,
            true => Ev2::AtLeastOneEventH,
        }
    }
    #[doc = "No event change on channel 2."]
    #[inline(always)]
    pub fn is_no_event_change_on_c(&self) -> bool {
        *self == Ev2::NoEventChangeOnC
    }
    #[doc = "At least one event has occurred on channel 2."]
    #[inline(always)]
    pub fn is_at_least_one_event_h(&self) -> bool {
        *self == Ev2::AtLeastOneEventH
    }
}
#[doc = "Field `EV2` writer - Event flag for channel 2 (RTC_EV2 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub type Ev2W<'a, REG> = crate::BitWriter<'a, REG, Ev2>;
impl<'a, REG> Ev2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No event change on channel 2."]
    #[inline(always)]
    pub fn no_event_change_on_c(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2::NoEventChangeOnC)
    }
    #[doc = "At least one event has occurred on channel 2."]
    #[inline(always)]
    pub fn at_least_one_event_h(self) -> &'a mut crate::W<REG> {
        self.variant(Ev2::AtLeastOneEventH)
    }
}
#[doc = "General purpose register asynchronous clear flag. This bit is cleared by writing a 1 to it. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GpCleared {
    #[doc = "0: General purpose registers have not been asynchronous cleared."]
    Nogpclr = 0,
    #[doc = "1: General purpose registers have been asynchronous cleared."]
    Gpclr = 1,
}
impl From<GpCleared> for bool {
    #[inline(always)]
    fn from(variant: GpCleared) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GP_CLEARED` reader - General purpose register asynchronous clear flag. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub type GpClearedR = crate::BitReader<GpCleared>;
impl GpClearedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GpCleared {
        match self.bits {
            false => GpCleared::Nogpclr,
            true => GpCleared::Gpclr,
        }
    }
    #[doc = "General purpose registers have not been asynchronous cleared."]
    #[inline(always)]
    pub fn is_nogpclr(&self) -> bool {
        *self == GpCleared::Nogpclr
    }
    #[doc = "General purpose registers have been asynchronous cleared."]
    #[inline(always)]
    pub fn is_gpclr(&self) -> bool {
        *self == GpCleared::Gpclr
    }
}
#[doc = "Field `GP_CLEARED` writer - General purpose register asynchronous clear flag. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub type GpClearedW<'a, REG> = crate::BitWriter<'a, REG, GpCleared>;
impl<'a, REG> GpClearedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "General purpose registers have not been asynchronous cleared."]
    #[inline(always)]
    pub fn nogpclr(self) -> &'a mut crate::W<REG> {
        self.variant(GpCleared::Nogpclr)
    }
    #[doc = "General purpose registers have been asynchronous cleared."]
    #[inline(always)]
    pub fn gpclr(self) -> &'a mut crate::W<REG> {
        self.variant(GpCleared::Gpclr)
    }
}
#[doc = "Interrupt/wakeup request flag (Read-only). This bit is cleared by writing a 1 to it. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Wakeup {
    #[doc = "0: No interrupt/wakeup request is pending"]
    NoInterruptwakeup_ = 0,
    #[doc = "1: An interrupt/wakeup request is pending."]
    IntwakeupPend = 1,
}
impl From<Wakeup> for bool {
    #[inline(always)]
    fn from(variant: Wakeup) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP` reader - Interrupt/wakeup request flag (Read-only). This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub type WakeupR = crate::BitReader<Wakeup>;
impl WakeupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Wakeup {
        match self.bits {
            false => Wakeup::NoInterruptwakeup_,
            true => Wakeup::IntwakeupPend,
        }
    }
    #[doc = "No interrupt/wakeup request is pending"]
    #[inline(always)]
    pub fn is_no_interruptwakeup_(&self) -> bool {
        *self == Wakeup::NoInterruptwakeup_
    }
    #[doc = "An interrupt/wakeup request is pending."]
    #[inline(always)]
    pub fn is_intwakeup_pend(&self) -> bool {
        *self == Wakeup::IntwakeupPend
    }
}
#[doc = "Field `WAKEUP` writer - Interrupt/wakeup request flag (Read-only). This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub type WakeupW<'a, REG> = crate::BitWriter<'a, REG, Wakeup>;
impl<'a, REG> WakeupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt/wakeup request is pending"]
    #[inline(always)]
    pub fn no_interruptwakeup_(self) -> &'a mut crate::W<REG> {
        self.variant(Wakeup::NoInterruptwakeup_)
    }
    #[doc = "An interrupt/wakeup request is pending."]
    #[inline(always)]
    pub fn intwakeup_pend(self) -> &'a mut crate::W<REG> {
        self.variant(Wakeup::IntwakeupPend)
    }
}
impl R {
    #[doc = "Bit 0 - Event flag for channel 0 (RTC_EV0 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ev0(&self) -> Ev0R {
        Ev0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Event flag for channel 1 (RTC_EV1 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ev1(&self) -> Ev1R {
        Ev1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Event flag for channel 2 (RTC_EV2 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ev2(&self) -> Ev2R {
        Ev2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - General purpose register asynchronous clear flag. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline(always)]
    pub fn gp_cleared(&self) -> GpClearedR {
        GpClearedR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 31 - Interrupt/wakeup request flag (Read-only). This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline(always)]
    pub fn wakeup(&self) -> WakeupR {
        WakeupR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Event flag for channel 0 (RTC_EV0 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ev0(&mut self) -> Ev0W<'_, ErstatusSpec> {
        Ev0W::new(self, 0)
    }
    #[doc = "Bit 1 - Event flag for channel 1 (RTC_EV1 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ev1(&mut self) -> Ev1W<'_, ErstatusSpec> {
        Ev1W::new(self, 1)
    }
    #[doc = "Bit 2 - Event flag for channel 2 (RTC_EV2 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline(always)]
    pub fn ev2(&mut self) -> Ev2W<'_, ErstatusSpec> {
        Ev2W::new(self, 2)
    }
    #[doc = "Bit 3 - General purpose register asynchronous clear flag. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline(always)]
    pub fn gp_cleared(&mut self) -> GpClearedW<'_, ErstatusSpec> {
        GpClearedW::new(self, 3)
    }
    #[doc = "Bit 31 - Interrupt/wakeup request flag (Read-only). This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
    #[inline(always)]
    pub fn wakeup(&mut self) -> WakeupW<'_, ErstatusSpec> {
        WakeupW::new(self, 31)
    }
}
#[doc = "Event Monitor/Recorder Status register. Contains status flags for event channels and other Event Monitor/Recorder conditions.\n\nYou can [`read`](crate::Reg::read) this register and get [`erstatus::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`erstatus::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ErstatusSpec;
impl crate::RegisterSpec for ErstatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`erstatus::R`](R) reader structure"]
impl crate::Readable for ErstatusSpec {}
#[doc = "`write(|w| ..)` method takes [`erstatus::W`](W) writer structure"]
impl crate::Writable for ErstatusSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ERSTATUS to value 0"]
impl crate::Resettable for ErstatusSpec {}
