#[doc = "Register `CALIBRATION` reader"]
pub type R = crate::R<CalibrationSpec>;
#[doc = "Register `CALIBRATION` writer"]
pub type W = crate::W<CalibrationSpec>;
#[doc = "Field `CALVAL` reader - If enabled, the calibration counter counts up to this value. The maximum value is 131, 072 corresponding to about 36.4 hours. Calibration is disabled if CALVAL = 0."]
pub type CalvalR = crate::FieldReader<u32>;
#[doc = "Field `CALVAL` writer - If enabled, the calibration counter counts up to this value. The maximum value is 131, 072 corresponding to about 36.4 hours. Calibration is disabled if CALVAL = 0."]
pub type CalvalW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Calibration direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Caldir {
    #[doc = "1: Backward calibration. When CALVAL is equal to the calibration counter, the RTC timers will stop incrementing for 1 second."]
    BackwardCalibration = 1,
    #[doc = "0: Forward calibration. When CALVAL is equal to the calibration counter, the RTC timers will jump by 2 seconds."]
    ForwardCalibration_ = 0,
}
impl From<Caldir> for bool {
    #[inline(always)]
    fn from(variant: Caldir) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CALDIR` reader - Calibration direction"]
pub type CaldirR = crate::BitReader<Caldir>;
impl CaldirR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Caldir {
        match self.bits {
            true => Caldir::BackwardCalibration,
            false => Caldir::ForwardCalibration_,
        }
    }
    #[doc = "Backward calibration. When CALVAL is equal to the calibration counter, the RTC timers will stop incrementing for 1 second."]
    #[inline(always)]
    pub fn is_backward_calibration(&self) -> bool {
        *self == Caldir::BackwardCalibration
    }
    #[doc = "Forward calibration. When CALVAL is equal to the calibration counter, the RTC timers will jump by 2 seconds."]
    #[inline(always)]
    pub fn is_forward_calibration_(&self) -> bool {
        *self == Caldir::ForwardCalibration_
    }
}
#[doc = "Field `CALDIR` writer - Calibration direction"]
pub type CaldirW<'a, REG> = crate::BitWriter<'a, REG, Caldir>;
impl<'a, REG> CaldirW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Backward calibration. When CALVAL is equal to the calibration counter, the RTC timers will stop incrementing for 1 second."]
    #[inline(always)]
    pub fn backward_calibration(self) -> &'a mut crate::W<REG> {
        self.variant(Caldir::BackwardCalibration)
    }
    #[doc = "Forward calibration. When CALVAL is equal to the calibration counter, the RTC timers will jump by 2 seconds."]
    #[inline(always)]
    pub fn forward_calibration_(self) -> &'a mut crate::W<REG> {
        self.variant(Caldir::ForwardCalibration_)
    }
}
impl R {
    #[doc = "Bits 0:16 - If enabled, the calibration counter counts up to this value. The maximum value is 131, 072 corresponding to about 36.4 hours. Calibration is disabled if CALVAL = 0."]
    #[inline(always)]
    pub fn calval(&self) -> CalvalR {
        CalvalR::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bit 17 - Calibration direction"]
    #[inline(always)]
    pub fn caldir(&self) -> CaldirR {
        CaldirR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:16 - If enabled, the calibration counter counts up to this value. The maximum value is 131, 072 corresponding to about 36.4 hours. Calibration is disabled if CALVAL = 0."]
    #[inline(always)]
    pub fn calval(&mut self) -> CalvalW<'_, CalibrationSpec> {
        CalvalW::new(self, 0)
    }
    #[doc = "Bit 17 - Calibration direction"]
    #[inline(always)]
    pub fn caldir(&mut self) -> CaldirW<'_, CalibrationSpec> {
        CaldirW::new(self, 17)
    }
}
#[doc = "Calibration Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`calibration::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calibration::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalibrationSpec;
impl crate::RegisterSpec for CalibrationSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calibration::R`](R) reader structure"]
impl crate::Readable for CalibrationSpec {}
#[doc = "`write(|w| ..)` method takes [`calibration::W`](W) writer structure"]
impl crate::Writable for CalibrationSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CALIBRATION to value 0"]
impl crate::Resettable for CalibrationSpec {}
