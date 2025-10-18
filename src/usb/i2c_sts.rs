#[doc = "Register `I2C_STS` reader"]
pub type R = crate::R<I2cStsSpec>;
#[doc = "Transaction Done Interrupt. This flag is set if a transaction completes successfully. It is cleared by writing a one to bit 0 of the status register. It is unaffected by slave transactions.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Transaction has not completed."]
    NotComplete = 0,
    #[doc = "1: Transaction completed."]
    Complete = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TDI` reader - Transaction Done Interrupt. This flag is set if a transaction completes successfully. It is cleared by writing a one to bit 0 of the status register. It is unaffected by slave transactions."]
pub type TdiR = crate::BitReader<Enum>;
impl TdiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::NotComplete,
            true => Enum::Complete,
        }
    }
    #[doc = "Transaction has not completed."]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == Enum::NotComplete
    }
    #[doc = "Transaction completed."]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == Enum::Complete
    }
}
#[doc = "Arbitration Failure Interrupt. When transmitting, if the SDA is low when SDAOUT is high, then this I2C has lost the arbitration to another device on the bus. The Arbitration Failure bit is set when this happens. It is cleared by writing a one to bit 1 of the status register.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: No arbitration failure on last transmission."]
    NoArbitrationFailu = 0,
    #[doc = "1: Arbitration failure occurred on last transmission."]
    ArbitrationFailure_ = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `AFI` reader - Arbitration Failure Interrupt. When transmitting, if the SDA is low when SDAOUT is high, then this I2C has lost the arbitration to another device on the bus. The Arbitration Failure bit is set when this happens. It is cleared by writing a one to bit 1 of the status register."]
pub type AfiR = crate::BitReader<Enum>;
impl AfiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::NoArbitrationFailu,
            true => Enum::ArbitrationFailure_,
        }
    }
    #[doc = "No arbitration failure on last transmission."]
    #[inline(always)]
    pub fn is_no_arbitration_failu(&self) -> bool {
        *self == Enum::NoArbitrationFailu
    }
    #[doc = "Arbitration failure occurred on last transmission."]
    #[inline(always)]
    pub fn is_arbitration_failure_(&self) -> bool {
        *self == Enum::ArbitrationFailure_
    }
}
#[doc = "No Acknowledge Interrupt. After every byte of data is sent, the transmitter expects an acknowledge from the receiver. This bit is set if the acknowledge is not received. It is cleared when a byte is written to the master TX FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Last transmission received an acknowledge."]
    AcknowledgeRcvd = 0,
    #[doc = "1: Last transmission did not receive an acknowledge."]
    NoAcknowledgeRcvd = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `NAI` reader - No Acknowledge Interrupt. After every byte of data is sent, the transmitter expects an acknowledge from the receiver. This bit is set if the acknowledge is not received. It is cleared when a byte is written to the master TX FIFO."]
pub type NaiR = crate::BitReader<Enum>;
impl NaiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::AcknowledgeRcvd,
            true => Enum::NoAcknowledgeRcvd,
        }
    }
    #[doc = "Last transmission received an acknowledge."]
    #[inline(always)]
    pub fn is_acknowledge_rcvd(&self) -> bool {
        *self == Enum::AcknowledgeRcvd
    }
    #[doc = "Last transmission did not receive an acknowledge."]
    #[inline(always)]
    pub fn is_no_acknowledge_rcvd(&self) -> bool {
        *self == Enum::NoAcknowledgeRcvd
    }
}
#[doc = "Master Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a stop condition or it will hold SCL low until more data is available. The Master Data Request bit is set when the master transmitter is data-starved. If the master TX FIFO is empty and the last byte did not have a STOP condition flag, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the master TX FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Master transmitter does not need data."]
    Busy = 0,
    #[doc = "1: Master transmitter needs data."]
    NeedData = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRMI` reader - Master Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a stop condition or it will hold SCL low until more data is available. The Master Data Request bit is set when the master transmitter is data-starved. If the master TX FIFO is empty and the last byte did not have a STOP condition flag, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the master TX FIFO."]
