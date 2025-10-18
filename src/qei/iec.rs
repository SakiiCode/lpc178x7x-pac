#[doc = "Register `IEC` writer"]
pub type W = crate::W<IecSpec>;
#[doc = "Field `INX_INT` writer - Writing a 1 disables the INX_Int interrupt in the QEIIE register."]
pub type InxIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM_INT` writer - Writing a 1 disables the TIN_Int interrupt in the QEIIE register."]
pub type TimIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VELC_INT` writer - Writing a 1 disables the VELC_Int interrupt in the QEIIE register."]
pub type VelcIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR_INT` writer - Writing a 1 disables the DIR_Int interrupt in the QEIIE register."]
pub type DirIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_INT` writer - Writing a 1 disables the ERR_Int interrupt in the QEIIE register."]
pub type ErrIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENCLK_INT` writer - Writing a 1 disables the ENCLK_Int interrupt in the QEIIE register."]
pub type EnclkIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS0_INT` writer - Writing a 1 disables the POS0_Int interrupt in the QEIIE register."]
pub type Pos0IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS1_INT` writer - Writing a 1 disables the POS1_Int interrupt in the QEIIE register."]
pub type Pos1IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS2_INT` writer - Writing a 1 disables the POS2_Int interrupt in the QEIIE register."]
pub type Pos2IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV0_INT` writer - Writing a 1 disables the REV0_Int interrupt in the QEIIE register."]
pub type Rev0IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS0REV_INT` writer - Writing a 1 disables the POS0REV_Int interrupt in the QEIIE register."]
pub type Pos0revIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS1REV_INT` writer - Writing a 1 disables the POS1REV_Int interrupt in the QEIIE register."]
pub type Pos1revIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS2REV_INT` writer - Writing a 1 disables the POS2REV_Int interrupt in the QEIIE register."]
pub type Pos2revIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV1_INT` writer - Writing a 1 disables the REV1_Int interrupt in the QEIIE register."]
pub type Rev1IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV2_INT` writer - Writing a 1 disables the REV2_Int interrupt in the QEIIE register."]
pub type Rev2IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAXPOS_INT` writer - Writing a 1 disables the MAXPOS_Int interrupt in the QEIIE register."]
pub type MaxposIntW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Writing a 1 disables the INX_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn inx_int(&mut self) -> InxIntW<'_, IecSpec> {
        InxIntW::new(self, 0)
    }
    #[doc = "Bit 1 - Writing a 1 disables the TIN_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn tim_int(&mut self) -> TimIntW<'_, IecSpec> {
        TimIntW::new(self, 1)
    }
    #[doc = "Bit 2 - Writing a 1 disables the VELC_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn velc_int(&mut self) -> VelcIntW<'_, IecSpec> {
        VelcIntW::new(self, 2)
    }
    #[doc = "Bit 3 - Writing a 1 disables the DIR_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn dir_int(&mut self) -> DirIntW<'_, IecSpec> {
        DirIntW::new(self, 3)
    }
    #[doc = "Bit 4 - Writing a 1 disables the ERR_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn err_int(&mut self) -> ErrIntW<'_, IecSpec> {
        ErrIntW::new(self, 4)
    }
    #[doc = "Bit 5 - Writing a 1 disables the ENCLK_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn enclk_int(&mut self) -> EnclkIntW<'_, IecSpec> {
        EnclkIntW::new(self, 5)
    }
    #[doc = "Bit 6 - Writing a 1 disables the POS0_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn pos0_int(&mut self) -> Pos0IntW<'_, IecSpec> {
        Pos0IntW::new(self, 6)
    }
    #[doc = "Bit 7 - Writing a 1 disables the POS1_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn pos1_int(&mut self) -> Pos1IntW<'_, IecSpec> {
        Pos1IntW::new(self, 7)
    }
    #[doc = "Bit 8 - Writing a 1 disables the POS2_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn pos2_int(&mut self) -> Pos2IntW<'_, IecSpec> {
        Pos2IntW::new(self, 8)
    }
    #[doc = "Bit 9 - Writing a 1 disables the REV0_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn rev0_int(&mut self) -> Rev0IntW<'_, IecSpec> {
        Rev0IntW::new(self, 9)
    }
    #[doc = "Bit 10 - Writing a 1 disables the POS0REV_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn pos0rev_int(&mut self) -> Pos0revIntW<'_, IecSpec> {
        Pos0revIntW::new(self, 10)
    }
    #[doc = "Bit 11 - Writing a 1 disables the POS1REV_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn pos1rev_int(&mut self) -> Pos1revIntW<'_, IecSpec> {
        Pos1revIntW::new(self, 11)
    }
    #[doc = "Bit 12 - Writing a 1 disables the POS2REV_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn pos2rev_int(&mut self) -> Pos2revIntW<'_, IecSpec> {
        Pos2revIntW::new(self, 12)
    }
    #[doc = "Bit 13 - Writing a 1 disables the REV1_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn rev1_int(&mut self) -> Rev1IntW<'_, IecSpec> {
        Rev1IntW::new(self, 13)
    }
    #[doc = "Bit 14 - Writing a 1 disables the REV2_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn rev2_int(&mut self) -> Rev2IntW<'_, IecSpec> {
        Rev2IntW::new(self, 14)
    }
    #[doc = "Bit 15 - Writing a 1 disables the MAXPOS_Int interrupt in the QEIIE register."]
    #[inline(always)]
    pub fn maxpos_int(&mut self) -> MaxposIntW<'_, IecSpec> {
        MaxposIntW::new(self, 15)
    }
}
#[doc = "Interrupt enable clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iec::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IecSpec;
impl crate::RegisterSpec for IecSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`iec::W`](W) writer structure"]
impl crate::Writable for IecSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IEC to value 0"]
impl crate::Resettable for IecSpec {}
