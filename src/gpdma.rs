#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    intstat: Intstat,
    inttcstat: Inttcstat,
    inttcclear: Inttcclear,
    interrstat: Interrstat,
    interrclr: Interrclr,
    rawinttcstat: Rawinttcstat,
    rawinterrstat: Rawinterrstat,
    enbldchns: Enbldchns,
    softbreq: Softbreq,
    softsreq: Softsreq,
    softlbreq: Softlbreq,
    softlsreq: Softlsreq,
    config: Config,
    sync: Sync,
    _reserved14: [u8; 0xc8],
    srcaddr: (),
    _reserved15: [u8; 0x04],
    destaddr: (),
    _reserved16: [u8; 0x04],
    lli: (),
    _reserved17: [u8; 0x04],
    control: (),
    _reserved18: [u8; 0x04],
    channel: (),
}
impl RegisterBlock {
    #[doc = "0x00 - DMA Interrupt Status Register"]
    #[inline(always)]
    pub const fn intstat(&self) -> &Intstat {
        &self.intstat
    }
    #[doc = "0x04 - DMA Interrupt Terminal Count Request Status Register"]
    #[inline(always)]
    pub const fn inttcstat(&self) -> &Inttcstat {
        &self.inttcstat
    }
    #[doc = "0x08 - DMA Interrupt Terminal Count Request Clear Register"]
    #[inline(always)]
    pub const fn inttcclear(&self) -> &Inttcclear {
        &self.inttcclear
    }
    #[doc = "0x0c - DMA Interrupt Error Status Register"]
    #[inline(always)]
    pub const fn interrstat(&self) -> &Interrstat {
        &self.interrstat
    }
    #[doc = "0x10 - DMA Interrupt Error Clear Register"]
    #[inline(always)]
    pub const fn interrclr(&self) -> &Interrclr {
        &self.interrclr
    }
    #[doc = "0x14 - DMA Raw Interrupt Terminal Count Status Register"]
    #[inline(always)]
    pub const fn rawinttcstat(&self) -> &Rawinttcstat {
        &self.rawinttcstat
    }
    #[doc = "0x18 - DMA Raw Error Interrupt Status Register"]
    #[inline(always)]
    pub const fn rawinterrstat(&self) -> &Rawinterrstat {
        &self.rawinterrstat
    }
    #[doc = "0x1c - DMA Enabled Channel Register"]
    #[inline(always)]
    pub const fn enbldchns(&self) -> &Enbldchns {
        &self.enbldchns
    }
    #[doc = "0x20 - DMA Software Burst Request Register"]
    #[inline(always)]
    pub const fn softbreq(&self) -> &Softbreq {
        &self.softbreq
    }
    #[doc = "0x24 - DMA Software Single Request Register"]
    #[inline(always)]
    pub const fn softsreq(&self) -> &Softsreq {
        &self.softsreq
    }
    #[doc = "0x28 - DMA Software Last Burst Request Register"]
    #[inline(always)]
    pub const fn softlbreq(&self) -> &Softlbreq {
        &self.softlbreq
    }
    #[doc = "0x2c - DMA Software Last Single Request Register"]
    #[inline(always)]
    pub const fn softlsreq(&self) -> &Softlsreq {
        &self.softlsreq
    }
    #[doc = "0x30 - DMA Configuration Register"]
    #[inline(always)]
    pub const fn config(&self) -> &Config {
        &self.config
    }
    #[doc = "0x34 - DMA Synchronization Register"]
    #[inline(always)]
    pub const fn sync(&self) -> &Sync {
        &self.sync
    }
    #[doc = "0x100..0x120 - DMA Channel 0 Source Address Register"]
    #[inline(always)]
    pub const fn srcaddr(&self, n: usize) -> &Srcaddr {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(256)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x120 - DMA Channel 0 Source Address Register"]
    #[inline(always)]
    pub fn srcaddr_iter(&self) -> impl Iterator<Item = &Srcaddr> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(256)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x104..0x124 - DMA Channel 0 Destination Address Register"]
    #[inline(always)]
    pub const fn destaddr(&self, n: usize) -> &Destaddr {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(260)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x104..0x124 - DMA Channel 0 Destination Address Register"]
    #[inline(always)]
    pub fn destaddr_iter(&self) -> impl Iterator<Item = &Destaddr> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(260)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x108..0x128 - DMA Channel 0 Linked List Item Register"]
    #[inline(always)]
    pub const fn lli(&self, n: usize) -> &Lli {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(264)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x108..0x128 - DMA Channel 0 Linked List Item Register"]
    #[inline(always)]
    pub fn lli_iter(&self) -> impl Iterator<Item = &Lli> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(264)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x10c..0x12c - DMA Channel 0 Control Register"]
    #[inline(always)]
    pub const fn control(&self, n: usize) -> &Control {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(268)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x10c..0x12c - DMA Channel 0 Control Register"]
    #[inline(always)]
    pub fn control_iter(&self) -> impl Iterator<Item = &Control> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(268)
                .add(32 * n)
                .cast()
        })
    }
    #[doc = "0x110..0x130 - DMA Channel 0 Configuration Register\\[1\\]"]
    #[inline(always)]
    pub const fn channel(&self, n: usize) -> &Channel {
        #[allow(clippy::no_effect)]
        [(); 8][n];
        unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(272)
                .add(32 * n)
                .cast()
        }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x110..0x130 - DMA Channel 0 Configuration Register\\[1\\]"]
    #[inline(always)]
    pub fn channel_iter(&self) -> impl Iterator<Item = &Channel> {
        (0..8).map(move |n| unsafe {
            &*core::ptr::from_ref(self)
                .cast::<u8>()
                .add(272)
                .add(32 * n)
                .cast()
        })
    }
}
#[doc = "INTSTAT (r) register accessor: DMA Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`intstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstat`] module"]
#[doc(alias = "INTSTAT")]
pub type Intstat = crate::Reg<intstat::IntstatSpec>;
#[doc = "DMA Interrupt Status Register"]
pub mod intstat;
#[doc = "INTTCSTAT (r) register accessor: DMA Interrupt Terminal Count Request Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`inttcstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inttcstat`] module"]
#[doc(alias = "INTTCSTAT")]
pub type Inttcstat = crate::Reg<inttcstat::InttcstatSpec>;
#[doc = "DMA Interrupt Terminal Count Request Status Register"]
pub mod inttcstat;
#[doc = "INTTCCLEAR (w) register accessor: DMA Interrupt Terminal Count Request Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inttcclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inttcclear`] module"]
#[doc(alias = "INTTCCLEAR")]
pub type Inttcclear = crate::Reg<inttcclear::InttcclearSpec>;
#[doc = "DMA Interrupt Terminal Count Request Clear Register"]
pub mod inttcclear;
#[doc = "INTERRSTAT (r) register accessor: DMA Interrupt Error Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`interrstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrstat`] module"]
#[doc(alias = "INTERRSTAT")]
pub type Interrstat = crate::Reg<interrstat::InterrstatSpec>;
#[doc = "DMA Interrupt Error Status Register"]
pub mod interrstat;
#[doc = "INTERRCLR (w) register accessor: DMA Interrupt Error Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@interrclr`] module"]
#[doc(alias = "INTERRCLR")]
pub type Interrclr = crate::Reg<interrclr::InterrclrSpec>;
#[doc = "DMA Interrupt Error Clear Register"]
pub mod interrclr;
#[doc = "RAWINTTCSTAT (r) register accessor: DMA Raw Interrupt Terminal Count Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rawinttcstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rawinttcstat`] module"]
#[doc(alias = "RAWINTTCSTAT")]
pub type Rawinttcstat = crate::Reg<rawinttcstat::RawinttcstatSpec>;
#[doc = "DMA Raw Interrupt Terminal Count Status Register"]
pub mod rawinttcstat;
#[doc = "RAWINTERRSTAT (r) register accessor: DMA Raw Error Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rawinterrstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rawinterrstat`] module"]
#[doc(alias = "RAWINTERRSTAT")]
pub type Rawinterrstat = crate::Reg<rawinterrstat::RawinterrstatSpec>;
#[doc = "DMA Raw Error Interrupt Status Register"]
pub mod rawinterrstat;
#[doc = "ENBLDCHNS (r) register accessor: DMA Enabled Channel Register\n\nYou can [`read`](crate::Reg::read) this register and get [`enbldchns::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enbldchns`] module"]
#[doc(alias = "ENBLDCHNS")]
pub type Enbldchns = crate::Reg<enbldchns::EnbldchnsSpec>;
#[doc = "DMA Enabled Channel Register"]
pub mod enbldchns;
#[doc = "SOFTBREQ (rw) register accessor: DMA Software Burst Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`softbreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`softbreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softbreq`] module"]
#[doc(alias = "SOFTBREQ")]
pub type Softbreq = crate::Reg<softbreq::SoftbreqSpec>;
#[doc = "DMA Software Burst Request Register"]
pub mod softbreq;
#[doc = "SOFTSREQ (rw) register accessor: DMA Software Single Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`softsreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`softsreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softsreq`] module"]
#[doc(alias = "SOFTSREQ")]
pub type Softsreq = crate::Reg<softsreq::SoftsreqSpec>;
#[doc = "DMA Software Single Request Register"]
pub mod softsreq;
#[doc = "SOFTLBREQ (rw) register accessor: DMA Software Last Burst Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`softlbreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`softlbreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softlbreq`] module"]
#[doc(alias = "SOFTLBREQ")]
pub type Softlbreq = crate::Reg<softlbreq::SoftlbreqSpec>;
#[doc = "DMA Software Last Burst Request Register"]
pub mod softlbreq;
#[doc = "SOFTLSREQ (rw) register accessor: DMA Software Last Single Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`softlsreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`softlsreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@softlsreq`] module"]
#[doc(alias = "SOFTLSREQ")]
pub type Softlsreq = crate::Reg<softlsreq::SoftlsreqSpec>;
#[doc = "DMA Software Last Single Request Register"]
pub mod softlsreq;
#[doc = "CONFIG (rw) register accessor: DMA Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@config`] module"]
#[doc(alias = "CONFIG")]
pub type Config = crate::Reg<config::ConfigSpec>;
#[doc = "DMA Configuration Register"]
pub mod config;
#[doc = "SYNC (rw) register accessor: DMA Synchronization Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sync::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sync::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sync`] module"]
#[doc(alias = "SYNC")]
pub type Sync = crate::Reg<sync::SyncSpec>;
#[doc = "DMA Synchronization Register"]
pub mod sync;
#[doc = "SRCADDR (rw) register accessor: DMA Channel 0 Source Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srcaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srcaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@srcaddr`] module"]
#[doc(alias = "SRCADDR")]
pub type Srcaddr = crate::Reg<srcaddr::SrcaddrSpec>;
#[doc = "DMA Channel 0 Source Address Register"]
pub mod srcaddr;
#[doc = "DESTADDR (rw) register accessor: DMA Channel 0 Destination Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`destaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`destaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@destaddr`] module"]
#[doc(alias = "DESTADDR")]
pub type Destaddr = crate::Reg<destaddr::DestaddrSpec>;
#[doc = "DMA Channel 0 Destination Address Register"]
pub mod destaddr;
#[doc = "LLI (rw) register accessor: DMA Channel 0 Linked List Item Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lli::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lli::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lli`] module"]
#[doc(alias = "LLI")]
pub type Lli = crate::Reg<lli::LliSpec>;
#[doc = "DMA Channel 0 Linked List Item Register"]
pub mod lli;
#[doc = "CONTROL (rw) register accessor: DMA Channel 0 Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@control`] module"]
#[doc(alias = "CONTROL")]
pub type Control = crate::Reg<control::ControlSpec>;
#[doc = "DMA Channel 0 Control Register"]
pub mod control;
#[doc = "CHANNEL (rw) register accessor: DMA Channel 0 Configuration Register\\[1\\]\n\nYou can [`read`](crate::Reg::read) this register and get [`channel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`channel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@channel`] module"]
#[doc(alias = "CHANNEL")]
pub type Channel = crate::Reg<channel::ChannelSpec>;
#[doc = "DMA Channel 0 Configuration Register\\[1\\]"]
pub mod channel;
