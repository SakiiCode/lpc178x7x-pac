#[doc = "Register `CANSLEEPCLR` reader"]
pub type R = crate::R<CansleepclrSpec>;
#[doc = "Register `CANSLEEPCLR` writer"]
pub type W = crate::W<CansleepclrSpec>;
#[doc = "Field `CAN1SLEEP` reader - Sleep status and control for CAN channel 1. Read: when 1, indicates that CAN channel 1 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 1."]
pub type Can1sleepR = crate::BitReader;
#[doc = "Field `CAN1SLEEP` writer - Sleep status and control for CAN channel 1. Read: when 1, indicates that CAN channel 1 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 1."]
pub type Can1sleepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN2SLEEP` reader - Sleep status and control for CAN channel 2. Read: when 1, indicates that CAN channel 2 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 2."]
pub type Can2sleepR = crate::BitReader;
#[doc = "Field `CAN2SLEEP` writer - Sleep status and control for CAN channel 2. Read: when 1, indicates that CAN channel 2 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 2."]
pub type Can2sleepW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Sleep status and control for CAN channel 1. Read: when 1, indicates that CAN channel 1 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 1."]
    #[inline(always)]
    pub fn can1sleep(&self) -> Can1sleepR {
        Can1sleepR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Sleep status and control for CAN channel 2. Read: when 1, indicates that CAN channel 2 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 2."]
    #[inline(always)]
    pub fn can2sleep(&self) -> Can2sleepR {
        Can2sleepR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Sleep status and control for CAN channel 1. Read: when 1, indicates that CAN channel 1 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 1."]
    #[inline(always)]
    pub fn can1sleep(&mut self) -> Can1sleepW<'_, CansleepclrSpec> {
        Can1sleepW::new(self, 1)
    }
    #[doc = "Bit 2 - Sleep status and control for CAN channel 2. Read: when 1, indicates that CAN channel 2 is in the sleep mode. Write: writing a 1 causes clocks to be restored to CAN channel 2."]
    #[inline(always)]
    pub fn can2sleep(&mut self) -> Can2sleepW<'_, CansleepclrSpec> {
        Can2sleepW::new(self, 2)
    }
}
#[doc = "Allows clearing the current CAN channel sleep state as well as reading that state.\n\nYou can [`read`](crate::Reg::read) this register and get [`cansleepclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cansleepclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CansleepclrSpec;
impl crate::RegisterSpec for CansleepclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cansleepclr::R`](R) reader structure"]
impl crate::Readable for CansleepclrSpec {}
#[doc = "`write(|w| ..)` method takes [`cansleepclr::W`](W) writer structure"]
impl crate::Writable for CansleepclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CANSLEEPCLR to value 0"]
impl crate::Resettable for CansleepclrSpec {}
