#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `RIE` reader - Receiver Interrupt Enable. When the Receive Buffer Status is 'full', the CAN Controller requests the respective interrupt."]
pub type RieR = crate::BitReader;
#[doc = "Field `RIE` writer - Receiver Interrupt Enable. When the Receive Buffer Status is 'full', the CAN Controller requests the respective interrupt."]
pub type RieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE1` reader - Transmit Interrupt Enable for Buffer1. When a message has been successfully transmitted out of TXB1 or Transmit Buffer 1 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
pub type Tie1R = crate::BitReader;
#[doc = "Field `TIE1` writer - Transmit Interrupt Enable for Buffer1. When a message has been successfully transmitted out of TXB1 or Transmit Buffer 1 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
pub type Tie1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EIE` reader - Error Warning Interrupt Enable. If the Error or Bus Status change (see Status Register), the CAN Controller requests the respective interrupt."]
pub type EieR = crate::BitReader;
#[doc = "Field `EIE` writer - Error Warning Interrupt Enable. If the Error or Bus Status change (see Status Register), the CAN Controller requests the respective interrupt."]
pub type EieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOIE` reader - Data Overrun Interrupt Enable. If the Data Overrun Status bit is set (see Status Register), the CAN Controller requests the respective interrupt."]
pub type DoieR = crate::BitReader;
#[doc = "Field `DOIE` writer - Data Overrun Interrupt Enable. If the Data Overrun Status bit is set (see Status Register), the CAN Controller requests the respective interrupt."]
pub type DoieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUIE` reader - Wake-Up Interrupt Enable. If the sleeping CAN controller wakes up, the respective interrupt is requested."]
pub type WuieR = crate::BitReader;
#[doc = "Field `WUIE` writer - Wake-Up Interrupt Enable. If the sleeping CAN controller wakes up, the respective interrupt is requested."]
pub type WuieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPIE` reader - Error Passive Interrupt Enable. If the error status of the CAN Controller changes from error active to error passive or vice versa, the respective interrupt is requested."]
pub type EpieR = crate::BitReader;
#[doc = "Field `EPIE` writer - Error Passive Interrupt Enable. If the error status of the CAN Controller changes from error active to error passive or vice versa, the respective interrupt is requested."]
pub type EpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALIE` reader - Arbitration Lost Interrupt Enable. If the CAN Controller has lost arbitration, the respective interrupt is requested."]
pub type AlieR = crate::BitReader;
#[doc = "Field `ALIE` writer - Arbitration Lost Interrupt Enable. If the CAN Controller has lost arbitration, the respective interrupt is requested."]
pub type AlieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BEIE` reader - Bus Error Interrupt Enable. If a bus error has been detected, the CAN Controller requests the respective interrupt."]
pub type BeieR = crate::BitReader;
#[doc = "Field `BEIE` writer - Bus Error Interrupt Enable. If a bus error has been detected, the CAN Controller requests the respective interrupt."]
pub type BeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDIE` reader - ID Ready Interrupt Enable. When a CAN identifier has been received, the CAN Controller requests the respective interrupt."]
pub type IdieR = crate::BitReader;
#[doc = "Field `IDIE` writer - ID Ready Interrupt Enable. When a CAN identifier has been received, the CAN Controller requests the respective interrupt."]
pub type IdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE2` reader - Transmit Interrupt Enable for Buffer2. When a message has been successfully transmitted out of TXB2 or Transmit Buffer 2 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
pub type Tie2R = crate::BitReader;
#[doc = "Field `TIE2` writer - Transmit Interrupt Enable for Buffer2. When a message has been successfully transmitted out of TXB2 or Transmit Buffer 2 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
pub type Tie2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIE3` reader - Transmit Interrupt Enable for Buffer3. When a message has been successfully transmitted out of TXB3 or Transmit Buffer 3 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
pub type Tie3R = crate::BitReader;
#[doc = "Field `TIE3` writer - Transmit Interrupt Enable for Buffer3. When a message has been successfully transmitted out of TXB3 or Transmit Buffer 3 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
pub type Tie3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receiver Interrupt Enable. When the Receive Buffer Status is 'full', the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn rie(&self) -> RieR {
        RieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Interrupt Enable for Buffer1. When a message has been successfully transmitted out of TXB1 or Transmit Buffer 1 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn tie1(&self) -> Tie1R {
        Tie1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Error Warning Interrupt Enable. If the Error or Bus Status change (see Status Register), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn eie(&self) -> EieR {
        EieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data Overrun Interrupt Enable. If the Data Overrun Status bit is set (see Status Register), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn doie(&self) -> DoieR {
        DoieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Wake-Up Interrupt Enable. If the sleeping CAN controller wakes up, the respective interrupt is requested."]
    #[inline(always)]
    pub fn wuie(&self) -> WuieR {
        WuieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error Passive Interrupt Enable. If the error status of the CAN Controller changes from error active to error passive or vice versa, the respective interrupt is requested."]
    #[inline(always)]
    pub fn epie(&self) -> EpieR {
        EpieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Arbitration Lost Interrupt Enable. If the CAN Controller has lost arbitration, the respective interrupt is requested."]
    #[inline(always)]
    pub fn alie(&self) -> AlieR {
        AlieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus Error Interrupt Enable. If a bus error has been detected, the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn beie(&self) -> BeieR {
        BeieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ID Ready Interrupt Enable. When a CAN identifier has been received, the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn idie(&self) -> IdieR {
        IdieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmit Interrupt Enable for Buffer2. When a message has been successfully transmitted out of TXB2 or Transmit Buffer 2 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn tie2(&self) -> Tie2R {
        Tie2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit Interrupt Enable for Buffer3. When a message has been successfully transmitted out of TXB3 or Transmit Buffer 3 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn tie3(&self) -> Tie3R {
        Tie3R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Interrupt Enable. When the Receive Buffer Status is 'full', the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn rie(&mut self) -> RieW<'_, IerSpec> {
        RieW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Interrupt Enable for Buffer1. When a message has been successfully transmitted out of TXB1 or Transmit Buffer 1 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn tie1(&mut self) -> Tie1W<'_, IerSpec> {
        Tie1W::new(self, 1)
    }
    #[doc = "Bit 2 - Error Warning Interrupt Enable. If the Error or Bus Status change (see Status Register), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn eie(&mut self) -> EieW<'_, IerSpec> {
        EieW::new(self, 2)
    }
    #[doc = "Bit 3 - Data Overrun Interrupt Enable. If the Data Overrun Status bit is set (see Status Register), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn doie(&mut self) -> DoieW<'_, IerSpec> {
        DoieW::new(self, 3)
    }
    #[doc = "Bit 4 - Wake-Up Interrupt Enable. If the sleeping CAN controller wakes up, the respective interrupt is requested."]
    #[inline(always)]
    pub fn wuie(&mut self) -> WuieW<'_, IerSpec> {
        WuieW::new(self, 4)
    }
    #[doc = "Bit 5 - Error Passive Interrupt Enable. If the error status of the CAN Controller changes from error active to error passive or vice versa, the respective interrupt is requested."]
    #[inline(always)]
    pub fn epie(&mut self) -> EpieW<'_, IerSpec> {
        EpieW::new(self, 5)
    }
    #[doc = "Bit 6 - Arbitration Lost Interrupt Enable. If the CAN Controller has lost arbitration, the respective interrupt is requested."]
    #[inline(always)]
    pub fn alie(&mut self) -> AlieW<'_, IerSpec> {
        AlieW::new(self, 6)
    }
    #[doc = "Bit 7 - Bus Error Interrupt Enable. If a bus error has been detected, the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn beie(&mut self) -> BeieW<'_, IerSpec> {
        BeieW::new(self, 7)
    }
    #[doc = "Bit 8 - ID Ready Interrupt Enable. When a CAN identifier has been received, the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn idie(&mut self) -> IdieW<'_, IerSpec> {
        IdieW::new(self, 8)
    }
    #[doc = "Bit 9 - Transmit Interrupt Enable for Buffer2. When a message has been successfully transmitted out of TXB2 or Transmit Buffer 2 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn tie2(&mut self) -> Tie2W<'_, IerSpec> {
        Tie2W::new(self, 9)
    }
    #[doc = "Bit 10 - Transmit Interrupt Enable for Buffer3. When a message has been successfully transmitted out of TXB3 or Transmit Buffer 3 is accessible again (e.g. after an Abort Transmission command), the CAN Controller requests the respective interrupt."]
    #[inline(always)]
    pub fn tie3(&mut self) -> Tie3W<'_, IerSpec> {
        Tie3W::new(self, 10)
    }
}
#[doc = "Interrupt Enable\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
