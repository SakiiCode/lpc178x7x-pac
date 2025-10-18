#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dao: Dao,
    dai: Dai,
    txfifo: Txfifo,
    rxfifo: Rxfifo,
    state: State,
    dma1: Dma1,
    dma2: Dma2,
    irq: Irq,
    txrate: Txrate,
    rxrate: Rxrate,
    txbitrate: Txbitrate,
    rxbitrate: Rxbitrate,
    txmode: Txmode,
    rxmode: Rxmode,
}
impl RegisterBlock {
    #[doc = "0x00 - I2S Digital Audio Output Register. Contains control bits for the I2S transmit channel."]
    #[inline(always)]
    pub const fn dao(&self) -> &Dao {
        &self.dao
    }
    #[doc = "0x04 - I2S Digital Audio Input Register. Contains control bits for the I2S receive channel."]
    #[inline(always)]
    pub const fn dai(&self) -> &Dai {
        &self.dai
    }
    #[doc = "0x08 - I2S Transmit FIFO. Access register for the 8 x 32-bit transmitter FIFO."]
    #[inline(always)]
    pub const fn txfifo(&self) -> &Txfifo {
        &self.txfifo
    }
    #[doc = "0x0c - I2S Receive FIFO. Access register for the 8 x 32-bit receiver FIFO."]
    #[inline(always)]
    pub const fn rxfifo(&self) -> &Rxfifo {
        &self.rxfifo
    }
    #[doc = "0x10 - I2S Status Feedback Register. Contains status information about the I2S interface."]
    #[inline(always)]
    pub const fn state(&self) -> &State {
        &self.state
    }
    #[doc = "0x14 - I2S DMA Configuration Register 1. Contains control information for DMA request 1."]
    #[inline(always)]
    pub const fn dma1(&self) -> &Dma1 {
        &self.dma1
    }
    #[doc = "0x18 - I2S DMA Configuration Register 2. Contains control information for DMA request 2."]
    #[inline(always)]
    pub const fn dma2(&self) -> &Dma2 {
        &self.dma2
    }
    #[doc = "0x1c - I2S Interrupt Request Control Register. Contains bits that control how the I2S interrupt request is generated."]
    #[inline(always)]
    pub const fn irq(&self) -> &Irq {
        &self.irq
    }
    #[doc = "0x20 - I2S Transmit MCLK divider. This register determines the I2S TX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK."]
    #[inline(always)]
    pub const fn txrate(&self) -> &Txrate {
        &self.txrate
    }
    #[doc = "0x24 - I2S Receive MCLK divider. This register determines the I2S RX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK."]
    #[inline(always)]
    pub const fn rxrate(&self) -> &Rxrate {
        &self.rxrate
    }
    #[doc = "0x28 - I2S Transmit bit rate divider. This register determines the I2S transmit bit rate by specifying the value to divide TX_MCLK by in order to produce the transmit bit clock."]
    #[inline(always)]
    pub const fn txbitrate(&self) -> &Txbitrate {
        &self.txbitrate
    }
    #[doc = "0x2c - I2S Receive bit rate divider. This register determines the I2S receive bit rate by specifying the value to divide RX_MCLK by in order to produce the receive bit clock."]
    #[inline(always)]
    pub const fn rxbitrate(&self) -> &Rxbitrate {
        &self.rxbitrate
    }
    #[doc = "0x30 - I2S Transmit mode control."]
    #[inline(always)]
    pub const fn txmode(&self) -> &Txmode {
        &self.txmode
    }
    #[doc = "0x34 - I2S Receive mode control."]
    #[inline(always)]
    pub const fn rxmode(&self) -> &Rxmode {
        &self.rxmode
    }
}
#[doc = "DAO (rw) register accessor: I2S Digital Audio Output Register. Contains control bits for the I2S transmit channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`dao::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dao::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dao`] module"]
#[doc(alias = "DAO")]
pub type Dao = crate::Reg<dao::DaoSpec>;
#[doc = "I2S Digital Audio Output Register. Contains control bits for the I2S transmit channel."]
pub mod dao;
#[doc = "DAI (rw) register accessor: I2S Digital Audio Input Register. Contains control bits for the I2S receive channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`dai::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dai::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dai`] module"]
#[doc(alias = "DAI")]
pub type Dai = crate::Reg<dai::DaiSpec>;
#[doc = "I2S Digital Audio Input Register. Contains control bits for the I2S receive channel."]
pub mod dai;
#[doc = "TXFIFO (w) register accessor: I2S Transmit FIFO. Access register for the 8 x 32-bit transmitter FIFO.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txfifo::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txfifo`] module"]
#[doc(alias = "TXFIFO")]
pub type Txfifo = crate::Reg<txfifo::TxfifoSpec>;
#[doc = "I2S Transmit FIFO. Access register for the 8 x 32-bit transmitter FIFO."]
pub mod txfifo;
#[doc = "RXFIFO (r) register accessor: I2S Receive FIFO. Access register for the 8 x 32-bit receiver FIFO.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfifo::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfifo`] module"]
#[doc(alias = "RXFIFO")]
pub type Rxfifo = crate::Reg<rxfifo::RxfifoSpec>;
#[doc = "I2S Receive FIFO. Access register for the 8 x 32-bit receiver FIFO."]
pub mod rxfifo;
#[doc = "STATE (r) register accessor: I2S Status Feedback Register. Contains status information about the I2S interface.\n\nYou can [`read`](crate::Reg::read) this register and get [`state::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@state`] module"]
#[doc(alias = "STATE")]
pub type State = crate::Reg<state::StateSpec>;
#[doc = "I2S Status Feedback Register. Contains status information about the I2S interface."]
pub mod state;
#[doc = "DMA1 (rw) register accessor: I2S DMA Configuration Register 1. Contains control information for DMA request 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma1`] module"]
#[doc(alias = "DMA1")]
pub type Dma1 = crate::Reg<dma1::Dma1Spec>;
#[doc = "I2S DMA Configuration Register 1. Contains control information for DMA request 1."]
pub mod dma1;
#[doc = "DMA2 (rw) register accessor: I2S DMA Configuration Register 2. Contains control information for DMA request 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`dma2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dma2`] module"]
#[doc(alias = "DMA2")]
pub type Dma2 = crate::Reg<dma2::Dma2Spec>;
#[doc = "I2S DMA Configuration Register 2. Contains control information for DMA request 2."]
pub mod dma2;
#[doc = "IRQ (rw) register accessor: I2S Interrupt Request Control Register. Contains bits that control how the I2S interrupt request is generated.\n\nYou can [`read`](crate::Reg::read) this register and get [`irq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@irq`] module"]
#[doc(alias = "IRQ")]
pub type Irq = crate::Reg<irq::IrqSpec>;
#[doc = "I2S Interrupt Request Control Register. Contains bits that control how the I2S interrupt request is generated."]
pub mod irq;
#[doc = "TXRATE (rw) register accessor: I2S Transmit MCLK divider. This register determines the I2S TX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK.\n\nYou can [`read`](crate::Reg::read) this register and get [`txrate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txrate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txrate`] module"]
#[doc(alias = "TXRATE")]
pub type Txrate = crate::Reg<txrate::TxrateSpec>;
#[doc = "I2S Transmit MCLK divider. This register determines the I2S TX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK."]
pub mod txrate;
#[doc = "RXRATE (rw) register accessor: I2S Receive MCLK divider. This register determines the I2S RX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxrate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxrate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxrate`] module"]
#[doc(alias = "RXRATE")]
pub type Rxrate = crate::Reg<rxrate::RxrateSpec>;
#[doc = "I2S Receive MCLK divider. This register determines the I2S RX MCLK rate by specifying the value to divide PCLK by in order to produce MCLK."]
pub mod rxrate;
#[doc = "TXBITRATE (rw) register accessor: I2S Transmit bit rate divider. This register determines the I2S transmit bit rate by specifying the value to divide TX_MCLK by in order to produce the transmit bit clock.\n\nYou can [`read`](crate::Reg::read) this register and get [`txbitrate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txbitrate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txbitrate`] module"]
#[doc(alias = "TXBITRATE")]
pub type Txbitrate = crate::Reg<txbitrate::TxbitrateSpec>;
#[doc = "I2S Transmit bit rate divider. This register determines the I2S transmit bit rate by specifying the value to divide TX_MCLK by in order to produce the transmit bit clock."]
pub mod txbitrate;
#[doc = "RXBITRATE (rw) register accessor: I2S Receive bit rate divider. This register determines the I2S receive bit rate by specifying the value to divide RX_MCLK by in order to produce the receive bit clock.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxbitrate::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxbitrate::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxbitrate`] module"]
#[doc(alias = "RXBITRATE")]
pub type Rxbitrate = crate::Reg<rxbitrate::RxbitrateSpec>;
#[doc = "I2S Receive bit rate divider. This register determines the I2S receive bit rate by specifying the value to divide RX_MCLK by in order to produce the receive bit clock."]
pub mod rxbitrate;
#[doc = "TXMODE (rw) register accessor: I2S Transmit mode control.\n\nYou can [`read`](crate::Reg::read) this register and get [`txmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txmode`] module"]
#[doc(alias = "TXMODE")]
pub type Txmode = crate::Reg<txmode::TxmodeSpec>;
#[doc = "I2S Transmit mode control."]
pub mod txmode;
#[doc = "RXMODE (rw) register accessor: I2S Receive mode control.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxmode`] module"]
#[doc(alias = "RXMODE")]
pub type Rxmode = crate::Reg<rxmode::RxmodeSpec>;
#[doc = "I2S Receive mode control."]
pub mod rxmode;
