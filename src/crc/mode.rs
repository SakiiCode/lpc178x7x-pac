#[doc = "Register `MODE` reader"]
pub type R = crate::R<ModeSpec>;
#[doc = "Register `MODE` writer"]
pub type W = crate::W<ModeSpec>;
#[doc = "Select CRC polynomial\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CrcPoly {
    #[doc = "0: CRC-CCITT polynomial"]
    CrcCcitt = 0,
    #[doc = "1: CRC-16 polynomial"]
    Crc16 = 1,
    #[doc = "2: CRC-32 polynomial"]
    Crc32 = 2,
}
impl From<CrcPoly> for u8 {
    #[inline(always)]
    fn from(variant: CrcPoly) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CrcPoly {
    type Ux = u8;
}
impl crate::IsEnum for CrcPoly {}
#[doc = "Field `CRC_POLY` reader - Select CRC polynomial"]
pub type CrcPolyR = crate::FieldReader<CrcPoly>;
impl CrcPolyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CrcPoly> {
        match self.bits {
            0 => Some(CrcPoly::CrcCcitt),
            1 => Some(CrcPoly::Crc16),
            2 => Some(CrcPoly::Crc32),
            _ => None,
        }
    }
    #[doc = "CRC-CCITT polynomial"]
    #[inline(always)]
    pub fn is_crc_ccitt(&self) -> bool {
        *self == CrcPoly::CrcCcitt
    }
    #[doc = "CRC-16 polynomial"]
    #[inline(always)]
    pub fn is_crc_16(&self) -> bool {
        *self == CrcPoly::Crc16
    }
    #[doc = "CRC-32 polynomial"]
    #[inline(always)]
    pub fn is_crc_32(&self) -> bool {
        *self == CrcPoly::Crc32
    }
}
#[doc = "Field `CRC_POLY` writer - Select CRC polynomial"]
pub type CrcPolyW<'a, REG> = crate::FieldWriter<'a, REG, 2, CrcPoly>;
impl<'a, REG> CrcPolyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CRC-CCITT polynomial"]
    #[inline(always)]
    pub fn crc_ccitt(self) -> &'a mut crate::W<REG> {
        self.variant(CrcPoly::CrcCcitt)
    }
    #[doc = "CRC-16 polynomial"]
    #[inline(always)]
    pub fn crc_16(self) -> &'a mut crate::W<REG> {
        self.variant(CrcPoly::Crc16)
    }
    #[doc = "CRC-32 polynomial"]
    #[inline(always)]
    pub fn crc_32(self) -> &'a mut crate::W<REG> {
        self.variant(CrcPoly::Crc32)
    }
}
#[doc = "Select bit order for CRC_WR_DATA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BitRvsWr {
    #[doc = "0: No bit order reverse for CRC_WR_DATA (per byte)"]
    NoReverse = 0,
    #[doc = "1: Bit order reverse for CRC_WR_DATA (per byte)"]
    Reverse = 1,
}
impl From<BitRvsWr> for bool {
    #[inline(always)]
    fn from(variant: BitRvsWr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT_RVS_WR` reader - Select bit order for CRC_WR_DATA"]
pub type BitRvsWrR = crate::BitReader<BitRvsWr>;
impl BitRvsWrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BitRvsWr {
        match self.bits {
            false => BitRvsWr::NoReverse,
            true => BitRvsWr::Reverse,
        }
    }
    #[doc = "No bit order reverse for CRC_WR_DATA (per byte)"]
    #[inline(always)]
    pub fn is_no_reverse(&self) -> bool {
        *self == BitRvsWr::NoReverse
    }
    #[doc = "Bit order reverse for CRC_WR_DATA (per byte)"]
    #[inline(always)]
    pub fn is_reverse(&self) -> bool {
        *self == BitRvsWr::Reverse
    }
}
#[doc = "Field `BIT_RVS_WR` writer - Select bit order for CRC_WR_DATA"]
pub type BitRvsWrW<'a, REG> = crate::BitWriter<'a, REG, BitRvsWr>;
impl<'a, REG> BitRvsWrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No bit order reverse for CRC_WR_DATA (per byte)"]
    #[inline(always)]
    pub fn no_reverse(self) -> &'a mut crate::W<REG> {
        self.variant(BitRvsWr::NoReverse)
    }
    #[doc = "Bit order reverse for CRC_WR_DATA (per byte)"]
    #[inline(always)]
    pub fn reverse(self) -> &'a mut crate::W<REG> {
        self.variant(BitRvsWr::Reverse)
    }
}
#[doc = "Select one's complement for CRC_WR_DATA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmplWr {
    #[doc = "0: No one's complement for CRC_WR_DATA"]
    Disabled = 0,
    #[doc = "1: One's complement for CRC_WR_DATA"]
    Enabled = 1,
}
impl From<CmplWr> for bool {
    #[inline(always)]
    fn from(variant: CmplWr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPL_WR` reader - Select one's complement for CRC_WR_DATA"]
pub type CmplWrR = crate::BitReader<CmplWr>;
impl CmplWrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CmplWr {
        match self.bits {
            false => CmplWr::Disabled,
            true => CmplWr::Enabled,
        }
    }
    #[doc = "No one's complement for CRC_WR_DATA"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CmplWr::Disabled
    }
    #[doc = "One's complement for CRC_WR_DATA"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CmplWr::Enabled
    }
}
#[doc = "Field `CMPL_WR` writer - Select one's complement for CRC_WR_DATA"]
pub type CmplWrW<'a, REG> = crate::BitWriter<'a, REG, CmplWr>;
impl<'a, REG> CmplWrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No one's complement for CRC_WR_DATA"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CmplWr::Disabled)
    }
    #[doc = "One's complement for CRC_WR_DATA"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CmplWr::Enabled)
    }
}
#[doc = "Select bit order revers for CRC_SUM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BitRvsSum {
    #[doc = "0: No bit order reverse for CRC_SUM"]
    NoReverse = 0,
    #[doc = "1: Bit order reverse for CRC_SUM"]
    Reverse = 1,
}
impl From<BitRvsSum> for bool {
    #[inline(always)]
    fn from(variant: BitRvsSum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BIT_RVS_SUM` reader - Select bit order revers for CRC_SUM"]
pub type BitRvsSumR = crate::BitReader<BitRvsSum>;
impl BitRvsSumR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> BitRvsSum {
        match self.bits {
            false => BitRvsSum::NoReverse,
            true => BitRvsSum::Reverse,
        }
    }
    #[doc = "No bit order reverse for CRC_SUM"]
    #[inline(always)]
    pub fn is_no_reverse(&self) -> bool {
        *self == BitRvsSum::NoReverse
    }
    #[doc = "Bit order reverse for CRC_SUM"]
    #[inline(always)]
    pub fn is_reverse(&self) -> bool {
        *self == BitRvsSum::Reverse
    }
}
#[doc = "Field `BIT_RVS_SUM` writer - Select bit order revers for CRC_SUM"]
pub type BitRvsSumW<'a, REG> = crate::BitWriter<'a, REG, BitRvsSum>;
impl<'a, REG> BitRvsSumW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No bit order reverse for CRC_SUM"]
    #[inline(always)]
    pub fn no_reverse(self) -> &'a mut crate::W<REG> {
        self.variant(BitRvsSum::NoReverse)
    }
    #[doc = "Bit order reverse for CRC_SUM"]
    #[inline(always)]
    pub fn reverse(self) -> &'a mut crate::W<REG> {
        self.variant(BitRvsSum::Reverse)
    }
}
#[doc = "Select one's complement for CRC_SUM\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CmplSum {
    #[doc = "0: No one's complement for CRC_SUM"]
    Disabled = 0,
    #[doc = "1: One's complement for CRC_SUM"]
    Enabled = 1,
}
impl From<CmplSum> for bool {
    #[inline(always)]
    fn from(variant: CmplSum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CMPL_SUM` reader - Select one's complement for CRC_SUM"]
pub type CmplSumR = crate::BitReader<CmplSum>;
impl CmplSumR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CmplSum {
        match self.bits {
            false => CmplSum::Disabled,
            true => CmplSum::Enabled,
        }
    }
    #[doc = "No one's complement for CRC_SUM"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CmplSum::Disabled
    }
    #[doc = "One's complement for CRC_SUM"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CmplSum::Enabled
    }
}
#[doc = "Field `CMPL_SUM` writer - Select one's complement for CRC_SUM"]
pub type CmplSumW<'a, REG> = crate::BitWriter<'a, REG, CmplSum>;
impl<'a, REG> CmplSumW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No one's complement for CRC_SUM"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(CmplSum::Disabled)
    }
    #[doc = "One's complement for CRC_SUM"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(CmplSum::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:1 - Select CRC polynomial"]
    #[inline(always)]
    pub fn crc_poly(&self) -> CrcPolyR {
        CrcPolyR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Select bit order for CRC_WR_DATA"]
    #[inline(always)]
    pub fn bit_rvs_wr(&self) -> BitRvsWrR {
        BitRvsWrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Select one's complement for CRC_WR_DATA"]
    #[inline(always)]
    pub fn cmpl_wr(&self) -> CmplWrR {
        CmplWrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Select bit order revers for CRC_SUM"]
    #[inline(always)]
    pub fn bit_rvs_sum(&self) -> BitRvsSumR {
        BitRvsSumR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Select one's complement for CRC_SUM"]
    #[inline(always)]
    pub fn cmpl_sum(&self) -> CmplSumR {
        CmplSumR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select CRC polynomial"]
    #[inline(always)]
    pub fn crc_poly(&mut self) -> CrcPolyW<'_, ModeSpec> {
        CrcPolyW::new(self, 0)
    }
    #[doc = "Bit 2 - Select bit order for CRC_WR_DATA"]
    #[inline(always)]
    pub fn bit_rvs_wr(&mut self) -> BitRvsWrW<'_, ModeSpec> {
        BitRvsWrW::new(self, 2)
    }
    #[doc = "Bit 3 - Select one's complement for CRC_WR_DATA"]
    #[inline(always)]
    pub fn cmpl_wr(&mut self) -> CmplWrW<'_, ModeSpec> {
        CmplWrW::new(self, 3)
    }
    #[doc = "Bit 4 - Select bit order revers for CRC_SUM"]
    #[inline(always)]
    pub fn bit_rvs_sum(&mut self) -> BitRvsSumW<'_, ModeSpec> {
        BitRvsSumW::new(self, 4)
    }
    #[doc = "Bit 5 - Select one's complement for CRC_SUM"]
    #[inline(always)]
    pub fn cmpl_sum(&mut self) -> CmplSumW<'_, ModeSpec> {
        CmplSumW::new(self, 5)
    }
}
#[doc = "CRC mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeSpec;
impl crate::RegisterSpec for ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for ModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for ModeSpec {}
