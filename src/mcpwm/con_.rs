#[doc = "Register `CON` reader"]
pub type R = crate::R<ConSpec>;
#[doc = "Stops/starts timer channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Run0 {
    #[doc = "0: Stop."]
    Stop_ = 0,
    #[doc = "1: Run."]
    Run_ = 1,
}
impl From<Run0> for bool {
    #[inline(always)]
    fn from(variant: Run0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUN0` reader - Stops/starts timer channel 0."]
pub type Run0R = crate::BitReader<Run0>;
impl Run0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Run0 {
        match self.bits {
            false => Run0::Stop_,
            true => Run0::Run_,
        }
    }
    #[doc = "Stop."]
    #[inline(always)]
    pub fn is_stop_(&self) -> bool {
        *self == Run0::Stop_
    }
    #[doc = "Run."]
    #[inline(always)]
    pub fn is_run_(&self) -> bool {
        *self == Run0::Run_
    }
}
#[doc = "Edge/center aligned operation for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Center0 {
    #[doc = "0: Edge-aligned."]
    EdgeAligned_ = 0,
    #[doc = "1: Center-aligned."]
    CenterAligned_ = 1,
}
impl From<Center0> for bool {
    #[inline(always)]
    fn from(variant: Center0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CENTER0` reader - Edge/center aligned operation for channel 0."]
pub type Center0R = crate::BitReader<Center0>;
impl Center0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Center0 {
        match self.bits {
            false => Center0::EdgeAligned_,
            true => Center0::CenterAligned_,
        }
    }
    #[doc = "Edge-aligned."]
    #[inline(always)]
    pub fn is_edge_aligned_(&self) -> bool {
        *self == Center0::EdgeAligned_
    }
    #[doc = "Center-aligned."]
    #[inline(always)]
    pub fn is_center_aligned_(&self) -> bool {
        *self == Center0::CenterAligned_
    }
}
#[doc = "Selects polarity of the MCOA0 and MCOB0 pins.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pola0 {
    #[doc = "0: Passive state is LOW, active state is HIGH."]
    PassiveStateIsLow = 0,
    #[doc = "1: Passive state is HIGH, active state is LOW."]
    PassiveStateIsHig = 1,
}
impl From<Pola0> for bool {
    #[inline(always)]
    fn from(variant: Pola0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POLA0` reader - Selects polarity of the MCOA0 and MCOB0 pins."]
pub type Pola0R = crate::BitReader<Pola0>;
impl Pola0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pola0 {
        match self.bits {
            false => Pola0::PassiveStateIsLow,
            true => Pola0::PassiveStateIsHig,
        }
    }
    #[doc = "Passive state is LOW, active state is HIGH."]
    #[inline(always)]
    pub fn is_passive_state_is_low(&self) -> bool {
        *self == Pola0::PassiveStateIsLow
    }
    #[doc = "Passive state is HIGH, active state is LOW."]
    #[inline(always)]
    pub fn is_passive_state_is_hig(&self) -> bool {
        *self == Pola0::PassiveStateIsHig
    }
}
#[doc = "Controls the dead-time feature for channel 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dte0 {
    #[doc = "0: Dead-time disabled."]
    DeadTimeDisabled_ = 0,
    #[doc = "1: Dead-time enabled."]
    DeadTimeEnabled_ = 1,
}
impl From<Dte0> for bool {
    #[inline(always)]
    fn from(variant: Dte0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTE0` reader - Controls the dead-time feature for channel 0."]
pub type Dte0R = crate::BitReader<Dte0>;
impl Dte0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dte0 {
        match self.bits {
            false => Dte0::DeadTimeDisabled_,
            true => Dte0::DeadTimeEnabled_,
        }
    }
    #[doc = "Dead-time disabled."]
    #[inline(always)]
    pub fn is_dead_time_disabled_(&self) -> bool {
        *self == Dte0::DeadTimeDisabled_
    }
    #[doc = "Dead-time enabled."]
    #[inline(always)]
    pub fn is_dead_time_enabled_(&self) -> bool {
        *self == Dte0::DeadTimeEnabled_
    }
}
#[doc = "Enable/disable updates of functional registers for channel 0 (see Section 24.8.2).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disup0 {
    #[doc = "0: Functional registers are updated from the write registers at the end of each PWM cycle."]
    Update = 0,
    #[doc = "1: Functional registers remain the same as long as the timer is running."]
    Noupdate = 1,
}
impl From<Disup0> for bool {
    #[inline(always)]
    fn from(variant: Disup0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISUP0` reader - Enable/disable updates of functional registers for channel 0 (see Section 24.8.2)."]
pub type Disup0R = crate::BitReader<Disup0>;
impl Disup0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Disup0 {
        match self.bits {
            false => Disup0::Update,
            true => Disup0::Noupdate,
        }
    }
    #[doc = "Functional registers are updated from the write registers at the end of each PWM cycle."]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == Disup0::Update
    }
    #[doc = "Functional registers remain the same as long as the timer is running."]
    #[inline(always)]
    pub fn is_noupdate(&self) -> bool {
        *self == Disup0::Noupdate
    }
}
#[doc = "Stops/starts timer channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Run1 {
    #[doc = "0: Stop."]
    Stop_ = 0,
    #[doc = "1: Run."]
    Run_ = 1,
}
impl From<Run1> for bool {
    #[inline(always)]
    fn from(variant: Run1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUN1` reader - Stops/starts timer channel 1."]
pub type Run1R = crate::BitReader<Run1>;
impl Run1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Run1 {
        match self.bits {
            false => Run1::Stop_,
            true => Run1::Run_,
        }
    }
    #[doc = "Stop."]
    #[inline(always)]
    pub fn is_stop_(&self) -> bool {
        *self == Run1::Stop_
    }
    #[doc = "Run."]
    #[inline(always)]
    pub fn is_run_(&self) -> bool {
        *self == Run1::Run_
    }
}
#[doc = "Edge/center aligned operation for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Center1 {
    #[doc = "0: Edge-aligned."]
    EdgeAligned_ = 0,
    #[doc = "1: Center-aligned."]
    CenterAligned_ = 1,
}
impl From<Center1> for bool {
    #[inline(always)]
    fn from(variant: Center1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CENTER1` reader - Edge/center aligned operation for channel 1."]
pub type Center1R = crate::BitReader<Center1>;
impl Center1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Center1 {
        match self.bits {
            false => Center1::EdgeAligned_,
            true => Center1::CenterAligned_,
        }
    }
    #[doc = "Edge-aligned."]
    #[inline(always)]
    pub fn is_edge_aligned_(&self) -> bool {
        *self == Center1::EdgeAligned_
    }
    #[doc = "Center-aligned."]
    #[inline(always)]
    pub fn is_center_aligned_(&self) -> bool {
        *self == Center1::CenterAligned_
    }
}
#[doc = "Selects polarity of the MCOA1 and MCOB1 pins.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pola1 {
    #[doc = "0: Passive state is LOW, active state is HIGH."]
    PassiveStateIsLow = 0,
    #[doc = "1: Passive state is HIGH, active state is LOW."]
    PassiveStateIsHig = 1,
}
impl From<Pola1> for bool {
    #[inline(always)]
    fn from(variant: Pola1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POLA1` reader - Selects polarity of the MCOA1 and MCOB1 pins."]
pub type Pola1R = crate::BitReader<Pola1>;
impl Pola1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pola1 {
        match self.bits {
            false => Pola1::PassiveStateIsLow,
            true => Pola1::PassiveStateIsHig,
        }
    }
    #[doc = "Passive state is LOW, active state is HIGH."]
    #[inline(always)]
    pub fn is_passive_state_is_low(&self) -> bool {
        *self == Pola1::PassiveStateIsLow
    }
    #[doc = "Passive state is HIGH, active state is LOW."]
    #[inline(always)]
    pub fn is_passive_state_is_hig(&self) -> bool {
        *self == Pola1::PassiveStateIsHig
    }
}
#[doc = "Controls the dead-time feature for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dte1 {
    #[doc = "0: Dead-time disabled."]
    DeadTimeDisabled_ = 0,
    #[doc = "1: Dead-time enabled."]
    DeadTimeEnabled_ = 1,
}
impl From<Dte1> for bool {
    #[inline(always)]
    fn from(variant: Dte1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTE1` reader - Controls the dead-time feature for channel 1."]
pub type Dte1R = crate::BitReader<Dte1>;
impl Dte1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dte1 {
        match self.bits {
            false => Dte1::DeadTimeDisabled_,
            true => Dte1::DeadTimeEnabled_,
        }
    }
    #[doc = "Dead-time disabled."]
    #[inline(always)]
    pub fn is_dead_time_disabled_(&self) -> bool {
        *self == Dte1::DeadTimeDisabled_
    }
    #[doc = "Dead-time enabled."]
    #[inline(always)]
    pub fn is_dead_time_enabled_(&self) -> bool {
        *self == Dte1::DeadTimeEnabled_
    }
}
#[doc = "Enable/disable updates of functional registers for channel 1 (see Section 24.8.2).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disup1 {
    #[doc = "0: Functional registers are updated from the write registers at the end of each PWM cycle."]
    Update = 0,
    #[doc = "1: Functional registers remain the same as long as the timer is running."]
    Noupdate = 1,
}
impl From<Disup1> for bool {
    #[inline(always)]
    fn from(variant: Disup1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISUP1` reader - Enable/disable updates of functional registers for channel 1 (see Section 24.8.2)."]
pub type Disup1R = crate::BitReader<Disup1>;
impl Disup1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Disup1 {
        match self.bits {
            false => Disup1::Update,
            true => Disup1::Noupdate,
        }
    }
    #[doc = "Functional registers are updated from the write registers at the end of each PWM cycle."]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == Disup1::Update
    }
    #[doc = "Functional registers remain the same as long as the timer is running."]
    #[inline(always)]
    pub fn is_noupdate(&self) -> bool {
        *self == Disup1::Noupdate
    }
}
#[doc = "Stops/starts timer channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Run2 {
    #[doc = "0: Stop."]
    Stop_ = 0,
    #[doc = "1: Run."]
    Run_ = 1,
}
impl From<Run2> for bool {
    #[inline(always)]
    fn from(variant: Run2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RUN2` reader - Stops/starts timer channel 2."]
pub type Run2R = crate::BitReader<Run2>;
impl Run2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Run2 {
        match self.bits {
            false => Run2::Stop_,
            true => Run2::Run_,
        }
    }
    #[doc = "Stop."]
    #[inline(always)]
    pub fn is_stop_(&self) -> bool {
        *self == Run2::Stop_
    }
    #[doc = "Run."]
    #[inline(always)]
    pub fn is_run_(&self) -> bool {
        *self == Run2::Run_
    }
}
#[doc = "Edge/center aligned operation for channel 2.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Center2 {
    #[doc = "0: Edge-aligned."]
    EdgeAligned_ = 0,
    #[doc = "1: Center-aligned."]
    CenterAligned_ = 1,
}
impl From<Center2> for bool {
    #[inline(always)]
    fn from(variant: Center2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CENTER2` reader - Edge/center aligned operation for channel 2."]
pub type Center2R = crate::BitReader<Center2>;
impl Center2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Center2 {
        match self.bits {
            false => Center2::EdgeAligned_,
            true => Center2::CenterAligned_,
        }
    }
    #[doc = "Edge-aligned."]
    #[inline(always)]
    pub fn is_edge_aligned_(&self) -> bool {
        *self == Center2::EdgeAligned_
    }
    #[doc = "Center-aligned."]
    #[inline(always)]
    pub fn is_center_aligned_(&self) -> bool {
        *self == Center2::CenterAligned_
    }
}
#[doc = "Selects polarity of the MCOA2 and MCOB2 pins.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pola2 {
    #[doc = "0: Passive state is LOW, active state is HIGH."]
    PassiveStateIsLow = 0,
    #[doc = "1: Passive state is HIGH, active state is LOW."]
    PassiveStateIsHig = 1,
}
impl From<Pola2> for bool {
    #[inline(always)]
    fn from(variant: Pola2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `POLA2` reader - Selects polarity of the MCOA2 and MCOB2 pins."]
pub type Pola2R = crate::BitReader<Pola2>;
impl Pola2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pola2 {
        match self.bits {
            false => Pola2::PassiveStateIsLow,
            true => Pola2::PassiveStateIsHig,
        }
    }
    #[doc = "Passive state is LOW, active state is HIGH."]
    #[inline(always)]
    pub fn is_passive_state_is_low(&self) -> bool {
        *self == Pola2::PassiveStateIsLow
    }
    #[doc = "Passive state is HIGH, active state is LOW."]
    #[inline(always)]
    pub fn is_passive_state_is_hig(&self) -> bool {
        *self == Pola2::PassiveStateIsHig
    }
}
#[doc = "Controls the dead-time feature for channel 1.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dte2 {
    #[doc = "0: Dead-time disabled."]
    DeadTimeDisabled_ = 0,
    #[doc = "1: Dead-time enabled."]
    DeadTimeEnabled_ = 1,
}
impl From<Dte2> for bool {
    #[inline(always)]
    fn from(variant: Dte2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DTE2` reader - Controls the dead-time feature for channel 1."]
pub type Dte2R = crate::BitReader<Dte2>;
impl Dte2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dte2 {
        match self.bits {
            false => Dte2::DeadTimeDisabled_,
            true => Dte2::DeadTimeEnabled_,
        }
    }
    #[doc = "Dead-time disabled."]
    #[inline(always)]
    pub fn is_dead_time_disabled_(&self) -> bool {
        *self == Dte2::DeadTimeDisabled_
    }
    #[doc = "Dead-time enabled."]
    #[inline(always)]
    pub fn is_dead_time_enabled_(&self) -> bool {
        *self == Dte2::DeadTimeEnabled_
    }
}
#[doc = "Enable/disable updates of functional registers for channel 2 (see Section 24.8.2).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Disup2 {
    #[doc = "0: Functional registers are updated from the write registers at the end of each PWM cycle."]
    Update = 0,
    #[doc = "1: Functional registers remain the same as long as the timer is running."]
    Noupdate = 1,
}
impl From<Disup2> for bool {
    #[inline(always)]
    fn from(variant: Disup2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DISUP2` reader - Enable/disable updates of functional registers for channel 2 (see Section 24.8.2)."]
pub type Disup2R = crate::BitReader<Disup2>;
impl Disup2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Disup2 {
        match self.bits {
            false => Disup2::Update,
            true => Disup2::Noupdate,
        }
    }
    #[doc = "Functional registers are updated from the write registers at the end of each PWM cycle."]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == Disup2::Update
    }
    #[doc = "Functional registers remain the same as long as the timer is running."]
    #[inline(always)]
    pub fn is_noupdate(&self) -> bool {
        *self == Disup2::Noupdate
    }
}
#[doc = "Controls the polarity of the MCOB outputs for all 3 channels. This bit is typically set to 1 only in 3-phase DC mode.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Invbdc {
    #[doc = "0: The MCOB outputs have opposite polarity from the MCOA outputs (aside from dead time)."]
    Opposite = 0,
    #[doc = "1: The MCOB outputs have the same basic polarity as the MCOA outputs. (see Section 24.8.6)"]
    Same = 1,
}
impl From<Invbdc> for bool {
    #[inline(always)]
    fn from(variant: Invbdc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `INVBDC` reader - Controls the polarity of the MCOB outputs for all 3 channels. This bit is typically set to 1 only in 3-phase DC mode."]
pub type InvbdcR = crate::BitReader<Invbdc>;
impl InvbdcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Invbdc {
        match self.bits {
            false => Invbdc::Opposite,
            true => Invbdc::Same,
        }
    }
    #[doc = "The MCOB outputs have opposite polarity from the MCOA outputs (aside from dead time)."]
    #[inline(always)]
    pub fn is_opposite(&self) -> bool {
        *self == Invbdc::Opposite
    }
    #[doc = "The MCOB outputs have the same basic polarity as the MCOA outputs. (see Section 24.8.6)"]
    #[inline(always)]
    pub fn is_same(&self) -> bool {
        *self == Invbdc::Same
    }
}
#[doc = "3-phase AC mode select (see Section 24.8.7).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Acmode {
    #[doc = "0: 3-phase AC-mode off: Each PWM channel uses its own timer-counter and period register."]
    _3PhaseAcModeOff = 0,
    #[doc = "1: 3-phase AC-mode on: All PWM channels use the timer-counter and period register of channel 0."]
    _3PhaseAcModeOn_ = 1,
}
impl From<Acmode> for bool {
    #[inline(always)]
    fn from(variant: Acmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ACMODE` reader - 3-phase AC mode select (see Section 24.8.7)."]
pub type AcmodeR = crate::BitReader<Acmode>;
impl AcmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Acmode {
        match self.bits {
            false => Acmode::_3PhaseAcModeOff,
            true => Acmode::_3PhaseAcModeOn_,
        }
    }
    #[doc = "3-phase AC-mode off: Each PWM channel uses its own timer-counter and period register."]
    #[inline(always)]
    pub fn is_3_phase_ac_mode_off(&self) -> bool {
        *self == Acmode::_3PhaseAcModeOff
    }
    #[doc = "3-phase AC-mode on: All PWM channels use the timer-counter and period register of channel 0."]
    #[inline(always)]
    pub fn is_3_phase_ac_mode_on_(&self) -> bool {
        *self == Acmode::_3PhaseAcModeOn_
    }
}
#[doc = "3-phase DC mode select (see Section 24.8.6).\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dcmode {
    #[doc = "0: 3-phase DC mode off: PWM channels are independent (unless bit ACMODE = 1)"]
    _3PhaseDcModeOff = 0,
    #[doc = "1: 3-phase DC mode on: The internal MCOA0 output is routed through the CP register (i.e. a mask) register to all six PWM outputs."]
    _3PhaseDcModeOn_ = 1,
}
impl From<Dcmode> for bool {
    #[inline(always)]
    fn from(variant: Dcmode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DCMODE` reader - 3-phase DC mode select (see Section 24.8.6)."]
pub type DcmodeR = crate::BitReader<Dcmode>;
impl DcmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Dcmode {
        match self.bits {
            false => Dcmode::_3PhaseDcModeOff,
            true => Dcmode::_3PhaseDcModeOn_,
        }
    }
    #[doc = "3-phase DC mode off: PWM channels are independent (unless bit ACMODE = 1)"]
    #[inline(always)]
    pub fn is_3_phase_dc_mode_off(&self) -> bool {
        *self == Dcmode::_3PhaseDcModeOff
    }
    #[doc = "3-phase DC mode on: The internal MCOA0 output is routed through the CP register (i.e. a mask) register to all six PWM outputs."]
    #[inline(always)]
    pub fn is_3_phase_dc_mode_on_(&self) -> bool {
        *self == Dcmode::_3PhaseDcModeOn_
    }
}
impl R {
    #[doc = "Bit 0 - Stops/starts timer channel 0."]
    #[inline(always)]
    pub fn run0(&self) -> Run0R {
        Run0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Edge/center aligned operation for channel 0."]
    #[inline(always)]
    pub fn center0(&self) -> Center0R {
        Center0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects polarity of the MCOA0 and MCOB0 pins."]
    #[inline(always)]
    pub fn pola0(&self) -> Pola0R {
        Pola0R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Controls the dead-time feature for channel 0."]
    #[inline(always)]
    pub fn dte0(&self) -> Dte0R {
        Dte0R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable/disable updates of functional registers for channel 0 (see Section 24.8.2)."]
    #[inline(always)]
    pub fn disup0(&self) -> Disup0R {
        Disup0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Stops/starts timer channel 1."]
    #[inline(always)]
    pub fn run1(&self) -> Run1R {
        Run1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Edge/center aligned operation for channel 1."]
    #[inline(always)]
    pub fn center1(&self) -> Center1R {
        Center1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Selects polarity of the MCOA1 and MCOB1 pins."]
    #[inline(always)]
    pub fn pola1(&self) -> Pola1R {
        Pola1R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Controls the dead-time feature for channel 1."]
    #[inline(always)]
    pub fn dte1(&self) -> Dte1R {
        Dte1R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable/disable updates of functional registers for channel 1 (see Section 24.8.2)."]
    #[inline(always)]
    pub fn disup1(&self) -> Disup1R {
        Disup1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Stops/starts timer channel 2."]
    #[inline(always)]
    pub fn run2(&self) -> Run2R {
        Run2R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Edge/center aligned operation for channel 2."]
    #[inline(always)]
    pub fn center2(&self) -> Center2R {
        Center2R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Selects polarity of the MCOA2 and MCOB2 pins."]
    #[inline(always)]
    pub fn pola2(&self) -> Pola2R {
        Pola2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Controls the dead-time feature for channel 1."]
    #[inline(always)]
    pub fn dte2(&self) -> Dte2R {
        Dte2R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable/disable updates of functional registers for channel 2 (see Section 24.8.2)."]
    #[inline(always)]
    pub fn disup2(&self) -> Disup2R {
        Disup2R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 29 - Controls the polarity of the MCOB outputs for all 3 channels. This bit is typically set to 1 only in 3-phase DC mode."]
    #[inline(always)]
    pub fn invbdc(&self) -> InvbdcR {
        InvbdcR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 3-phase AC mode select (see Section 24.8.7)."]
    #[inline(always)]
    pub fn acmode(&self) -> AcmodeR {
        AcmodeR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 3-phase DC mode select (see Section 24.8.6)."]
    #[inline(always)]
    pub fn dcmode(&self) -> DcmodeR {
        DcmodeR::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "PWM Control read address\n\nYou can [`read`](crate::Reg::read) this register and get [`con::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConSpec;
impl crate::RegisterSpec for ConSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`con::R`](R) reader structure"]
impl crate::Readable for ConSpec {}
#[doc = "`reset()` method sets CON to value 0"]
impl crate::Resettable for ConSpec {}
