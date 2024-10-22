use crate::pac;

/// Represents PLL DSM PD setting.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Dsmpd {
    Off = 0,
    On = 1,
}

impl From<Dsmpd> for pac::sys_syscon::sys_syscfg6::Pll0Dsmpd {
    fn from(val: Dsmpd) -> Self {
        match val {
            Dsmpd::Off => Self::Off,
            Dsmpd::On => Self::On,
        }
    }
}

impl From<Dsmpd> for pac::sys_syscon::sys_syscfg9::Pll1Dsmpd {
    fn from(val: Dsmpd) -> Self {
        match val {
            Dsmpd::Off => Self::Off,
            Dsmpd::On => Self::On,
        }
    }
}

impl From<Dsmpd> for pac::sys_syscon::sys_syscfg11::Pll2Dsmpd {
    fn from(val: Dsmpd) -> Self {
        match val {
            Dsmpd::Off => Self::Off,
            Dsmpd::On => Self::On,
        }
    }
}
