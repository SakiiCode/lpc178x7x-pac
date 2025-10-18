#[doc = "Register `SA2` reader"]
pub type R = crate::R<Sa2Spec>;
#[doc = "Register `SA2` writer"]
pub type W = crate::W<Sa2Spec>;
#[doc = "Field `SADDR6` reader - STATION ADDRESS, 6th octet. This field holds the sixth octet of the station address."]
pub type Saddr6R = crate::FieldReader;
#[doc = "Field `SADDR6` writer - STATION ADDRESS, 6th octet. This field holds the sixth octet of the station address."]
pub type Saddr6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SADDR5` reader - STATION ADDRESS, 5th octet. This field holds the fifth octet of the station address."]
pub type Saddr5R = crate::FieldReader;
#[doc = "Field `SADDR5` writer - STATION ADDRESS, 5th octet. This field holds the fifth octet of the station address."]
pub type Saddr5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - STATION ADDRESS, 6th octet. This field holds the sixth octet of the station address."]
    #[inline(always)]
    pub fn saddr6(&self) -> Saddr6R {
        Saddr6R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - STATION ADDRESS, 5th octet. This field holds the fifth octet of the station address."]
    #[inline(always)]
    pub fn saddr5(&self) -> Saddr5R {
        Saddr5R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - STATION ADDRESS, 6th octet. This field holds the sixth octet of the station address."]
    #[inline(always)]
    pub fn saddr6(&mut self) -> Saddr6W<'_, Sa2Spec> {
        Saddr6W::new(self, 0)
    }
    #[doc = "Bits 8:15 - STATION ADDRESS, 5th octet. This field holds the fifth octet of the station address."]
    #[inline(always)]
    pub fn saddr5(&mut self) -> Saddr5W<'_, Sa2Spec> {
        Saddr5W::new(self, 8)
    }
}
#[doc = "Station Address 2 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sa2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sa2Spec;
impl crate::RegisterSpec for Sa2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sa2::R`](R) reader structure"]
impl crate::Readable for Sa2Spec {}
#[doc = "`write(|w| ..)` method takes [`sa2::W`](W) writer structure"]
impl crate::Writable for Sa2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SA2 to value 0"]
impl crate::Resettable for Sa2Spec {}
