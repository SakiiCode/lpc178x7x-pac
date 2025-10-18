#[doc = "Register `FLASHCFG` reader"]
pub type R = crate::R<FlashcfgSpec>;
#[doc = "Register `FLASHCFG` writer"]
pub type W = crate::W<FlashcfgSpec>;
#[doc = "Flash access time. The value of this field plus 1 gives the number of CPU clocks used for a flash access. Warning: improper setting of this value may result in incorrect operation of the device. All other values are reserved.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Flashtim {
    #[doc = "0: Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock with power boost off."]
    FlashAccessesUse1 = 0,
    #[doc = "1: Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock with power boost off."]
    FlashAccessesUse2 = 1,
    #[doc = "2: Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock with power boost off."]
    FlashAccessesUse3 = 2,
    #[doc = "3: Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock with power boost off. Use this setting for operation from 100 to 120 MHz operation with power boost on."]
    FlashAccessesUse4 = 3,
    #[doc = "4: Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock with power boost off."]
    FlashAccessesUse5 = 4,
    #[doc = "5: Flash accesses use 6 CPU clocks. Safe setting for any allowed conditions."]
    FlashAccessesUse6 = 5,
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
            0 => Some(Flashtim::FlashAccessesUse1),
            1 => Some(Flashtim::FlashAccessesUse2),
            2 => Some(Flashtim::FlashAccessesUse3),
            3 => Some(Flashtim::FlashAccessesUse4),
            4 => Some(Flashtim::FlashAccessesUse5),
            5 => Some(Flashtim::FlashAccessesUse6),
            _ => None,
        }
    }
    #[doc = "Flash accesses use 1 CPU clock. Use for up to 20 MHz CPU clock with power boost off."]
    #[inline(always)]
    pub fn is_flash_accesses_use_1(&self) -> bool {
        *self == Flashtim::FlashAccessesUse1
    }
    #[doc = "Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock with power boost off."]
    #[inline(always)]
    pub fn is_flash_accesses_use_2(&self) -> bool {
        *self == Flashtim::FlashAccessesUse2
    }
    #[doc = "Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock with power boost off."]
    #[inline(always)]
    pub fn is_flash_accesses_use_3(&self) -> bool {
        *self == Flashtim::FlashAccessesUse3
    }
    #[doc = "Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock with power boost off. Use this setting for operation from 100 to 120 MHz operation with power boost on."]
    #[inline(always)]
    pub fn is_flash_accesses_use_4(&self) -> bool {
        *self == Flashtim::FlashAccessesUse4
    }
    #[doc = "Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock with power boost off."]
    #[inline(always)]
    pub fn is_flash_accesses_use_5(&self) -> bool {
        *self == Flashtim::FlashAccessesUse5
    }
    #[doc = "Flash accesses use 6 CPU clocks. Safe setting for any allowed conditions."]
    #[inline(always)]
    pub fn is_flash_accesses_use_6(&self) -> bool {
        *self == Flashtim::FlashAccessesUse6
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
    pub fn flash_accesses_use_1(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::FlashAccessesUse1)
    }
    #[doc = "Flash accesses use 2 CPU clocks. Use for up to 40 MHz CPU clock with power boost off."]
    #[inline(always)]
    pub fn flash_accesses_use_2(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::FlashAccessesUse2)
    }
    #[doc = "Flash accesses use 3 CPU clocks. Use for up to 60 MHz CPU clock with power boost off."]
    #[inline(always)]
    pub fn flash_accesses_use_3(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::FlashAccessesUse3)
    }
    #[doc = "Flash accesses use 4 CPU clocks. Use for up to 80 MHz CPU clock with power boost off. Use this setting for operation from 100 to 120 MHz operation with power boost on."]
    #[inline(always)]
    pub fn flash_accesses_use_4(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::FlashAccessesUse4)
    }
    #[doc = "Flash accesses use 5 CPU clocks. Use for up to 100 MHz CPU clock with power boost off."]
    #[inline(always)]
    pub fn flash_accesses_use_5(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::FlashAccessesUse5)
    }
    #[doc = "Flash accesses use 6 CPU clocks. Safe setting for any allowed conditions."]
    #[inline(always)]
    pub fn flash_accesses_use_6(self) -> &'a mut crate::W<REG> {
        self.variant(Flashtim::FlashAccessesUse6)
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
