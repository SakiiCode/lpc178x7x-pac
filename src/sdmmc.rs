#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    pwr: Pwr,
    clock: Clock,
    argument: Argument,
    command: Command,
    respcmd: Respcmd,
    response: [Response; 4],
    datatimer: Datatimer,
    datalength: Datalength,
    datactrl: Datactrl,
    datacnt: Datacnt,
    status: Status,
    clear: Clear,
    mask0: Mask0,
    _reserved13: [u8; 0x08],
    fifocnt: Fifocnt,
    _reserved14: [u8; 0x34],
    fifo: [Fifo; 16],
}
impl RegisterBlock {
    #[doc = "0x00 - Power control register."]
    #[inline(always)]
    pub const fn pwr(&self) -> &Pwr {
        &self.pwr
    }
    #[doc = "0x04 - Clock control register."]
    #[inline(always)]
    pub const fn clock(&self) -> &Clock {
        &self.clock
    }
    #[doc = "0x08 - Argument register."]
    #[inline(always)]
    pub const fn argument(&self) -> &Argument {
        &self.argument
    }
    #[doc = "0x0c - Command register."]
    #[inline(always)]
    pub const fn command(&self) -> &Command {
        &self.command
    }
    #[doc = "0x10 - Response command register."]
    #[inline(always)]
    pub const fn respcmd(&self) -> &Respcmd {
        &self.respcmd
    }
    #[doc = "0x14..0x24 - Response register."]
    #[inline(always)]
    pub const fn response(&self, n: usize) -> &Response {
        &self.response[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x14..0x24 - Response register."]
    #[inline(always)]
    pub fn response_iter(&self) -> impl Iterator<Item = &Response> {
        self.response.iter()
    }
    #[doc = "0x24 - Data Timer."]
    #[inline(always)]
    pub const fn datatimer(&self) -> &Datatimer {
        &self.datatimer
    }
    #[doc = "0x28 - Data length register."]
    #[inline(always)]
    pub const fn datalength(&self) -> &Datalength {
        &self.datalength
    }
    #[doc = "0x2c - Data control register."]
    #[inline(always)]
    pub const fn datactrl(&self) -> &Datactrl {
        &self.datactrl
    }
    #[doc = "0x30 - Data counter."]
    #[inline(always)]
    pub const fn datacnt(&self) -> &Datacnt {
        &self.datacnt
    }
    #[doc = "0x34 - Status register."]
    #[inline(always)]
    pub const fn status(&self) -> &Status {
        &self.status
    }
    #[doc = "0x38 - Clear register."]
    #[inline(always)]
    pub const fn clear(&self) -> &Clear {
        &self.clear
    }
    #[doc = "0x3c - Interrupt 0 mask register."]
    #[inline(always)]
    pub const fn mask0(&self) -> &Mask0 {
        &self.mask0
    }
    #[doc = "0x48 - FIFO Counter."]
    #[inline(always)]
    pub const fn fifocnt(&self) -> &Fifocnt {
        &self.fifocnt
    }
    #[doc = "0x80..0xc0 - Data FIFO Register."]
    #[inline(always)]
    pub const fn fifo(&self, n: usize) -> &Fifo {
        &self.fifo[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0xc0 - Data FIFO Register."]
    #[inline(always)]
    pub fn fifo_iter(&self) -> impl Iterator<Item = &Fifo> {
        self.fifo.iter()
    }
}
#[doc = "PWR (rw) register accessor: Power control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`pwr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwr`] module"]
#[doc(alias = "PWR")]
pub type Pwr = crate::Reg<pwr::PwrSpec>;
#[doc = "Power control register."]
pub mod pwr;
#[doc = "CLOCK (rw) register accessor: Clock control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`clock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clock`] module"]
#[doc(alias = "CLOCK")]
pub type Clock = crate::Reg<clock::ClockSpec>;
#[doc = "Clock control register."]
pub mod clock;
#[doc = "ARGUMENT (rw) register accessor: Argument register.\n\nYou can [`read`](crate::Reg::read) this register and get [`argument::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`argument::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@argument`] module"]
#[doc(alias = "ARGUMENT")]
pub type Argument = crate::Reg<argument::ArgumentSpec>;
#[doc = "Argument register."]
pub mod argument;
#[doc = "COMMAND (rw) register accessor: Command register.\n\nYou can [`read`](crate::Reg::read) this register and get [`command::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`command::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@command`] module"]
#[doc(alias = "COMMAND")]
pub type Command = crate::Reg<command::CommandSpec>;
#[doc = "Command register."]
pub mod command;
#[doc = "RESPCMD (r) register accessor: Response command register.\n\nYou can [`read`](crate::Reg::read) this register and get [`respcmd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@respcmd`] module"]
#[doc(alias = "RESPCMD")]
pub type Respcmd = crate::Reg<respcmd::RespcmdSpec>;
#[doc = "Response command register."]
pub mod respcmd;
#[doc = "RESPONSE (r) register accessor: Response register.\n\nYou can [`read`](crate::Reg::read) this register and get [`response::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@response`] module"]
#[doc(alias = "RESPONSE")]
pub type Response = crate::Reg<response::ResponseSpec>;
#[doc = "Response register."]
pub mod response;
#[doc = "DATATIMER (rw) register accessor: Data Timer.\n\nYou can [`read`](crate::Reg::read) this register and get [`datatimer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datatimer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datatimer`] module"]
#[doc(alias = "DATATIMER")]
pub type Datatimer = crate::Reg<datatimer::DatatimerSpec>;
#[doc = "Data Timer."]
pub mod datatimer;
#[doc = "DATALENGTH (rw) register accessor: Data length register.\n\nYou can [`read`](crate::Reg::read) this register and get [`datalength::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datalength::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datalength`] module"]
#[doc(alias = "DATALENGTH")]
pub type Datalength = crate::Reg<datalength::DatalengthSpec>;
#[doc = "Data length register."]
pub mod datalength;
#[doc = "DATACTRL (rw) register accessor: Data control register.\n\nYou can [`read`](crate::Reg::read) this register and get [`datactrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datactrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datactrl`] module"]
#[doc(alias = "DATACTRL")]
pub type Datactrl = crate::Reg<datactrl::DatactrlSpec>;
#[doc = "Data control register."]
pub mod datactrl;
#[doc = "DATACNT (r) register accessor: Data counter.\n\nYou can [`read`](crate::Reg::read) this register and get [`datacnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@datacnt`] module"]
#[doc(alias = "DATACNT")]
pub type Datacnt = crate::Reg<datacnt::DatacntSpec>;
#[doc = "Data counter."]
pub mod datacnt;
#[doc = "STATUS (r) register accessor: Status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`status::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@status`] module"]
#[doc(alias = "STATUS")]
pub type Status = crate::Reg<status::StatusSpec>;
#[doc = "Status register."]
pub mod status;
#[doc = "CLEAR (w) register accessor: Clear register.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear`] module"]
#[doc(alias = "CLEAR")]
pub type Clear = crate::Reg<clear::ClearSpec>;
#[doc = "Clear register."]
pub mod clear;
#[doc = "MASK0 (rw) register accessor: Interrupt 0 mask register.\n\nYou can [`read`](crate::Reg::read) this register and get [`mask0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mask0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mask0`] module"]
#[doc(alias = "MASK0")]
pub type Mask0 = crate::Reg<mask0::Mask0Spec>;
#[doc = "Interrupt 0 mask register."]
pub mod mask0;
#[doc = "FIFOCNT (r) register accessor: FIFO Counter.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifocnt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifocnt`] module"]
#[doc(alias = "FIFOCNT")]
pub type Fifocnt = crate::Reg<fifocnt::FifocntSpec>;
#[doc = "FIFO Counter."]
pub mod fifocnt;
#[doc = "FIFO (rw) register accessor: Data FIFO Register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fifo`] module"]
#[doc(alias = "FIFO")]
pub type Fifo = crate::Reg<fifo::FifoSpec>;
#[doc = "Data FIFO Register."]
pub mod fifo;
