#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    txsr: Txsr,
    rxsr: Rxsr,
    msr: Msr,
}
impl RegisterBlock {
    #[doc = "0x00 - CAN Central Transmit Status Register"]
    #[inline(always)]
    pub const fn txsr(&self) -> &Txsr {
        &self.txsr
    }
    #[doc = "0x04 - CAN Central Receive Status Register"]
    #[inline(always)]
    pub const fn rxsr(&self) -> &Rxsr {
        &self.rxsr
    }
    #[doc = "0x08 - CAN Central Miscellaneous Register"]
    #[inline(always)]
    pub const fn msr(&self) -> &Msr {
        &self.msr
    }
}
#[doc = "TXSR (r) register accessor: CAN Central Transmit Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txsr`] module"]
#[doc(alias = "TXSR")]
pub type Txsr = crate::Reg<txsr::TxsrSpec>;
#[doc = "CAN Central Transmit Status Register"]
pub mod txsr;
#[doc = "RXSR (r) register accessor: CAN Central Receive Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxsr`] module"]
#[doc(alias = "RXSR")]
pub type Rxsr = crate::Reg<rxsr::RxsrSpec>;
#[doc = "CAN Central Receive Status Register"]
pub mod rxsr;
#[doc = "MSR (r) register accessor: CAN Central Miscellaneous Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr`] module"]
#[doc(alias = "MSR")]
pub type Msr = crate::Reg<msr::MsrSpec>;
#[doc = "CAN Central Miscellaneous Register"]
pub mod msr;
