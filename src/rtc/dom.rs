#[doc = "Register `DOM` reader"]
pub type R = crate::R<DomSpec>;
#[doc = "Register `DOM` writer"]
pub type W = crate::W<DomSpec>;
#[doc = "Field `DOM` reader - Day of month value in the range of 1 to 28, 29, 30, or 31 (depending on the month and whether it is a leap year)."]
pub type DomR = crate::FieldReader;
#[doc = "Field `DOM` writer - Day of month value in the range of 1 to 28, 29, 30, or 31 (depending on the month and whether it is a leap year)."]
pub type DomW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Day of month value in the range of 1 to 28, 29, 30, or 31 (depending on the month and whether it is a leap year)."]
    #[inline(always)]
    pub fn dom(&self) -> DomR {
        DomR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Day of month value in the range of 1 to 28, 29, 30, or 31 (depending on the month and whether it is a leap year)."]
    #[inline(always)]
    pub fn dom(&mut self) -> DomW<'_, DomSpec> {
        DomW::new(self, 0)
    }
}
#[doc = "Day of Month Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dom::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dom::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DomSpec;
impl crate::RegisterSpec for DomSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dom::R`](R) reader structure"]
impl crate::Readable for DomSpec {}
#[doc = "`write(|w| ..)` method takes [`dom::W`](W) writer structure"]
impl crate::Writable for DomSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DOM to value 0"]
impl crate::Resettable for DomSpec {}
