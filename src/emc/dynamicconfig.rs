#[doc = "Register `DYNAMICCONFIG%s` reader"]
pub type R = crate::R<DynamicconfigSpec>;
#[doc = "Register `DYNAMICCONFIG%s` writer"]
pub type W = crate::W<DynamicconfigSpec>;
#[doc = "Memory device.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Md {
    #[doc = "0: SDRAM (POR reset value)."]
    SdramPorResetVal = 0,
    #[doc = "1: Low-power SDRAM."]
    LowPowerSdram_ = 1,
    #[doc = "2: Reserved."]
    Reserved0x2 = 2,
    #[doc = "3: Reserved."]
    Reserved0x3 = 3,
}
impl From<Md> for u8 {
    #[inline(always)]
    fn from(variant: Md) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Md {
    type Ux = u8;
}
impl crate::IsEnum for Md {}
#[doc = "Field `MD` reader - Memory device."]
pub type MdR = crate::FieldReader<Md>;
impl MdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Md {
        match self.bits {
            0 => Md::SdramPorResetVal,
            1 => Md::LowPowerSdram_,
            2 => Md::Reserved0x2,
            3 => Md::Reserved0x3,
            _ => unreachable!(),
        }
    }
    #[doc = "SDRAM (POR reset value)."]
    #[inline(always)]
    pub fn is_sdram_por_reset_val(&self) -> bool {
        *self == Md::SdramPorResetVal
    }
    #[doc = "Low-power SDRAM."]
    #[inline(always)]
    pub fn is_low_power_sdram_(&self) -> bool {
        *self == Md::LowPowerSdram_
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn is_reserved_0x2(&self) -> bool {
        *self == Md::Reserved0x2
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn is_reserved_0x3(&self) -> bool {
        *self == Md::Reserved0x3
    }
}
#[doc = "Field `MD` writer - Memory device."]
pub type MdW<'a, REG> = crate::FieldWriter<'a, REG, 2, Md, crate::Safe>;
impl<'a, REG> MdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "SDRAM (POR reset value)."]
    #[inline(always)]
    pub fn sdram_por_reset_val(self) -> &'a mut crate::W<REG> {
        self.variant(Md::SdramPorResetVal)
    }
    #[doc = "Low-power SDRAM."]
    #[inline(always)]
    pub fn low_power_sdram_(self) -> &'a mut crate::W<REG> {
        self.variant(Md::LowPowerSdram_)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn reserved_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(Md::Reserved0x2)
    }
    #[doc = "Reserved."]
    #[inline(always)]
    pub fn reserved_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(Md::Reserved0x3)
    }
}
#[doc = "Field `AM0` reader - See Table 133. 000000 = reset value.\\[1\\]"]
pub type Am0R = crate::FieldReader;
#[doc = "Field `AM0` writer - See Table 133. 000000 = reset value.\\[1\\]"]
pub type Am0W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `AM1` reader - See Table 133. 0 = reset value."]
pub type Am1R = crate::BitReader;
#[doc = "Field `AM1` writer - See Table 133. 0 = reset value."]
pub type Am1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Buffer enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B {
    #[doc = "0: Buffer disabled for accesses to this chip select (POR reset value)."]
    BufferDisabledFor_ = 0,
    #[doc = "1: Buffer enabled for accesses to this chip select.\\[2\\]"]
    BufferEnabledForA = 1,
}
impl From<B> for bool {
    #[inline(always)]
    fn from(variant: B) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B` reader - Buffer enable."]
pub type BR = crate::BitReader<B>;
impl BR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> B {
        match self.bits {
            false => B::BufferDisabledFor_,
            true => B::BufferEnabledForA,
        }
    }
    #[doc = "Buffer disabled for accesses to this chip select (POR reset value)."]
    #[inline(always)]
    pub fn is_buffer_disabled_for_(&self) -> bool {
        *self == B::BufferDisabledFor_
    }
    #[doc = "Buffer enabled for accesses to this chip select.\\[2\\]"]
    #[inline(always)]
    pub fn is_buffer_enabled_for_a(&self) -> bool {
        *self == B::BufferEnabledForA
    }
}
#[doc = "Field `B` writer - Buffer enable."]
pub type BW<'a, REG> = crate::BitWriter<'a, REG, B>;
impl<'a, REG> BW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Buffer disabled for accesses to this chip select (POR reset value)."]
    #[inline(always)]
    pub fn buffer_disabled_for_(self) -> &'a mut crate::W<REG> {
        self.variant(B::BufferDisabledFor_)
    }
    #[doc = "Buffer enabled for accesses to this chip select.\\[2\\]"]
    #[inline(always)]
    pub fn buffer_enabled_for_a(self) -> &'a mut crate::W<REG> {
        self.variant(B::BufferEnabledForA)
    }
}
#[doc = "Write protect.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P {
    #[doc = "0: Writes not protected (POR reset value)."]
    WritesNotProtected = 0,
    #[doc = "1: Writes protected."]
    WritesProtected_ = 1,
}
impl From<P> for bool {
    #[inline(always)]
    fn from(variant: P) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P` reader - Write protect."]
pub type PR = crate::BitReader<P>;
impl PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P {
        match self.bits {
            false => P::WritesNotProtected,
            true => P::WritesProtected_,
        }
    }
    #[doc = "Writes not protected (POR reset value)."]
    #[inline(always)]
    pub fn is_writes_not_protected(&self) -> bool {
        *self == P::WritesNotProtected
    }
    #[doc = "Writes protected."]
    #[inline(always)]
    pub fn is_writes_protected_(&self) -> bool {
        *self == P::WritesProtected_
    }
}
#[doc = "Field `P` writer - Write protect."]
pub type PW<'a, REG> = crate::BitWriter<'a, REG, P>;
impl<'a, REG> PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writes not protected (POR reset value)."]
    #[inline(always)]
    pub fn writes_not_protected(self) -> &'a mut crate::W<REG> {
        self.variant(P::WritesNotProtected)
    }
    #[doc = "Writes protected."]
    #[inline(always)]
    pub fn writes_protected_(self) -> &'a mut crate::W<REG> {
        self.variant(P::WritesProtected_)
    }
}
impl R {
    #[doc = "Bits 3:4 - Memory device."]
    #[inline(always)]
    pub fn md(&self) -> MdR {
        MdR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 7:12 - See Table 133. 000000 = reset value.\\[1\\]"]
    #[inline(always)]
    pub fn am0(&self) -> Am0R {
        Am0R::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - See Table 133. 0 = reset value."]
    #[inline(always)]
    pub fn am1(&self) -> Am1R {
        Am1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 19 - Buffer enable."]
    #[inline(always)]
    pub fn b(&self) -> BR {
        BR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Write protect."]
    #[inline(always)]
    pub fn p(&self) -> PR {
        PR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 3:4 - Memory device."]
    #[inline(always)]
    pub fn md(&mut self) -> MdW<'_, DynamicconfigSpec> {
        MdW::new(self, 3)
    }
    #[doc = "Bits 7:12 - See Table 133. 000000 = reset value.\\[1\\]"]
    #[inline(always)]
    pub fn am0(&mut self) -> Am0W<'_, DynamicconfigSpec> {
        Am0W::new(self, 7)
    }
    #[doc = "Bit 14 - See Table 133. 0 = reset value."]
    #[inline(always)]
    pub fn am1(&mut self) -> Am1W<'_, DynamicconfigSpec> {
        Am1W::new(self, 14)
    }
    #[doc = "Bit 19 - Buffer enable."]
    #[inline(always)]
    pub fn b(&mut self) -> BW<'_, DynamicconfigSpec> {
        BW::new(self, 19)
    }
    #[doc = "Bit 20 - Write protect."]
    #[inline(always)]
    pub fn p(&mut self) -> PW<'_, DynamicconfigSpec> {
        PW::new(self, 20)
    }
}
#[doc = "Configuration information for EMC_DYCS0.\n\nYou can [`read`](crate::Reg::read) this register and get [`dynamicconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dynamicconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DynamicconfigSpec;
impl crate::RegisterSpec for DynamicconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dynamicconfig::R`](R) reader structure"]
impl crate::Readable for DynamicconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`dynamicconfig::W`](W) writer structure"]
impl crate::Writable for DynamicconfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DYNAMICCONFIG%s to value 0"]
impl crate::Resettable for DynamicconfigSpec {}
