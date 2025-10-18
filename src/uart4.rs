#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved_0_dll: [u8; 0x04],
    _reserved_1_dlm: [u8; 0x04],
    _reserved_2_fcr: [u8; 0x04],
    lcr: Lcr,
    _reserved4: [u8; 0x04],
    lsr: Lsr,
    _reserved5: [u8; 0x04],
    scr: Scr,
    acr: Acr,
    icr: Icr,
    fdr: Fdr,
    osr: Osr,
    _reserved10: [u8; 0x18],
    scictrl: Scictrl,
    rs485ctrl: Rs485ctrl,
    rs485adrmatch: Rs485adrmatch,
    rs485dly: Rs485dly,
    syncctrl: Syncctrl,
}
impl RegisterBlock {
    #[doc = "0x00 - Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider (DLAB =1)."]
    #[inline(always)]
    pub const fn dll(&self) -> &Dll {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - Transmit Holding Register. The next character to be transmitted is written here (DLAB =0)."]
    #[inline(always)]
    pub const fn thr(&self) -> &Thr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x00 - Receiver Buffer Register. Contains the next received character to be read (DLAB =0)."]
    #[inline(always)]
    pub const fn rbr(&self) -> &Rbr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    #[doc = "0x04 - Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential UART interrupts (DLAB =0)."]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x04 - Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider (DLAB =1)."]
    #[inline(always)]
    pub const fn dlm(&self) -> &Dlm {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    #[doc = "0x08 - FIFO Control Register. Controls UART FIFO usage and modes."]
    #[inline(always)]
    pub const fn fcr(&self) -> &Fcr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x08 - Interrupt ID Register. Identifies which interrupt(s) are pending."]
    #[inline(always)]
    pub const fn iir(&self) -> &Iir {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    #[doc = "0x0c - Line Control Register. Contains controls for frame formatting and break generation."]
    #[inline(always)]
    pub const fn lcr(&self) -> &Lcr {
        &self.lcr
    }
    #[doc = "0x14 - Line Status Register. Contains flags for transmit and receive status, including line errors."]
    #[inline(always)]
    pub const fn lsr(&self) -> &Lsr {
        &self.lsr
    }
    #[doc = "0x1c - Scratch Pad Register. 8-bit temporary storage for software."]
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        &self.scr
    }
    #[doc = "0x20 - Auto-baud Control Register. Contains controls for the auto-baud feature."]
    #[inline(always)]
    pub const fn acr(&self) -> &Acr {
        &self.acr
    }
    #[doc = "0x24 - IrDA Control Register. Enables and configures the IrDA mode."]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x28 - Fractional Divider Register. Generates a clock input for the baud rate divider."]
    #[inline(always)]
    pub const fn fdr(&self) -> &Fdr {
        &self.fdr
    }
    #[doc = "0x2c - Oversampling register. Controls the degree of oversampling during each bit time."]
    #[inline(always)]
    pub const fn osr(&self) -> &Osr {
        &self.osr
    }
    #[doc = "0x48 - Smart Card Interface control register. Enables and configures the smartcard Interface feature."]
    #[inline(always)]
    pub const fn scictrl(&self) -> &Scictrl {
        &self.scictrl
    }
    #[doc = "0x4c - RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes."]
    #[inline(always)]
    pub const fn rs485ctrl(&self) -> &Rs485ctrl {
        &self.rs485ctrl
    }
    #[doc = "0x50 - RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode."]
    #[inline(always)]
    pub const fn rs485adrmatch(&self) -> &Rs485adrmatch {
        &self.rs485adrmatch
    }
    #[doc = "0x54 - RS-485/EIA-485 direction control delay."]
    #[inline(always)]
    pub const fn rs485dly(&self) -> &Rs485dly {
        &self.rs485dly
    }
    #[doc = "0x58 - Synchronous mode control register."]
    #[inline(always)]
    pub const fn syncctrl(&self) -> &Syncctrl {
        &self.syncctrl
    }
}
#[doc = "RBR (r) register accessor: Receiver Buffer Register. Contains the next received character to be read (DLAB =0).\n\nYou can [`read`](crate::Reg::read) this register and get [`rbr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rbr`] module"]
#[doc(alias = "RBR")]
pub type Rbr = crate::Reg<rbr::RbrSpec>;
#[doc = "Receiver Buffer Register. Contains the next received character to be read (DLAB =0)."]
pub mod rbr;
#[doc = "THR (w) register accessor: Transmit Holding Register. The next character to be transmitted is written here (DLAB =0).\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thr`] module"]
#[doc(alias = "THR")]
pub type Thr = crate::Reg<thr::ThrSpec>;
#[doc = "Transmit Holding Register. The next character to be transmitted is written here (DLAB =0)."]
pub mod thr;
#[doc = "DLL (rw) register accessor: Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider (DLAB =1).\n\nYou can [`read`](crate::Reg::read) this register and get [`dll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dll`] module"]
#[doc(alias = "DLL")]
pub type Dll = crate::Reg<dll::DllSpec>;
#[doc = "Divisor Latch LSB. Least significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider (DLAB =1)."]
pub mod dll;
#[doc = "DLM (rw) register accessor: Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider (DLAB =1).\n\nYou can [`read`](crate::Reg::read) this register and get [`dlm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlm`] module"]
#[doc(alias = "DLM")]
pub type Dlm = crate::Reg<dlm::DlmSpec>;
#[doc = "Divisor Latch MSB. Most significant byte of the baud rate divisor value. The full divisor is used to generate a baud rate from the fractional rate divider (DLAB =1)."]
pub mod dlm;
#[doc = "IER (rw) register accessor: Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential UART interrupts (DLAB =0).\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt Enable Register. Contains individual interrupt enable bits for the 7 potential UART interrupts (DLAB =0)."]
pub mod ier;
#[doc = "IIR (r) register accessor: Interrupt ID Register. Identifies which interrupt(s) are pending.\n\nYou can [`read`](crate::Reg::read) this register and get [`iir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@iir`] module"]
#[doc(alias = "IIR")]
pub type Iir = crate::Reg<iir::IirSpec>;
#[doc = "Interrupt ID Register. Identifies which interrupt(s) are pending."]
pub mod iir;
#[doc = "FCR (w) register accessor: FIFO Control Register. Controls UART FIFO usage and modes.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`] module"]
#[doc(alias = "FCR")]
pub type Fcr = crate::Reg<fcr::FcrSpec>;
#[doc = "FIFO Control Register. Controls UART FIFO usage and modes."]
pub mod fcr;
#[doc = "LCR (rw) register accessor: Line Control Register. Contains controls for frame formatting and break generation.\n\nYou can [`read`](crate::Reg::read) this register and get [`lcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcr`] module"]
#[doc(alias = "LCR")]
pub type Lcr = crate::Reg<lcr::LcrSpec>;
#[doc = "Line Control Register. Contains controls for frame formatting and break generation."]
pub mod lcr;
#[doc = "LSR (r) register accessor: Line Status Register. Contains flags for transmit and receive status, including line errors.\n\nYou can [`read`](crate::Reg::read) this register and get [`lsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lsr`] module"]
#[doc(alias = "LSR")]
pub type Lsr = crate::Reg<lsr::LsrSpec>;
#[doc = "Line Status Register. Contains flags for transmit and receive status, including line errors."]
pub mod lsr;
#[doc = "SCR (rw) register accessor: Scratch Pad Register. 8-bit temporary storage for software.\n\nYou can [`read`](crate::Reg::read) this register and get [`scr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`] module"]
#[doc(alias = "SCR")]
pub type Scr = crate::Reg<scr::ScrSpec>;
#[doc = "Scratch Pad Register. 8-bit temporary storage for software."]
pub mod scr;
#[doc = "ACR (rw) register accessor: Auto-baud Control Register. Contains controls for the auto-baud feature.\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`] module"]
#[doc(alias = "ACR")]
pub type Acr = crate::Reg<acr::AcrSpec>;
#[doc = "Auto-baud Control Register. Contains controls for the auto-baud feature."]
pub mod acr;
#[doc = "ICR (rw) register accessor: IrDA Control Register. Enables and configures the IrDA mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`] module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "IrDA Control Register. Enables and configures the IrDA mode."]
pub mod icr;
#[doc = "FDR (rw) register accessor: Fractional Divider Register. Generates a clock input for the baud rate divider.\n\nYou can [`read`](crate::Reg::read) this register and get [`fdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdr`] module"]
#[doc(alias = "FDR")]
pub type Fdr = crate::Reg<fdr::FdrSpec>;
#[doc = "Fractional Divider Register. Generates a clock input for the baud rate divider."]
pub mod fdr;
#[doc = "OSR (rw) register accessor: Oversampling register. Controls the degree of oversampling during each bit time.\n\nYou can [`read`](crate::Reg::read) this register and get [`osr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`osr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@osr`] module"]
#[doc(alias = "OSR")]
pub type Osr = crate::Reg<osr::OsrSpec>;
#[doc = "Oversampling register. Controls the degree of oversampling during each bit time."]
pub mod osr;
#[doc = "SCICTRL (rw) register accessor: Smart Card Interface control register. Enables and configures the smartcard Interface feature.\n\nYou can [`read`](crate::Reg::read) this register and get [`scictrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scictrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scictrl`] module"]
#[doc(alias = "SCICTRL")]
pub type Scictrl = crate::Reg<scictrl::ScictrlSpec>;
#[doc = "Smart Card Interface control register. Enables and configures the smartcard Interface feature."]
pub mod scictrl;
#[doc = "RS485CTRL (rw) register accessor: RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes.\n\nYou can [`read`](crate::Reg::read) this register and get [`rs485ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rs485ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rs485ctrl`] module"]
#[doc(alias = "RS485CTRL")]
pub type Rs485ctrl = crate::Reg<rs485ctrl::Rs485ctrlSpec>;
#[doc = "RS-485/EIA-485 Control. Contains controls to configure various aspects of RS-485/EIA-485 modes."]
pub mod rs485ctrl;
#[doc = "RS485ADRMATCH (rw) register accessor: RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`rs485adrmatch::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rs485adrmatch::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rs485adrmatch`] module"]
#[doc(alias = "RS485ADRMATCH")]
pub type Rs485adrmatch = crate::Reg<rs485adrmatch::Rs485adrmatchSpec>;
#[doc = "RS-485/EIA-485 address match. Contains the address match value for RS-485/EIA-485 mode."]
pub mod rs485adrmatch;
#[doc = "RS485DLY (rw) register accessor: RS-485/EIA-485 direction control delay.\n\nYou can [`read`](crate::Reg::read) this register and get [`rs485dly::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rs485dly::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rs485dly`] module"]
#[doc(alias = "RS485DLY")]
pub type Rs485dly = crate::Reg<rs485dly::Rs485dlySpec>;
#[doc = "RS-485/EIA-485 direction control delay."]
pub mod rs485dly;
#[doc = "SYNCCTRL (rw) register accessor: Synchronous mode control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`syncctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syncctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@syncctrl`] module"]
#[doc(alias = "SYNCCTRL")]
pub type Syncctrl = crate::Reg<syncctrl::SyncctrlSpec>;
#[doc = "Synchronous mode control register."]
pub mod syncctrl;
