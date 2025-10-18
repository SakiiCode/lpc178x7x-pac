#[doc = "Register `CIIR` reader"]
pub type R = crate::R<CiirSpec>;
#[doc = "Register `CIIR` writer"]
pub type W = crate::W<CiirSpec>;
#[doc = "Field `IMSEC` reader - When 1, an increment of the Second value generates an interrupt."]
pub type ImsecR = crate::BitReader;
#[doc = "Field `IMSEC` writer - When 1, an increment of the Second value generates an interrupt."]
pub type ImsecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMMIN` reader - When 1, an increment of the Minute value generates an interrupt."]
pub type ImminR = crate::BitReader;
#[doc = "Field `IMMIN` writer - When 1, an increment of the Minute value generates an interrupt."]
pub type ImminW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMHOUR` reader - When 1, an increment of the Hour value generates an interrupt."]
pub type ImhourR = crate::BitReader;
#[doc = "Field `IMHOUR` writer - When 1, an increment of the Hour value generates an interrupt."]
pub type ImhourW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMDOM` reader - When 1, an increment of the Day of Month value generates an interrupt."]
pub type ImdomR = crate::BitReader;
#[doc = "Field `IMDOM` writer - When 1, an increment of the Day of Month value generates an interrupt."]
pub type ImdomW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMDOW` reader - When 1, an increment of the Day of Week value generates an interrupt."]
pub type ImdowR = crate::BitReader;
#[doc = "Field `IMDOW` writer - When 1, an increment of the Day of Week value generates an interrupt."]
pub type ImdowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMDOY` reader - When 1, an increment of the Day of Year value generates an interrupt."]
pub type ImdoyR = crate::BitReader;
#[doc = "Field `IMDOY` writer - When 1, an increment of the Day of Year value generates an interrupt."]
pub type ImdoyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMMON` reader - When 1, an increment of the Month value generates an interrupt."]
pub type ImmonR = crate::BitReader;
#[doc = "Field `IMMON` writer - When 1, an increment of the Month value generates an interrupt."]
pub type ImmonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMYEAR` reader - When 1, an increment of the Year value generates an interrupt."]
pub type ImyearR = crate::BitReader;
#[doc = "Field `IMYEAR` writer - When 1, an increment of the Year value generates an interrupt."]
pub type ImyearW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When 1, an increment of the Second value generates an interrupt."]
    #[inline(always)]
    pub fn imsec(&self) -> ImsecR {
        ImsecR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When 1, an increment of the Minute value generates an interrupt."]
    #[inline(always)]
    pub fn immin(&self) -> ImminR {
        ImminR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - When 1, an increment of the Hour value generates an interrupt."]
    #[inline(always)]
    pub fn imhour(&self) -> ImhourR {
        ImhourR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When 1, an increment of the Day of Month value generates an interrupt."]
    #[inline(always)]
    pub fn imdom(&self) -> ImdomR {
        ImdomR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - When 1, an increment of the Day of Week value generates an interrupt."]
    #[inline(always)]
    pub fn imdow(&self) -> ImdowR {
        ImdowR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - When 1, an increment of the Day of Year value generates an interrupt."]
    #[inline(always)]
    pub fn imdoy(&self) -> ImdoyR {
        ImdoyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When 1, an increment of the Month value generates an interrupt."]
    #[inline(always)]
    pub fn immon(&self) -> ImmonR {
        ImmonR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When 1, an increment of the Year value generates an interrupt."]
    #[inline(always)]
    pub fn imyear(&self) -> ImyearR {
        ImyearR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - When 1, an increment of the Second value generates an interrupt."]
    #[inline(always)]
    pub fn imsec(&mut self) -> ImsecW<'_, CiirSpec> {
        ImsecW::new(self, 0)
    }
    #[doc = "Bit 1 - When 1, an increment of the Minute value generates an interrupt."]
    #[inline(always)]
    pub fn immin(&mut self) -> ImminW<'_, CiirSpec> {
        ImminW::new(self, 1)
    }
    #[doc = "Bit 2 - When 1, an increment of the Hour value generates an interrupt."]
    #[inline(always)]
    pub fn imhour(&mut self) -> ImhourW<'_, CiirSpec> {
        ImhourW::new(self, 2)
    }
    #[doc = "Bit 3 - When 1, an increment of the Day of Month value generates an interrupt."]
    #[inline(always)]
    pub fn imdom(&mut self) -> ImdomW<'_, CiirSpec> {
        ImdomW::new(self, 3)
    }
    #[doc = "Bit 4 - When 1, an increment of the Day of Week value generates an interrupt."]
    #[inline(always)]
    pub fn imdow(&mut self) -> ImdowW<'_, CiirSpec> {
        ImdowW::new(self, 4)
    }
    #[doc = "Bit 5 - When 1, an increment of the Day of Year value generates an interrupt."]
    #[inline(always)]
    pub fn imdoy(&mut self) -> ImdoyW<'_, CiirSpec> {
        ImdoyW::new(self, 5)
    }
    #[doc = "Bit 6 - When 1, an increment of the Month value generates an interrupt."]
    #[inline(always)]
    pub fn immon(&mut self) -> ImmonW<'_, CiirSpec> {
        ImmonW::new(self, 6)
    }
    #[doc = "Bit 7 - When 1, an increment of the Year value generates an interrupt."]
    #[inline(always)]
    pub fn imyear(&mut self) -> ImyearW<'_, CiirSpec> {
        ImyearW::new(self, 7)
    }
}
#[doc = "Counter Increment Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ciir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ciir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CiirSpec;
impl crate::RegisterSpec for CiirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ciir::R`](R) reader structure"]
impl crate::Readable for CiirSpec {}
#[doc = "`write(|w| ..)` method takes [`ciir::W`](W) writer structure"]
impl crate::Writable for CiirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CIIR to value 0"]
impl crate::Resettable for CiirSpec {}
