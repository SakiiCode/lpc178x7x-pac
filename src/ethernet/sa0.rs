#[doc = "Register `SA0` reader"]
pub type R = crate::R<Sa0Spec>;
#[doc = "Register `SA0` writer"]
pub type W = crate::W<Sa0Spec>;
#[doc = "Field `SADDR2` reader - STATION ADDRESS, 2nd octet. This field holds the second octet of the station address."]
pub type Saddr2R = crate::FieldReader;
#[doc = "Field `SADDR2` writer - STATION ADDRESS, 2nd octet. This field holds the second octet of the station address."]
pub type Saddr2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SADDR1` reader - STATION ADDRESS, 1st octet. This field holds the first octet of the station address."]
pub type Saddr1R = crate::FieldReader;
#[doc = "Field `SADDR1` writer - STATION ADDRESS, 1st octet. This field holds the first octet of the station address."]
pub type Saddr1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - STATION ADDRESS, 2nd octet. This field holds the second octet of the station address."]
    #[inline(always)]
    pub fn saddr2(&self) -> Saddr2R {
        Saddr2R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - STATION ADDRESS, 1st octet. This field holds the first octet of the station address."]
    #[inline(always)]
    pub fn saddr1(&self) -> Saddr1R {
        Saddr1R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - STATION ADDRESS, 2nd octet. This field holds the second octet of the station address."]
    #[inline(always)]
    pub fn saddr2(&mut self) -> Saddr2W<'_, Sa0Spec> {
        Saddr2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - STATION ADDRESS, 1st octet. This field holds the first octet of the station address."]
    #[inline(always)]
    pub fn saddr1(&mut self) -> Saddr1W<'_, Sa0Spec> {
        Saddr1W::new(self, 8)
    }
}
#[doc = "Station Address 0 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sa0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Sa0Spec;
impl crate::RegisterSpec for Sa0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sa0::R`](R) reader structure"]
impl crate::Readable for Sa0Spec {}
#[doc = "`write(|w| ..)` method takes [`sa0::W`](W) writer structure"]
impl crate::Writable for Sa0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SA0 to value 0"]
impl crate::Resettable for Sa0Spec {}
