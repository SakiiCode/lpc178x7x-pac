#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ir: Ir,
    tcr: Tcr,
    tc: Tc,
    pr: Pr,
    pc: Pc,
    mcr: Mcr,
    mr: [Mr; 4],
    ccr: Ccr,
    cr: [Cr; 2],
    _reserved9: [u8; 0x0c],
    mr4: Mr4,
    mr5: Mr4,
    mr6: Mr4,
    pcr: Pcr,
    ler: Ler,
    _reserved14: [u8; 0x1c],
    ctcr: Ctcr,
}
impl RegisterBlock {
    #[doc = "0x00 - Interrupt Register. The IR can be written to clear interrupts, or read to identify which PWM interrupt sources are pending."]
    #[inline(always)]
    pub const fn ir(&self) -> &Ir {
        &self.ir
    }
    #[doc = "0x04 - Timer Control Register. The TCR is used to control the Timer Counter functions."]
    #[inline(always)]
    pub const fn tcr(&self) -> &Tcr {
        &self.tcr
    }
    #[doc = "0x08 - Timer Counter. The 32 bit TC is incremented every PR+1 cycles of PCLK. The TC is controlled through the TCR."]
    #[inline(always)]
    pub const fn tc(&self) -> &Tc {
        &self.tc
    }
    #[doc = "0x0c - Prescale Register. Determines how often the PWM counter is incremented."]
    #[inline(always)]
    pub const fn pr(&self) -> &Pr {
        &self.pr
    }
    #[doc = "0x10 - Prescale Counter. Prescaler for the main PWM counter."]
    #[inline(always)]
    pub const fn pc(&self) -> &Pc {
        &self.pc
    }
    #[doc = "0x14 - Match Control Register. The MCR is used to control whether an interrupt is generated and if the PWM counter is reset when a Match occurs."]
    #[inline(always)]
    pub const fn mcr(&self) -> &Mcr {
        &self.mcr
    }
    #[doc = "0x18..0x28 - Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
    #[inline(always)]
    pub const fn mr(&self, n: usize) -> &Mr {
        &self.mr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x18..0x28 - Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
    #[inline(always)]
    pub fn mr_iter(&self) -> impl Iterator<Item = &Mr> {
        self.mr.iter()
    }
    #[doc = "0x28 - Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated for a capture event."]
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
    #[doc = "0x2c..0x34 - PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs."]
    #[inline(always)]
    pub const fn cr(&self, n: usize) -> &Cr {
        &self.cr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x2c..0x34 - PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs."]
    #[inline(always)]
    pub fn cr_iter(&self) -> impl Iterator<Item = &Cr> {
        self.cr.iter()
    }
    #[doc = "0x40 - Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
    #[inline(always)]
    pub const fn mr4(&self) -> &Mr4 {
        &self.mr4
    }
    #[doc = "0x40 - Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
    #[inline(always)]
    pub const fn mr5(&self) -> &Mr4 {
        &self.mr5
    }
    #[doc = "0x40 - Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
    #[inline(always)]
    pub const fn mr6(&self) -> &Mr4 {
        &self.mr6
    }
    #[doc = "0x4c - PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs."]
    #[inline(always)]
    pub const fn pcr(&self) -> &Pcr {
        &self.pcr
    }
    #[doc = "0x50 - Load Enable Register. Enables use of updated PWM match values."]
    #[inline(always)]
    pub const fn ler(&self) -> &Ler {
        &self.ler
    }
    #[doc = "0x70 - Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting."]
    #[inline(always)]
    pub const fn ctcr(&self) -> &Ctcr {
        &self.ctcr
    }
}
#[doc = "IR (rw) register accessor: Interrupt Register. The IR can be written to clear interrupts, or read to identify which PWM interrupt sources are pending.\n\nYou can [`read`](crate::Reg::read) this register and get [`ir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir`] module"]
#[doc(alias = "IR")]
pub type Ir = crate::Reg<ir::IrSpec>;
#[doc = "Interrupt Register. The IR can be written to clear interrupts, or read to identify which PWM interrupt sources are pending."]
pub mod ir;
#[doc = "TCR (rw) register accessor: Timer Control Register. The TCR is used to control the Timer Counter functions.\n\nYou can [`read`](crate::Reg::read) this register and get [`tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tcr`] module"]
#[doc(alias = "TCR")]
pub type Tcr = crate::Reg<tcr::TcrSpec>;
#[doc = "Timer Control Register. The TCR is used to control the Timer Counter functions."]
pub mod tcr;
#[doc = "TC (rw) register accessor: Timer Counter. The 32 bit TC is incremented every PR+1 cycles of PCLK. The TC is controlled through the TCR.\n\nYou can [`read`](crate::Reg::read) this register and get [`tc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tc`] module"]
#[doc(alias = "TC")]
pub type Tc = crate::Reg<tc::TcSpec>;
#[doc = "Timer Counter. The 32 bit TC is incremented every PR+1 cycles of PCLK. The TC is controlled through the TCR."]
pub mod tc;
#[doc = "PR (rw) register accessor: Prescale Register. Determines how often the PWM counter is incremented.\n\nYou can [`read`](crate::Reg::read) this register and get [`pr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr`] module"]
#[doc(alias = "PR")]
pub type Pr = crate::Reg<pr::PrSpec>;
#[doc = "Prescale Register. Determines how often the PWM counter is incremented."]
pub mod pr;
#[doc = "PC (rw) register accessor: Prescale Counter. Prescaler for the main PWM counter.\n\nYou can [`read`](crate::Reg::read) this register and get [`pc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pc`] module"]
#[doc(alias = "PC")]
pub type Pc = crate::Reg<pc::PcSpec>;
#[doc = "Prescale Counter. Prescaler for the main PWM counter."]
pub mod pc;
#[doc = "MCR (rw) register accessor: Match Control Register. The MCR is used to control whether an interrupt is generated and if the PWM counter is reset when a Match occurs.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`] module"]
#[doc(alias = "MCR")]
pub type Mcr = crate::Reg<mcr::McrSpec>;
#[doc = "Match Control Register. The MCR is used to control whether an interrupt is generated and if the PWM counter is reset when a Match occurs."]
pub mod mcr;
#[doc = "MR (rw) register accessor: Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges.\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`] module"]
#[doc(alias = "MR")]
pub type Mr = crate::Reg<mr::MrSpec>;
#[doc = "Match Register. Match registers are continuously compared to the PWM counter in order to control PWM output edges."]
pub mod mr;
#[doc = "CCR (rw) register accessor: Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated for a capture event.\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`] module"]
#[doc(alias = "CCR")]
pub type Ccr = crate::Reg<ccr::CcrSpec>;
#[doc = "Capture Control Register. The CCR controls which edges of the capture inputs are used to load the Capture Registers and whether or not an interrupt is generated for a capture event."]
pub mod ccr;
#[doc = "CR (rw) register accessor: PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs.\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs."]
pub mod cr;
pub use Mr as Mr4;
pub use mr as mr4;
#[doc = "PCR (rw) register accessor: PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs.\n\nYou can [`read`](crate::Reg::read) this register and get [`pcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcr`] module"]
#[doc(alias = "PCR")]
pub type Pcr = crate::Reg<pcr::PcrSpec>;
#[doc = "PWM Control Register. Enables PWM outputs and selects either single edge or double edge controlled PWM outputs."]
pub mod pcr;
#[doc = "LER (rw) register accessor: Load Enable Register. Enables use of updated PWM match values.\n\nYou can [`read`](crate::Reg::read) this register and get [`ler::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ler::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ler`] module"]
#[doc(alias = "LER")]
pub type Ler = crate::Reg<ler::LerSpec>;
#[doc = "Load Enable Register. Enables use of updated PWM match values."]
pub mod ler;
#[doc = "CTCR (rw) register accessor: Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting.\n\nYou can [`read`](crate::Reg::read) this register and get [`ctcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctcr`] module"]
#[doc(alias = "CTCR")]
pub type Ctcr = crate::Reg<ctcr::CtcrSpec>;
#[doc = "Count Control Register. The CTCR selects between Timer and Counter mode, and in Counter mode selects the signal and edge(s) for counting."]
pub mod ctcr;
