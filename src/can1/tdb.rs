#[doc = "Register `TDB%s` reader"]
pub type R = crate::R<TdbSpec>;
#[doc = "Register `TDB%s` writer"]
pub type W = crate::W<TdbSpec>;
#[doc = "Field `DATA5` reader - Data 5. If RTR = 0 and DLC >= 0101 in the corresponding CANTFI, this byte is sent as the 5th Data byte of the next transmit message."]
pub type Data5R = crate::FieldReader;
#[doc = "Field `DATA5` writer - Data 5. If RTR = 0 and DLC >= 0101 in the corresponding CANTFI, this byte is sent as the 5th Data byte of the next transmit message."]
pub type Data5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA6` reader - Data 6. If RTR = 0 and DLC >= 0110 in the corresponding CANTFI, this byte is sent as the 6th Data byte of the next transmit message."]
pub type Data6R = crate::FieldReader;
#[doc = "Field `DATA6` writer - Data 6. If RTR = 0 and DLC >= 0110 in the corresponding CANTFI, this byte is sent as the 6th Data byte of the next transmit message."]
pub type Data6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA7` reader - Data 7. If RTR = 0 and DLC >= 0111 in the corresponding CANTFI, this byte is sent as the 7th Data byte of the next transmit message."]
pub type Data7R = crate::FieldReader;
#[doc = "Field `DATA7` writer - Data 7. If RTR = 0 and DLC >= 0111 in the corresponding CANTFI, this byte is sent as the 7th Data byte of the next transmit message."]
pub type Data7W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA8` reader - Data 8. If RTR = 0 and DLC >= 1000 in the corresponding CANTFI, this byte is sent as the 8th Data byte of the next transmit message."]
pub type Data8R = crate::FieldReader;
#[doc = "Field `DATA8` writer - Data 8. If RTR = 0 and DLC >= 1000 in the corresponding CANTFI, this byte is sent as the 8th Data byte of the next transmit message."]
pub type Data8W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data 5. If RTR = 0 and DLC >= 0101 in the corresponding CANTFI, this byte is sent as the 5th Data byte of the next transmit message."]
    #[inline(always)]
    pub fn data5(&self) -> Data5R {
        Data5R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data 6. If RTR = 0 and DLC >= 0110 in the corresponding CANTFI, this byte is sent as the 6th Data byte of the next transmit message."]
    #[inline(always)]
    pub fn data6(&self) -> Data6R {
        Data6R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data 7. If RTR = 0 and DLC >= 0111 in the corresponding CANTFI, this byte is sent as the 7th Data byte of the next transmit message."]
    #[inline(always)]
    pub fn data7(&self) -> Data7R {
        Data7R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data 8. If RTR = 0 and DLC >= 1000 in the corresponding CANTFI, this byte is sent as the 8th Data byte of the next transmit message."]
    #[inline(always)]
    pub fn data8(&self) -> Data8R {
        Data8R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 5. If RTR = 0 and DLC >= 0101 in the corresponding CANTFI, this byte is sent as the 5th Data byte of the next transmit message."]
    #[inline(always)]
    pub fn data5(&mut self) -> Data5W<'_, TdbSpec> {
        Data5W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Data 6. If RTR = 0 and DLC >= 0110 in the corresponding CANTFI, this byte is sent as the 6th Data byte of the next transmit message."]
    #[inline(always)]
    pub fn data6(&mut self) -> Data6W<'_, TdbSpec> {
        Data6W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Data 7. If RTR = 0 and DLC >= 0111 in the corresponding CANTFI, this byte is sent as the 7th Data byte of the next transmit message."]
    #[inline(always)]
    pub fn data7(&mut self) -> Data7W<'_, TdbSpec> {
        Data7W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Data 8. If RTR = 0 and DLC >= 1000 in the corresponding CANTFI, this byte is sent as the 8th Data byte of the next transmit message."]
    #[inline(always)]
    pub fn data8(&mut self) -> Data8W<'_, TdbSpec> {
        Data8W::new(self, 24)
    }
}
#[doc = "Transmit data bytes 5-8 (Tx Buffer )\n\nYou can [`read`](crate::Reg::read) this register and get [`tdb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdbSpec;
impl crate::RegisterSpec for TdbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdb::R`](R) reader structure"]
impl crate::Readable for TdbSpec {}
#[doc = "`write(|w| ..)` method takes [`tdb::W`](W) writer structure"]
impl crate::Writable for TdbSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TDB%s to value 0"]
impl crate::Resettable for TdbSpec {}
