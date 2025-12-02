#[doc = "Register `STATICCONFIG%s` reader"]
pub type R = crate::R<StaticconfigSpec>;
#[doc = "Register `STATICCONFIG%s` writer"]
pub type W = crate::W<StaticconfigSpec>;
#[doc = "Memory width.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Mw {
    #[doc = "0: 8 bit (POR reset value)."]
    _8Bit = 0,
    #[doc = "1: 16 bit."]
    _16Bit = 1,
    #[doc = "2: 32 bit."]
    _32Bit = 2,
}
impl From<Mw> for u8 {
    #[inline(always)]
    fn from(variant: Mw) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Mw {
    type Ux = u8;
}
impl crate::IsEnum for Mw {}
#[doc = "Field `MW` reader - Memory width."]
pub type MwR = crate::FieldReader<Mw>;
impl MwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mw {
        match self.bits {
            0 => Mw::_8Bit,
            1 => Mw::_16Bit,
            2 => Mw::_32Bit,
            _ => unreachable!(),
        }
    }
    #[doc = "8 bit (POR reset value)."]
    #[inline(always)]
    pub fn is_8_bit(&self) -> bool {
        *self == Mw::_8Bit
    }
    #[doc = "16 bit."]
    #[inline(always)]
    pub fn is_16_bit(&self) -> bool {
        *self == Mw::_16Bit
    }
    #[doc = "32 bit."]
    #[inline(always)]
    pub fn is_32_bit(&self) -> bool {
        *self == Mw::_32Bit
    }
}
#[doc = "Field `MW` writer - Memory width."]
pub type MwW<'a, REG> = crate::FieldWriter<'a, REG, 2, Mw>;
impl<'a, REG> MwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "8 bit (POR reset value)."]
    #[inline(always)]
    pub fn _8_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mw::_8Bit)
    }
    #[doc = "16 bit."]
    #[inline(always)]
    pub fn _16_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mw::_16Bit)
    }
    #[doc = "32 bit."]
    #[inline(always)]
    pub fn _32_bit(self) -> &'a mut crate::W<REG> {
        self.variant(Mw::_32Bit)
    }
}
#[doc = "Page mode. In page mode the EMC can burst up to four external accesses. Therefore devices with asynchronous page mode burst four or higher devices are supported. Asynchronous page mode burst two devices are not supported and must be accessed normally.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pm {
    #[doc = "0: Disabled (POR reset value)."]
    Disabled = 0,
    #[doc = "1: Asynchronous page mode enabled (page length four)."]
    Enabled = 1,
}
impl From<Pm> for bool {
    #[inline(always)]
    fn from(variant: Pm) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PM` reader - Page mode. In page mode the EMC can burst up to four external accesses. Therefore devices with asynchronous page mode burst four or higher devices are supported. Asynchronous page mode burst two devices are not supported and must be accessed normally."]
pub type PmR = crate::BitReader<Pm>;
impl PmR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pm {
        match self.bits {
            false => Pm::Disabled,
            true => Pm::Enabled,
        }
    }
    #[doc = "Disabled (POR reset value)."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pm::Disabled
    }
    #[doc = "Asynchronous page mode enabled (page length four)."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pm::Enabled
    }
}
#[doc = "Field `PM` writer - Page mode. In page mode the EMC can burst up to four external accesses. Therefore devices with asynchronous page mode burst four or higher devices are supported. Asynchronous page mode burst two devices are not supported and must be accessed normally."]
pub type PmW<'a, REG> = crate::BitWriter<'a, REG, Pm>;
impl<'a, REG> PmW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled (POR reset value)."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pm::Disabled)
    }
    #[doc = "Asynchronous page mode enabled (page length four)."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pm::Enabled)
    }
}
#[doc = "Chip select polarity. The value of the chip select polarity on power-on reset is 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pc {
    #[doc = "0: Active LOW chip select."]
    ActiveLow = 0,
    #[doc = "1: Active HIGH chip select."]
    ActiveHigh = 1,
}
impl From<Pc> for bool {
    #[inline(always)]
    fn from(variant: Pc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PC` reader - Chip select polarity. The value of the chip select polarity on power-on reset is 0."]
pub type PcR = crate::BitReader<Pc>;
impl PcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pc {
        match self.bits {
            false => Pc::ActiveLow,
            true => Pc::ActiveHigh,
        }
    }
    #[doc = "Active LOW chip select."]
    #[inline(always)]
    pub fn is_active_low(&self) -> bool {
        *self == Pc::ActiveLow
    }
    #[doc = "Active HIGH chip select."]
    #[inline(always)]
    pub fn is_active_high(&self) -> bool {
        *self == Pc::ActiveHigh
    }
}
#[doc = "Field `PC` writer - Chip select polarity. The value of the chip select polarity on power-on reset is 0."]
pub type PcW<'a, REG> = crate::BitWriter<'a, REG, Pc>;
impl<'a, REG> PcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active LOW chip select."]
    #[inline(always)]
    pub fn active_low(self) -> &'a mut crate::W<REG> {
        self.variant(Pc::ActiveLow)
    }
    #[doc = "Active HIGH chip select."]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(Pc::ActiveHigh)
    }
}
#[doc = "Byte lane state. The byte lane state bit, PB, enables different types of memory to be connected. For byte-wide static memories the BLS3:0 signal from the EMC is usually connected to WE (write enable). In this case for reads all the BLS3:0 bits must be HIGH. This means that the byte lane state (PB) bit must be LOW. 16 bit wide static memory devices usually have the BLS3:0 signals connected to the UBn and LBn (upper byte and lower byte) signals in the static memory. In this case a write to a particular byte must assert the appropriate UBn or LBn signal LOW. For reads, all the UB and LB signals must be asserted LOW so that the bus is driven. In this case the byte lane state (PB) bit must be HIGH.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pb {
    #[doc = "0: For reads all the bits in BLS3:0 are HIGH. For writes the respective active bits in BLS3:0 are LOW (POR reset value)."]
    Blshigh = 0,
    #[doc = "1: For reads the respective active bits in BLS3:0 are LOW. For writes the respective active bits in BLS3:0 are LOW."]
    Blslow = 1,
}
impl From<Pb> for bool {
    #[inline(always)]
    fn from(variant: Pb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PB` reader - Byte lane state. The byte lane state bit, PB, enables different types of memory to be connected. For byte-wide static memories the BLS3:0 signal from the EMC is usually connected to WE (write enable). In this case for reads all the BLS3:0 bits must be HIGH. This means that the byte lane state (PB) bit must be LOW. 16 bit wide static memory devices usually have the BLS3:0 signals connected to the UBn and LBn (upper byte and lower byte) signals in the static memory. In this case a write to a particular byte must assert the appropriate UBn or LBn signal LOW. For reads, all the UB and LB signals must be asserted LOW so that the bus is driven. In this case the byte lane state (PB) bit must be HIGH."]
pub type PbR = crate::BitReader<Pb>;
impl PbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pb {
        match self.bits {
            false => Pb::Blshigh,
            true => Pb::Blslow,
        }
    }
    #[doc = "For reads all the bits in BLS3:0 are HIGH. For writes the respective active bits in BLS3:0 are LOW (POR reset value)."]
    #[inline(always)]
    pub fn is_blshigh(&self) -> bool {
        *self == Pb::Blshigh
    }
    #[doc = "For reads the respective active bits in BLS3:0 are LOW. For writes the respective active bits in BLS3:0 are LOW."]
    #[inline(always)]
    pub fn is_blslow(&self) -> bool {
        *self == Pb::Blslow
    }
}
#[doc = "Field `PB` writer - Byte lane state. The byte lane state bit, PB, enables different types of memory to be connected. For byte-wide static memories the BLS3:0 signal from the EMC is usually connected to WE (write enable). In this case for reads all the BLS3:0 bits must be HIGH. This means that the byte lane state (PB) bit must be LOW. 16 bit wide static memory devices usually have the BLS3:0 signals connected to the UBn and LBn (upper byte and lower byte) signals in the static memory. In this case a write to a particular byte must assert the appropriate UBn or LBn signal LOW. For reads, all the UB and LB signals must be asserted LOW so that the bus is driven. In this case the byte lane state (PB) bit must be HIGH."]
pub type PbW<'a, REG> = crate::BitWriter<'a, REG, Pb>;
impl<'a, REG> PbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "For reads all the bits in BLS3:0 are HIGH. For writes the respective active bits in BLS3:0 are LOW (POR reset value)."]
    #[inline(always)]
    pub fn blshigh(self) -> &'a mut crate::W<REG> {
        self.variant(Pb::Blshigh)
    }
    #[doc = "For reads the respective active bits in BLS3:0 are LOW. For writes the respective active bits in BLS3:0 are LOW."]
    #[inline(always)]
    pub fn blslow(self) -> &'a mut crate::W<REG> {
        self.variant(Pb::Blslow)
    }
}
#[doc = "Extended wait (EW) uses the EMCStaticExtendedWait register to time both the read and write transfers rather than the EMCStaticWaitRd and EMCStaticWaitWr registers. This enables much longer transactions. \\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ew {
    #[doc = "0: Extended wait disabled (POR reset value)."]
    Disabled = 0,
    #[doc = "1: Extended wait enabled."]
    Enabled = 1,
}
impl From<Ew> for bool {
    #[inline(always)]
    fn from(variant: Ew) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `EW` reader - Extended wait (EW) uses the EMCStaticExtendedWait register to time both the read and write transfers rather than the EMCStaticWaitRd and EMCStaticWaitWr registers. This enables much longer transactions. \\[1\\]"]
pub type EwR = crate::BitReader<Ew>;
impl EwR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ew {
        match self.bits {
            false => Ew::Disabled,
            true => Ew::Enabled,
        }
    }
    #[doc = "Extended wait disabled (POR reset value)."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Ew::Disabled
    }
    #[doc = "Extended wait enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Ew::Enabled
    }
}
#[doc = "Field `EW` writer - Extended wait (EW) uses the EMCStaticExtendedWait register to time both the read and write transfers rather than the EMCStaticWaitRd and EMCStaticWaitWr registers. This enables much longer transactions. \\[1\\]"]
pub type EwW<'a, REG> = crate::BitWriter<'a, REG, Ew>;
impl<'a, REG> EwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Extended wait disabled (POR reset value)."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ew::Disabled)
    }
    #[doc = "Extended wait enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Ew::Enabled)
    }
}
#[doc = "Buffer enable \\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum B {
    #[doc = "0: Buffer disabled (POR reset value)."]
    Disabled = 0,
    #[doc = "1: Buffer enabled."]
    Enabled = 1,
}
impl From<B> for bool {
    #[inline(always)]
    fn from(variant: B) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `B` reader - Buffer enable \\[2\\]"]
pub type BR = crate::BitReader<B>;
impl BR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> B {
        match self.bits {
            false => B::Disabled,
            true => B::Enabled,
        }
    }
    #[doc = "Buffer disabled (POR reset value)."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == B::Disabled
    }
    #[doc = "Buffer enabled."]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == B::Enabled
    }
}
#[doc = "Field `B` writer - Buffer enable \\[2\\]"]
pub type BW<'a, REG> = crate::BitWriter<'a, REG, B>;
impl<'a, REG> BW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Buffer disabled (POR reset value)."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(B::Disabled)
    }
    #[doc = "Buffer enabled."]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(B::Enabled)
    }
}
#[doc = "Write protect\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum P {
    #[doc = "0: Writes not protected (POR reset value)."]
    NotProtected = 0,
    #[doc = "1: Write protected."]
    Protected = 1,
}
impl From<P> for bool {
    #[inline(always)]
    fn from(variant: P) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `P` reader - Write protect"]
pub type PR = crate::BitReader<P>;
impl PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> P {
        match self.bits {
            false => P::NotProtected,
            true => P::Protected,
        }
    }
    #[doc = "Writes not protected (POR reset value)."]
    #[inline(always)]
    pub fn is_not_protected(&self) -> bool {
        *self == P::NotProtected
    }
    #[doc = "Write protected."]
    #[inline(always)]
    pub fn is_protected(&self) -> bool {
        *self == P::Protected
    }
}
#[doc = "Field `P` writer - Write protect"]
pub type PW<'a, REG> = crate::BitWriter<'a, REG, P>;
impl<'a, REG> PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writes not protected (POR reset value)."]
    #[inline(always)]
    pub fn not_protected(self) -> &'a mut crate::W<REG> {
        self.variant(P::NotProtected)
    }
    #[doc = "Write protected."]
    #[inline(always)]
    pub fn protected(self) -> &'a mut crate::W<REG> {
        self.variant(P::Protected)
    }
}
impl R {
    #[doc = "Bits 0:1 - Memory width."]
    #[inline(always)]
    pub fn mw(&self) -> MwR {
        MwR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Page mode. In page mode the EMC can burst up to four external accesses. Therefore devices with asynchronous page mode burst four or higher devices are supported. Asynchronous page mode burst two devices are not supported and must be accessed normally."]
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Chip select polarity. The value of the chip select polarity on power-on reset is 0."]
    #[inline(always)]
    pub fn pc(&self) -> PcR {
        PcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Byte lane state. The byte lane state bit, PB, enables different types of memory to be connected. For byte-wide static memories the BLS3:0 signal from the EMC is usually connected to WE (write enable). In this case for reads all the BLS3:0 bits must be HIGH. This means that the byte lane state (PB) bit must be LOW. 16 bit wide static memory devices usually have the BLS3:0 signals connected to the UBn and LBn (upper byte and lower byte) signals in the static memory. In this case a write to a particular byte must assert the appropriate UBn or LBn signal LOW. For reads, all the UB and LB signals must be asserted LOW so that the bus is driven. In this case the byte lane state (PB) bit must be HIGH."]
    #[inline(always)]
    pub fn pb(&self) -> PbR {
        PbR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Extended wait (EW) uses the EMCStaticExtendedWait register to time both the read and write transfers rather than the EMCStaticWaitRd and EMCStaticWaitWr registers. This enables much longer transactions. \\[1\\]"]
    #[inline(always)]
    pub fn ew(&self) -> EwR {
        EwR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 19 - Buffer enable \\[2\\]"]
    #[inline(always)]
    pub fn b(&self) -> BR {
        BR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Write protect"]
    #[inline(always)]
    pub fn p(&self) -> PR {
        PR::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory width."]
    #[inline(always)]
    pub fn mw(&mut self) -> MwW<'_, StaticconfigSpec> {
        MwW::new(self, 0)
    }
    #[doc = "Bit 3 - Page mode. In page mode the EMC can burst up to four external accesses. Therefore devices with asynchronous page mode burst four or higher devices are supported. Asynchronous page mode burst two devices are not supported and must be accessed normally."]
    #[inline(always)]
    pub fn pm(&mut self) -> PmW<'_, StaticconfigSpec> {
        PmW::new(self, 3)
    }
    #[doc = "Bit 6 - Chip select polarity. The value of the chip select polarity on power-on reset is 0."]
    #[inline(always)]
    pub fn pc(&mut self) -> PcW<'_, StaticconfigSpec> {
        PcW::new(self, 6)
    }
    #[doc = "Bit 7 - Byte lane state. The byte lane state bit, PB, enables different types of memory to be connected. For byte-wide static memories the BLS3:0 signal from the EMC is usually connected to WE (write enable). In this case for reads all the BLS3:0 bits must be HIGH. This means that the byte lane state (PB) bit must be LOW. 16 bit wide static memory devices usually have the BLS3:0 signals connected to the UBn and LBn (upper byte and lower byte) signals in the static memory. In this case a write to a particular byte must assert the appropriate UBn or LBn signal LOW. For reads, all the UB and LB signals must be asserted LOW so that the bus is driven. In this case the byte lane state (PB) bit must be HIGH."]
    #[inline(always)]
    pub fn pb(&mut self) -> PbW<'_, StaticconfigSpec> {
        PbW::new(self, 7)
    }
    #[doc = "Bit 8 - Extended wait (EW) uses the EMCStaticExtendedWait register to time both the read and write transfers rather than the EMCStaticWaitRd and EMCStaticWaitWr registers. This enables much longer transactions. \\[1\\]"]
    #[inline(always)]
    pub fn ew(&mut self) -> EwW<'_, StaticconfigSpec> {
        EwW::new(self, 8)
    }
    #[doc = "Bit 19 - Buffer enable \\[2\\]"]
    #[inline(always)]
    pub fn b(&mut self) -> BW<'_, StaticconfigSpec> {
        BW::new(self, 19)
    }
    #[doc = "Bit 20 - Write protect"]
    #[inline(always)]
    pub fn p(&mut self) -> PW<'_, StaticconfigSpec> {
        PW::new(self, 20)
    }
}
#[doc = "Configuration for EMC_CS0.\n\nYou can [`read`](crate::Reg::read) this register and get [`staticconfig::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`staticconfig::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StaticconfigSpec;
impl crate::RegisterSpec for StaticconfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`staticconfig::R`](R) reader structure"]
impl crate::Readable for StaticconfigSpec {}
#[doc = "`write(|w| ..)` method takes [`staticconfig::W`](W) writer structure"]
impl crate::Writable for StaticconfigSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets STATICCONFIG%s to value 0"]
impl crate::Resettable for StaticconfigSpec {}
