use super::{Dacpd, Dsmpd};

/// Represents PLL frequency settings.
pub struct Freq {
    pub prediv: u8,
    pub fbdiv: u16,
    pub postdiv1: u8,
    pub dacpd: Dacpd,
    pub dsmpd: Dsmpd,
}

impl Freq {
    /// Creates a new [Freq].
    pub const fn new() -> Self {
        Self {
            prediv: 1,
            fbdiv: 1,
            postdiv1: 1,
            dacpd: Dacpd::On,
            dsmpd: Dsmpd::On,
        }
    }

    /// Creates [Freq] settings for PLL0 at 1 GHz.
    pub const fn pll0_1ghz() -> Self {
        Self {
            prediv: 3,
            fbdiv: 125,
            postdiv1: 1,
            dacpd: Dacpd::On,
            dsmpd: Dsmpd::On,
        }
    }

    /// Creates [Freq] settings for PLL1 DDR 2133 at 1,066 MHz.
    pub const fn pll1_ddr2133_1066mhz() -> Self {
        Self {
            prediv: 12,
            fbdiv: 533,
            postdiv1: 1,
            dacpd: Dacpd::On,
            dsmpd: Dsmpd::On,
        }
    }

    /// Creates [Freq] settings for PLL1 DDR at low-speed.
    pub const fn pll1_ddr_low_speed() -> Self {
        Self {
            prediv: 12,
            fbdiv: 533,
            postdiv1: 1,
            dacpd: Dacpd::Off,
            dsmpd: Dsmpd::Off,
        }
    }

    /// Creates [Freq] settings for PLL2 at 1,188 MHz.
    pub const fn pll2_1188mhz() -> Self {
        Self {
            prediv: 2,
            fbdiv: 99,
            postdiv1: 1,
            dacpd: Dacpd::On,
            dsmpd: Dsmpd::On,
        }
    }
}

impl Default for Freq {
    fn default() -> Self {
        Self::new()
    }
}
