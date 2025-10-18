#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Endian mode. On power-on reset, the value of the endian bit is 0. All data must be flushed in the EMC before switching between little-endian and big-endian modes.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Little-endian mode (POR reset value)."]
    Littleendian = 0,
    #[doc = "1: Big-endian mode."]
    Bigendian = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EM` reader - Endian mode. On power-on reset, the value of the endian bit is 0. All data must be flushed in the EMC before switching between little-endian and big-endian modes."]
pub type EmR = crate::BitReader<Enum>;
impl EmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Littleendian,
            true => Enum::Bigendian,
        }
    }
    #[doc = "Little-endian mode (POR reset value)."]
    #[inline(always)]
    pub fn is_littleendian(&self) -> bool {
        *self == Enum::Littleendian
    }
    #[doc = "Big-endian mode."]
    #[inline(always)]
    pub fn is_bigendian(&self) -> bool {
        *self == Enum::Bigendian
    }
}
#[doc = "Field `EM` writer - Endian mode. On power-on reset, the value of the endian bit is 0. All data must be flushed in the EMC before switching between little-endian and big-endian modes."]
pub type EmW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> EmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Little-endian mode (POR reset value)."]
    #[inline(always)]
    pub fn littleendian(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Littleendian)
    }
    #[doc = "Big-endian mode."]
    #[inline(always)]
    pub fn bigendian(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Bigendian)
    }
}
#[doc = "CCLK: CLKOUT ratio. This bit must contain 0 for proper operation of the EMC.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: 1:1(POR reset value)"]
    Porreset = 0,
    #[doc = "1: 1:2 (this option is not available on the LPC178x/177x)"]
    Donotuse = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CLKR` reader - CCLK: CLKOUT ratio. This bit must contain 0 for proper operation of the EMC."]
pub type ClkrR = crate::BitReader<Enum>;
impl ClkrR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Porreset,
            true => Enum::Donotuse,
        }
    }
    #[doc = "1:1(POR reset value)"]
    #[inline(always)]
    pub fn is_porreset(&self) -> bool {
        *self == Enum::Porreset
    }
    #[doc = "1:2 (this option is not available on the LPC178x/177x)"]
    #[inline(always)]
    pub fn is_donotuse(&self) -> bool {
        *self == Enum::Donotuse
    }
}
#[doc = "Field `CLKR` writer - CCLK: CLKOUT ratio. This bit must contain 0 for proper operation of the EMC."]
pub type ClkrW<'a, REG> = crate::BitWriter<'a, REG, Enum>;
impl<'a, REG> ClkrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "1:1(POR reset value)"]
    #[inline(always)]
    pub fn porreset(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Porreset)
    }
    #[doc = "1:2 (this option is not available on the LPC178x/177x)"]
    #[inline(always)]
    pub fn donotuse(self) -> &'a mut crate::W<REG> {
        self.variant(Enum::Donotuse)
    }
}
impl R {
    #[doc = "Bit 0 - Endian mode. On power-on reset, the value of the endian bit is 0. All data must be flushed in the EMC before switching between little-endian and big-endian modes."]
    #[inline(always)]
    pub fn em(&self) -> EmR {
        EmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - CCLK: CLKOUT ratio. This bit must contain 0 for proper operation of the EMC."]
    #[inline(always)]
    pub fn clkr(&self) -> ClkrR {
        ClkrR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Endian mode. On power-on reset, the value of the endian bit is 0. All data must be flushed in the EMC before switching between little-endian and big-endian modes."]
    #[inline(always)]
    pub fn em(&mut self) -> EmW<'_, ConfigSpec> {
        EmW::new(self, 0)
    }
    #[doc = "Bit 8 - CCLK: CLKOUT ratio. This bit must contain 0 for proper operation of the EMC."]
    #[inline(always)]
    pub fn clkr(&mut self) -> ClkrW<'_, ConfigSpec> {
        ClkrW::new(self, 8)
    }
}
#[doc = "Configures operation of the memory controller\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for ConfigSpec {}
