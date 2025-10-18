#[doc = "Register `P2_11` reader"]
pub type R = crate::R<P2_11Spec>;
#[doc = "Register `P2_11` writer"]
pub type W = crate::W<P2_11Spec>;
#[doc = "Selects pin function for pin P2\\[11\\]"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Func {
    #[doc = "0: General purpose digital input/output pin. This pin includes a 5 ns input glitch filter."]
    P2_11 = 0,
    #[doc = "1: External interrupt 1 input."]
    Eint1 = 1,
    #[doc = "2: Data line 1 for SD card interface."]
    SdDat1 = 2,
    #[doc = "3: Transmit Clock. It is driven by the master and received by the slave. Corresponds to the signal SCK in the I2S-bus specification."]
    I2sTxSck = 3,
    #[doc = "7: LCD clock."]
    LcdClkin = 7,
}
impl From<Func> for u8 {
    #[inline(always)]
    fn from(variant: Func) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Func {
    type Ux = u8;
}
impl crate::IsEnum for Func {}
#[doc = "Field `FUNC` reader - Selects pin function for pin P2\\[11\\]"]
pub type FuncR = crate::FieldReader<Func>;
impl FuncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Func> {
        match self.bits {
            0 => Some(Func::P2_11),
            1 => Some(Func::Eint1),
            2 => Some(Func::SdDat1),
            3 => Some(Func::I2sTxSck),
            7 => Some(Func::LcdClkin),
            _ => None,
        }
    }
    #[doc = "General purpose digital input/output pin. This pin includes a 5 ns input glitch filter."]
    #[inline(always)]
    pub fn is_p2_11(&self) -> bool {
        *self == Func::P2_11
    }
    #[doc = "External interrupt 1 input."]
    #[inline(always)]
    pub fn is_eint1(&self) -> bool {
        *self == Func::Eint1
    }
    #[doc = "Data line 1 for SD card interface."]
    #[inline(always)]
    pub fn is_sd_dat_1(&self) -> bool {
        *self == Func::SdDat1
    }
    #[doc = "Transmit Clock. It is driven by the master and received by the slave. Corresponds to the signal SCK in the I2S-bus specification."]
    #[inline(always)]
    pub fn is_i2s_tx_sck(&self) -> bool {
        *self == Func::I2sTxSck
    }
    #[doc = "LCD clock."]
    #[inline(always)]
    pub fn is_lcd_clkin(&self) -> bool {
        *self == Func::LcdClkin
    }
}
#[doc = "Field `FUNC` writer - Selects pin function for pin P2\\[11\\]"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 3, Func>;
impl<'a, REG> FuncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "General purpose digital input/output pin. This pin includes a 5 ns input glitch filter."]
    #[inline(always)]
    pub fn p2_11(self) -> &'a mut crate::W<REG> {
        self.variant(Func::P2_11)
    }
    #[doc = "External interrupt 1 input."]
    #[inline(always)]
    pub fn eint1(self) -> &'a mut crate::W<REG> {
        self.variant(Func::Eint1)
    }
    #[doc = "Data line 1 for SD card interface."]
    #[inline(always)]
    pub fn sd_dat_1(self) -> &'a mut crate::W<REG> {
        self.variant(Func::SdDat1)
    }
    #[doc = "Transmit Clock. It is driven by the master and received by the slave. Corresponds to the signal SCK in the I2S-bus specification."]
    #[inline(always)]
    pub fn i2s_tx_sck(self) -> &'a mut crate::W<REG> {
        self.variant(Func::I2sTxSck)
    }
    #[doc = "LCD clock."]
    #[inline(always)]
    pub fn lcd_clkin(self) -> &'a mut crate::W<REG> {
        self.variant(Func::LcdClkin)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P2\\[11\\]"]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P2\\[11\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FuncW<'_, P2_11Spec> {
        FuncW::new(self, 0)
    }
}
#[doc = "I/O configuration register for pin P2\\[11\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_11::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2_11Spec;
impl crate::RegisterSpec for P2_11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p2_11::R`](R) reader structure"]
impl crate::Readable for P2_11Spec {}
#[doc = "`write(|w| ..)` method takes [`p2_11::W`](W) writer structure"]
impl crate::Writable for P2_11Spec {
    type Safety = crate::Unsafe;
}
