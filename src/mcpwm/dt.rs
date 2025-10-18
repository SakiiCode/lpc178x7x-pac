#[doc = "Register `DT` reader"]
pub type R = crate::R<DtSpec>;
#[doc = "Register `DT` writer"]
pub type W = crate::W<DtSpec>;
#[doc = "Field `DT0` reader - Dead time for channel 0.\\[1\\]"]
pub type Dt0R = crate::FieldReader<u16>;
#[doc = "Field `DT0` writer - Dead time for channel 0.\\[1\\]"]
pub type Dt0W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DT1` reader - Dead time for channel 1.\\[2\\]"]
pub type Dt1R = crate::FieldReader<u16>;
#[doc = "Field `DT1` writer - Dead time for channel 1.\\[2\\]"]
pub type Dt1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `DT2` reader - Dead time for channel 2.\\[2\\]"]
pub type Dt2R = crate::FieldReader<u16>;
#[doc = "Field `DT2` writer - Dead time for channel 2.\\[2\\]"]
pub type Dt2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Dead time for channel 0.\\[1\\]"]
    #[inline(always)]
    pub fn dt0(&self) -> Dt0R {
        Dt0R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - Dead time for channel 1.\\[2\\]"]
    #[inline(always)]
    pub fn dt1(&self) -> Dt1R {
        Dt1R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - Dead time for channel 2.\\[2\\]"]
    #[inline(always)]
    pub fn dt2(&self) -> Dt2R {
        Dt2R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Dead time for channel 0.\\[1\\]"]
    #[inline(always)]
    pub fn dt0(&mut self) -> Dt0W<'_, DtSpec> {
        Dt0W::new(self, 0)
    }
    #[doc = "Bits 10:19 - Dead time for channel 1.\\[2\\]"]
    #[inline(always)]
    pub fn dt1(&mut self) -> Dt1W<'_, DtSpec> {
        Dt1W::new(self, 10)
    }
    #[doc = "Bits 20:29 - Dead time for channel 2.\\[2\\]"]
    #[inline(always)]
    pub fn dt2(&mut self) -> Dt2W<'_, DtSpec> {
        Dt2W::new(self, 20)
    }
}
#[doc = "Dead time register\n\nYou can [`read`](crate::Reg::read) this register and get [`dt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtSpec;
impl crate::RegisterSpec for DtSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dt::R`](R) reader structure"]
impl crate::Readable for DtSpec {}
#[doc = "`write(|w| ..)` method takes [`dt::W`](W) writer structure"]
impl crate::Writable for DtSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DT to value 0x3fff_ffff"]
impl crate::Resettable for DtSpec {
    const RESET_VALUE: u32 = 0x3fff_ffff;
}
