#[doc = "Register `CANWAKEFLAGS` reader"]
pub type R = crate::R<CanwakeflagsSpec>;
#[doc = "Register `CANWAKEFLAGS` writer"]
pub type W = crate::W<CanwakeflagsSpec>;
#[doc = "Field `CAN1WAKE` reader - Wake-up status for CAN channel 1. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 1. Write: writing a 1 clears this bit."]
pub type Can1wakeR = crate::BitReader;
#[doc = "Field `CAN1WAKE` writer - Wake-up status for CAN channel 1. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 1. Write: writing a 1 clears this bit."]
pub type Can1wakeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAN2WAKE` reader - Wake-up status for CAN channel 2. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 2. Write: writing a 1 clears this bit."]
pub type Can2wakeR = crate::BitReader;
#[doc = "Field `CAN2WAKE` writer - Wake-up status for CAN channel 2. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 2. Write: writing a 1 clears this bit."]
pub type Can2wakeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Wake-up status for CAN channel 1. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 1. Write: writing a 1 clears this bit."]
    #[inline(always)]
    pub fn can1wake(&self) -> Can1wakeR {
        Can1wakeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Wake-up status for CAN channel 2. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 2. Write: writing a 1 clears this bit."]
    #[inline(always)]
    pub fn can2wake(&self) -> Can2wakeR {
        Can2wakeR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Wake-up status for CAN channel 1. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 1. Write: writing a 1 clears this bit."]
    #[inline(always)]
    pub fn can1wake(&mut self) -> Can1wakeW<'_, CanwakeflagsSpec> {
        Can1wakeW::new(self, 1)
    }
    #[doc = "Bit 2 - Wake-up status for CAN channel 2. Read: when 1, indicates that a falling edge has occurred on the receive data line of CAN channel 2. Write: writing a 1 clears this bit."]
    #[inline(always)]
    pub fn can2wake(&mut self) -> Can2wakeW<'_, CanwakeflagsSpec> {
        Can2wakeW::new(self, 2)
    }
}
#[doc = "Allows reading the wake-up state of the CAN channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`canwakeflags::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canwakeflags::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanwakeflagsSpec;
impl crate::RegisterSpec for CanwakeflagsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`canwakeflags::R`](R) reader structure"]
impl crate::Readable for CanwakeflagsSpec {}
#[doc = "`write(|w| ..)` method takes [`canwakeflags::W`](W) writer structure"]
impl crate::Writable for CanwakeflagsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CANWAKEFLAGS to value 0"]
impl crate::Resettable for CanwakeflagsSpec {}
