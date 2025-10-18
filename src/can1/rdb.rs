#[doc = "Register `RDB` reader"]
pub type R = crate::R<RdbSpec>;
#[doc = "Register `RDB` writer"]
pub type W = crate::W<RdbSpec>;
#[doc = "Field `DATA5` reader - Data 5. If the DLC field in CANRFS >= 0101, this contains the first Data byte of the current received message."]
pub type Data5R = crate::FieldReader;
#[doc = "Field `DATA5` writer - Data 5. If the DLC field in CANRFS >= 0101, this contains the first Data byte of the current received message."]
pub type Data5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA6` reader - Data 6. If the DLC field in CANRFS >= 0110, this contains the first Data byte of the current received message."]
pub type Data6R = crate::FieldReader;
#[doc = "Field `DATA6` writer - Data 6. If the DLC field in CANRFS >= 0110, this contains the first Data byte of the current received message."]
pub type Data6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA7` reader - Data 7. If the DLC field in CANRFS >= 0111, this contains the first Data byte of the current received message."]
pub type Data7R = crate::FieldReader;
#[doc = "Field `DATA7` writer - Data 7. If the DLC field in CANRFS >= 0111, this contains the first Data byte of the current received message."]
pub type Data7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA8` reader - Data 8. If the DLC field in CANRFS >= 1000, this contains the first Data byte of the current received message."]
pub type Data8R = crate::FieldReader;
#[doc = "Field `DATA8` writer - Data 8. If the DLC field in CANRFS >= 1000, this contains the first Data byte of the current received message."]
pub type Data8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data 5. If the DLC field in CANRFS >= 0101, this contains the first Data byte of the current received message."]
    #[inline(always)]
    pub fn data5(&self) -> Data5R {
        Data5R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data 6. If the DLC field in CANRFS >= 0110, this contains the first Data byte of the current received message."]
    #[inline(always)]
    pub fn data6(&self) -> Data6R {
        Data6R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data 7. If the DLC field in CANRFS >= 0111, this contains the first Data byte of the current received message."]
    #[inline(always)]
    pub fn data7(&self) -> Data7R {
        Data7R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data 8. If the DLC field in CANRFS >= 1000, this contains the first Data byte of the current received message."]
    #[inline(always)]
    pub fn data8(&self) -> Data8R {
        Data8R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 5. If the DLC field in CANRFS >= 0101, this contains the first Data byte of the current received message."]
    #[inline(always)]
    pub fn data5(&mut self) -> Data5W<'_, RdbSpec> {
        Data5W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Data 6. If the DLC field in CANRFS >= 0110, this contains the first Data byte of the current received message."]
    #[inline(always)]
    pub fn data6(&mut self) -> Data6W<'_, RdbSpec> {
        Data6W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Data 7. If the DLC field in CANRFS >= 0111, this contains the first Data byte of the current received message."]
    #[inline(always)]
    pub fn data7(&mut self) -> Data7W<'_, RdbSpec> {
        Data7W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Data 8. If the DLC field in CANRFS >= 1000, this contains the first Data byte of the current received message."]
    #[inline(always)]
    pub fn data8(&mut self) -> Data8W<'_, RdbSpec> {
        Data8W::new(self, 24)
    }
}
#[doc = "Received data bytes 5-8. Can only be written when RM in CANMOD is 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`rdb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RdbSpec;
impl crate::RegisterSpec for RdbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rdb::R`](R) reader structure"]
impl crate::Readable for RdbSpec {}
#[doc = "`write(|w| ..)` method takes [`rdb::W`](W) writer structure"]
impl crate::Writable for RdbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RDB to value 0"]
impl crate::Resettable for RdbSpec {}
