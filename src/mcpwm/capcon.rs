#[doc = "Register `CAPCON` reader"]
pub type R = crate::R<CapconSpec>;
#[doc = "Field `CAP0MCI0_RE` reader - A 1 in this bit enables a channel 0 capture event on a rising edge on MCI0."]
pub type Cap0mci0ReR = crate::BitReader;
#[doc = "Field `CAP0MCI0_FE` reader - A 1 in this bit enables a channel 0 capture event on a falling edge on MCI0."]
pub type Cap0mci0FeR = crate::BitReader;
#[doc = "Field `CAP0MCI1_RE` reader - A 1 in this bit enables a channel 0 capture event on a rising edge on MCI1."]
pub type Cap0mci1ReR = crate::BitReader;
#[doc = "Field `CAP0MCI1_FE` reader - A 1 in this bit enables a channel 0 capture event on a falling edge on MCI1."]
pub type Cap0mci1FeR = crate::BitReader;
#[doc = "Field `CAP0MCI2_RE` reader - A 1 in this bit enables a channel 0 capture event on a rising edge on MCI2."]
pub type Cap0mci2ReR = crate::BitReader;
#[doc = "Field `CAP0MCI2_FE` reader - A 1 in this bit enables a channel 0 capture event on a falling edge on MCI2."]
pub type Cap0mci2FeR = crate::BitReader;
#[doc = "Field `CAP1MCI0_RE` reader - A 1 in this bit enables a channel 1 capture event on a rising edge on MCI0."]
pub type Cap1mci0ReR = crate::BitReader;
#[doc = "Field `CAP1MCI0_FE` reader - A 1 in this bit enables a channel 1 capture event on a falling edge on MCI0."]
pub type Cap1mci0FeR = crate::BitReader;
#[doc = "Field `CAP1MCI1_RE` reader - A 1 in this bit enables a channel 1 capture event on a rising edge on MCI1."]
pub type Cap1mci1ReR = crate::BitReader;
#[doc = "Field `CAP1MCI1_FE` reader - A 1 in this bit enables a channel 1 capture event on a falling edge on MCI1."]
pub type Cap1mci1FeR = crate::BitReader;
#[doc = "Field `CAP1MCI2_RE` reader - A 1 in this bit enables a channel 1 capture event on a rising edge on MCI2."]
pub type Cap1mci2ReR = crate::BitReader;
#[doc = "Field `CAP1MCI2_FE` reader - A 1 in this bit enables a channel 1 capture event on a falling edge on MCI2."]
pub type Cap1mci2FeR = crate::BitReader;
#[doc = "Field `CAP2MCI0_RE` reader - A 1 in this bit enables a channel 2 capture event on a rising edge on MCI0."]
pub type Cap2mci0ReR = crate::BitReader;
#[doc = "Field `CAP2MCI0_FE` reader - A 1 in this bit enables a channel 2 capture event on a falling edge on MCI0."]
pub type Cap2mci0FeR = crate::BitReader;
#[doc = "Field `CAP2MCI1_RE` reader - A 1 in this bit enables a channel 2 capture event on a rising edge on MCI1."]
pub type Cap2mci1ReR = crate::BitReader;
#[doc = "Field `CAP2MCI1_FE` reader - A 1 in this bit enables a channel 2 capture event on a falling edge on MCI1."]
pub type Cap2mci1FeR = crate::BitReader;
#[doc = "Field `CAP2MCI2_RE` reader - A 1 in this bit enables a channel 2 capture event on a rising edge on MCI2."]
pub type Cap2mci2ReR = crate::BitReader;
#[doc = "Field `CAP2MCI2_FE` reader - A 1 in this bit enables a channel 2 capture event on a falling edge on MCI2."]
pub type Cap2mci2FeR = crate::BitReader;
#[doc = "Field `RT0` reader - If this bit is 1, TC0 is reset by a channel 0 capture event."]
pub type Rt0R = crate::BitReader;
#[doc = "Field `RT1` reader - If this bit is 1, TC1 is reset by a channel 1 capture event."]
pub type Rt1R = crate::BitReader;
#[doc = "Field `RT2` reader - If this bit is 1, TC2 is reset by a channel 2 capture event."]
pub type Rt2R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - A 1 in this bit enables a channel 0 capture event on a rising edge on MCI0."]
    #[inline(always)]
    pub fn cap0mci0_re(&self) -> Cap0mci0ReR {
        Cap0mci0ReR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - A 1 in this bit enables a channel 0 capture event on a falling edge on MCI0."]
    #[inline(always)]
    pub fn cap0mci0_fe(&self) -> Cap0mci0FeR {
        Cap0mci0FeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - A 1 in this bit enables a channel 0 capture event on a rising edge on MCI1."]
    #[inline(always)]
    pub fn cap0mci1_re(&self) -> Cap0mci1ReR {
        Cap0mci1ReR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - A 1 in this bit enables a channel 0 capture event on a falling edge on MCI1."]
    #[inline(always)]
    pub fn cap0mci1_fe(&self) -> Cap0mci1FeR {
        Cap0mci1FeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - A 1 in this bit enables a channel 0 capture event on a rising edge on MCI2."]
    #[inline(always)]
    pub fn cap0mci2_re(&self) -> Cap0mci2ReR {
        Cap0mci2ReR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - A 1 in this bit enables a channel 0 capture event on a falling edge on MCI2."]
    #[inline(always)]
    pub fn cap0mci2_fe(&self) -> Cap0mci2FeR {
        Cap0mci2FeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - A 1 in this bit enables a channel 1 capture event on a rising edge on MCI0."]
    #[inline(always)]
    pub fn cap1mci0_re(&self) -> Cap1mci0ReR {
        Cap1mci0ReR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - A 1 in this bit enables a channel 1 capture event on a falling edge on MCI0."]
    #[inline(always)]
    pub fn cap1mci0_fe(&self) -> Cap1mci0FeR {
        Cap1mci0FeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - A 1 in this bit enables a channel 1 capture event on a rising edge on MCI1."]
    #[inline(always)]
    pub fn cap1mci1_re(&self) -> Cap1mci1ReR {
        Cap1mci1ReR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - A 1 in this bit enables a channel 1 capture event on a falling edge on MCI1."]
    #[inline(always)]
    pub fn cap1mci1_fe(&self) -> Cap1mci1FeR {
        Cap1mci1FeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - A 1 in this bit enables a channel 1 capture event on a rising edge on MCI2."]
    #[inline(always)]
    pub fn cap1mci2_re(&self) -> Cap1mci2ReR {
        Cap1mci2ReR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - A 1 in this bit enables a channel 1 capture event on a falling edge on MCI2."]
    #[inline(always)]
    pub fn cap1mci2_fe(&self) -> Cap1mci2FeR {
        Cap1mci2FeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - A 1 in this bit enables a channel 2 capture event on a rising edge on MCI0."]
    #[inline(always)]
    pub fn cap2mci0_re(&self) -> Cap2mci0ReR {
        Cap2mci0ReR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - A 1 in this bit enables a channel 2 capture event on a falling edge on MCI0."]
    #[inline(always)]
    pub fn cap2mci0_fe(&self) -> Cap2mci0FeR {
        Cap2mci0FeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - A 1 in this bit enables a channel 2 capture event on a rising edge on MCI1."]
    #[inline(always)]
    pub fn cap2mci1_re(&self) -> Cap2mci1ReR {
        Cap2mci1ReR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - A 1 in this bit enables a channel 2 capture event on a falling edge on MCI1."]
    #[inline(always)]
    pub fn cap2mci1_fe(&self) -> Cap2mci1FeR {
        Cap2mci1FeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - A 1 in this bit enables a channel 2 capture event on a rising edge on MCI2."]
    #[inline(always)]
    pub fn cap2mci2_re(&self) -> Cap2mci2ReR {
        Cap2mci2ReR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - A 1 in this bit enables a channel 2 capture event on a falling edge on MCI2."]
    #[inline(always)]
    pub fn cap2mci2_fe(&self) -> Cap2mci2FeR {
        Cap2mci2FeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - If this bit is 1, TC0 is reset by a channel 0 capture event."]
    #[inline(always)]
    pub fn rt0(&self) -> Rt0R {
        Rt0R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - If this bit is 1, TC1 is reset by a channel 1 capture event."]
    #[inline(always)]
    pub fn rt1(&self) -> Rt1R {
        Rt1R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - If this bit is 1, TC2 is reset by a channel 2 capture event."]
    #[inline(always)]
    pub fn rt2(&self) -> Rt2R {
        Rt2R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "Capture Control read address\n\nYou can [`read`](crate::Reg::read) this register and get [`capcon::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CapconSpec;
impl crate::RegisterSpec for CapconSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`capcon::R`](R) reader structure"]
impl crate::Readable for CapconSpec {}
#[doc = "`reset()` method sets CAPCON to value 0"]
impl crate::Resettable for CapconSpec {}
