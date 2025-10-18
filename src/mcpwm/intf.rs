#[doc = "Register `INTF` reader"]
pub type R = crate::R<IntfSpec>;
#[doc = "Limit interrupt flag for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ilim0F {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    ThisInterruptSourc = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IfTheCorresponding = 1,
}
impl From<Ilim0F> for bool {
    #[inline(always)]
    fn from(variant: Ilim0F) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILIM0_F` reader - Limit interrupt flag for channel 0."]
pub type Ilim0FR = crate::BitReader<Ilim0F>;
impl Ilim0FR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ilim0F {
        match self.bits {
            false => Ilim0F::ThisInterruptSourc,
            true => Ilim0F::IfTheCorresponding,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == Ilim0F::ThisInterruptSourc
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == Ilim0F::IfTheCorresponding
    }
}
#[doc = "Match interrupt flag for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Imat0F {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    ThisInterruptSourc = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IfTheCorresponding = 1,
}
impl From<Imat0F> for bool {
    #[inline(always)]
    fn from(variant: Imat0F) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMAT0_F` reader - Match interrupt flag for channel 0."]
pub type Imat0FR = crate::BitReader<Imat0F>;
impl Imat0FR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Imat0F {
        match self.bits {
            false => Imat0F::ThisInterruptSourc,
            true => Imat0F::IfTheCorresponding,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == Imat0F::ThisInterruptSourc
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == Imat0F::IfTheCorresponding
    }
}
#[doc = "Capture interrupt flag for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icap0F {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    ThisInterruptSourc = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IfTheCorresponding = 1,
}
impl From<Icap0F> for bool {
    #[inline(always)]
    fn from(variant: Icap0F) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICAP0_F` reader - Capture interrupt flag for channel 0."]
pub type Icap0FR = crate::BitReader<Icap0F>;
impl Icap0FR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Icap0F {
        match self.bits {
            false => Icap0F::ThisInterruptSourc,
            true => Icap0F::IfTheCorresponding,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == Icap0F::ThisInterruptSourc
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == Icap0F::IfTheCorresponding
    }
}
#[doc = "Limit interrupt flag for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ilim1F {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    ThisInterruptSourc = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IfTheCorresponding = 1,
}
impl From<Ilim1F> for bool {
    #[inline(always)]
    fn from(variant: Ilim1F) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILIM1_F` reader - Limit interrupt flag for channel 1."]
pub type Ilim1FR = crate::BitReader<Ilim1F>;
impl Ilim1FR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ilim1F {
        match self.bits {
            false => Ilim1F::ThisInterruptSourc,
            true => Ilim1F::IfTheCorresponding,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == Ilim1F::ThisInterruptSourc
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == Ilim1F::IfTheCorresponding
    }
}
#[doc = "Match interrupt flag for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Imat1F {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    ThisInterruptSourc = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IfTheCorresponding = 1,
}
impl From<Imat1F> for bool {
    #[inline(always)]
    fn from(variant: Imat1F) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMAT1_F` reader - Match interrupt flag for channel 1."]
pub type Imat1FR = crate::BitReader<Imat1F>;
impl Imat1FR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Imat1F {
        match self.bits {
            false => Imat1F::ThisInterruptSourc,
            true => Imat1F::IfTheCorresponding,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == Imat1F::ThisInterruptSourc
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == Imat1F::IfTheCorresponding
    }
}
#[doc = "Capture interrupt flag for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icap1F {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    ThisInterruptSourc = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IfTheCorresponding = 1,
}
impl From<Icap1F> for bool {
    #[inline(always)]
    fn from(variant: Icap1F) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICAP1_F` reader - Capture interrupt flag for channel 1."]
pub type Icap1FR = crate::BitReader<Icap1F>;
impl Icap1FR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Icap1F {
        match self.bits {
            false => Icap1F::ThisInterruptSourc,
            true => Icap1F::IfTheCorresponding,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == Icap1F::ThisInterruptSourc
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == Icap1F::IfTheCorresponding
    }
}
#[doc = "Limit interrupt flag for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ilim2F {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    ThisInterruptSourc = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IfTheCorresponding = 1,
}
impl From<Ilim2F> for bool {
    #[inline(always)]
    fn from(variant: Ilim2F) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ILIM2_F` reader - Limit interrupt flag for channel 2."]
pub type Ilim2FR = crate::BitReader<Ilim2F>;
impl Ilim2FR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ilim2F {
        match self.bits {
            false => Ilim2F::ThisInterruptSourc,
            true => Ilim2F::IfTheCorresponding,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == Ilim2F::ThisInterruptSourc
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == Ilim2F::IfTheCorresponding
    }
}
#[doc = "Match interrupt flag for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Imat2F {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    ThisInterruptSourc = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IfTheCorresponding = 1,
}
impl From<Imat2F> for bool {
    #[inline(always)]
    fn from(variant: Imat2F) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IMAT2_F` reader - Match interrupt flag for channel 2."]
pub type Imat2FR = crate::BitReader<Imat2F>;
impl Imat2FR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Imat2F {
        match self.bits {
            false => Imat2F::ThisInterruptSourc,
            true => Imat2F::IfTheCorresponding,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == Imat2F::ThisInterruptSourc
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == Imat2F::IfTheCorresponding
    }
}
#[doc = "Capture interrupt flag for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Icap2F {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    ThisInterruptSourc = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IfTheCorresponding = 1,
}
impl From<Icap2F> for bool {
    #[inline(always)]
    fn from(variant: Icap2F) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ICAP2_F` reader - Capture interrupt flag for channel 2."]
pub type Icap2FR = crate::BitReader<Icap2F>;
impl Icap2FR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Icap2F {
        match self.bits {
            false => Icap2F::ThisInterruptSourc,
            true => Icap2F::IfTheCorresponding,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == Icap2F::ThisInterruptSourc
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == Icap2F::IfTheCorresponding
    }
}
#[doc = "Fast abort interrupt flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AbortF {
    #[doc = "0: This interrupt source is not contributing to the MCPWM interrupt request."]
    ThisInterruptSourc = 0,
    #[doc = "1: If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    IfTheCorresponding = 1,
}
impl From<AbortF> for bool {
    #[inline(always)]
    fn from(variant: AbortF) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ABORT_F` reader - Fast abort interrupt flag."]
pub type AbortFR = crate::BitReader<AbortF>;
impl AbortFR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> AbortF {
        match self.bits {
            false => AbortF::ThisInterruptSourc,
            true => AbortF::IfTheCorresponding,
        }
    }
    #[doc = "This interrupt source is not contributing to the MCPWM interrupt request."]
    #[inline(always)]
    pub fn is_this_interrupt_sourc(&self) -> bool {
        *self == AbortF::ThisInterruptSourc
    }
    #[doc = "If the corresponding bit in INTEN is 1, the MCPWM module is asserting its interrupt request to the Interrupt Controller."]
    #[inline(always)]
    pub fn is_if_the_corresponding(&self) -> bool {
        *self == AbortF::IfTheCorresponding
    }
}
impl R {
    #[doc = "Bit 0 - Limit interrupt flag for channel 0."]
    #[inline(always)]
    pub fn ilim0_f(&self) -> Ilim0FR {
        Ilim0FR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Match interrupt flag for channel 0."]
    #[inline(always)]
    pub fn imat0_f(&self) -> Imat0FR {
        Imat0FR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Capture interrupt flag for channel 0."]
    #[inline(always)]
    pub fn icap0_f(&self) -> Icap0FR {
        Icap0FR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Limit interrupt flag for channel 1."]
    #[inline(always)]
    pub fn ilim1_f(&self) -> Ilim1FR {
        Ilim1FR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Match interrupt flag for channel 1."]
    #[inline(always)]
    pub fn imat1_f(&self) -> Imat1FR {
        Imat1FR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Capture interrupt flag for channel 1."]
    #[inline(always)]
    pub fn icap1_f(&self) -> Icap1FR {
        Icap1FR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Limit interrupt flag for channel 2."]
    #[inline(always)]
    pub fn ilim2_f(&self) -> Ilim2FR {
        Ilim2FR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Match interrupt flag for channel 2."]
    #[inline(always)]
    pub fn imat2_f(&self) -> Imat2FR {
        Imat2FR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Capture interrupt flag for channel 2."]
    #[inline(always)]
    pub fn icap2_f(&self) -> Icap2FR {
        Icap2FR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 15 - Fast abort interrupt flag."]
    #[inline(always)]
    pub fn abort_f(&self) -> AbortFR {
        AbortFR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Interrupt flags read address\n\nYou can [`read`](crate::Reg::read) this register and get [`intf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfSpec;
impl crate::RegisterSpec for IntfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf::R`](R) reader structure"]
impl crate::Readable for IntfSpec {}
#[doc = "`reset()` method sets INTF to value 0"]
impl crate::Resettable for IntfSpec {}
