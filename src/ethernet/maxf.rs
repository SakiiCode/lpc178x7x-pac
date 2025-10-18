#[doc = "Register `MAXF` reader"]
pub type R = crate::R<MaxfSpec>;
#[doc = "Register `MAXF` writer"]
pub type W = crate::W<MaxfSpec>;
#[doc = "Field `MAXFLEN` reader - MAXIMUM FRAME LENGTH. This field resets to the value 0x0600, which represents a maximum receive frame of 1536 octets. An untagged maximum size Ethernet frame is 1518 octets. A tagged frame adds four octets for a total of 1522 octets. If a shorter maximum length restriction is desired, program this 16-bit field."]
pub type MaxflenR = crate::FieldReader<u16>;
#[doc = "Field `MAXFLEN` writer - MAXIMUM FRAME LENGTH. This field resets to the value 0x0600, which represents a maximum receive frame of 1536 octets. An untagged maximum size Ethernet frame is 1518 octets. A tagged frame adds four octets for a total of 1522 octets. If a shorter maximum length restriction is desired, program this 16-bit field."]
pub type MaxflenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - MAXIMUM FRAME LENGTH. This field resets to the value 0x0600, which represents a maximum receive frame of 1536 octets. An untagged maximum size Ethernet frame is 1518 octets. A tagged frame adds four octets for a total of 1522 octets. If a shorter maximum length restriction is desired, program this 16-bit field."]
    #[inline(always)]
    pub fn maxflen(&self) -> MaxflenR {
        MaxflenR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - MAXIMUM FRAME LENGTH. This field resets to the value 0x0600, which represents a maximum receive frame of 1536 octets. An untagged maximum size Ethernet frame is 1518 octets. A tagged frame adds four octets for a total of 1522 octets. If a shorter maximum length restriction is desired, program this 16-bit field."]
    #[inline(always)]
    pub fn maxflen(&mut self) -> MaxflenW<'_, MaxfSpec> {
        MaxflenW::new(self, 0)
    }
}
#[doc = "Maximum Frame register.\n\nYou can [`read`](crate::Reg::read) this register and get [`maxf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaxfSpec;
impl crate::RegisterSpec for MaxfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maxf::R`](R) reader structure"]
impl crate::Readable for MaxfSpec {}
#[doc = "`write(|w| ..)` method takes [`maxf::W`](W) writer structure"]
impl crate::Writable for MaxfSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAXF to value 0x0600"]
impl crate::Resettable for MaxfSpec {
    const RESET_VALUE: u32 = 0x0600;
}
