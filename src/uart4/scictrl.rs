#[doc = "Register `SCICTRL` reader"]
pub type R = crate::R<ScictrlSpec>;
#[doc = "Register `SCICTRL` writer"]
pub type W = crate::W<ScictrlSpec>;
#[doc = "Smart Card Interface Enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Smart card interface disabled."]
    SmartCardInterface = 0,
    #[doc = "1: Asynchronous half duplex smart card interface is enabled."]
    AsynchronousHalfDu = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SCIEN` reader - Smart Card Interface Enable."]
pub type ScienR = crate::BitReader<Enum>;
impl ScienR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::SmartCardInterface,
            true => Enum::AsynchronousHalfDu,
        }
    }
    #[doc = "Smart card interface disabled."]
    #[inline(always)]
    pub fn is_smart_card_interface(&self) -> bool {
        *self == Enum::SmartCardInterface
    }
    #[doc = "Asynchronous half duplex smart card interface is enabled."]
    #[inline(always)]
    pub fn is_asynchronous_half_du(&self) -> bool {
        *self == Enum::AsynchronousHalfDu
    }
}
#[doc = "Field `SCIEN` writer - Smart Card Interface Enable."]
pub type ScienW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> ScienW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Smart card interface disabled."]
    #[inline(always)]
    pub fn smart_card_interface(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::SmartCardInterface)
    }
    #[doc = "Asynchronous half duplex smart card interface is enabled."]
    #[inline(always)]
    pub fn asynchronous_half_du(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::AsynchronousHalfDu)
    }
}
#[doc = "NACK response disable. Only applicable in T=0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: A NACK response is enabled."]
    ANackResponseIsE = 0,
    #[doc = "1: A NACK response is inhibited."]
    ANackResponseIsI = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NACKDIS` reader - NACK response disable. Only applicable in T=0."]
