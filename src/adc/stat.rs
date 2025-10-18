#[doc = "Register `STAT` reader"]
pub type R = crate::R<StatSpec>;
#[doc = "Field `DONE0` reader - This bit mirrors the DONE status flag from the result register for A/D channel 0."]
pub type Done0R = crate::BitReader;
#[doc = "Field `DONE1` reader - This bit mirrors the DONE status flag from the result register for A/D channel 1."]
pub type Done1R = crate::BitReader;
#[doc = "Field `DONE2` reader - This bit mirrors the DONE status flag from the result register for A/D channel 2."]
pub type Done2R = crate::BitReader;
#[doc = "Field `DONE3` reader - This bit mirrors the DONE status flag from the result register for A/D channel 3."]
pub type Done3R = crate::BitReader;
#[doc = "Field `DONE4` reader - This bit mirrors the DONE status flag from the result register for A/D channel 4."]
pub type Done4R = crate::BitReader;
#[doc = "Field `DONE5` reader - This bit mirrors the DONE status flag from the result register for A/D channel 5."]
pub type Done5R = crate::BitReader;
#[doc = "Field `DONE6` reader - This bit mirrors the DONE status flag from the result register for A/D channel 6."]
pub type Done6R = crate::BitReader;
#[doc = "Field `DONE7` reader - This bit mirrors the DONE status flag from the result register for A/D channel 7."]
pub type Done7R = crate::BitReader;
#[doc = "Field `OVERRUN0` reader - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 0."]
pub type Overrun0R = crate::BitReader;
#[doc = "Field `OVERRUN1` reader - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 1."]
pub type Overrun1R = crate::BitReader;
#[doc = "Field `OVERRUN2` reader - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 2."]
pub type Overrun2R = crate::BitReader;
#[doc = "Field `OVERRUN3` reader - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 3."]
pub type Overrun3R = crate::BitReader;
#[doc = "Field `OVERRUN4` reader - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 4."]
pub type Overrun4R = crate::BitReader;
#[doc = "Field `OVERRUN5` reader - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 5."]
pub type Overrun5R = crate::BitReader;
#[doc = "Field `OVERRUN6` reader - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 6."]
pub type Overrun6R = crate::BitReader;
#[doc = "Field `OVERRUN7` reader - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 7."]
pub type Overrun7R = crate::BitReader;
#[doc = "Field `ADINT` reader - This bit is the A/D interrupt flag. It is one when any of the individual A/D channel Done flags is asserted and enabled to contribute to the A/D interrupt via the ADINTEN register."]
pub type AdintR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This bit mirrors the DONE status flag from the result register for A/D channel 0."]
    #[inline(always)]
    pub fn done0(&self) -> Done0R {
        Done0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This bit mirrors the DONE status flag from the result register for A/D channel 1."]
    #[inline(always)]
    pub fn done1(&self) -> Done1R {
        Done1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This bit mirrors the DONE status flag from the result register for A/D channel 2."]
    #[inline(always)]
    pub fn done2(&self) -> Done2R {
        Done2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit mirrors the DONE status flag from the result register for A/D channel 3."]
    #[inline(always)]
    pub fn done3(&self) -> Done3R {
        Done3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This bit mirrors the DONE status flag from the result register for A/D channel 4."]
    #[inline(always)]
    pub fn done4(&self) -> Done4R {
        Done4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This bit mirrors the DONE status flag from the result register for A/D channel 5."]
    #[inline(always)]
    pub fn done5(&self) -> Done5R {
        Done5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This bit mirrors the DONE status flag from the result register for A/D channel 6."]
    #[inline(always)]
    pub fn done6(&self) -> Done6R {
        Done6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This bit mirrors the DONE status flag from the result register for A/D channel 7."]
    #[inline(always)]
    pub fn done7(&self) -> Done7R {
        Done7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 0."]
    #[inline(always)]
    pub fn overrun0(&self) -> Overrun0R {
        Overrun0R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 1."]
    #[inline(always)]
    pub fn overrun1(&self) -> Overrun1R {
        Overrun1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 2."]
    #[inline(always)]
    pub fn overrun2(&self) -> Overrun2R {
        Overrun2R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 3."]
    #[inline(always)]
    pub fn overrun3(&self) -> Overrun3R {
        Overrun3R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 4."]
    #[inline(always)]
    pub fn overrun4(&self) -> Overrun4R {
        Overrun4R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 5."]
    #[inline(always)]
    pub fn overrun5(&self) -> Overrun5R {
        Overrun5R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 6."]
    #[inline(always)]
    pub fn overrun6(&self) -> Overrun6R {
        Overrun6R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This bit mirrors the OVERRRUN status flag from the result register for A/D channel 7."]
    #[inline(always)]
    pub fn overrun7(&self) -> Overrun7R {
        Overrun7R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - This bit is the A/D interrupt flag. It is one when any of the individual A/D channel Done flags is asserted and enabled to contribute to the A/D interrupt via the ADINTEN register."]
    #[inline(always)]
    pub fn adint(&self) -> AdintR {
        AdintR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "A/D Status Register. This register contains DONE and OVERRUN flags for all of the A/D channels, as well as the A/D interrupt/DMA flag.\n\nYou can [`read`](crate::Reg::read) this register and get [`stat::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StatSpec;
impl crate::RegisterSpec for StatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`stat::R`](R) reader structure"]
impl crate::Readable for StatSpec {}
#[doc = "`reset()` method sets STAT to value 0"]
impl crate::Resettable for StatSpec {}
