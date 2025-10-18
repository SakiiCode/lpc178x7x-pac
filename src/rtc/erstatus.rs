#[doc = "Register `ERSTATUS` reader"]
pub type R = crate::R<ErstatusSpec>;
#[doc = "Register `ERSTATUS` writer"]
pub type W = crate::W<ErstatusSpec>;
#[doc = "Event flag for channel 0 (RTC_EV0 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: No event change on channel 0."]
    NoEventChangeOnC = 0,
    #[doc = "1: At least one event has occurred on channel 0."]
    AtLeastOneEventH = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV0` reader - Event flag for channel 0 (RTC_EV0 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub type Ev0R = crate::BitReader<Enum>;
impl Ev0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::NoEventChangeOnC,
            true => Enum::AtLeastOneEventH,
        }
    }
    #[doc = "No event change on channel 0."]
    #[inline(always)]
    pub fn is_no_event_change_on_c(&self) -> bool {
        *self == Enum::NoEventChangeOnC
    }
    #[doc = "At least one event has occurred on channel 0."]
    #[inline(always)]
    pub fn is_at_least_one_event_h(&self) -> bool {
        *self == Enum::AtLeastOneEventH
    }
}
#[doc = "Field `EV0` writer - Event flag for channel 0 (RTC_EV0 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub type Ev0W<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Ev0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No event change on channel 0."]
    #[inline(always)]
    pub fn no_event_change_on_c(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::NoEventChangeOnC)
    }
    #[doc = "At least one event has occurred on channel 0."]
    #[inline(always)]
    pub fn at_least_one_event_h(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::AtLeastOneEventH)
    }
}
#[doc = "Event flag for channel 1 (RTC_EV1 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: No event change on channel 1."]
    NoEventChangeOnC = 0,
    #[doc = "1: At least one event has occurred on channel 1."]
    AtLeastOneEventH = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV1` reader - Event flag for channel 1 (RTC_EV1 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub type Ev1R = crate::BitReader<Enum>;
impl Ev1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::NoEventChangeOnC,
            true => Enum::AtLeastOneEventH,
        }
    }
    #[doc = "No event change on channel 1."]
    #[inline(always)]
    pub fn is_no_event_change_on_c(&self) -> bool {
        *self == Enum::NoEventChangeOnC
    }
    #[doc = "At least one event has occurred on channel 1."]
    #[inline(always)]
    pub fn is_at_least_one_event_h(&self) -> bool {
        *self == Enum::AtLeastOneEventH
    }
}
#[doc = "Field `EV1` writer - Event flag for channel 1 (RTC_EV1 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub type Ev1W<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Ev1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No event change on channel 1."]
    #[inline(always)]
    pub fn no_event_change_on_c(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::NoEventChangeOnC)
    }
    #[doc = "At least one event has occurred on channel 1."]
    #[inline(always)]
    pub fn at_least_one_event_h(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::AtLeastOneEventH)
    }
}
#[doc = "Event flag for channel 2 (RTC_EV2 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: No event change on channel 2."]
    NoEventChangeOnC = 0,
    #[doc = "1: At least one event has occurred on channel 2."]
    AtLeastOneEventH = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EV2` reader - Event flag for channel 2 (RTC_EV2 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub type Ev2R = crate::BitReader<Enum>;
impl Ev2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::NoEventChangeOnC,
            true => Enum::AtLeastOneEventH,
        }
    }
    #[doc = "No event change on channel 2."]
    #[inline(always)]
    pub fn is_no_event_change_on_c(&self) -> bool {
        *self == Enum::NoEventChangeOnC
    }
    #[doc = "At least one event has occurred on channel 2."]
    #[inline(always)]
    pub fn is_at_least_one_event_h(&self) -> bool {
        *self == Enum::AtLeastOneEventH
    }
}
#[doc = "Field `EV2` writer - Event flag for channel 2 (RTC_EV2 pin). Set at the end of any second if there has been an event during the preceding second. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub type Ev2W<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> Ev2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No event change on channel 2."]
    #[inline(always)]
    pub fn no_event_change_on_c(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::NoEventChangeOnC)
    }
    #[doc = "At least one event has occurred on channel 2."]
    #[inline(always)]
    pub fn at_least_one_event_h(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::AtLeastOneEventH)
    }
}
#[doc = "General purpose register asynchronous clear flag. This bit is cleared by writing a 1 to it. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: General purpose registers have not been asynchronous cleared."]
    Nogpclr = 0,
    #[doc = "1: General purpose registers have been asynchronous cleared."]
    Gpclr = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GP_CLEARED` reader - General purpose register asynchronous clear flag. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub type GpClearedR = crate::BitReader<Enum>;
impl GpClearedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Nogpclr,
            true => Enum::Gpclr,
        }
    }
    #[doc = "General purpose registers have not been asynchronous cleared."]
    #[inline(always)]
    pub fn is_nogpclr(&self) -> bool {
        *self == Enum::Nogpclr
    }
    #[doc = "General purpose registers have been asynchronous cleared."]
    #[inline(always)]
    pub fn is_gpclr(&self) -> bool {
        *self == Enum::Gpclr
    }
}
#[doc = "Field `GP_CLEARED` writer - General purpose register asynchronous clear flag. This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub type GpClearedW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> GpClearedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "General purpose registers have not been asynchronous cleared."]
    #[inline(always)]
    pub fn nogpclr(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Nogpclr)
    }
    #[doc = "General purpose registers have been asynchronous cleared."]
    #[inline(always)]
    pub fn gpclr(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Gpclr)
    }
}
#[doc = "Interrupt/wakeup request flag (Read-only). This bit is cleared by writing a 1 to it. Writing 0 has no effect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: No interrupt/wakeup request is pending"]
    NoInterruptwakeup_ = 0,
    #[doc = "1: An interrupt/wakeup request is pending."]
    IntwakeupPend = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WAKEUP` reader - Interrupt/wakeup request flag (Read-only). This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub type WakeupR = crate::BitReader<Enum>;
impl WakeupR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::NoInterruptwakeup_,
            true => Enum::IntwakeupPend,
        }
    }
    #[doc = "No interrupt/wakeup request is pending"]
    #[inline(always)]
    pub fn is_no_interruptwakeup_(&self) -> bool {
        *self == Enum::NoInterruptwakeup_
    }
    #[doc = "An interrupt/wakeup request is pending."]
    #[inline(always)]
    pub fn is_intwakeup_pend(&self) -> bool {
        *self == Enum::IntwakeupPend
    }
}
#[doc = "Field `WAKEUP` writer - Interrupt/wakeup request flag (Read-only). This bit is cleared by writing a 1 to it. Writing 0 has no effect."]
pub type WakeupW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> WakeupW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No interrupt/wakeup request is pending"]
    #[inline(always)]
    pub fn no_interruptwakeup_(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::NoInterruptwakeup_)
    }
    #[doc = "An interrupt/wakeup request is pending."]
    #[inline(always)]
    pub fn intwakeup_pend(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::IntwakeupPend)
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
