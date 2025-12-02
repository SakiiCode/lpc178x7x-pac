#[doc = "Register `FLASHCFG` reader"]
pub type R = crate::R<FlashcfgSpec>;
#[doc = "Register `FLASHCFG` writer"]
pub type W = crate::W<FlashcfgSpec>;
#[doc = "Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. All other values are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flashtim {
    #[doc = "0: Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock with power boost off."]
    _1Clock = 0,
    #[doc = "1: Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock with power boost off."]
    _2Clocks = 1,
    #[doc = "2: Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock with power boost off."]
    _3Clocks = 2,
    #[doc = "3: Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock with power boost off. Use this setting for operation from 100 to 120 MHz operation with power boost on."]
    _4Clocks = 3,
    #[doc = "4: Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock with power boost off."]
    _5Clocks = 4,
    #[doc = "5: Flash accesses use 6 CPU clocks. Safe setting for any allowed conditions."]
    _6Clocks = 5,
}
impl From<Flashtim> for u8 {
    #[inline(always)]
    fn from(variant: Flashtim) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Flashtim {
    type Ux = u8;
}
impl crate::IsEnum for Flashtim {}
#[doc = "Field `FLASHTIM` reader - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. All other values are reserved."]
pub type FlashtimR = crate::FieldReader<Flashtim>;
impl FlashtimR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Flashtim> {
        match self.bits {
            0 => Some(Flashtim::_1Clock),
            1 => Some(Flashtim::_2Clocks),
            2 => Some(Flashtim::_3Clocks),
            3 => Some(Flashtim::_4Clocks),
            4 => Some(Flashtim::_5Clocks),
            5 => Some(Flashtim::_6Clocks),
            _ => None,
        }
    }
    #[doc = "Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock with power boost off."]
    #[inline(always)]
    pub fn is_1_clock(&self) -> bool {
        *self == Flashtim::_1Clock
    }
    #[doc = "Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock with power boost off."]
    #[inline(always)]
    pub fn is_2_clocks(&self) -> bool {
        *self == Flashtim::_2Clocks
    }
    #[doc = "Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock with power boost off."]
    #[inline(always)]
    pub fn is_3_clocks(&self) -> bool {
        *self == Flashtim::_3Clocks
    }
    #[doc = "Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock with power boost off. Use this setting for operation from 100 to 120 MHz operation with power boost on."]
    #[inline(always)]
    pub fn is_4_clocks(&self) -> bool {
        *self == Flashtim::_4Clocks
    }
    #[doc = "Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock with power boost off."]
    #[inline(always)]
    pub fn is_5_clocks(&self) -> bool {
        *self == Flashtim::_5Clocks
    }
    #[doc = "Flash accesses use 6 CPU clocks. Safe setting for any allowed conditions."]
    #[inline(always)]
    pub fn is_6_clocks(&self) -> bool {
        *self == Flashtim::_6Clocks
    }
}
#[doc = "Field `FLASHTIM` writer - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. All other values are reserved."]
pub type FlashtimW<'a, REG> = crate::FieldWriter<'a, REG, 4, Flashtim>;
impl<'a, REG> FlashtimW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock with power boost off."]
    #[inline(always)]
    pub fn _1_clock(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::_1Clock)
    }
    #[doc = "Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock with power boost off."]
    #[inline(always)]
    pub fn _2_clocks(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::_2Clocks)
    }
    #[doc = "Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock with power boost off."]
    #[inline(always)]
    pub fn _3_clocks(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::_3Clocks)
    }
    #[doc = "Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock with power boost off. Use this setting for operation from 100 to 120 MHz operation with power boost on."]
    #[inline(always)]
    pub fn _4_clocks(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::_4Clocks)
    }
    #[doc = "Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock with power boost off."]
    #[inline(always)]
    pub fn _5_clocks(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::_5Clocks)
    }
    #[doc = "Flash accesses use 6 CPU clocks. Safe setting for any allowed conditions."]
    #[inline(always)]
    pub fn _6_clocks(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::_6Clocks)
    }
}
impl R {
    #[doc = "Bits 12:15 - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. All other values are reserved."]
    #[inline(always)]
    pub fn flashtim(&self) -> FlashtimR {
        FlashtimR::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. All other values are reserved."]
    #[inline(always)]
    pub fn flashtim(&mut self) -> FlashtimW<'_, FlashcfgSpec> {
        FlashtimW::new(self, 12)
    }
}
#[doc = "Flash Accelerator Configuration Register. Controls flash access timing.\n\nYou can [`read`](crate::Reg::read) this register and get [`flashcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FlashcfgSpec;
impl crate::RegisterSpec for FlashcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`flashcfg::R`](R) reader structure"]
impl crate::Readable for FlashcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`flashcfg::W`](W) writer structure"]
impl crate::Writable for FlashcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FLASHCFG to value 0"]
impl crate::Resettable for FlashcfgSpec {}
