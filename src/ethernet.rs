#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mac1: Mac1,
    mac2: Mac2,
    ipgt: Ipgt,
    ipgr: Ipgr,
    clrt: Clrt,
    maxf: Maxf,
    supp: Supp,
    test: Test,
    mcfg: Mcfg,
    mcmd: Mcmd,
    madr: Madr,
    mwtd: Mwtd,
    mrdd: Mrdd,
    mind: Mind,
    _reserved14: [u8; 0x08],
    sa0: Sa0,
    sa1: Sa1,
    sa2: Sa2,
    _reserved17: [u8; 0xb4],
    command: Command,
    status: Status,
    rxdescriptor: Rxdescriptor,
    rxstatus: Rxstatus,
    rxdescriptornumber: Rxdescriptornumber,
    rxproduceindex: Rxproduceindex,
    rxconsumeindex: Rxconsumeindex,
    txdescriptor: Txdescriptor,
    txstatus: Txstatus,
    txdescriptornumber: Txdescriptornumber,
    txproduceindex: Txproduceindex,
    txconsumeindex: Txconsumeindex,
    _reserved29: [u8; 0x28],
    tsv0: Tsv0,
    tsv1: Tsv1,
    rsv: Rsv,
    _reserved32: [u8; 0x0c],
    flowcontrolcounter: Flowcontrolcounter,
    flowcontrolstatus: Flowcontrolstatus,
    _reserved34: [u8; 0x88],
    rxfilterctrl: Rxfilterctrl,
    rxfilterwolstatus: Rxfilterwolstatus,
    rxfilterwolclear: Rxfilterwolclear,
    _reserved37: [u8; 0x04],
    hashfilterl: Hashfilterl,
    hashfilterh: Hashfilterh,
    _reserved39: [u8; 0x0dc8],
    intstatus: Intstatus,
    intenable: Intenable,
    intclear: Intclear,
    intset: Intset,
    _reserved43: [u8; 0x04],
    powerdown: Powerdown,
}
impl RegisterBlock {
    #[doc = "0x00 - MAC configuration register 1."]
    #[inline(always)]
    pub const fn mac1(&self) -> &Mac1 {
        &self.mac1
    }
    #[doc = "0x04 - MAC configuration register 2."]
    #[inline(always)]
    pub const fn mac2(&self) -> &Mac2 {
        &self.mac2
    }
    #[doc = "0x08 - Back-to-Back Inter-Packet-Gap register."]
    #[inline(always)]
    pub const fn ipgt(&self) -> &Ipgt {
        &self.ipgt
    }
    #[doc = "0x0c - Non Back-to-Back Inter-Packet-Gap register."]
    #[inline(always)]
    pub const fn ipgr(&self) -> &Ipgr {
        &self.ipgr
    }
    #[doc = "0x10 - Collision window / Retry register."]
    #[inline(always)]
    pub const fn clrt(&self) -> &Clrt {
        &self.clrt
    }
    #[doc = "0x14 - Maximum Frame register."]
    #[inline(always)]
    pub const fn maxf(&self) -> &Maxf {
        &self.maxf
    }
    #[doc = "0x18 - PHY Support register."]
    #[inline(always)]
    pub const fn supp(&self) -> &Supp {
        &self.supp
    }
    #[doc = "0x1c - Test register."]
    #[inline(always)]
    pub const fn test(&self) -> &Test {
        &self.test
    }
    #[doc = "0x20 - MII Mgmt Configuration register."]
    #[inline(always)]
    pub const fn mcfg(&self) -> &Mcfg {
        &self.mcfg
    }
    #[doc = "0x24 - MII Mgmt Command register."]
    #[inline(always)]
    pub const fn mcmd(&self) -> &Mcmd {
        &self.mcmd
    }
    #[doc = "0x28 - MII Mgmt Address register."]
    #[inline(always)]
    pub const fn madr(&self) -> &Madr {
        &self.madr
    }
    #[doc = "0x2c - MII Mgmt Write Data register."]
    #[inline(always)]
    pub const fn mwtd(&self) -> &Mwtd {
        &self.mwtd
    }
    #[doc = "0x30 - MII Mgmt Read Data register."]
    #[inline(always)]
    pub const fn mrdd(&self) -> &Mrdd {
        &self.mrdd
    }
    #[doc = "0x34 - MII Mgmt Indicators register."]
    #[inline(always)]
    pub const fn mind(&self) -> &Mind {
        &self.mind
    }
    #[doc = "0x40 - Station Address 0 register."]
    #[inline(always)]
    pub const fn sa0(&self) -> &Sa0 {
        &self.sa0
    }
    #[doc = "0x44 - Station Address 1 register."]
    #[inline(always)]
    pub const fn sa1(&self) -> &Sa1 {
        &self.sa1
    }
    #[doc = "0x48 - Station Address 2 register."]
    #[inline(always)]
    pub const fn sa2(&self) -> &Sa2 {
        &self.sa2
    }
    #[doc = "0x100 - Command register."]
    #[inline(always)]
    pub const fn command(&self) -> &Command {
        &self.command
    }
    #[doc = "0x104 - Status register."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x108 - Receive descriptor base address register."]
    #[inline(always)]
    pub const fn rxdescriptor(&self) -> &Rxdescriptor {
        &self.rxdescriptor
    }
    #[doc = "0x10c - Receive status base address register."]
    #[inline(always)]
    pub const fn rxstatus(&self) -> &Rxstatus {
        &self.rxstatus
    }
    #[doc = "0x110 - Receive number of descriptors register."]
    #[inline(always)]
    pub const fn rxdescriptornumber(&self) -> &Rxdescriptornumber {
        &self.rxdescriptornumber
    }
    #[doc = "0x114 - Receive produce index register."]
    #[inline(always)]
    pub const fn rxproduceindex(&self) -> &Rxproduceindex {
        &self.rxproduceindex
    }
    #[doc = "0x118 - Receive consume index register."]
    #[inline(always)]
    pub const fn rxconsumeindex(&self) -> &Rxconsumeindex {
        &self.rxconsumeindex
    }
    #[doc = "0x11c - Transmit descriptor base address register."]
    #[inline(always)]
    pub const fn txdescriptor(&self) -> &Txdescriptor {
        &self.txdescriptor
    }
    #[doc = "0x120 - Transmit status base address register."]
    #[inline(always)]
    pub const fn txstatus(&self) -> &Txstatus {
        &self.txstatus
    }
    #[doc = "0x124 - Transmit number of descriptors register."]
    #[inline(always)]
    pub const fn txdescriptornumber(&self) -> &Txdescriptornumber {
        &self.txdescriptornumber
    }
    #[doc = "0x128 - Transmit produce index register."]
    #[inline(always)]
    pub const fn txproduceindex(&self) -> &Txproduceindex {
        &self.txproduceindex
    }
    #[doc = "0x12c - Transmit consume index register."]
    #[inline(always)]
    pub const fn txconsumeindex(&self) -> &Txconsumeindex {
        &self.txconsumeindex
    }
    #[doc = "0x158 - Transmit status vector 0 register."]
    #[inline(always)]
    pub const fn tsv0(&self) -> &Tsv0 {
        &self.tsv0
    }
    #[doc = "0x15c - Transmit status vector 1 register."]
    #[inline(always)]
    pub const fn tsv1(&self) -> &Tsv1 {
        &self.tsv1
    }
    #[doc = "0x160 - Receive status vector register."]
    #[inline(always)]
    pub const fn rsv(&self) -> &Rsv {
        &self.rsv
    }
    #[doc = "0x170 - Flow control counter register."]
    #[inline(always)]
    pub const fn flowcontrolcounter(&self) -> &Flowcontrolcounter {
        &self.flowcontrolcounter
    }
    #[doc = "0x174 - Flow control status register."]
    #[inline(always)]
    pub const fn flowcontrolstatus(&self) -> &Flowcontrolstatus {
        &self.flowcontrolstatus
    }
    #[doc = "0x200 - Receive filter control register."]
    #[inline(always)]
    pub const fn rxfilterctrl(&self) -> &Rxfilterctrl {
        &self.rxfilterctrl
    }
    #[doc = "0x204 - Receive filter WoL status register."]
    #[inline(always)]
    pub const fn rxfilterwolstatus(&self) -> &Rxfilterwolstatus {
        &self.rxfilterwolstatus
    }
    #[doc = "0x208 - Receive filter WoL clear register."]
    #[inline(always)]
    pub const fn rxfilterwolclear(&self) -> &Rxfilterwolclear {
        &self.rxfilterwolclear
    }
    #[doc = "0x210 - Hash filter table LSBs register."]
    #[inline(always)]
    pub const fn hashfilterl(&self) -> &Hashfilterl {
        &self.hashfilterl
    }
    #[doc = "0x214 - Hash filter table MSBs register."]
    #[inline(always)]
    pub const fn hashfilterh(&self) -> &Hashfilterh {
        &self.hashfilterh
    }
    #[doc = "0xfe0 - Interrupt status register."]
    #[inline(always)]
    pub const fn intstatus(&self) -> &Intstatus {
        &self.intstatus
    }
    #[doc = "0xfe4 - Interrupt enable register."]
    #[inline(always)]
    pub const fn intenable(&self) -> &Intenable {
        &self.intenable
    }
    #[doc = "0xfe8 - Interrupt clear register."]
    #[inline(always)]
    pub const fn intclear(&self) -> &Intclear {
        &self.intclear
    }
    #[doc = "0xfec - Interrupt set register."]
    #[inline(always)]
    pub const fn intset(&self) -> &Intset {
        &self.intset
    }
    #[doc = "0xff4 - Power-down register."]
    #[inline(always)]
    pub const fn powerdown(&self) -> &Powerdown {
        &self.powerdown
    }
}
#[doc = "MAC1 (rw) register accessor: MAC configuration register 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`mac1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac1`] module"]
#[doc(alias = "MAC1")]
pub type Mac1 = crate::Reg<mac1::Mac1Spec>;
#[doc = "MAC configuration register 1."]
pub mod mac1;
#[doc = "MAC2 (rw) register accessor: MAC configuration register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`mac2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mac2`] module"]
#[doc(alias = "MAC2")]
pub type Mac2 = crate::Reg<mac2::Mac2Spec>;
#[doc = "MAC configuration register 2."]
pub mod mac2;
#[doc = "IPGT (rw) register accessor: Back-to-Back Inter-Packet-Gap register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ipgt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipgt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipgt`] module"]
#[doc(alias = "IPGT")]
pub type Ipgt = crate::Reg<ipgt::IpgtSpec>;
#[doc = "Back-to-Back Inter-Packet-Gap register."]
pub mod ipgt;
#[doc = "IPGR (rw) register accessor: Non Back-to-Back Inter-Packet-Gap register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ipgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipgr`] module"]
#[doc(alias = "IPGR")]
pub type Ipgr = crate::Reg<ipgr::IpgrSpec>;
#[doc = "Non Back-to-Back Inter-Packet-Gap register."]
pub mod ipgr;
#[doc = "CLRT (rw) register accessor: Collision window / Retry register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clrt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clrt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clrt`] module"]
#[doc(alias = "CLRT")]
pub type Clrt = crate::Reg<clrt::ClrtSpec>;
#[doc = "Collision window / Retry register."]
pub mod clrt;
#[doc = "MAXF (rw) register accessor: Maximum Frame register.\n\nYou can [`read`](crate::Reg::read) this register and get [`maxf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maxf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maxf`] module"]
#[doc(alias = "MAXF")]
pub type Maxf = crate::Reg<maxf::MaxfSpec>;
#[doc = "Maximum Frame register."]
pub mod maxf;
#[doc = "SUPP (rw) register accessor: PHY Support register.\n\nYou can [`read`](crate::Reg::read) this register and get [`supp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`supp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@supp`] module"]
#[doc(alias = "SUPP")]
pub type Supp = crate::Reg<supp::SuppSpec>;
#[doc = "PHY Support register."]
pub mod supp;
#[doc = "TEST (rw) register accessor: Test register.\n\nYou can [`read`](crate::Reg::read) this register and get [`test::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@test`] module"]
#[doc(alias = "TEST")]
pub type Test = crate::Reg<test::TestSpec>;
#[doc = "Test register."]
pub mod test;
#[doc = "MCFG (rw) register accessor: MII Mgmt Configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcfg`] module"]
#[doc(alias = "MCFG")]
pub type Mcfg = crate::Reg<mcfg::McfgSpec>;
#[doc = "MII Mgmt Configuration register."]
pub mod mcfg;
#[doc = "MCMD (rw) register accessor: MII Mgmt Command register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mcmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcmd`] module"]
#[doc(alias = "MCMD")]
pub type Mcmd = crate::Reg<mcmd::McmdSpec>;
#[doc = "MII Mgmt Command register."]
pub mod mcmd;
#[doc = "MADR (rw) register accessor: MII Mgmt Address register.\n\nYou can [`read`](crate::Reg::read) this register and get [`madr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`madr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@madr`] module"]
#[doc(alias = "MADR")]
pub type Madr = crate::Reg<madr::MadrSpec>;
#[doc = "MII Mgmt Address register."]
pub mod madr;
#[doc = "MWTD (w) register accessor: MII Mgmt Write Data register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mwtd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mwtd`] module"]
#[doc(alias = "MWTD")]
pub type Mwtd = crate::Reg<mwtd::MwtdSpec>;
#[doc = "MII Mgmt Write Data register."]
pub mod mwtd;
#[doc = "MRDD (r) register accessor: MII Mgmt Read Data register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mrdd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mrdd`] module"]
#[doc(alias = "MRDD")]
pub type Mrdd = crate::Reg<mrdd::MrddSpec>;
#[doc = "MII Mgmt Read Data register."]
pub mod mrdd;
#[doc = "MIND (r) register accessor: MII Mgmt Indicators register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mind::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mind`] module"]
#[doc(alias = "MIND")]
pub type Mind = crate::Reg<mind::MindSpec>;
#[doc = "MII Mgmt Indicators register."]
pub mod mind;
#[doc = "SA0 (rw) register accessor: Station Address 0 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sa0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa0`] module"]
#[doc(alias = "SA0")]
pub type Sa0 = crate::Reg<sa0::Sa0Spec>;
#[doc = "Station Address 0 register."]
pub mod sa0;
#[doc = "SA1 (rw) register accessor: Station Address 1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sa1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa1`] module"]
#[doc(alias = "SA1")]
pub type Sa1 = crate::Reg<sa1::Sa1Spec>;
#[doc = "Station Address 1 register."]
pub mod sa1;
#[doc = "SA2 (rw) register accessor: Station Address 2 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`sa2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sa2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sa2`] module"]
#[doc(alias = "SA2")]
pub type Sa2 = crate::Reg<sa2::Sa2Spec>;
#[doc = "Station Address 2 register."]
pub mod sa2;
#[doc = "COMMAND (rw) register accessor: Command register.\n\nYou can [`read`](crate::Reg::read) this register and get [`command::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@command`] module"]
#[doc(alias = "COMMAND")]
pub type Command = crate::Reg<command::CommandSpec>;
#[doc = "Command register."]
pub mod command;
#[doc = "STATUS (r) register accessor: Status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status register."]
pub mod status;
#[doc = "RXDESCRIPTOR (rw) register accessor: Receive descriptor base address register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdescriptor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdescriptor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdescriptor`] module"]
#[doc(alias = "RXDESCRIPTOR")]
pub type Rxdescriptor = crate::Reg<rxdescriptor::RxdescriptorSpec>;
#[doc = "Receive descriptor base address register."]
pub mod rxdescriptor;
#[doc = "RXSTATUS (rw) register accessor: Receive status base address register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxstatus`] module"]
#[doc(alias = "RXSTATUS")]
pub type Rxstatus = crate::Reg<rxstatus::RxstatusSpec>;
#[doc = "Receive status base address register."]
pub mod rxstatus;
#[doc = "RXDESCRIPTORNUMBER (rw) register accessor: Receive number of descriptors register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxdescriptornumber::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxdescriptornumber::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxdescriptornumber`] module"]
#[doc(alias = "RXDESCRIPTORNUMBER")]
pub type Rxdescriptornumber = crate::Reg<rxdescriptornumber::RxdescriptornumberSpec>;
#[doc = "Receive number of descriptors register."]
pub mod rxdescriptornumber;
#[doc = "RXPRODUCEINDEX (r) register accessor: Receive produce index register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxproduceindex::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxproduceindex`] module"]
#[doc(alias = "RXPRODUCEINDEX")]
pub type Rxproduceindex = crate::Reg<rxproduceindex::RxproduceindexSpec>;
#[doc = "Receive produce index register."]
pub mod rxproduceindex;
#[doc = "RXCONSUMEINDEX (rw) register accessor: Receive consume index register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxconsumeindex::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxconsumeindex::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxconsumeindex`] module"]
#[doc(alias = "RXCONSUMEINDEX")]
pub type Rxconsumeindex = crate::Reg<rxconsumeindex::RxconsumeindexSpec>;
#[doc = "Receive consume index register."]
pub mod rxconsumeindex;
#[doc = "TXDESCRIPTOR (rw) register accessor: Transmit descriptor base address register.\n\nYou can [`read`](crate::Reg::read) this register and get [`txdescriptor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdescriptor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdescriptor`] module"]
#[doc(alias = "TXDESCRIPTOR")]
pub type Txdescriptor = crate::Reg<txdescriptor::TxdescriptorSpec>;
#[doc = "Transmit descriptor base address register."]
pub mod txdescriptor;
#[doc = "TXSTATUS (rw) register accessor: Transmit status base address register.\n\nYou can [`read`](crate::Reg::read) this register and get [`txstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txstatus`] module"]
#[doc(alias = "TXSTATUS")]
pub type Txstatus = crate::Reg<txstatus::TxstatusSpec>;
#[doc = "Transmit status base address register."]
pub mod txstatus;
#[doc = "TXDESCRIPTORNUMBER (rw) register accessor: Transmit number of descriptors register.\n\nYou can [`read`](crate::Reg::read) this register and get [`txdescriptornumber::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txdescriptornumber::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txdescriptornumber`] module"]
#[doc(alias = "TXDESCRIPTORNUMBER")]
pub type Txdescriptornumber = crate::Reg<txdescriptornumber::TxdescriptornumberSpec>;
#[doc = "Transmit number of descriptors register."]
pub mod txdescriptornumber;
#[doc = "TXPRODUCEINDEX (rw) register accessor: Transmit produce index register.\n\nYou can [`read`](crate::Reg::read) this register and get [`txproduceindex::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txproduceindex::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txproduceindex`] module"]
#[doc(alias = "TXPRODUCEINDEX")]
pub type Txproduceindex = crate::Reg<txproduceindex::TxproduceindexSpec>;
#[doc = "Transmit produce index register."]
pub mod txproduceindex;
#[doc = "TXCONSUMEINDEX (r) register accessor: Transmit consume index register.\n\nYou can [`read`](crate::Reg::read) this register and get [`txconsumeindex::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txconsumeindex`] module"]
#[doc(alias = "TXCONSUMEINDEX")]
pub type Txconsumeindex = crate::Reg<txconsumeindex::TxconsumeindexSpec>;
#[doc = "Transmit consume index register."]
pub mod txconsumeindex;
#[doc = "TSV0 (r) register accessor: Transmit status vector 0 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`tsv0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsv0`] module"]
#[doc(alias = "TSV0")]
pub type Tsv0 = crate::Reg<tsv0::Tsv0Spec>;
#[doc = "Transmit status vector 0 register."]
pub mod tsv0;
#[doc = "TSV1 (r) register accessor: Transmit status vector 1 register.\n\nYou can [`read`](crate::Reg::read) this register and get [`tsv1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tsv1`] module"]
#[doc(alias = "TSV1")]
pub type Tsv1 = crate::Reg<tsv1::Tsv1Spec>;
#[doc = "Transmit status vector 1 register."]
pub mod tsv1;
#[doc = "RSV (r) register accessor: Receive status vector register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rsv::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsv`] module"]
#[doc(alias = "RSV")]
pub type Rsv = crate::Reg<rsv::RsvSpec>;
#[doc = "Receive status vector register."]
pub mod rsv;
#[doc = "FLOWCONTROLCOUNTER (rw) register accessor: Flow control counter register.\n\nYou can [`read`](crate::Reg::read) this register and get [`flowcontrolcounter::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flowcontrolcounter::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flowcontrolcounter`] module"]
#[doc(alias = "FLOWCONTROLCOUNTER")]
pub type Flowcontrolcounter = crate::Reg<flowcontrolcounter::FlowcontrolcounterSpec>;
#[doc = "Flow control counter register."]
pub mod flowcontrolcounter;
#[doc = "FLOWCONTROLSTATUS (r) register accessor: Flow control status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`flowcontrolstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@flowcontrolstatus`] module"]
#[doc(alias = "FLOWCONTROLSTATUS")]
pub type Flowcontrolstatus = crate::Reg<flowcontrolstatus::FlowcontrolstatusSpec>;
#[doc = "Flow control status register."]
pub mod flowcontrolstatus;
#[doc = "RXFILTERCTRL (rw) register accessor: Receive filter control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfilterctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxfilterctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfilterctrl`] module"]
#[doc(alias = "RXFILTERCTRL")]
pub type Rxfilterctrl = crate::Reg<rxfilterctrl::RxfilterctrlSpec>;
#[doc = "Receive filter control register."]
pub mod rxfilterctrl;
#[doc = "RXFILTERWOLSTATUS (r) register accessor: Receive filter WoL status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rxfilterwolstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfilterwolstatus`] module"]
#[doc(alias = "RXFILTERWOLSTATUS")]
pub type Rxfilterwolstatus = crate::Reg<rxfilterwolstatus::RxfilterwolstatusSpec>;
#[doc = "Receive filter WoL status register."]
pub mod rxfilterwolstatus;
#[doc = "RXFILTERWOLCLEAR (w) register accessor: Receive filter WoL clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rxfilterwolclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rxfilterwolclear`] module"]
#[doc(alias = "RXFILTERWOLCLEAR")]
pub type Rxfilterwolclear = crate::Reg<rxfilterwolclear::RxfilterwolclearSpec>;
#[doc = "Receive filter WoL clear register."]
pub mod rxfilterwolclear;
#[doc = "HASHFILTERL (rw) register accessor: Hash filter table LSBs register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hashfilterl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashfilterl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashfilterl`] module"]
#[doc(alias = "HASHFILTERL")]
pub type Hashfilterl = crate::Reg<hashfilterl::HashfilterlSpec>;
#[doc = "Hash filter table LSBs register."]
pub mod hashfilterl;
#[doc = "HASHFILTERH (rw) register accessor: Hash filter table MSBs register.\n\nYou can [`read`](crate::Reg::read) this register and get [`hashfilterh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hashfilterh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hashfilterh`] module"]
#[doc(alias = "HASHFILTERH")]
pub type Hashfilterh = crate::Reg<hashfilterh::HashfilterhSpec>;
#[doc = "Hash filter table MSBs register."]
pub mod hashfilterh;
#[doc = "INTSTATUS (r) register accessor: Interrupt status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intstatus::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intstatus`] module"]
#[doc(alias = "INTSTATUS")]
pub type Intstatus = crate::Reg<intstatus::IntstatusSpec>;
#[doc = "Interrupt status register."]
pub mod intstatus;
#[doc = "INTENABLE (rw) register accessor: Interrupt enable register.\n\nYou can [`read`](crate::Reg::read) this register and get [`intenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenable`] module"]
#[doc(alias = "INTENABLE")]
pub type Intenable = crate::Reg<intenable::IntenableSpec>;
#[doc = "Interrupt enable register."]
pub mod intenable;
#[doc = "INTCLEAR (w) register accessor: Interrupt clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intclear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intclear`] module"]
#[doc(alias = "INTCLEAR")]
pub type Intclear = crate::Reg<intclear::IntclearSpec>;
#[doc = "Interrupt clear register."]
pub mod intclear;
#[doc = "INTSET (w) register accessor: Interrupt set register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intset::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intset`] module"]
#[doc(alias = "INTSET")]
pub type Intset = crate::Reg<intset::IntsetSpec>;
#[doc = "Interrupt set register."]
pub mod intset;
#[doc = "POWERDOWN (rw) register accessor: Power-down register.\n\nYou can [`read`](crate::Reg::read) this register and get [`powerdown::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`powerdown::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@powerdown`] module"]
#[doc(alias = "POWERDOWN")]
pub type Powerdown = crate::Reg<powerdown::PowerdownSpec>;
#[doc = "Power-down register."]
pub mod powerdown;
