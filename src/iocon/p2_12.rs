#[doc = "Register `P2_12` reader"]
pub type R = crate::R<P2_12Spec>;
#[doc = "Register `P2_12` writer"]
pub type W = crate::W<P2_12Spec>;
#[doc = "Selects pin function for pin P2\\[12\\]"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Func {
    #[doc = "0: General purpose digital input/output pin. This pin includes a 5 ns input glitch filter."]
    P2_12 = 0,
    #[doc = "1: External interrupt 2 input."]
    Eint2 = 1,
    #[doc = "2: Data line 2 for SD card interface."]
    SdDat2 = 2,
    #[doc = "3: Transmit Word Select. It is driven by the master and received by the slave. Corresponds to the signal WS in the I2S-bus specification."]
    I2sTxWs = 3,
    #[doc = "4: LCD data."]
    LcdVd4 = 4,
    #[doc = "5: LCD data."]
    LcdVd3 = 5,
    #[doc = "6: LCD data."]
    LcdVd8 = 6,
    #[doc = "7: LCD data."]
    LcdVd18 = 7,
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
#[doc = "Field `FUNC` reader - Selects pin function for pin P2\\[12\\]"]
pub type FuncR = crate::FieldReader<Func>;
impl FuncR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Func {
        match self.bits {
            0 => Func::P2_12,
            1 => Func::Eint2,
            2 => Func::SdDat2,
            3 => Func::I2sTxWs,
            4 => Func::LcdVd4,
            5 => Func::LcdVd3,
            6 => Func::LcdVd8,
            7 => Func::LcdVd18,
            _ => unreachable!(),
        }
    }
    #[doc = "General purpose digital input/output pin. This pin includes a 5 ns input glitch filter."]
    #[inline(always)]
    pub fn is_p2_12(&self) -> bool {
        *self == Func::P2_12
    }
    #[doc = "External interrupt 2 input."]
    #[inline(always)]
    pub fn is_eint2(&self) -> bool {
        *self == Func::Eint2
    }
    #[doc = "Data line 2 for SD card interface."]
    #[inline(always)]
    pub fn is_sd_dat_2(&self) -> bool {
        *self == Func::SdDat2
    }
    #[doc = "Transmit Word Select. It is driven by the master and received by the slave. Corresponds to the signal WS in the I2S-bus specification."]
    #[inline(always)]
    pub fn is_i2s_tx_ws(&self) -> bool {
        *self == Func::I2sTxWs
    }
    #[doc = "LCD data."]
    #[inline(always)]
    pub fn is_lcd_vd_4(&self) -> bool {
        *self == Func::LcdVd4
    }
    #[doc = "LCD data."]
    #[inline(always)]
    pub fn is_lcd_vd_3(&self) -> bool {
        *self == Func::LcdVd3
    }
    #[doc = "LCD data."]
    #[inline(always)]
    pub fn is_lcd_vd_8(&self) -> bool {
        *self == Func::LcdVd8
    }
    #[doc = "LCD data."]
    #[inline(always)]
    pub fn is_lcd_vd_18(&self) -> bool {
        *self == Func::LcdVd18
    }
}
#[doc = "Field `FUNC` writer - Selects pin function for pin P2\\[12\\]"]
pub type FuncW<'a, REG> = crate::FieldWriter<'a, REG, 3, Func, crate::Safe>;
impl<'a, REG> FuncW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "General purpose digital input/output pin. This pin includes a 5 ns input glitch filter."]
    #[inline(always)]
    pub fn p2_12(self) -> &'a mut crate::W<REG> {
        self.variant(Func::P2_12)
    }
    #[doc = "External interrupt 2 input."]
    #[inline(always)]
    pub fn eint2(self) -> &'a mut crate::W<REG> {
        self.variant(Func::Eint2)
    }
    #[doc = "Data line 2 for SD card interface."]
    #[inline(always)]
    pub fn sd_dat_2(self) -> &'a mut crate::W<REG> {
        self.variant(Func::SdDat2)
    }
    #[doc = "Transmit Word Select. It is driven by the master and received by the slave. Corresponds to the signal WS in the I2S-bus specification."]
    #[inline(always)]
    pub fn i2s_tx_ws(self) -> &'a mut crate::W<REG> {
        self.variant(Func::I2sTxWs)
    }
    #[doc = "LCD data."]
    #[inline(always)]
    pub fn lcd_vd_4(self) -> &'a mut crate::W<REG> {
        self.variant(Func::LcdVd4)
    }
    #[doc = "LCD data."]
    #[inline(always)]
    pub fn lcd_vd_3(self) -> &'a mut crate::W<REG> {
        self.variant(Func::LcdVd3)
    }
    #[doc = "LCD data."]
    #[inline(always)]
    pub fn lcd_vd_8(self) -> &'a mut crate::W<REG> {
        self.variant(Func::LcdVd8)
    }
    #[doc = "LCD data."]
    #[inline(always)]
    pub fn lcd_vd_18(self) -> &'a mut crate::W<REG> {
        self.variant(Func::LcdVd18)
    }
}
impl R {
    #[doc = "Bits 0:2 - Selects pin function for pin P2\\[12\\]"]
    #[inline(always)]
    pub fn func(&self) -> FuncR {
        FuncR::new((self.bits & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Selects pin function for pin P2\\[12\\]"]
    #[inline(always)]
    pub fn func(&mut self) -> FuncW<'_, P2_12Spec> {
        FuncW::new(self, 0)
    }
}
#[doc = "I/O configuration register for pin P2\\[12\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`p2_12::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p2_12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct P2_12Spec;
impl crate::RegisterSpec for P2_12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`p2_12::R`](R) reader structure"]
impl crate::Readable for P2_12Spec {}
#[doc = "`write(|w| ..)` method takes [`p2_12::W`](W) writer structure"]
impl crate::Writable for P2_12Spec {
    type Safety = crate::Unsafe;
}
