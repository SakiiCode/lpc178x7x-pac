#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    timh: Timh,
    timv: Timv,
    pol: Pol,
    le: Le,
    upbase: Upbase,
    lpbase: Lpbase,
    ctrl: Ctrl,
    intmsk: Intmsk,
    intraw: Intraw,
    intstat: Intstat,
    intclr: Intclr,
    upcurr: Upcurr,
    lpcurr: Lpcurr,
    _reserved13: [u8; 0x01cc],
    pal: [Pal; 128],
    _reserved14: [u8; 0x0400],
    crsr_img: [CrsrImg; 256],
    crsr_ctrl: CrsrCtrl,
    crsr_cfg: CrsrCfg,
    crsr_pal0: CrsrPal0,
    crsr_pal1: CrsrPal1,
    crsr_xy: CrsrXy,
    crsr_clip: CrsrClip,
    _reserved21: [u8; 0x08],
    crsr_intmsk: CrsrIntmsk,
    crsr_intclr: CrsrIntclr,
    crsr_intraw: CrsrIntraw,
    crsr_intstat: CrsrIntstat,
}
impl RegisterBlock {
    #[doc = "0x00 - Horizontal Timing Control register"]
    #[inline(always)]
    pub const fn timh(&self) -> &Timh {
        &self.timh
    }
    #[doc = "0x04 - Vertical Timing Control register"]
    #[inline(always)]
    pub const fn timv(&self) -> &Timv {
        &self.timv
    }
    #[doc = "0x08 - Clock and Signal Polarity Control register"]
    #[inline(always)]
    pub const fn pol(&self) -> &Pol {
        &self.pol
    }
    #[doc = "0x0c - Line End Control register"]
    #[inline(always)]
    pub const fn le(&self) -> &Le {
        &self.le
    }
    #[doc = "0x10 - Upper Panel Frame Base Address register"]
    #[inline(always)]
    pub const fn upbase(&self) -> &Upbase {
        &self.upbase
    }
    #[doc = "0x14 - Lower Panel Frame Base Address register"]
    #[inline(always)]
    pub const fn lpbase(&self) -> &Lpbase {
        &self.lpbase
    }
    #[doc = "0x18 - LCD Control register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x1c - Interrupt Mask register"]
    #[inline(always)]
    pub const fn intmsk(&self) -> &Intmsk {
        &self.intmsk
    }
    #[doc = "0x20 - Raw Interrupt Status register"]
    #[inline(always)]
    pub const fn intraw(&self) -> &Intraw {
        &self.intraw
    }
    #[doc = "0x24 - Masked Interrupt Status register"]
    #[inline(always)]
    pub const fn intstat(&self) -> &Intstat {
        &self.intstat
    }
    #[doc = "0x28 - Interrupt Clear register"]
    #[inline(always)]
    pub const fn intclr(&self) -> &Intclr {
        &self.intclr
    }
    #[doc = "0x2c - Upper Panel Current Address Value register"]
    #[inline(always)]
    pub const fn upcurr(&self) -> &Upcurr {
        &self.upcurr
    }
    #[doc = "0x30 - Lower Panel Current Address Value register"]
    #[inline(always)]
    pub const fn lpcurr(&self) -> &Lpcurr {
        &self.lpcurr
    }
    #[doc = "0x200..0x400 - 256x16-bit Color Palette registers"]
    #[inline(always)]
    pub const fn pal(&self, n: usize) -> &Pal {
        &self.pal[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x200..0x400 - 256x16-bit Color Palette registers"]
    #[inline(always)]
    pub fn pal_iter(&self) -> impl Iterator<Item = &Pal> {
        self.pal.iter()
    }
    #[doc = "0x800..0xc00 - Cursor Image registers"]
    #[inline(always)]
    pub const fn crsr_img(&self, n: usize) -> &CrsrImg {
        &self.crsr_img[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0xc00 - Cursor Image registers"]
    #[inline(always)]
    pub fn crsr_img_iter(&self) -> impl Iterator<Item = &CrsrImg> {
        self.crsr_img.iter()
    }
    #[doc = "0xc00 - Cursor Control register"]
    #[inline(always)]
    pub const fn crsr_ctrl(&self) -> &CrsrCtrl {
        &self.crsr_ctrl
    }
    #[doc = "0xc04 - Cursor Configuration register"]
    #[inline(always)]
    pub const fn crsr_cfg(&self) -> &CrsrCfg {
        &self.crsr_cfg
    }
    #[doc = "0xc08 - Cursor Palette register 0"]
    #[inline(always)]
    pub const fn crsr_pal0(&self) -> &CrsrPal0 {
        &self.crsr_pal0
    }
    #[doc = "0xc0c - Cursor Palette register 1"]
    #[inline(always)]
    pub const fn crsr_pal1(&self) -> &CrsrPal1 {
        &self.crsr_pal1
    }
    #[doc = "0xc10 - Cursor XY Position register"]
    #[inline(always)]
    pub const fn crsr_xy(&self) -> &CrsrXy {
        &self.crsr_xy
    }
    #[doc = "0xc14 - Cursor Clip Position register"]
    #[inline(always)]
    pub const fn crsr_clip(&self) -> &CrsrClip {
        &self.crsr_clip
    }
    #[doc = "0xc20 - Cursor Interrupt Mask register"]
    #[inline(always)]
    pub const fn crsr_intmsk(&self) -> &CrsrIntmsk {
        &self.crsr_intmsk
    }
    #[doc = "0xc24 - Cursor Interrupt Clear register"]
    #[inline(always)]
    pub const fn crsr_intclr(&self) -> &CrsrIntclr {
        &self.crsr_intclr
    }
    #[doc = "0xc28 - Cursor Raw Interrupt Status register"]
    #[inline(always)]
    pub const fn crsr_intraw(&self) -> &CrsrIntraw {
        &self.crsr_intraw
    }
    #[doc = "0xc2c - Cursor Masked Interrupt Status register"]
    #[inline(always)]
    pub const fn crsr_intstat(&self) -> &CrsrIntstat {
        &self.crsr_intstat
    }
}
#[doc = "TIMH (rw) register accessor: Horizontal Timing Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`timh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timh`] module"]
#[doc(alias = "TIMH")]
pub type Timh = crate::Reg<timh::TimhSpec>;
#[doc = "Horizontal Timing Control register"]
pub mod timh;
#[doc = "TIMV (rw) register accessor: Vertical Timing Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`timv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timv`] module"]
#[doc(alias = "TIMV")]
pub type Timv = crate::Reg<timv::TimvSpec>;
#[doc = "Vertical Timing Control register"]
pub mod timv;
#[doc = "POL (rw) register accessor: Clock and Signal Polarity Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pol`] module"]
#[doc(alias = "POL")]
pub type Pol = crate::Reg<pol::PolSpec>;
#[doc = "Clock and Signal Polarity Control register"]
pub mod pol;
#[doc = "LE (rw) register accessor: Line End Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`le::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`le::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@le`] module"]
#[doc(alias = "LE")]
pub type Le = crate::Reg<le::LeSpec>;
#[doc = "Line End Control register"]
pub mod le;
#[doc = "UPBASE (rw) register accessor: Upper Panel Frame Base Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`upbase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`upbase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@upbase`] module"]
#[doc(alias = "UPBASE")]
pub type Upbase = crate::Reg<upbase::UpbaseSpec>;
#[doc = "Upper Panel Frame Base Address register"]
pub mod upbase;
#[doc = "LPBASE (rw) register accessor: Lower Panel Frame Base Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpbase::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpbase::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpbase`] module"]
#[doc(alias = "LPBASE")]
pub type Lpbase = crate::Reg<lpbase::LpbaseSpec>;
#[doc = "Lower Panel Frame Base Address register"]
pub mod lpbase;
#[doc = "CTRL (rw) register accessor: LCD Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`] module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "LCD Control register"]
pub mod ctrl;
#[doc = "INTMSK (rw) register accessor: Interrupt Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`intmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intmsk`] module"]
#[doc(alias = "INTMSK")]
pub type Intmsk = crate::Reg<intmsk::IntmskSpec>;
#[doc = "Interrupt Mask register"]
pub mod intmsk;
#[doc = "INTRAW (r) register accessor: Raw Interrupt Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intraw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intraw`] module"]
#[doc(alias = "INTRAW")]
pub type Intraw = crate::Reg<intraw::IntrawSpec>;
#[doc = "Raw Interrupt Status register"]
pub mod intraw;
#[doc = "INTSTAT (r) register accessor: Masked Interrupt Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat`] module"]
#[doc(alias = "INTSTAT")]
pub type Intstat = crate::Reg<intstat::IntstatSpec>;
#[doc = "Masked Interrupt Status register"]
pub mod intstat;
#[doc = "INTCLR (w) register accessor: Interrupt Clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intclr`] module"]
#[doc(alias = "INTCLR")]
pub type Intclr = crate::Reg<intclr::IntclrSpec>;
#[doc = "Interrupt Clear register"]
pub mod intclr;
#[doc = "UPCURR (r) register accessor: Upper Panel Current Address Value register\n\nYou can [`read`](crate::Reg::read) this register and get [`upcurr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@upcurr`] module"]
#[doc(alias = "UPCURR")]
pub type Upcurr = crate::Reg<upcurr::UpcurrSpec>;
#[doc = "Upper Panel Current Address Value register"]
pub mod upcurr;
#[doc = "LPCURR (r) register accessor: Lower Panel Current Address Value register\n\nYou can [`read`](crate::Reg::read) this register and get [`lpcurr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lpcurr`] module"]
#[doc(alias = "LPCURR")]
pub type Lpcurr = crate::Reg<lpcurr::LpcurrSpec>;
#[doc = "Lower Panel Current Address Value register"]
pub mod lpcurr;
#[doc = "PAL (rw) register accessor: 256x16-bit Color Palette registers\n\nYou can [`read`](crate::Reg::read) this register and get [`pal::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pal::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pal`] module"]
#[doc(alias = "PAL")]
pub type Pal = crate::Reg<pal::PalSpec>;
#[doc = "256x16-bit Color Palette registers"]
pub mod pal;
#[doc = "CRSR_IMG (rw) register accessor: Cursor Image registers\n\nYou can [`read`](crate::Reg::read) this register and get [`crsr_img::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crsr_img::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crsr_img`] module"]
#[doc(alias = "CRSR_IMG")]
pub type CrsrImg = crate::Reg<crsr_img::CrsrImgSpec>;
#[doc = "Cursor Image registers"]
pub mod crsr_img;
#[doc = "CRSR_CTRL (rw) register accessor: Cursor Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`crsr_ctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crsr_ctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crsr_ctrl`] module"]
#[doc(alias = "CRSR_CTRL")]
pub type CrsrCtrl = crate::Reg<crsr_ctrl::CrsrCtrlSpec>;
#[doc = "Cursor Control register"]
pub mod crsr_ctrl;
#[doc = "CRSR_CFG (rw) register accessor: Cursor Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`crsr_cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crsr_cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crsr_cfg`] module"]
#[doc(alias = "CRSR_CFG")]
pub type CrsrCfg = crate::Reg<crsr_cfg::CrsrCfgSpec>;
#[doc = "Cursor Configuration register"]
pub mod crsr_cfg;
#[doc = "CRSR_PAL0 (rw) register accessor: Cursor Palette register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`crsr_pal0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crsr_pal0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crsr_pal0`] module"]
#[doc(alias = "CRSR_PAL0")]
pub type CrsrPal0 = crate::Reg<crsr_pal0::CrsrPal0Spec>;
#[doc = "Cursor Palette register 0"]
pub mod crsr_pal0;
#[doc = "CRSR_PAL1 (rw) register accessor: Cursor Palette register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`crsr_pal1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crsr_pal1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crsr_pal1`] module"]
#[doc(alias = "CRSR_PAL1")]
pub type CrsrPal1 = crate::Reg<crsr_pal1::CrsrPal1Spec>;
#[doc = "Cursor Palette register 1"]
pub mod crsr_pal1;
#[doc = "CRSR_XY (rw) register accessor: Cursor XY Position register\n\nYou can [`read`](crate::Reg::read) this register and get [`crsr_xy::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crsr_xy::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crsr_xy`] module"]
#[doc(alias = "CRSR_XY")]
pub type CrsrXy = crate::Reg<crsr_xy::CrsrXySpec>;
#[doc = "Cursor XY Position register"]
pub mod crsr_xy;
#[doc = "CRSR_CLIP (rw) register accessor: Cursor Clip Position register\n\nYou can [`read`](crate::Reg::read) this register and get [`crsr_clip::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crsr_clip::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crsr_clip`] module"]
#[doc(alias = "CRSR_CLIP")]
pub type CrsrClip = crate::Reg<crsr_clip::CrsrClipSpec>;
#[doc = "Cursor Clip Position register"]
pub mod crsr_clip;
#[doc = "CRSR_INTMSK (rw) register accessor: Cursor Interrupt Mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`crsr_intmsk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crsr_intmsk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crsr_intmsk`] module"]
#[doc(alias = "CRSR_INTMSK")]
pub type CrsrIntmsk = crate::Reg<crsr_intmsk::CrsrIntmskSpec>;
#[doc = "Cursor Interrupt Mask register"]
pub mod crsr_intmsk;
#[doc = "CRSR_INTCLR (w) register accessor: Cursor Interrupt Clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crsr_intclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crsr_intclr`] module"]
#[doc(alias = "CRSR_INTCLR")]
pub type CrsrIntclr = crate::Reg<crsr_intclr::CrsrIntclrSpec>;
#[doc = "Cursor Interrupt Clear register"]
pub mod crsr_intclr;
#[doc = "CRSR_INTRAW (r) register accessor: Cursor Raw Interrupt Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`crsr_intraw::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crsr_intraw`] module"]
#[doc(alias = "CRSR_INTRAW")]
pub type CrsrIntraw = crate::Reg<crsr_intraw::CrsrIntrawSpec>;
#[doc = "Cursor Raw Interrupt Status register"]
pub mod crsr_intraw;
#[doc = "CRSR_INTSTAT (r) register accessor: Cursor Masked Interrupt Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`crsr_intstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crsr_intstat`] module"]
#[doc(alias = "CRSR_INTSTAT")]
pub type CrsrIntstat = crate::Reg<crsr_intstat::CrsrIntstatSpec>;
#[doc = "Cursor Masked Interrupt Status register"]
pub mod crsr_intstat;
