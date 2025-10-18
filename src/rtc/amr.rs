#[doc = "Register `AMR` reader"]
pub type R = crate::R<AmrSpec>;
#[doc = "Register `AMR` writer"]
pub type W = crate::W<AmrSpec>;
#[doc = "Field `AMRSEC` reader - When 1, the Second value is not compared for the alarm."]
pub type AmrsecR = crate::BitReader;
#[doc = "Field `AMRSEC` writer - When 1, the Second value is not compared for the alarm."]
pub type AmrsecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMRMIN` reader - When 1, the Minutes value is not compared for the alarm."]
pub type AmrminR = crate::BitReader;
#[doc = "Field `AMRMIN` writer - When 1, the Minutes value is not compared for the alarm."]
pub type AmrminW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMRHOUR` reader - When 1, the Hour value is not compared for the alarm."]
pub type AmrhourR = crate::BitReader;
#[doc = "Field `AMRHOUR` writer - When 1, the Hour value is not compared for the alarm."]
pub type AmrhourW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMRDOM` reader - When 1, the Day of Month value is not compared for the alarm."]
pub type AmrdomR = crate::BitReader;
#[doc = "Field `AMRDOM` writer - When 1, the Day of Month value is not compared for the alarm."]
pub type AmrdomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMRDOW` reader - When 1, the Day of Week value is not compared for the alarm."]
pub type AmrdowR = crate::BitReader;
#[doc = "Field `AMRDOW` writer - When 1, the Day of Week value is not compared for the alarm."]
pub type AmrdowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMRDOY` reader - When 1, the Day of Year value is not compared for the alarm."]
pub type AmrdoyR = crate::BitReader;
#[doc = "Field `AMRDOY` writer - When 1, the Day of Year value is not compared for the alarm."]
pub type AmrdoyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMRMON` reader - When 1, the Month value is not compared for the alarm."]
pub type AmrmonR = crate::BitReader;
#[doc = "Field `AMRMON` writer - When 1, the Month value is not compared for the alarm."]
pub type AmrmonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AMRYEAR` reader - When 1, the Year value is not compared for the alarm."]
pub type AmryearR = crate::BitReader;
#[doc = "Field `AMRYEAR` writer - When 1, the Year value is not compared for the alarm."]
pub type AmryearW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When 1, the Second value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrsec(&self) -> AmrsecR {
        AmrsecR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, the Minutes value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrmin(&self) -> AmrminR {
        AmrminR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When 1, the Hour value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrhour(&self) -> AmrhourR {
        AmrhourR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When 1, the Day of Month value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrdom(&self) -> AmrdomR {
        AmrdomR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When 1, the Day of Week value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrdow(&self) -> AmrdowR {
        AmrdowR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When 1, the Day of Year value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrdoy(&self) -> AmrdoyR {
        AmrdoyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When 1, the Month value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrmon(&self) -> AmrmonR {
        AmrmonR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When 1, the Year value is not compared for the alarm."]
    #[inline(always)]
    pub fn amryear(&self) -> AmryearR {
        AmryearR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, the Second value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrsec(&mut self) -> AmrsecW<'_, AmrSpec> {
        AmrsecW::new(self, 0)
    }
    #[doc = "Bit 1 - When 1, the Minutes value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrmin(&mut self) -> AmrminW<'_, AmrSpec> {
        AmrminW::new(self, 1)
    }
    #[doc = "Bit 2 - When 1, the Hour value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrhour(&mut self) -> AmrhourW<'_, AmrSpec> {
        AmrhourW::new(self, 2)
    }
    #[doc = "Bit 3 - When 1, the Day of Month value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrdom(&mut self) -> AmrdomW<'_, AmrSpec> {
        AmrdomW::new(self, 3)
    }
    #[doc = "Bit 4 - When 1, the Day of Week value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrdow(&mut self) -> AmrdowW<'_, AmrSpec> {
        AmrdowW::new(self, 4)
    }
    #[doc = "Bit 5 - When 1, the Day of Year value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrdoy(&mut self) -> AmrdoyW<'_, AmrSpec> {
        AmrdoyW::new(self, 5)
    }
    #[doc = "Bit 6 - When 1, the Month value is not compared for the alarm."]
    #[inline(always)]
    pub fn amrmon(&mut self) -> AmrmonW<'_, AmrSpec> {
        AmrmonW::new(self, 6)
    }
    #[doc = "Bit 7 - When 1, the Year value is not compared for the alarm."]
    #[inline(always)]
    pub fn amryear(&mut self) -> AmryearW<'_, AmrSpec> {
        AmryearW::new(self, 7)
    }
}
#[doc = "Alarm Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`amr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AmrSpec;
impl crate::RegisterSpec for AmrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`amr::R`](R) reader structure"]
impl crate::Readable for AmrSpec {}
#[doc = "`write(|w| ..)` method takes [`amr::W`](W) writer structure"]
impl crate::Writable for AmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AMR to value 0"]
impl crate::Resettable for AmrSpec {}
