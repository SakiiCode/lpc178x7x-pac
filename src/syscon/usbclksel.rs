#[doc = "Register `USBCLKSEL` reader"]
pub type R = crate::R<UsbclkselSpec>;
#[doc = "Register `USBCLKSEL` writer"]
pub type W = crate::W<UsbclkselSpec>;
#[doc = "Selects the divide value for creating the USB clock from the selected PLL output. Only the values shown below can produce even number multiples of 48 MHz from the PLL. Warning: Improper setting of this value will result in incorrect operation of the USB interface. Only the main oscillator in conjunction with either PLL0 or PLL1 can provide a clock that meets USB accuracy and jitter specifications. Other values cannot produce the 48 MHz clock required for USB operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usbdiv {
    #[doc = "0: The divider is turned off, no clock will be provided to the USB subsystem."]
    Disabled = 0,
    #[doc = "4: PLL0 output is divided by 4. PLL0 output must be 192 MHz."]
    Div4 = 4,
    #[doc = "6: PLL0 output is divided by 6. PLL0 output must be 288 MHz."]
    Div6 = 6,
}
impl From<Usbdiv> for u8 {
    #[inline(always)]
    fn from(variant: Usbdiv) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usbdiv {
    type Ux = u8;
}
impl crate::IsEnum for Usbdiv {}
#[doc = "Field `USBDIV` reader - Selects the divide value for creating the USB clock from the selected PLL output. Only the values shown below can produce even number multiples of 48 MHz from the PLL. Warning: Improper setting of this value will result in incorrect operation of the USB interface. Only the main oscillator in conjunction with either PLL0 or PLL1 can provide a clock that meets USB accuracy and jitter specifications. Other values cannot produce the 48 MHz clock required for USB operation."]
pub type UsbdivR = crate::FieldReader<Usbdiv>;
impl UsbdivR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Usbdiv> {
        match self.bits {
            0 => Some(Usbdiv::Disabled),
            4 => Some(Usbdiv::Div4),
            6 => Some(Usbdiv::Div6),
            _ => None,
        }
    }
    #[doc = "The divider is turned off, no clock will be provided to the USB subsystem."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Usbdiv::Disabled
    }
    #[doc = "PLL0 output is divided by 4. PLL0 output must be 192 MHz."]
    #[inline(always)]
    pub fn is_div_4(&self) -> bool {
        *self == Usbdiv::Div4
    }
    #[doc = "PLL0 output is divided by 6. PLL0 output must be 288 MHz."]
    #[inline(always)]
    pub fn is_div_6(&self) -> bool {
        *self == Usbdiv::Div6
    }
}
#[doc = "Field `USBDIV` writer - Selects the divide value for creating the USB clock from the selected PLL output. Only the values shown below can produce even number multiples of 48 MHz from the PLL. Warning: Improper setting of this value will result in incorrect operation of the USB interface. Only the main oscillator in conjunction with either PLL0 or PLL1 can provide a clock that meets USB accuracy and jitter specifications. Other values cannot produce the 48 MHz clock required for USB operation."]
pub type UsbdivW<'a, REG> = crate::FieldWriter<'a, REG, 5, Usbdiv>;
impl<'a, REG> UsbdivW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "The divider is turned off, no clock will be provided to the USB subsystem."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Usbdiv::Disabled)
    }
    #[doc = "PLL0 output is divided by 4. PLL0 output must be 192 MHz."]
    #[inline(always)]
    pub fn div_4(self) -> &'a mut crate::W<REG> {
        self.variant(Usbdiv::Div4)
    }
    #[doc = "PLL0 output is divided by 6. PLL0 output must be 288 MHz."]
    #[inline(always)]
    pub fn div_6(self) -> &'a mut crate::W<REG> {
        self.variant(Usbdiv::Div6)
    }
}
#[doc = "Selects the input clock for the USB clock divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Usbsel {
    #[doc = "0: Sysclk is used as the input to the USB clock divider. When this clock is selected, the USB can be accessed by software but cannot perform USB functions."]
    Sysclk = 0,
    #[doc = "1: The output of the Main PLL is used as the input to the USB clock divider."]
    MainPll = 1,
    #[doc = "2: The output of the Alt PLL is used as the input to the USB clock divider."]
    AltPll = 2,
}
impl From<Usbsel> for u8 {
    #[inline(always)]
    fn from(variant: Usbsel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Usbsel {
    type Ux = u8;
}
impl crate::IsEnum for Usbsel {}
#[doc = "Field `USBSEL` reader - Selects the input clock for the USB clock divider."]
pub type UsbselR = crate::FieldReader<Usbsel>;
impl UsbselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Usbsel> {
        match self.bits {
            0 => Some(Usbsel::Sysclk),
            1 => Some(Usbsel::MainPll),
            2 => Some(Usbsel::AltPll),
            _ => None,
        }
    }
    #[doc = "Sysclk is used as the input to the USB clock divider. When this clock is selected, the USB can be accessed by software but cannot perform USB functions."]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == Usbsel::Sysclk
    }
    #[doc = "The output of the Main PLL is used as the input to the USB clock divider."]
    #[inline(always)]
    pub fn is_main_pll(&self) -> bool {
        *self == Usbsel::MainPll
    }
    #[doc = "The output of the Alt PLL is used as the input to the USB clock divider."]
    #[inline(always)]
    pub fn is_alt_pll(&self) -> bool {
        *self == Usbsel::AltPll
    }
}
#[doc = "Field `USBSEL` writer - Selects the input clock for the USB clock divider."]
pub type UsbselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Usbsel>;
impl<'a, REG> UsbselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Sysclk is used as the input to the USB clock divider. When this clock is selected, the USB can be accessed by software but cannot perform USB functions."]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(Usbsel::Sysclk)
    }
    #[doc = "The output of the Main PLL is used as the input to the USB clock divider."]
    #[inline(always)]
    pub fn main_pll(self) -> &'a mut crate::W<REG> {
        self.variant(Usbsel::MainPll)
    }
    #[doc = "The output of the Alt PLL is used as the input to the USB clock divider."]
    #[inline(always)]
    pub fn alt_pll(self) -> &'a mut crate::W<REG> {
        self.variant(Usbsel::AltPll)
    }
}
impl R {
    #[doc = "Bits 0:4 - Selects the divide value for creating the USB clock from the selected PLL output. Only the values shown below can produce even number multiples of 48 MHz from the PLL. Warning: Improper setting of this value will result in incorrect operation of the USB interface. Only the main oscillator in conjunction with either PLL0 or PLL1 can provide a clock that meets USB accuracy and jitter specifications. Other values cannot produce the 48 MHz clock required for USB operation."]
    #[inline(always)]
    pub fn usbdiv(&self) -> UsbdivR {
        UsbdivR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:9 - Selects the input clock for the USB clock divider."]
    #[inline(always)]
    pub fn usbsel(&self) -> UsbselR {
        UsbselR::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects the divide value for creating the USB clock from the selected PLL output. Only the values shown below can produce even number multiples of 48 MHz from the PLL. Warning: Improper setting of this value will result in incorrect operation of the USB interface. Only the main oscillator in conjunction with either PLL0 or PLL1 can provide a clock that meets USB accuracy and jitter specifications. Other values cannot produce the 48 MHz clock required for USB operation."]
    #[inline(always)]
    pub fn usbdiv(&mut self) -> UsbdivW<'_, UsbclkselSpec> {
        UsbdivW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Selects the input clock for the USB clock divider."]
    #[inline(always)]
    pub fn usbsel(&mut self) -> UsbselW<'_, UsbclkselSpec> {
        UsbselW::new(self, 8)
    }
}
#[doc = "USB Clock Selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbclksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbclksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UsbclkselSpec;
impl crate::RegisterSpec for UsbclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usbclksel::R`](R) reader structure"]
impl crate::Readable for UsbclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`usbclksel::W`](W) writer structure"]
impl crate::Writable for UsbclkselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USBCLKSEL to value 0"]
impl crate::Resettable for UsbclkselSpec {}