pub type DrmiR = crate::BitReader<Enum>;
impl DrmiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Busy,
            true => Enum::NeedData,
        }
    }
    #[doc = "Master transmitter does not need data."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Enum::Busy
    }
    #[doc = "Master transmitter needs data."]
    #[inline(always)]
    pub fn is_need_data(&self) -> bool {
        *self == Enum::NeedData
    }
}
#[doc = "Slave Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a STOP condition or it will hold SCL low until more data is available. The Slave Data Request bit is set when the slave transmitter is data-starved. If the slave TX FIFO is empty and the last byte transmitted was acknowledged, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the slave Tx FIFO.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: Slave transmitter does not need data."]
    Busy = 0,
    #[doc = "1: Slave transmitter needs data."]
    NeedData = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DRSI` reader - Slave Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a STOP condition or it will hold SCL low until more data is available. The Slave Data Request bit is set when the slave transmitter is data-starved. If the slave TX FIFO is empty and the last byte transmitted was acknowledged, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the slave Tx FIFO."]
pub type DrsiR = crate::BitReader<Enum>;
impl DrsiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Busy,
            true => Enum::NeedData,
        }
    }
    #[doc = "Slave transmitter does not need data."]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == Enum::Busy
    }
    #[doc = "Slave transmitter needs data."]
    #[inline(always)]
    pub fn is_need_data(&self) -> bool {
        *self == Enum::NeedData
    }
}
#[doc = "Field `Active` reader - Indicates whether the bus is busy. This bit is set when a START condition has been seen. It is cleared when a STOP condition is seen.."]
pub type ActiveR = crate::BitReader;
#[doc = "Field `SCL` reader - The current value of the SCL signal."]
pub type SclR = crate::BitReader;
#[doc = "Field `SDA` reader - The current value of the SDA signal."]
pub type SdaR = crate::BitReader;
#[doc = "Receive FIFO Full (RFF). This bit is set when the RX FIFO is full and cannot accept any more data. It is cleared when the RX FIFO is not full. If a byte arrives when the Receive FIFO is full, the SCL is held low until the CPU reads the RX FIFO and makes room for it.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: RX FIFO is not full"]
    RxFifoIsNotFull = 0,
    #[doc = "1: RX FIFO is full"]
    RxFifoIsFull = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFF` reader - Receive FIFO Full (RFF). This bit is set when the RX FIFO is full and cannot accept any more data. It is cleared when the RX FIFO is not full. If a byte arrives when the Receive FIFO is full, the SCL is held low until the CPU reads the RX FIFO and makes room for it."]
pub type RffR = crate::BitReader<Enum>;
impl RffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::RxFifoIsNotFull,
            true => Enum::RxFifoIsFull,
        }
    }
    #[doc = "RX FIFO is not full"]
    #[inline(always)]
    pub fn is_rx_fifo_is_not_full(&self) -> bool {
        *self == Enum::RxFifoIsNotFull
    }
    #[doc = "RX FIFO is full"]
    #[inline(always)]
    pub fn is_rx_fifo_is_full(&self) -> bool {
        *self == Enum::RxFifoIsFull
    }
}
#[doc = "Receive FIFO Empty. RFE is set when the RX FIFO is empty and is cleared when the RX FIFO contains valid data.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: RX FIFO contains data."]
    Data = 0,
    #[doc = "1: RX FIFO is empty"]
    Empty = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RFE` reader - Receive FIFO Empty. RFE is set when the RX FIFO is empty and is cleared when the RX FIFO contains valid data."]
pub type RfeR = crate::BitReader<Enum>;
impl RfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::Data,
            true => Enum::Empty,
        }
    }
    #[doc = "RX FIFO contains data."]
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        *self == Enum::Data
    }
    #[doc = "RX FIFO is empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Enum::Empty
    }
}
#[doc = "Transmit FIFO Full. TFF is set when the TX FIFO is full and is cleared when the TX FIFO is not full.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: TX FIFO is not full."]
    TxFifoIsNotFull_ = 0,
    #[doc = "1: TX FIFO is full"]
    TxFifoIsFull = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFF` reader - Transmit FIFO Full. TFF is set when the TX FIFO is full and is cleared when the TX FIFO is not full."]
