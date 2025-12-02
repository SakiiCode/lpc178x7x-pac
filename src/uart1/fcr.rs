#[doc = "Register `FCR` writer"]
pub type W = crate::W<FcrSpec>;
#[doc = "FIFO enable.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fifoen {
    #[doc = "1: Active high enable for both UART1 Rx and TX FIFOs and FCR\\[7:1\\] access. This bit must be set for proper UART1 operation. Any transition on this bit will automatically clear the UART1 FIFOs."]
    ActiveHigh = 1,
}
impl From<Fifoen> for bool {
    #[inline(always)]
    fn from(variant: Fifoen) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIFOEN` writer - FIFO enable."]
pub type FifoenW<'a, REG> = crate::BitWriter<'a, REG, Fifoen>;
impl<'a, REG> FifoenW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Active high enable for both UART1 Rx and TX FIFOs and FCR\\[7:1\\] access. This bit must be set for proper UART1 operation. Any transition on this bit will automatically clear the UART1 FIFOs."]
    #[inline(always)]
    pub fn active_high(self) -> &'a mut crate::W<REG> {
        self.variant(Fifoen::ActiveHigh)
    }
}
#[doc = "RX FIFO Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxfifores {
    #[doc = "0: No impact on either of UART1 FIFOs."]
    DoNothing = 0,
    #[doc = "1: Writing a logic 1 to FCR\\[1\\] will clear all bytes in UART1 Rx FIFO, reset the pointer logic. This bit is self-clearing."]
    Reset = 1,
}
impl From<Rxfifores> for bool {
    #[inline(always)]
    fn from(variant: Rxfifores) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXFIFORES` writer - RX FIFO Reset."]
pub type RxfiforesW<'a, REG> = crate::BitWriter<'a, REG, Rxfifores>;
impl<'a, REG> RxfiforesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No impact on either of UART1 FIFOs."]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfifores::DoNothing)
    }
    #[doc = "Writing a logic 1 to FCR\\[1\\] will clear all bytes in UART1 Rx FIFO, reset the pointer logic. This bit is self-clearing."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Rxfifores::Reset)
    }
}
#[doc = "TX FIFO Reset.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txfifores {
    #[doc = "0: No impact on either of UART1 FIFOs."]
    DoNothing = 0,
    #[doc = "1: Writing a logic 1 to FCR\\[2\\] will clear all bytes in UART1 TX FIFO, reset the pointer logic. This bit is self-clearing."]
    Reset = 1,
}
impl From<Txfifores> for bool {
    #[inline(always)]
    fn from(variant: Txfifores) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXFIFORES` writer - TX FIFO Reset."]
pub type TxfiforesW<'a, REG> = crate::BitWriter<'a, REG, Txfifores>;
impl<'a, REG> TxfiforesW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No impact on either of UART1 FIFOs."]
    #[inline(always)]
    pub fn do_nothing(self) -> &'a mut crate::W<REG> {
        self.variant(Txfifores::DoNothing)
    }
    #[doc = "Writing a logic 1 to FCR\\[2\\] will clear all bytes in UART1 TX FIFO, reset the pointer logic. This bit is self-clearing."]
    #[inline(always)]
    pub fn reset(self) -> &'a mut crate::W<REG> {
        self.variant(Txfifores::Reset)
    }
}
#[doc = "Field `DMAMODE` writer - DMA Mode Select. When the FIFO enable bit (bit 0 of this register) is set, this bit selects the DMA mode. See Section 36.6.6.1."]
pub type DmamodeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "RX Trigger Level. These two bits determine how many receiver UART1 FIFO characters must be written before an interrupt is activated.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Rxtriglvl {
    #[doc = "0: Trigger level 0 (1 character or 0x01)."]
    Level0 = 0,
    #[doc = "1: Trigger level 1 (4 characters or 0x04)."]
    Level1 = 1,
    #[doc = "2: Trigger level 2 (8 characters or 0x08)."]
    Level2 = 2,
    #[doc = "3: Trigger level 3 (14 characters or 0x0E)."]
    Level3 = 3,
}
impl From<Rxtriglvl> for u8 {
    #[inline(always)]
    fn from(variant: Rxtriglvl) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Rxtriglvl {
    type Ux = u8;
}
impl crate::IsEnum for Rxtriglvl {}
#[doc = "Field `RXTRIGLVL` writer - RX Trigger Level. These two bits determine how many receiver UART1 FIFO characters must be written before an interrupt is activated."]
pub type RxtriglvlW<'a, REG> = crate::FieldWriter<'a, REG, 2, Rxtriglvl, crate::Safe>;
impl<'a, REG> RxtriglvlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Trigger level 0 (1 character or 0x01)."]
    #[inline(always)]
    pub fn level_0(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtriglvl::Level0)
    }
    #[doc = "Trigger level 1 (4 characters or 0x04)."]
    #[inline(always)]
    pub fn level_1(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtriglvl::Level1)
    }
    #[doc = "Trigger level 2 (8 characters or 0x08)."]
    #[inline(always)]
    pub fn level_2(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtriglvl::Level2)
    }
    #[doc = "Trigger level 3 (14 characters or 0x0E)."]
    #[inline(always)]
    pub fn level_3(self) -> &'a mut crate::W<REG> {
        self.variant(Rxtriglvl::Level3)
    }
}
impl W {
    #[doc = "Bit 0 - FIFO enable."]
    #[inline(always)]
    pub fn fifoen(&mut self) -> FifoenW<'_, FcrSpec> {
        FifoenW::new(self, 0)
    }
    #[doc = "Bit 1 - RX FIFO Reset."]
    #[inline(always)]
    pub fn rxfifores(&mut self) -> RxfiforesW<'_, FcrSpec> {
        RxfiforesW::new(self, 1)
    }
    #[doc = "Bit 2 - TX FIFO Reset."]
    #[inline(always)]
    pub fn txfifores(&mut self) -> TxfiforesW<'_, FcrSpec> {
        TxfiforesW::new(self, 2)
    }
    #[doc = "Bit 3 - DMA Mode Select. When the FIFO enable bit (bit 0 of this register) is set, this bit selects the DMA mode. See Section 36.6.6.1."]
    #[inline(always)]
    pub fn dmamode(&mut self) -> DmamodeW<'_, FcrSpec> {
        DmamodeW::new(self, 3)
    }
    #[doc = "Bits 6:7 - RX Trigger Level. These two bits determine how many receiver UART1 FIFO characters must be written before an interrupt is activated."]
    #[inline(always)]
    pub fn rxtriglvl(&mut self) -> RxtriglvlW<'_, FcrSpec> {
        RxtriglvlW::new(self, 6)
    }
}
#[doc = "FIFO Control Register. Controls UART1 FIFO usage and modes.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcrSpec;
impl crate::RegisterSpec for FcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FcrSpec {}
