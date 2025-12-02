#[doc = "Register `CCLKSEL` reader"]
pub type R = crate::R<CclkselSpec>;
#[doc = "Register `CCLKSEL` writer"]
pub type W = crate::W<CclkselSpec>;
#[doc = "Field `CCLKDIV` reader - Selects the divide value for creating the CPU clock (CCLK) from the selected clock source. 0 = The divider is turned off., no clock will be provided to the CPU. This setting should typically not be used, the CPU will be halted and a reset will be required to restore operation. 1 = The input clock is divided by 1 to produce the CPU clock. 2 = The input clock is divided by 2 to produce the CPU clock. 3 = The input clock is divided by 3 to produce the CPU clock. ... 31 = The input clock is divided by 31 to produce the CPU clock."]
pub type CclkdivR = crate::FieldReader;
#[doc = "Field `CCLKDIV` writer - Selects the divide value for creating the CPU clock (CCLK) from the selected clock source. 0 = The divider is turned off., no clock will be provided to the CPU. This setting should typically not be used, the CPU will be halted and a reset will be required to restore operation. 1 = The input clock is divided by 1 to produce the CPU clock. 2 = The input clock is divided by 2 to produce the CPU clock. 3 = The input clock is divided by 3 to produce the CPU clock. ... 31 = The input clock is divided by 31 to produce the CPU clock."]
pub type CclkdivW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Selects the input clock for the CPU clock divider.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cclksel {
    #[doc = "0: Sysclk is used as the input to the CPU clock divider."]
    Sysclk = 0,
    #[doc = "1: The output of the Main PLL is used as the input to the CPU clock divider."]
    MainPll = 1,
}
impl From<Cclksel> for bool {
    #[inline(always)]
    fn from(variant: Cclksel) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CCLKSEL` reader - Selects the input clock for the CPU clock divider."]
pub type CclkselR = crate::BitReader<Cclksel>;
impl CclkselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Cclksel {
        match self.bits {
            false => Cclksel::Sysclk,
            true => Cclksel::MainPll,
        }
    }
    #[doc = "Sysclk is used as the input to the CPU clock divider."]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == Cclksel::Sysclk
    }
    #[doc = "The output of the Main PLL is used as the input to the CPU clock divider."]
    #[inline(always)]
    pub fn is_main_pll(&self) -> bool {
        *self == Cclksel::MainPll
    }
}
#[doc = "Field `CCLKSEL` writer - Selects the input clock for the CPU clock divider."]
pub type CclkselW<'a, REG> = crate::BitWriter<'a, REG, Cclksel>;
impl<'a, REG> CclkselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Sysclk is used as the input to the CPU clock divider."]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut crate::W<REG> {
        self.variant(Cclksel::Sysclk)
    }
    #[doc = "The output of the Main PLL is used as the input to the CPU clock divider."]
    #[inline(always)]
    pub fn main_pll(self) -> &'a mut crate::W<REG> {
        self.variant(Cclksel::MainPll)
    }
}
impl R {
    #[doc = "Bits 0:4 - Selects the divide value for creating the CPU clock (CCLK) from the selected clock source. 0 = The divider is turned off., no clock will be provided to the CPU. This setting should typically not be used, the CPU will be halted and a reset will be required to restore operation. 1 = The input clock is divided by 1 to produce the CPU clock. 2 = The input clock is divided by 2 to produce the CPU clock. 3 = The input clock is divided by 3 to produce the CPU clock. ... 31 = The input clock is divided by 31 to produce the CPU clock."]
    #[inline(always)]
    pub fn cclkdiv(&self) -> CclkdivR {
        CclkdivR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 8 - Selects the input clock for the CPU clock divider."]
    #[inline(always)]
    pub fn cclksel(&self) -> CclkselR {
        CclkselR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Selects the divide value for creating the CPU clock (CCLK) from the selected clock source. 0 = The divider is turned off., no clock will be provided to the CPU. This setting should typically not be used, the CPU will be halted and a reset will be required to restore operation. 1 = The input clock is divided by 1 to produce the CPU clock. 2 = The input clock is divided by 2 to produce the CPU clock. 3 = The input clock is divided by 3 to produce the CPU clock. ... 31 = The input clock is divided by 31 to produce the CPU clock."]
    #[inline(always)]
    pub fn cclkdiv(&mut self) -> CclkdivW<'_, CclkselSpec> {
        CclkdivW::new(self, 0)
    }
    #[doc = "Bit 8 - Selects the input clock for the CPU clock divider."]
    #[inline(always)]
    pub fn cclksel(&mut self) -> CclkselW<'_, CclkselSpec> {
        CclkselW::new(self, 8)
    }
}
#[doc = "CPU Clock Selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`cclksel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cclksel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CclkselSpec;
impl crate::RegisterSpec for CclkselSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cclksel::R`](R) reader structure"]
impl crate::Readable for CclkselSpec {}
#[doc = "`write(|w| ..)` method takes [`cclksel::W`](W) writer structure"]
impl crate::Writable for CclkselSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCLKSEL to value 0"]
impl crate::Resettable for CclkselSpec {}
