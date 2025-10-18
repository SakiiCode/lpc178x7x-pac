#[doc = "Register `CMR` writer"]
pub type W = crate::W<CmrSpec>;
#[doc = "Transmission Request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tr {
    #[doc = "0: Absent.No transmission request."]
    AbsentNoTransmissi = 0,
    #[doc = "1: Present. The message, previously written to the CANxTFI, CANxTID, and optionally the CANxTDA and CANxTDB registers, is queued for transmission from the selected Transmit Buffer. If at two or all three of STB1, STB2 and STB3 bits are selected when TR=1 is written, Transmit Buffer will be selected based on the chosen priority scheme (for details see Section 21.5.3 Transmit Buffers (TXB))"]
    PresentTheMessage = 1,
}
impl From<Tr> for bool {
    #[inline(always)]
    fn from(variant: Tr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TR` writer - Transmission Request."]
pub type TrW<'a, REG> = crate::BitWriter<'a, REG, Tr>;
impl<'a, REG> TrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Absent.No transmission request."]
    #[inline(always)]
    pub fn absent_no_transmissi(self) -> &'a mut crate::W<REG> {
        self.variant(Tr::AbsentNoTransmissi)
    }
    #[doc = "Present. The message, previously written to the CANxTFI, CANxTID, and optionally the CANxTDA and CANxTDB registers, is queued for transmission from the selected Transmit Buffer. If at two or all three of STB1, STB2 and STB3 bits are selected when TR=1 is written, Transmit Buffer will be selected based on the chosen priority scheme (for details see Section 21.5.3 Transmit Buffers (TXB))"]
    #[inline(always)]
    pub fn present_the_message(self) -> &'a mut crate::W<REG> {
        self.variant(Tr::PresentTheMessage)
    }
}
#[doc = "Abort Transmission.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum At {
    #[doc = "0: No action. Do not abort the transmission."]
    NoActionDoNotAb = 0,
    #[doc = "1: Present. if not already in progress, a pending Transmission Request for the selected Transmit Buffer is cancelled."]
    PresentIfNotAlre = 1,
}
impl From<At> for bool {
    #[inline(always)]
    fn from(variant: At) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AT` writer - Abort Transmission."]
pub type AtW<'a, REG> = crate::BitWriter<'a, REG, At>;
impl<'a, REG> AtW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action. Do not abort the transmission."]
    #[inline(always)]
    pub fn no_action_do_not_ab(self) -> &'a mut crate::W<REG> {
        self.variant(At::NoActionDoNotAb)
    }
    #[doc = "Present. if not already in progress, a pending Transmission Request for the selected Transmit Buffer is cancelled."]
    #[inline(always)]
    pub fn present_if_not_alre(self) -> &'a mut crate::W<REG> {
        self.variant(At::PresentIfNotAlre)
    }
}
#[doc = "Release Receive Buffer.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rrb {
    #[doc = "0: No action. Do not release the receive buffer."]
    NoActionDoNotRe = 0,
    #[doc = "1: Released. The information in the Receive Buffer (consisting of CANxRFS, CANxRID, and if applicable the CANxRDA and CANxRDB registers) is released, and becomes eligible for replacement by the next received frame. If the next received frame is not available, writing this command clears the RBS bit in the Status Register(s)."]
    ReleasedTheInform = 1,
}
impl From<Rrb> for bool {
    #[inline(always)]
    fn from(variant: Rrb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RRB` writer - Release Receive Buffer."]
pub type RrbW<'a, REG> = crate::BitWriter<'a, REG, Rrb>;
impl<'a, REG> RrbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action. Do not release the receive buffer."]
    #[inline(always)]
    pub fn no_action_do_not_re(self) -> &'a mut crate::W<REG> {
        self.variant(Rrb::NoActionDoNotRe)
    }
    #[doc = "Released. The information in the Receive Buffer (consisting of CANxRFS, CANxRID, and if applicable the CANxRDA and CANxRDB registers) is released, and becomes eligible for replacement by the next received frame. If the next received frame is not available, writing this command clears the RBS bit in the Status Register(s)."]
    #[inline(always)]
    pub fn released_the_inform(self) -> &'a mut crate::W<REG> {
        self.variant(Rrb::ReleasedTheInform)
    }
}
#[doc = "Clear Data Overrun.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cdo {
    #[doc = "0: No action. Do not clear the data overrun bit."]
    NoActionDoNotCl = 0,
    #[doc = "1: Clear. The Data Overrun bit in Status Register(s) is cleared."]
    ClearTheDataOver = 1,
}
impl From<Cdo> for bool {
    #[inline(always)]
    fn from(variant: Cdo) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CDO` writer - Clear Data Overrun."]
pub type CdoW<'a, REG> = crate::BitWriter<'a, REG, Cdo>;
impl<'a, REG> CdoW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No action. Do not clear the data overrun bit."]
    #[inline(always)]
    pub fn no_action_do_not_cl(self) -> &'a mut crate::W<REG> {
        self.variant(Cdo::NoActionDoNotCl)
    }
    #[doc = "Clear. The Data Overrun bit in Status Register(s) is cleared."]
    #[inline(always)]
    pub fn clear_the_data_over(self) -> &'a mut crate::W<REG> {
        self.variant(Cdo::ClearTheDataOver)
    }
}
#[doc = "Self Reception Request.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Srr {
    #[doc = "0: Absent. No self reception request."]
    AbsentNoSelfRece = 0,
    #[doc = "1: Present. The message, previously written to the CANxTFS, CANxTID, and optionally the CANxTDA and CANxTDB registers, is queued for transmission from the selected Transmit Buffer and received simultaneously. This differs from the TR bit above in that the receiver is not disabled during the transmission, so that it receives the message if its Identifier is recognized by the Acceptance Filter."]
    PresentTheMessage = 1,
}
impl From<Srr> for bool {
    #[inline(always)]
    fn from(variant: Srr) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SRR` writer - Self Reception Request."]
pub type SrrW<'a, REG> = crate::BitWriter<'a, REG, Srr>;
impl<'a, REG> SrrW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Absent. No self reception request."]
    #[inline(always)]
    pub fn absent_no_self_rece(self) -> &'a mut crate::W<REG> {
        self.variant(Srr::AbsentNoSelfRece)
    }
    #[doc = "Present. The message, previously written to the CANxTFS, CANxTID, and optionally the CANxTDA and CANxTDB registers, is queued for transmission from the selected Transmit Buffer and received simultaneously. This differs from the TR bit above in that the receiver is not disabled during the transmission, so that it receives the message if its Identifier is recognized by the Acceptance Filter."]
    #[inline(always)]
    pub fn present_the_message(self) -> &'a mut crate::W<REG> {
        self.variant(Srr::PresentTheMessage)
    }
}
#[doc = "Select Tx Buffer 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stb1 {
    #[doc = "0: Not selected. Tx Buffer 1 is not selected for transmission."]
    NotSelectedTxBuf = 0,
    #[doc = "1: Selected. Tx Buffer 1 is selected for transmission."]
    SelectedTxBuffer_ = 1,
}
impl From<Stb1> for bool {
    #[inline(always)]
    fn from(variant: Stb1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STB1` writer - Select Tx Buffer 1."]
pub type Stb1W<'a, REG> = crate::BitWriter<'a, REG, Stb1>;
impl<'a, REG> Stb1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected. Tx Buffer 1 is not selected for transmission."]
    #[inline(always)]
    pub fn not_selected_tx_buf(self) -> &'a mut crate::W<REG> {
        self.variant(Stb1::NotSelectedTxBuf)
    }
    #[doc = "Selected. Tx Buffer 1 is selected for transmission."]
    #[inline(always)]
    pub fn selected_tx_buffer_(self) -> &'a mut crate::W<REG> {
        self.variant(Stb1::SelectedTxBuffer_)
    }
}
#[doc = "Select Tx Buffer 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stb2 {
    #[doc = "0: Not selected. Tx Buffer 2 is not selected for transmission."]
    NotSelectedTxBuf = 0,
    #[doc = "1: Selected. Tx Buffer 2 is selected for transmission."]
    SelectedTxBuffer_ = 1,
}
impl From<Stb2> for bool {
    #[inline(always)]
    fn from(variant: Stb2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STB2` writer - Select Tx Buffer 2."]
pub type Stb2W<'a, REG> = crate::BitWriter<'a, REG, Stb2>;
impl<'a, REG> Stb2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected. Tx Buffer 2 is not selected for transmission."]
    #[inline(always)]
    pub fn not_selected_tx_buf(self) -> &'a mut crate::W<REG> {
        self.variant(Stb2::NotSelectedTxBuf)
    }
    #[doc = "Selected. Tx Buffer 2 is selected for transmission."]
    #[inline(always)]
    pub fn selected_tx_buffer_(self) -> &'a mut crate::W<REG> {
        self.variant(Stb2::SelectedTxBuffer_)
    }
}
#[doc = "Select Tx Buffer 3.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stb3 {
    #[doc = "0: Not selected. Tx Buffer 3 is not selected for transmission."]
    NotSelectedTxBuf = 0,
    #[doc = "1: Selected. Tx Buffer 3 is selected for transmission."]
    SelectedTxBuffer_ = 1,
}
impl From<Stb3> for bool {
    #[inline(always)]
    fn from(variant: Stb3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STB3` writer - Select Tx Buffer 3."]
pub type Stb3W<'a, REG> = crate::BitWriter<'a, REG, Stb3>;
impl<'a, REG> Stb3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not selected. Tx Buffer 3 is not selected for transmission."]
    #[inline(always)]
    pub fn not_selected_tx_buf(self) -> &'a mut crate::W<REG> {
        self.variant(Stb3::NotSelectedTxBuf)
    }
    #[doc = "Selected. Tx Buffer 3 is selected for transmission."]
    #[inline(always)]
    pub fn selected_tx_buffer_(self) -> &'a mut crate::W<REG> {
        self.variant(Stb3::SelectedTxBuffer_)
    }
}
impl W {
    #[doc = "Bit 0 - Transmission Request."]
    #[inline(always)]
    pub fn tr(&mut self) -> TrW<'_, CmrSpec> {
        TrW::new(self, 0)
    }
    #[doc = "Bit 1 - Abort Transmission."]
    #[inline(always)]
    pub fn at(&mut self) -> AtW<'_, CmrSpec> {
        AtW::new(self, 1)
    }
    #[doc = "Bit 2 - Release Receive Buffer."]
    #[inline(always)]
    pub fn rrb(&mut self) -> RrbW<'_, CmrSpec> {
        RrbW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear Data Overrun."]
    #[inline(always)]
    pub fn cdo(&mut self) -> CdoW<'_, CmrSpec> {
        CdoW::new(self, 3)
    }
    #[doc = "Bit 4 - Self Reception Request."]
    #[inline(always)]
    pub fn srr(&mut self) -> SrrW<'_, CmrSpec> {
        SrrW::new(self, 4)
    }
    #[doc = "Bit 5 - Select Tx Buffer 1."]
    #[inline(always)]
    pub fn stb1(&mut self) -> Stb1W<'_, CmrSpec> {
        Stb1W::new(self, 5)
    }
    #[doc = "Bit 6 - Select Tx Buffer 2."]
    #[inline(always)]
    pub fn stb2(&mut self) -> Stb2W<'_, CmrSpec> {
        Stb2W::new(self, 6)
    }
    #[doc = "Bit 7 - Select Tx Buffer 3."]
    #[inline(always)]
    pub fn stb3(&mut self) -> Stb3W<'_, CmrSpec> {
        Stb3W::new(self, 7)
    }
}
#[doc = "Command bits that affect the state of the CAN Controller\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmrSpec;
impl crate::RegisterSpec for CmrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`cmr::W`](W) writer structure"]
impl crate::Writable for CmrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMR to value 0"]
impl crate::Resettable for CmrSpec {}
