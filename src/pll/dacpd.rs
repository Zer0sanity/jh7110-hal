use crate::pac;

/// Represents PLL DAC PD setting.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Dacpd {
    Off = 0,
    On = 1,
}

impl From<Dacpd> for pac::sys_syscon::sys_syscfg6::Pll0Dacpd {
    fn from(val: Dacpd) -> Self {
        match val {
            Dacpd::Off => Self::Off,
            Dacpd::On => Self::On,
        }
    }
}

impl From<Dacpd> for pac::sys_syscon::sys_syscfg9::Pll1Dacpd {
    fn from(val: Dacpd) -> Self {
        match val {
            Dacpd::Off => Self::Off,
            Dacpd::On => Self::On,
        }
    }
}

impl From<Dacpd> for pac::sys_syscon::sys_syscfg11::Pll2Dacpd {
    fn from(val: Dacpd) -> Self {
        match val {
            Dacpd::Off => Self::Off,
            Dacpd::On => Self::On,
        }
    }
}
