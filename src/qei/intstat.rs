#[doc = "Register `INTSTAT` reader"]
pub type R = crate::R<IntstatSpec>;
#[doc = "Field `INX_INT` reader - Indicates that an index pulse was detected."]
pub type InxIntR = crate::BitReader;
#[doc = "Field `TIM_INT` reader - Indicates that a velocity timer overflow occurred"]
pub type TimIntR = crate::BitReader;
#[doc = "Field `VELC_INT` reader - Indicates that captured velocity is less than compare velocity."]
pub type VelcIntR = crate::BitReader;
#[doc = "Field `DIR_INT` reader - Indicates that a change of direction was detected."]
pub type DirIntR = crate::BitReader;
#[doc = "Field `ERR_INT` reader - Indicates that an encoder phase error was detected."]
pub type ErrIntR = crate::BitReader;
#[doc = "Field `ENCLK_INT` reader - Indicates that and encoder clock pulse was detected."]
pub type EnclkIntR = crate::BitReader;
#[doc = "Field `POS0_INT` reader - Indicates that the position 0 compare value is equal to the current position."]
pub type Pos0IntR = crate::BitReader;
#[doc = "Field `POS1_INT` reader - Indicates that the position 1compare value is equal to the current position."]
pub type Pos1IntR = crate::BitReader;
#[doc = "Field `POS2_INT` reader - Indicates that the position 2 compare value is equal to the current position."]
pub type Pos2IntR = crate::BitReader;
#[doc = "Field `REV0_INT` reader - Indicates that the index compare 0 value is equal to the current index count."]
pub type Rev0IntR = crate::BitReader;
#[doc = "Field `POS0REV_INT` reader - Combined position 0 and revolution count interrupt. Set when both the POS0_Int bit is set and the REV0_Int is set."]
pub type Pos0revIntR = crate::BitReader;
#[doc = "Field `POS1REV_INT` reader - Combined position 1 and revolution count interrupt. Set when both the POS1_Int bit is set and the REV1_Int is set."]
pub type Pos1revIntR = crate::BitReader;
#[doc = "Field `POS2REV_INT` reader - Combined position 2 and revolution count interrupt. Set when both the POS2_Int bit is set and the REV2_Int is set."]
pub type Pos2revIntR = crate::BitReader;
#[doc = "Field `REV1_INT` reader - Indicates that the index compare 1value is equal to the current index count."]
pub type Rev1IntR = crate::BitReader;
#[doc = "Field `REV2_INT` reader - Indicates that the index compare 2 value is equal to the current index count."]
pub type Rev2IntR = crate::BitReader;
#[doc = "Field `MAXPOS_INT` reader - Indicates that the current position count goes through the MAXPOS value to zero in the forward direction, or through zero to MAXPOS in the reverse direction."]
pub type MaxposIntR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Indicates that an index pulse was detected."]
    #[inline(always)]
    pub fn inx_int(&self) -> InxIntR {
        InxIntR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Indicates that a velocity timer overflow occurred"]
    #[inline(always)]
    pub fn tim_int(&self) -> TimIntR {
        TimIntR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Indicates that captured velocity is less than compare velocity."]
    #[inline(always)]
    pub fn velc_int(&self) -> VelcIntR {
        VelcIntR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Indicates that a change of direction was detected."]
    #[inline(always)]
    pub fn dir_int(&self) -> DirIntR {
        DirIntR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Indicates that an encoder phase error was detected."]
    #[inline(always)]
    pub fn err_int(&self) -> ErrIntR {
        ErrIntR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates that and encoder clock pulse was detected."]
    #[inline(always)]
    pub fn enclk_int(&self) -> EnclkIntR {
        EnclkIntR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Indicates that the position 0 compare value is equal to the current position."]
    #[inline(always)]
    pub fn pos0_int(&self) -> Pos0IntR {
        Pos0IntR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Indicates that the position 1compare value is equal to the current position."]
    #[inline(always)]
    pub fn pos1_int(&self) -> Pos1IntR {
        Pos1IntR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Indicates that the position 2 compare value is equal to the current position."]
    #[inline(always)]
    pub fn pos2_int(&self) -> Pos2IntR {
        Pos2IntR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Indicates that the index compare 0 value is equal to the current index count."]
    #[inline(always)]
    pub fn rev0_int(&self) -> Rev0IntR {
        Rev0IntR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Combined position 0 and revolution count interrupt. Set when both the POS0_Int bit is set and the REV0_Int is set."]
    #[inline(always)]
    pub fn pos0rev_int(&self) -> Pos0revIntR {
        Pos0revIntR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Combined position 1 and revolution count interrupt. Set when both the POS1_Int bit is set and the REV1_Int is set."]
    #[inline(always)]
    pub fn pos1rev_int(&self) -> Pos1revIntR {
        Pos1revIntR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Combined position 2 and revolution count interrupt. Set when both the POS2_Int bit is set and the REV2_Int is set."]
    #[inline(always)]
    pub fn pos2rev_int(&self) -> Pos2revIntR {
        Pos2revIntR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Indicates that the index compare 1value is equal to the current index count."]
    #[inline(always)]
    pub fn rev1_int(&self) -> Rev1IntR {
        Rev1IntR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Indicates that the index compare 2 value is equal to the current index count."]
    #[inline(always)]
    pub fn rev2_int(&self) -> Rev2IntR {
        Rev2IntR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Indicates that the current position count goes through the MAXPOS value to zero in the forward direction, or through zero to MAXPOS in the reverse direction."]
    #[inline(always)]
    pub fn maxpos_int(&self) -> MaxposIntR {
        MaxposIntR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntstatSpec;
impl crate::RegisterSpec for IntstatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intstat::R`](R) reader structure"]
impl crate::Readable for IntstatSpec {}
#[doc = "`reset()` method sets INTSTAT to value 0"]
impl crate::Resettable for IntstatSpec {}
