#[doc = "Register `TDA%s` reader"]
pub type R = crate::R<TdaSpec>;
#[doc = "Register `TDA%s` writer"]
pub type W = crate::W<TdaSpec>;
#[doc = "Field `DATA1` reader - Data 1. If RTR = 0 and DLC >= 0001 in the corresponding CANxTFI, this byte is sent as the first Data byte of the next transmit message."]
pub type Data1R = crate::FieldReader;
#[doc = "Field `DATA1` writer - Data 1. If RTR = 0 and DLC >= 0001 in the corresponding CANxTFI, this byte is sent as the first Data byte of the next transmit message."]
pub type Data1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA2` reader - Data 2. If RTR = 0 and DLC >= 0010 in the corresponding CANxTFI, this byte is sent as the 2nd Data byte of the next transmit message."]
pub type Data2R = crate::FieldReader;
#[doc = "Field `DATA2` writer - Data 2. If RTR = 0 and DLC >= 0010 in the corresponding CANxTFI, this byte is sent as the 2nd Data byte of the next transmit message."]
pub type Data2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA3` reader - Data 3. If RTR = 0 and DLC >= 0011 in the corresponding CANxTFI, this byte is sent as the 3rd Data byte of the next transmit message."]
pub type Data3R = crate::FieldReader;
#[doc = "Field `DATA3` writer - Data 3. If RTR = 0 and DLC >= 0011 in the corresponding CANxTFI, this byte is sent as the 3rd Data byte of the next transmit message."]
pub type Data3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA4` reader - Data 4. If RTR = 0 and DLC >= 0100 in the corresponding CANxTFI, this byte is sent as the 4th Data byte of the next transmit message."]
pub type Data4R = crate::FieldReader;
#[doc = "Field `DATA4` writer - Data 4. If RTR = 0 and DLC >= 0100 in the corresponding CANxTFI, this byte is sent as the 4th Data byte of the next transmit message."]
pub type Data4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data 1. If RTR = 0 and DLC >= 0001 in the corresponding CANxTFI, this byte is sent as the first Data byte of the next transmit message."]
    #[inline(always)]
    pub fn data1(&self) -> Data1R {
        Data1R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Data 2. If RTR = 0 and DLC >= 0010 in the corresponding CANxTFI, this byte is sent as the 2nd Data byte of the next transmit message."]
    #[inline(always)]
    pub fn data2(&self) -> Data2R {
        Data2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Data 3. If RTR = 0 and DLC >= 0011 in the corresponding CANxTFI, this byte is sent as the 3rd Data byte of the next transmit message."]
    #[inline(always)]
    pub fn data3(&self) -> Data3R {
        Data3R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data 4. If RTR = 0 and DLC >= 0100 in the corresponding CANxTFI, this byte is sent as the 4th Data byte of the next transmit message."]
    #[inline(always)]
    pub fn data4(&self) -> Data4R {
        Data4R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 1. If RTR = 0 and DLC >= 0001 in the corresponding CANxTFI, this byte is sent as the first Data byte of the next transmit message."]
    #[inline(always)]
    pub fn data1(&mut self) -> Data1W<'_, TdaSpec> {
        Data1W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Data 2. If RTR = 0 and DLC >= 0010 in the corresponding CANxTFI, this byte is sent as the 2nd Data byte of the next transmit message."]
    #[inline(always)]
    pub fn data2(&mut self) -> Data2W<'_, TdaSpec> {
        Data2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Data 3. If RTR = 0 and DLC >= 0011 in the corresponding CANxTFI, this byte is sent as the 3rd Data byte of the next transmit message."]
    #[inline(always)]
    pub fn data3(&mut self) -> Data3W<'_, TdaSpec> {
        Data3W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Data 4. If RTR = 0 and DLC >= 0100 in the corresponding CANxTFI, this byte is sent as the 4th Data byte of the next transmit message."]
    #[inline(always)]
    pub fn data4(&mut self) -> Data4W<'_, TdaSpec> {
        Data4W::new(self, 24)
    }
}
#[doc = "Transmit data bytes 1-4 (Tx Buffer)\n\nYou can [`read`](crate::Reg::read) this register and get [`tda::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tda::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdaSpec;
impl crate::RegisterSpec for TdaSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tda::R`](R) reader structure"]
impl crate::Readable for TdaSpec {}
#[doc = "`write(|w| ..)` method takes [`tda::W`](W) writer structure"]
impl crate::Writable for TdaSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TDA%s to value 0"]
impl crate::Resettable for TdaSpec {}