pub type TffR = crate::BitReader<Enum>;
impl TffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::TxFifoIsNotFull_,
            true => Enum::TxFifoIsFull,
        }
    }
    #[doc = "TX FIFO is not full."]
    #[inline(always)]
    pub fn is_tx_fifo_is_not_full_(&self) -> bool {
        *self == Enum::TxFifoIsNotFull_
    }
    #[doc = "TX FIFO is full"]
    #[inline(always)]
    pub fn is_tx_fifo_is_full(&self) -> bool {
        *self == Enum::TxFifoIsFull
    }
}
#[doc = "Transmit FIFO Empty. TFE is set when the TX FIFO is empty and is cleared when the TX FIFO contains valid data.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Enum {
    #[doc = "0: TX FIFO contains valid data."]
    ValidData = 0,
    #[doc = "1: TX FIFO is empty"]
    Empty = 1,
}
impl From<Enum> for bool {
    #[inline(always)]
    fn from(variant: Enum) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TFE` reader - Transmit FIFO Empty. TFE is set when the TX FIFO is empty and is cleared when the TX FIFO contains valid data."]
pub type TfeR = crate::BitReader<Enum>;
impl TfeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Enum {
        match self.bits {
            false => Enum::ValidData,
            true => Enum::Empty,
        }
    }
    #[doc = "TX FIFO contains valid data."]
    #[inline(always)]
    pub fn is_valid_data(&self) -> bool {
        *self == Enum::ValidData
    }
    #[doc = "TX FIFO is empty"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == Enum::Empty
    }
}
impl R {
    #[doc = "Bit 0 - Transaction Done Interrupt. This flag is set if a transaction completes successfully. It is cleared by writing a one to bit 0 of the status register. It is unaffected by slave transactions."]
    #[inline(always)]
    pub fn tdi(&self) -> TdiR {
        TdiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Arbitration Failure Interrupt. When transmitting, if the SDA is low when SDAOUT is high, then this I2C has lost the arbitration to another device on the bus. The Arbitration Failure bit is set when this happens. It is cleared by writing a one to bit 1 of the status register."]
    #[inline(always)]
    pub fn afi(&self) -> AfiR {
        AfiR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - No Acknowledge Interrupt. After every byte of data is sent, the transmitter expects an acknowledge from the receiver. This bit is set if the acknowledge is not received. It is cleared when a byte is written to the master TX FIFO."]
    #[inline(always)]
    pub fn nai(&self) -> NaiR {
        NaiR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Master Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a stop condition or it will hold SCL low until more data is available. The Master Data Request bit is set when the master transmitter is data-starved. If the master TX FIFO is empty and the last byte did not have a STOP condition flag, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the master TX FIFO."]
    #[inline(always)]
    pub fn drmi(&self) -> DrmiR {
        DrmiR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Slave Data Request Interrupt. Once a transmission is started, the transmitter must have data to transmit as long as it isn't followed by a STOP condition or it will hold SCL low until more data is available. The Slave Data Request bit is set when the slave transmitter is data-starved. If the slave TX FIFO is empty and the last byte transmitted was acknowledged, then SCL is held low until the CPU writes another byte to transmit. This bit is cleared when a byte is written to the slave Tx FIFO."]
    #[inline(always)]
    pub fn drsi(&self) -> DrsiR {
        DrsiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indicates whether the bus is busy. This bit is set when a START condition has been seen. It is cleared when a STOP condition is seen.."]
    #[inline(always)]
    pub fn active(&self) -> ActiveR {
        ActiveR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The current value of the SCL signal."]
    #[inline(always)]
    pub fn scl(&self) -> SclR {
        SclR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The current value of the SDA signal."]
    #[inline(always)]
    pub fn sda(&self) -> SdaR {
        SdaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive FIFO Full (RFF). This bit is set when the RX FIFO is full and cannot accept any more data. It is cleared when the RX FIFO is not full. If a byte arrives when the Receive FIFO is full, the SCL is held low until the CPU reads the RX FIFO and makes room for it."]
    #[inline(always)]
    pub fn rff(&self) -> RffR {
        RffR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive FIFO Empty. RFE is set when the RX FIFO is empty and is cleared when the RX FIFO contains valid data."]
    #[inline(always)]
    pub fn rfe(&self) -> RfeR {
        RfeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmit FIFO Full. TFF is set when the TX FIFO is full and is cleared when the TX FIFO is not full."]
    #[inline(always)]
    pub fn tff(&self) -> TffR {
        TffR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit FIFO Empty. TFE is set when the TX FIFO is empty and is cleared when the TX FIFO contains valid data."]
    #[inline(always)]
    pub fn tfe(&self) -> TfeR {
        TfeR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "I2C Status\n\nYou can [`read`](crate::Reg::read) this register and get [`i2c_sts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2cStsSpec;
impl crate::RegisterSpec for I2cStsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2c_sts::R`](R) reader structure"]
impl crate::Readable for I2cStsSpec {}
#[doc = "`reset()` method sets I2C_STS to value 0x0a00"]
impl crate::Resettable for I2cStsSpec {
    const RESET_VALUE: u32 = 0x0a00;
}
