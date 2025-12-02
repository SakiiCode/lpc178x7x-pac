#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "RBR Interrupt Enable. Enables the Receive Data Available interrupt for UARTn. It also controls the Character Receive Time-out interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rbrie {
    #[doc = "0: Disable the RDA interrupts."]
    Disabled = 0,
    #[doc = "1: Enable the RDA interrupts."]
    Enabled = 1,
}
impl From<Rbrie> for bool {
    #[inline(always)]
    fn from(variant: Rbrie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RBRIE` reader - RBR Interrupt Enable. Enables the Receive Data Available interrupt for UARTn. It also controls the Character Receive Time-out interrupt."]
pub type RbrieR = crate::BitReader<Rbrie>;
impl RbrieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rbrie {
        match self.bits {
            false => Rbrie::Disabled,
            true => Rbrie::Enabled,
        }
    }
    #[doc = "Disable the RDA interrupts."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rbrie::Disabled
    }
    #[doc = "Enable the RDA interrupts."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rbrie::Enabled
    }
}
#[doc = "Field `RBRIE` writer - RBR Interrupt Enable. Enables the Receive Data Available interrupt for UARTn. It also controls the Character Receive Time-out interrupt."]
pub type RbrieW<'a, REG> = crate::BitWriter<'a, REG, Rbrie>;
impl<'a, REG> RbrieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the RDA interrupts."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rbrie::Disabled)
    }
    #[doc = "Enable the RDA interrupts."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rbrie::Enabled)
    }
}
#[doc = "THRE Interrupt Enable. Enables the THRE interrupt for UARTn. The status of this can be read from UnLSR\\[5\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Threie {
    #[doc = "0: Disable the THRE interrupts."]
    Disabled = 0,
    #[doc = "1: Enable the THRE interrupts."]
    Enabled = 1,
}
impl From<Threie> for bool {
    #[inline(always)]
    fn from(variant: Threie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `THREIE` reader - THRE Interrupt Enable. Enables the THRE interrupt for UARTn. The status of this can be read from UnLSR\\[5\\]."]
pub type ThreieR = crate::BitReader<Threie>;
impl ThreieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Threie {
        match self.bits {
            false => Threie::Disabled,
            true => Threie::Enabled,
        }
    }
    #[doc = "Disable the THRE interrupts."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Threie::Disabled
    }
    #[doc = "Enable the THRE interrupts."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Threie::Enabled
    }
}
#[doc = "Field `THREIE` writer - THRE Interrupt Enable. Enables the THRE interrupt for UARTn. The status of this can be read from UnLSR\\[5\\]."]
pub type ThreieW<'a, REG> = crate::BitWriter<'a, REG, Threie>;
impl<'a, REG> ThreieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the THRE interrupts."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Threie::Disabled)
    }
    #[doc = "Enable the THRE interrupts."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Threie::Enabled)
    }
}
#[doc = "RX Line Status Interrupt Enable. Enables the UARTn RX line status interrupts. The status of this interrupt can be read from UnLSR\\[4:1\\].\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxie {
    #[doc = "0: Disable the RX line status interrupts."]
    Disabled = 0,
    #[doc = "1: Enable the RX line status interrupts."]
    Enabled = 1,
}
impl From<Rxie> for bool {
    #[inline(always)]
    fn from(variant: Rxie) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXIE` reader - RX Line Status Interrupt Enable. Enables the UARTn RX line status interrupts. The status of this interrupt can be read from UnLSR\\[4:1\\]."]
pub type RxieR = crate::BitReader<Rxie>;
impl RxieR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxie {
        match self.bits {
            false => Rxie::Disabled,
            true => Rxie::Enabled,
        }
    }
    #[doc = "Disable the RX line status interrupts."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxie::Disabled
    }
    #[doc = "Enable the RX line status interrupts."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxie::Enabled
    }
}
#[doc = "Field `RXIE` writer - RX Line Status Interrupt Enable. Enables the UARTn RX line status interrupts. The status of this interrupt can be read from UnLSR\\[4:1\\]."]
pub type RxieW<'a, REG> = crate::BitWriter<'a, REG, Rxie>;
impl<'a, REG> RxieW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable the RX line status interrupts."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxie::Disabled)
    }
    #[doc = "Enable the RX line status interrupts."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Rxie::Enabled)
    }
}
#[doc = "Enables the end of auto-baud interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Abeointen {
    #[doc = "0: Disable end of auto-baud Interrupt."]
    Disabled = 0,
    #[doc = "1: Enable end of auto-baud Interrupt."]
    Enabled = 1,
}
impl From<Abeointen> for bool {
    #[inline(always)]
    fn from(variant: Abeointen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABEOINTEN` reader - Enables the end of auto-baud interrupt."]
pub type AbeointenR = crate::BitReader<Abeointen>;
impl AbeointenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Abeointen {
        match self.bits {
            false => Abeointen::Disabled,
            true => Abeointen::Enabled,
        }
    }
    #[doc = "Disable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Abeointen::Disabled
    }
    #[doc = "Enable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Abeointen::Enabled
    }
}
#[doc = "Field `ABEOINTEN` writer - Enables the end of auto-baud interrupt."]
pub type AbeointenW<'a, REG> = crate::BitWriter<'a, REG, Abeointen>;
impl<'a, REG> AbeointenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Abeointen::Disabled)
    }
    #[doc = "Enable end of auto-baud Interrupt."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Abeointen::Enabled)
    }
}
#[doc = "Enables the auto-baud time-out interrupt.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Abtointen {
    #[doc = "0: Disable auto-baud time-out Interrupt."]
    Disabled = 0,
    #[doc = "1: Enable auto-baud time-out Interrupt."]
    Enabled = 1,
}
impl From<Abtointen> for bool {
    #[inline(always)]
    fn from(variant: Abtointen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABTOINTEN` reader - Enables the auto-baud time-out interrupt."]
pub type AbtointenR = crate::BitReader<Abtointen>;
impl AbtointenR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Abtointen {
        match self.bits {
            false => Abtointen::Disabled,
            true => Abtointen::Enabled,
        }
    }
    #[doc = "Disable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Abtointen::Disabled
    }
    #[doc = "Enable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Abtointen::Enabled
    }
}
#[doc = "Field `ABTOINTEN` writer - Enables the auto-baud time-out interrupt."]
pub type AbtointenW<'a, REG> = crate::BitWriter<'a, REG, Abtointen>;
impl<'a, REG> AbtointenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Abtointen::Disabled)
    }
    #[doc = "Enable auto-baud time-out Interrupt."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Abtointen::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt for UARTn. It also controls the Character Receive Time-out interrupt."]
    #[inline(always)]
    pub fn rbrie(&self) -> RbrieR {
        RbrieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt for UARTn. The status of this can be read from UnLSR\\[5\\]."]
    #[inline(always)]
    pub fn threie(&self) -> ThreieR {
        ThreieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Line Status Interrupt Enable. Enables the UARTn RX line status interrupts. The status of this interrupt can be read from UnLSR\\[4:1\\]."]
    #[inline(always)]
    pub fn rxie(&self) -> RxieR {
        RxieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline(always)]
    pub fn abeointen(&self) -> AbeointenR {
        AbeointenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline(always)]
    pub fn abtointen(&self) -> AbtointenR {
        AbtointenR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RBR Interrupt Enable. Enables the Receive Data Available interrupt for UARTn. It also controls the Character Receive Time-out interrupt."]
    #[inline(always)]
    pub fn rbrie(&mut self) -> RbrieW<'_, IerSpec> {
        RbrieW::new(self, 0)
    }
    #[doc = "Bit 1 - THRE Interrupt Enable. Enables the THRE interrupt for UARTn. The status of this can be read from UnLSR\\[5\\]."]
    #[inline(always)]
    pub fn threie(&mut self) -> ThreieW<'_, IerSpec> {
        ThreieW::new(self, 1)
    }
    #[doc = "Bit 2 - RX Line Status Interrupt Enable. Enables the UARTn RX line status interrupts. The status of this interrupt can be read from UnLSR\\[4:1\\]."]
    #[inline(always)]
    pub fn rxie(&mut self) -> RxieW<'_, IerSpec> {
        RxieW::new(self, 2)
    }
    #[doc = "Bit 8 - Enables the end of auto-baud interrupt."]
    #[inline(always)]
    pub fn abeointen(&mut self) -> AbeointenW<'_, IerSpec> {
        AbeointenW::new(self, 8)
    }
    #[doc = "Bit 9 - Enables the auto-baud time-out interrupt."]
    #[inline(always)]
    pub fn abtointen(&mut self) -> AbtointenW<'_, IerSpec> {
        AbtointenW::new(self, 9)
    }
}
#[doc = "Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential UART interrupts (DLAB =0).\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
