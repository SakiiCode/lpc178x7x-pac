#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    flashcfg: Flashcfg,
    _reserved1: [u8; 0x7c],
    pllcon: (),
    _reserved2: [u8; 0x04],
    pllcfg: (),
    _reserved3: [u8; 0x04],
    pllstat: (),
    _reserved4: [u8; 0x04],
    pllfeed: (),
    _reserved5: [u8; 0x34],
    pcon: Pcon,
    pconp: Pconp,
    _reserved7: [u8; 0x38],
    emcclksel: Emcclksel,
    cclksel: Cclksel,
    usbclksel: Usbclksel,
    clksrcsel: Clksrcsel,
    cansleepclr: Cansleepclr,
    canwakeflags: Canwakeflags,
    _reserved13: [u8; 0x28],
    extint: Extint,
    _reserved14: [u8; 0x04],
    extmode: Extmode,
    extpolar: Extpolar,
    _reserved16: [u8; 0x30],
    rsid: Rsid,
    _reserved17: [u8; 0x04],
    matrixarb: Matrixarb,
    _reserved18: [u8; 0x14],
    scs: Scs,
    _reserved19: [u8; 0x04],
    pclksel: Pclksel,
    _reserved20: [u8; 0x04],
    pboost: Pboost,
    spificlksel: Spificlksel,
    lcd_cfg: LcdCfg,
    _reserved23: [u8; 0x04],
    usbintst: Usbintst,
    dmacreqsel: Dmacreqsel,
    clkoutcfg: Clkoutcfg,
    rstcon0: Rstcon0,
    rstcon1: Rstcon1,
    _reserved28: [u8; 0x08],
    emcdlyctl: Emcdlyctl,
    emccal: Emccal,
}
impl RegisterBlock {
    #[doc = "0x00 - Flash Accelerator Configuration Register. Controls flash access timing."]
    #[inline(always)]
    pub const fn flashcfg(&self) -> &Flashcfg {
        &self.flashcfg
    }
    #[doc = "0x80..0x88 - PLL0 Control register"]
    #[inline(always)]
    pub const fn pllcon(&self, n: usize) -> &Pllcon {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(128)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x88 - PLL0 Control register"]
    #[inline(always)]
    pub fn pllcon_iter(&self) -> impl Iterator<Item = &Pllcon> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(128)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x80 - PLL0 Control register"]
    #[inline(always)]
    pub const fn pll0con(&self) -> &Pllcon {
        self.pllcon(0)
    }
    #[doc = "0xa0 - PLL0 Control register"]
    #[inline(always)]
    pub const fn pll1con(&self) -> &Pllcon {
        self.pllcon(1)
    }
    #[doc = "0x84..0x8c - PLL0 Configuration register"]
    #[inline(always)]
    pub const fn pllcfg(&self, n: usize) -> &Pllcfg {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(132)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x84..0x8c - PLL0 Configuration register"]
    #[inline(always)]
    pub fn pllcfg_iter(&self) -> impl Iterator<Item = &Pllcfg> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(132)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x84 - PLL0 Configuration register"]
    #[inline(always)]
    pub const fn pll0cfg(&self) -> &Pllcfg {
        self.pllcfg(0)
    }
    #[doc = "0xa4 - PLL0 Configuration register"]
    #[inline(always)]
    pub const fn pll1cfg(&self) -> &Pllcfg {
        self.pllcfg(1)
    }
    #[doc = "0x88..0x90 - PLL0 Status register"]
    #[inline(always)]
    pub const fn pllstat(&self, n: usize) -> &Pllstat {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(136)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x88..0x90 - PLL0 Status register"]
    #[inline(always)]
    pub fn pllstat_iter(&self) -> impl Iterator<Item = &Pllstat> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(136)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x88 - PLL0 Status register"]
    #[inline(always)]
    pub const fn pll0stat(&self) -> &Pllstat {
        self.pllstat(0)
    }
    #[doc = "0xa8 - PLL0 Status register"]
    #[inline(always)]
    pub const fn pll1stat(&self) -> &Pllstat {
        self.pllstat(1)
    }
    #[doc = "0x8c..0x94 - PLL0 Feed register"]
    #[inline(always)]
    pub const fn pllfeed(&self, n: usize) -> &Pllfeed {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(140)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x8c..0x94 - PLL0 Feed register"]
    #[inline(always)]
    pub fn pllfeed_iter(&self) -> impl Iterator<Item = &Pllfeed> {
        (0..2).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(140)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x8c - PLL0 Feed register"]
    #[inline(always)]
    pub const fn pll0feed(&self) -> &Pllfeed {
        self.pllfeed(0)
    }
    #[doc = "0xac - PLL0 Feed register"]
    #[inline(always)]
    pub const fn pll1feed(&self) -> &Pllfeed {
        self.pllfeed(1)
    }
    #[doc = "0xc0 - Power Control register"]
    #[inline(always)]
    pub const fn pcon(&self) -> &Pcon {
        &self.pcon
    }
    #[doc = "0xc4 - Power Control for Peripherals"]
    #[inline(always)]
    pub const fn pconp(&self) -> &Pconp {
        &self.pconp
    }
    #[doc = "0x100 - External Memory Controller Clock Selection register"]
    #[inline(always)]
    pub const fn emcclksel(&self) -> &Emcclksel {
        &self.emcclksel
    }
    #[doc = "0x104 - CPU Clock Selection register"]
    #[inline(always)]
    pub const fn cclksel(&self) -> &Cclksel {
        &self.cclksel
    }
    #[doc = "0x108 - USB Clock Selection register"]
    #[inline(always)]
    pub const fn usbclksel(&self) -> &Usbclksel {
        &self.usbclksel
    }
    #[doc = "0x10c - Clock Source Select Register"]
    #[inline(always)]
    pub const fn clksrcsel(&self) -> &Clksrcsel {
        &self.clksrcsel
    }
    #[doc = "0x110 - Allows clearing the current CAN channel sleep state as well as reading that state."]
    #[inline(always)]
    pub const fn cansleepclr(&self) -> &Cansleepclr {
        &self.cansleepclr
    }
    #[doc = "0x114 - Allows reading the wake-up state of the CAN channels."]
    #[inline(always)]
    pub const fn canwakeflags(&self) -> &Canwakeflags {
        &self.canwakeflags
    }
    #[doc = "0x140 - External Interrupt Flag Register"]
    #[inline(always)]
    pub const fn extint(&self) -> &Extint {
        &self.extint
    }
    #[doc = "0x148 - External Interrupt Mode register"]
    #[inline(always)]
    pub const fn extmode(&self) -> &Extmode {
        &self.extmode
    }
    #[doc = "0x14c - External Interrupt Polarity Register"]
    #[inline(always)]
    pub const fn extpolar(&self) -> &Extpolar {
        &self.extpolar
    }
    #[doc = "0x180 - Reset Source Identification Register"]
    #[inline(always)]
    pub const fn rsid(&self) -> &Rsid {
        &self.rsid
    }
    #[doc = "0x188 - Matrix arbitration register"]
    #[inline(always)]
    pub const fn matrixarb(&self) -> &Matrixarb {
        &self.matrixarb
    }
    #[doc = "0x1a0 - System Control and Status"]
    #[inline(always)]
    pub const fn scs(&self) -> &Scs {
        &self.scs
    }
    #[doc = "0x1a8 - Peripheral Clock Selection register"]
    #[inline(always)]
    pub const fn pclksel(&self) -> &Pclksel {
        &self.pclksel
    }
    #[doc = "0x1b0 - Power boost register"]
    #[inline(always)]
    pub const fn pboost(&self) -> &Pboost {
        &self.pboost
    }
    #[doc = "0x1b4 - SPIFI Clock Selection register"]
    #[inline(always)]
    pub const fn spificlksel(&self) -> &Spificlksel {
        &self.spificlksel
    }
    #[doc = "0x1b8 - LCD Clock configuration register"]
    #[inline(always)]
    pub const fn lcd_cfg(&self) -> &LcdCfg {
        &self.lcd_cfg
    }
    #[doc = "0x1c0 - USB Interrupt Status"]
    #[inline(always)]
    pub const fn usbintst(&self) -> &Usbintst {
        &self.usbintst
    }
    #[doc = "0x1c4 - Selects between alternative requests on DMA channels 0 through 7 and 10 through 15"]
    #[inline(always)]
    pub const fn dmacreqsel(&self) -> &Dmacreqsel {
        &self.dmacreqsel
    }
    #[doc = "0x1c8 - Clock Output Configuration register"]
    #[inline(always)]
    pub const fn clkoutcfg(&self) -> &Clkoutcfg {
        &self.clkoutcfg
    }
    #[doc = "0x1cc - Individual peripheral reset control bits"]
    #[inline(always)]
    pub const fn rstcon0(&self) -> &Rstcon0 {
        &self.rstcon0
    }
    #[doc = "0x1d0 - Individual peripheral reset control bits"]
    #[inline(always)]
    pub const fn rstcon1(&self) -> &Rstcon1 {
        &self.rstcon1
    }
    #[doc = "0x1dc - Values for the 4 programmable delays associated with SDRAM operation."]
    #[inline(always)]
    pub const fn emcdlyctl(&self) -> &Emcdlyctl {
        &self.emcdlyctl
    }
    #[doc = "0x1e0 - Controls the calibration counter for programmable delays and returns the result value."]
    #[inline(always)]
    pub const fn emccal(&self) -> &Emccal {
        &self.emccal
    }
}
#[doc = "FLASHCFG (rw) register accessor: Flash Accelerator Configuration Register. Controls flash access timing.\n\nYou can [`read`](crate::Reg::read) this register and get [`flashcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flashcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flashcfg`] module"]
#[doc(alias = "FLASHCFG")]
pub type Flashcfg = crate::Reg<flashcfg::FlashcfgSpec>;
#[doc = "Flash Accelerator Configuration Register. Controls flash access timing."]
pub mod flashcfg;
#[doc = "PLLCON (rw) register accessor: PLL0 Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pllcon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllcon`] module"]
#[doc(alias = "PLLCON")]
pub type Pllcon = crate::Reg<pllcon::PllconSpec>;
#[doc = "PLL0 Control register"]
pub mod pllcon;
#[doc = "PLLCFG (rw) register accessor: PLL0 Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pllcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllcfg`] module"]
#[doc(alias = "PLLCFG")]
pub type Pllcfg = crate::Reg<pllcfg::PllcfgSpec>;
#[doc = "PLL0 Configuration register"]
pub mod pllcfg;
#[doc = "PLLSTAT (r) register accessor: PLL0 Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`pllstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllstat`] module"]
#[doc(alias = "PLLSTAT")]
pub type Pllstat = crate::Reg<pllstat::PllstatSpec>;
#[doc = "PLL0 Status register"]
pub mod pllstat;
#[doc = "PLLFEED (w) register accessor: PLL0 Feed register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllfeed::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllfeed`] module"]
#[doc(alias = "PLLFEED")]
pub type Pllfeed = crate::Reg<pllfeed::PllfeedSpec>;
#[doc = "PLL0 Feed register"]
pub mod pllfeed;
#[doc = "PCON (rw) register accessor: Power Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcon`] module"]
#[doc(alias = "PCON")]
pub type Pcon = crate::Reg<pcon::PconSpec>;
#[doc = "Power Control register"]
pub mod pcon;
#[doc = "PCONP (rw) register accessor: Power Control for Peripherals\n\nYou can [`read`](crate::Reg::read) this register and get [`pconp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pconp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pconp`] module"]
#[doc(alias = "PCONP")]
pub type Pconp = crate::Reg<pconp::PconpSpec>;
#[doc = "Power Control for Peripherals"]
pub mod pconp;
#[doc = "EMCCLKSEL (rw) register accessor: External Memory Controller Clock Selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`emcclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emcclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emcclksel`] module"]
#[doc(alias = "EMCCLKSEL")]
pub type Emcclksel = crate::Reg<emcclksel::EmcclkselSpec>;
#[doc = "External Memory Controller Clock Selection register"]
pub mod emcclksel;
#[doc = "CCLKSEL (rw) register accessor: CPU Clock Selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`cclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cclksel`] module"]
#[doc(alias = "CCLKSEL")]
pub type Cclksel = crate::Reg<cclksel::CclkselSpec>;
#[doc = "CPU Clock Selection register"]
pub mod cclksel;
#[doc = "USBCLKSEL (rw) register accessor: USB Clock Selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`usbclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbclksel`] module"]
#[doc(alias = "USBCLKSEL")]
pub type Usbclksel = crate::Reg<usbclksel::UsbclkselSpec>;
#[doc = "USB Clock Selection register"]
pub mod usbclksel;
#[doc = "CLKSRCSEL (rw) register accessor: Clock Source Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`clksrcsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clksrcsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clksrcsel`] module"]
#[doc(alias = "CLKSRCSEL")]
pub type Clksrcsel = crate::Reg<clksrcsel::ClksrcselSpec>;
#[doc = "Clock Source Select Register"]
pub mod clksrcsel;
#[doc = "CANSLEEPCLR (rw) register accessor: Allows clearing the current CAN channel sleep state as well as reading that state.\n\nYou can [`read`](crate::Reg::read) this register and get [`cansleepclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cansleepclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cansleepclr`] module"]
#[doc(alias = "CANSLEEPCLR")]
pub type Cansleepclr = crate::Reg<cansleepclr::CansleepclrSpec>;
#[doc = "Allows clearing the current CAN channel sleep state as well as reading that state."]
pub mod cansleepclr;
#[doc = "CANWAKEFLAGS (rw) register accessor: Allows reading the wake-up state of the CAN channels.\n\nYou can [`read`](crate::Reg::read) this register and get [`canwakeflags::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canwakeflags::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@canwakeflags`] module"]
#[doc(alias = "CANWAKEFLAGS")]
pub type Canwakeflags = crate::Reg<canwakeflags::CanwakeflagsSpec>;
#[doc = "Allows reading the wake-up state of the CAN channels."]
pub mod canwakeflags;
#[doc = "EXTINT (rw) register accessor: External Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extint`] module"]
#[doc(alias = "EXTINT")]
pub type Extint = crate::Reg<extint::ExtintSpec>;
#[doc = "External Interrupt Flag Register"]
pub mod extint;
#[doc = "EXTMODE (rw) register accessor: External Interrupt Mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`extmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extmode`] module"]
#[doc(alias = "EXTMODE")]
pub type Extmode = crate::Reg<extmode::ExtmodeSpec>;
#[doc = "External Interrupt Mode register"]
pub mod extmode;
#[doc = "EXTPOLAR (rw) register accessor: External Interrupt Polarity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`extpolar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`extpolar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@extpolar`] module"]
#[doc(alias = "EXTPOLAR")]
pub type Extpolar = crate::Reg<extpolar::ExtpolarSpec>;
#[doc = "External Interrupt Polarity Register"]
pub mod extpolar;
#[doc = "RSID (rw) register accessor: Reset Source Identification Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsid::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsid::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsid`] module"]
#[doc(alias = "RSID")]
pub type Rsid = crate::Reg<rsid::RsidSpec>;
#[doc = "Reset Source Identification Register"]
pub mod rsid;
#[doc = "MATRIXARB (rw) register accessor: Matrix arbitration register\n\nYou can [`read`](crate::Reg::read) this register and get [`matrixarb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrixarb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrixarb`] module"]
#[doc(alias = "MATRIXARB")]
pub type Matrixarb = crate::Reg<matrixarb::MatrixarbSpec>;
#[doc = "Matrix arbitration register"]
pub mod matrixarb;
#[doc = "SCS (rw) register accessor: System Control and Status\n\nYou can [`read`](crate::Reg::read) this register and get [`scs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scs`] module"]
#[doc(alias = "SCS")]
pub type Scs = crate::Reg<scs::ScsSpec>;
#[doc = "System Control and Status"]
pub mod scs;
#[doc = "PCLKSEL (rw) register accessor: Peripheral Clock Selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`pclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pclksel`] module"]
#[doc(alias = "PCLKSEL")]
pub type Pclksel = crate::Reg<pclksel::PclkselSpec>;
#[doc = "Peripheral Clock Selection register"]
pub mod pclksel;
#[doc = "PBOOST (rw) register accessor: Power boost register\n\nYou can [`read`](crate::Reg::read) this register and get [`pboost::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pboost::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pboost`] module"]
#[doc(alias = "PBOOST")]
pub type Pboost = crate::Reg<pboost::PboostSpec>;
#[doc = "Power boost register"]
pub mod pboost;
#[doc = "SPIFICLKSEL (rw) register accessor: SPIFI Clock Selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`spificlksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spificlksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@spificlksel`] module"]
#[doc(alias = "SPIFICLKSEL")]
pub type Spificlksel = crate::Reg<spificlksel::SpificlkselSpec>;
#[doc = "SPIFI Clock Selection register"]
pub mod spificlksel;
#[doc = "LCD_CFG (rw) register accessor: LCD Clock configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`lcd_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lcd_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lcd_cfg`] module"]
#[doc(alias = "LCD_CFG")]
pub type LcdCfg = crate::Reg<lcd_cfg::LcdCfgSpec>;
#[doc = "LCD Clock configuration register"]
pub mod lcd_cfg;
#[doc = "USBINTST (rw) register accessor: USB Interrupt Status\n\nYou can [`read`](crate::Reg::read) this register and get [`usbintst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbintst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@usbintst`] module"]
#[doc(alias = "USBINTST")]
pub type Usbintst = crate::Reg<usbintst::UsbintstSpec>;
#[doc = "USB Interrupt Status"]
pub mod usbintst;
#[doc = "DMACREQSEL (rw) register accessor: Selects between alternative requests on DMA channels 0 through 7 and 10 through 15\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacreqsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacreqsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dmacreqsel`] module"]
#[doc(alias = "DMACREQSEL")]
pub type Dmacreqsel = crate::Reg<dmacreqsel::DmacreqselSpec>;
#[doc = "Selects between alternative requests on DMA channels 0 through 7 and 10 through 15"]
pub mod dmacreqsel;
#[doc = "CLKOUTCFG (rw) register accessor: Clock Output Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`clkoutcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkoutcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clkoutcfg`] module"]
#[doc(alias = "CLKOUTCFG")]
pub type Clkoutcfg = crate::Reg<clkoutcfg::ClkoutcfgSpec>;
#[doc = "Clock Output Configuration register"]
pub mod clkoutcfg;
#[doc = "RSTCON0 (rw) register accessor: Individual peripheral reset control bits\n\nYou can [`read`](crate::Reg::read) this register and get [`rstcon0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstcon0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstcon0`] module"]
#[doc(alias = "RSTCON0")]
pub type Rstcon0 = crate::Reg<rstcon0::Rstcon0Spec>;
#[doc = "Individual peripheral reset control bits"]
pub mod rstcon0;
#[doc = "RSTCON1 (rw) register accessor: Individual peripheral reset control bits\n\nYou can [`read`](crate::Reg::read) this register and get [`rstcon1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstcon1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstcon1`] module"]
#[doc(alias = "RSTCON1")]
pub type Rstcon1 = crate::Reg<rstcon1::Rstcon1Spec>;
#[doc = "Individual peripheral reset control bits"]
pub mod rstcon1;
#[doc = "EMCDLYCTL (rw) register accessor: Values for the 4 programmable delays associated with SDRAM operation.\n\nYou can [`read`](crate::Reg::read) this register and get [`emcdlyctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emcdlyctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emcdlyctl`] module"]
#[doc(alias = "EMCDLYCTL")]
pub type Emcdlyctl = crate::Reg<emcdlyctl::EmcdlyctlSpec>;
#[doc = "Values for the 4 programmable delays associated with SDRAM operation."]
pub mod emcdlyctl;
#[doc = "EMCCAL (rw) register accessor: Controls the calibration counter for programmable delays and returns the result value.\n\nYou can [`read`](crate::Reg::read) this register and get [`emccal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`emccal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@emccal`] module"]
#[doc(alias = "EMCCAL")]
pub type Emccal = crate::Reg<emccal::EmccalSpec>;
#[doc = "Controls the calibration counter for programmable delays and returns the result value."]
pub mod emccal;
