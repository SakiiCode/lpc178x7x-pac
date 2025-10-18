#[doc = "Register `SET` writer"]
pub type W = crate::W<SetSpec>;
#[doc = "Field `INX_INT` writer - Writing a 1 sets the INX_Int bit in QEIINTSTAT."]
pub type InxIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM_INT` writer - Writing a 1 sets the TIN_Int bit in QEIINTSTAT."]
pub type TimIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VELC_INT` writer - Writing a 1 sets the VELC_Int bit in QEIINTSTAT."]
pub type VelcIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR_INT` writer - Writing a 1 sets the DIR_Int bit in QEIINTSTAT."]
pub type DirIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERR_INT` writer - Writing a 1 sets the ERR_Int bit in QEIINTSTAT."]
pub type ErrIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENCLK_INT` writer - Writing a 1 sets the ENCLK_Int bit in QEIINTSTAT."]
pub type EnclkIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS0_INT` writer - Writing a 1 sets the POS0_Int bit in QEIINTSTAT."]
pub type Pos0IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS1_INT` writer - Writing a 1 sets the POS1_Int bit in QEIINTSTAT."]
pub type Pos1IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS2_INT` writer - Writing a 1 sets the POS2_Int bit in QEIINTSTAT."]
pub type Pos2IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV0_INT` writer - Writing a 1 sets the REV0_Int bit in QEIINTSTAT."]
pub type Rev0IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS0REV_INT` writer - Writing a 1 sets the POS0REV_Int bit in QEIINTSTAT."]
pub type Pos0revIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS1REV_INT` writer - Writing a 1 sets the POS1REV_Int bit in QEIINTSTAT."]
pub type Pos1revIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POS2REV_INT` writer - Writing a 1 sets the POS2REV_Int bit in QEIINTSTAT."]
pub type Pos2revIntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV1_INT` writer - Writing a 1 sets the REV1_Int bit in QEIINTSTAT."]
pub type Rev1IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REV2_INT` writer - Writing a 1 sets the REV2_Int bit in QEIINTSTAT."]
pub type Rev2IntW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAXPOS_INT` writer - Writing a 1 sets the MAXPOS_Int bit in QEIINTSTAT."]
pub type MaxposIntW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Writing a 1 sets the INX_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn inx_int(&mut self) -> InxIntW<'_, SetSpec> {
        InxIntW::new(self, 0)
    }
    #[doc = "Bit 1 - Writing a 1 sets the TIN_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn tim_int(&mut self) -> TimIntW<'_, SetSpec> {
        TimIntW::new(self, 1)
    }
    #[doc = "Bit 2 - Writing a 1 sets the VELC_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn velc_int(&mut self) -> VelcIntW<'_, SetSpec> {
        VelcIntW::new(self, 2)
    }
    #[doc = "Bit 3 - Writing a 1 sets the DIR_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn dir_int(&mut self) -> DirIntW<'_, SetSpec> {
        DirIntW::new(self, 3)
    }
    #[doc = "Bit 4 - Writing a 1 sets the ERR_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn err_int(&mut self) -> ErrIntW<'_, SetSpec> {
        ErrIntW::new(self, 4)
    }
    #[doc = "Bit 5 - Writing a 1 sets the ENCLK_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn enclk_int(&mut self) -> EnclkIntW<'_, SetSpec> {
        EnclkIntW::new(self, 5)
    }
    #[doc = "Bit 6 - Writing a 1 sets the POS0_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn pos0_int(&mut self) -> Pos0IntW<'_, SetSpec> {
        Pos0IntW::new(self, 6)
    }
    #[doc = "Bit 7 - Writing a 1 sets the POS1_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn pos1_int(&mut self) -> Pos1IntW<'_, SetSpec> {
        Pos1IntW::new(self, 7)
    }
    #[doc = "Bit 8 - Writing a 1 sets the POS2_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn pos2_int(&mut self) -> Pos2IntW<'_, SetSpec> {
        Pos2IntW::new(self, 8)
    }
    #[doc = "Bit 9 - Writing a 1 sets the REV0_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn rev0_int(&mut self) -> Rev0IntW<'_, SetSpec> {
        Rev0IntW::new(self, 9)
    }
    #[doc = "Bit 10 - Writing a 1 sets the POS0REV_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn pos0rev_int(&mut self) -> Pos0revIntW<'_, SetSpec> {
        Pos0revIntW::new(self, 10)
    }
    #[doc = "Bit 11 - Writing a 1 sets the POS1REV_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn pos1rev_int(&mut self) -> Pos1revIntW<'_, SetSpec> {
        Pos1revIntW::new(self, 11)
    }
    #[doc = "Bit 12 - Writing a 1 sets the POS2REV_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn pos2rev_int(&mut self) -> Pos2revIntW<'_, SetSpec> {
        Pos2revIntW::new(self, 12)
    }
    #[doc = "Bit 13 - Writing a 1 sets the REV1_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn rev1_int(&mut self) -> Rev1IntW<'_, SetSpec> {
        Rev1IntW::new(self, 13)
    }
    #[doc = "Bit 14 - Writing a 1 sets the REV2_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn rev2_int(&mut self) -> Rev2IntW<'_, SetSpec> {
        Rev2IntW::new(self, 14)
    }
    #[doc = "Bit 15 - Writing a 1 sets the MAXPOS_Int bit in QEIINTSTAT."]
    #[inline(always)]
    pub fn maxpos_int(&mut self) -> MaxposIntW<'_, SetSpec> {
        MaxposIntW::new(self, 15)
    }
}
#[doc = "Interrupt status set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SetSpec;
impl crate::RegisterSpec for SetSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`set::W`](W) writer structure"]
impl crate::Writable for SetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SET to value 0"]
impl crate::Resettable for SetSpec {}