pub type NackdisR = crate::BitReader<Enum>;
impl NackdisR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::ANackResponseIsE,
            true => Enum::ANackResponseIsI,
        }
    }
    #[doc = "A NACK response is enabled."]
    #[inline(always)]
    pub fn is_a_nack_response_is_e(&self) -> bool {
        *self == Enum::ANackResponseIsE
    }
    #[doc = "A NACK response is inhibited."]
    #[inline(always)]
    pub fn is_a_nack_response_is_i(&self) -> bool {
        *self == Enum::ANackResponseIsI
    }
}
#[doc = "Field `NACKDIS` writer - NACK response disable. Only applicable in T=0."]
pub type NackdisW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> NackdisW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "A NACK response is enabled."]
    #[inline(always)]
    pub fn a_nack_response_is_e(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::ANackResponseIsE)
    }
    #[doc = "A NACK response is inhibited."]
    #[inline(always)]
    pub fn a_nack_response_is_i(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::ANackResponseIsI)
    }
}
#[doc = "Protocol selection as defined in the ISO7816-3 standard.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: T = 0"]
    TEq0 = 0,
    #[doc = "1: T = 1"]
    TEq1 = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PROTSEL` reader - Protocol selection as defined in the ISO7816-3 standard."]
pub type ProtselR = crate::BitReader<Enum>;
impl ProtselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::TEq0,
            true => Enum::TEq1,
        }
    }
    #[doc = "T = 0"]
    #[inline(always)]
    pub fn is_t_eq_0(&self) -> bool {
        *self == Enum::TEq0
    }
    #[doc = "T = 1"]
    #[inline(always)]
    pub fn is_t_eq_1(&self) -> bool {
        *self == Enum::TEq1
    }
}
#[doc = "Field `PROTSEL` writer - Protocol selection as defined in the ISO7816-3 standard."]
pub type ProtselW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> ProtselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "T = 0"]
    #[inline(always)]
    pub fn t_eq_0(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::TEq0)
    }
    #[doc = "T = 1"]
    #[inline(always)]
    pub fn t_eq_1(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::TEq1)
    }
}
#[doc = "Field `TXRETRY` reader - Maximum number of retransmissions in case of a negative acknowledge (protocol T=0). When the retry counter is exceeded, the USART will be locked until the FIFO is cleared. A TX error interrupt is generated when enabled."]
pub type TxretryR = crate::FieldReader;
#[doc = "Field `TXRETRY` writer - Maximum number of retransmissions in case of a negative acknowledge (protocol T=0). When the retry counter is exceeded, the USART will be locked until the FIFO is cleared. A TX error interrupt is generated when enabled."]
pub type TxretryW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GUARDTIME` reader - Extra guard time. No extra guard time (0x0) results in a standard guard time as defined in ISO 7816-3, depending on the protocol type. A guard time of 0xFF indicates a minimal guard time as defined for the selected protocol."]
pub type GuardtimeR = crate::FieldReader;
#[doc = "Field `GUARDTIME` writer - Extra guard time. No extra guard time (0x0) results in a standard guard time as defined in ISO 7816-3, depending on the protocol type. A guard time of 0xFF indicates a minimal guard time as defined for the selected protocol."]
pub type GuardtimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Smart Card Interface Enable."]
    #[inline(always)]
    pub fn scien(&self) -> ScienR {
        ScienR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NACK response disable. Only applicable in T=0."]
    #[inline(always)]
    pub fn nackdis(&self) -> NackdisR {
        NackdisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Protocol selection as defined in the ISO7816-3 standard."]
    #[inline(always)]
    pub fn protsel(&self) -> ProtselR {
        ProtselR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Maximum number of retransmissions in case of a negative acknowledge (protocol T=0). When the retry counter is exceeded, the USART will be locked until the FIFO is cleared. A TX error interrupt is generated when enabled."]
    #[inline(always)]
    pub fn txretry(&self) -> TxretryR {
        TxretryR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:15 - Extra guard time. No extra guard time (0x0) results in a standard guard time as defined in ISO 7816-3, depending on the protocol type. A guard time of 0xFF indicates a minimal guard time as defined for the selected protocol."]
    #[inline(always)]
    pub fn guardtime(&self) -> GuardtimeR {
        GuardtimeR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Smart Card Interface Enable."]
    #[inline(always)]
    pub fn scien(&mut self) -> ScienW<'_, ScictrlSpec> {
        ScienW::new(self, 0)
    }
    #[doc = "Bit 1 - NACK response disable. Only applicable in T=0."]
    #[inline(always)]
    pub fn nackdis(&mut self) -> NackdisW<'_, ScictrlSpec> {
        NackdisW::new(self, 1)
    }
    #[doc = "Bit 2 - Protocol selection as defined in the ISO7816-3 standard."]
    #[inline(always)]
    pub fn protsel(&mut self) -> ProtselW<'_, ScictrlSpec> {
        ProtselW::new(self, 2)
    }
    #[doc = "Bits 5:7 - Maximum number of retransmissions in case of a negative acknowledge (protocol T=0). When the retry counter is exceeded, the USART will be locked until the FIFO is cleared. A TX error interrupt is generated when enabled."]
    #[inline(always)]
    pub fn txretry(&mut self) -> TxretryW<'_, ScictrlSpec> {
        TxretryW::new(self, 5)
    }
    #[doc = "Bits 8:15 - Extra guard time. No extra guard time (0x0) results in a standard guard time as defined in ISO 7816-3, depending on the protocol type. A guard time of 0xFF indicates a minimal guard time as defined for the selected protocol."]
    #[inline(always)]
    pub fn guardtime(&mut self) -> GuardtimeW<'_, ScictrlSpec> {
        GuardtimeW::new(self, 8)
    }
}
#[doc = "Smart Card Interface control register. Enables and configures the smartcard Interface feature.\n\nYou can [`read`](crate::Reg::read) this register and get [`scictrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scictrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScictrlSpec;
impl crate::RegisterSpec for ScictrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scictrl::R`](R) reader structure"]
impl crate::Readable for ScictrlSpec {}
#[doc = "`write(|w| ..)` method takes [`scictrl::W`](W) writer structure"]
impl crate::Writable for ScictrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCICTRL to value 0"]
impl crate::Resettable for ScictrlSpec {}
