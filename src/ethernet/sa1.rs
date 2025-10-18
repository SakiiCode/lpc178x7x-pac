#[doc = "Register `SA1` reader"]
pub type R = crate::R<Sa1Spec>;
#[doc = "Register `SA1` writer"]
pub type W = crate::W<Sa1Spec>;
#[doc = "Field `SADDR4` reader - STATION ADDRESS, 4th octet. This field holds the fourth octet of the station address."]
pub type Saddr4R = crate::FieldReader;
#[doc = "Field `SADDR4` writer - STATION ADDRESS, 4th octet. This field holds the fourth octet of the station address."]
pub type Saddr4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SADDR3` reader - STATION ADDRESS, 3rd octet. This field holds the third octet of the station address."]
pub type Saddr3R = crate::FieldReader;
#[doc = "Field `SADDR3` writer - STATION ADDRESS, 3rd octet. This field holds the third octet of the station address."]
pub type Saddr3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - STATION ADDRESS, 4th octet. This field holds the fourth octet of the station address."]
    #[inline(always)]
    pub fn saddr4(&self) -> Saddr4R {
        Saddr4R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - STATION ADDRESS, 3rd octet. This field holds the third octet of the station address."]
    #[inline(always)]
    pub fn saddr3(&self) -> Saddr3R {
        Saddr3R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - STATION ADDRESS, 4th octet. This field holds the fourth octet of the station address."]
    #[inline(always)]
    pub fn saddr4(&mut self) -> Saddr4W<'_, Sa1Spec> {
        Saddr4W::new(self, 0)
    }
    #[doc = "Bits 8:15 - STATION ADDRESS, 3rd octet. This field holds the third octet of the station address."]
    #[inline(always)]
    pub fn saddr3(&mut self) -> Saddr3W<'_, Sa1Spec> {
        Saddr3W::new(self, 8)
    }
}
#[doc = "Station Address 1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sa1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sa1Spec;
impl crate::RegisterSpec for Sa1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sa1::R`](R) reader structure"]
impl crate::Readable for Sa1Spec {}
#[doc = "`write(|w| ..)` method takes [`sa1::W`](W) writer structure"]
impl crate::Writable for Sa1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SA1 to value 0"]
impl crate::Resettable for Sa1Spec {}
