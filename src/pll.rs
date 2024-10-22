//! PLL initialization and configuration.
//!
//! Based on the [`oreboot`](https://github.com/oreboot/oreboot/blob/main/src/mainboard/starfive/visionfive2/bt0/src/pll.rs) implementation.

use crate::pac;

mod dacpd;
mod dsmpd;
mod freq;

pub use dacpd::*;
pub use dsmpd::*;
pub use freq::*;

/// Contains configuration registers for SoC PLL peripherals.
pub struct Pll {
    syscon: pac::SysSyscon,
}

impl Pll {
    /// Creates a new [Pll].
    ///
    /// # Example
    ///
    /// ```no_run
    /// use jh71xx_hal::{pac, pll};
    ///
    /// let dp = pac::Peripherals::take().unwrap();
    /// let _pll = pll::Pll::new(dp.sys_syscon);
    /// ```
    pub const fn new(syscon: pac::SysSyscon) -> Self {
        Self { syscon }
    }

    /// Sets the PLL0 clock frequency.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use jh71xx_hal::{pac, pll};
    ///
    /// let dp = pac::Peripherals::take().unwrap();
    /// let mut pll = pll::Pll::new(dp.sys_syscon);
    ///
    /// pll.set_pll0(pll::Freq::pll0_1ghz());
    /// ```
    pub fn set_pll0(&mut self, f: Freq) -> &mut Self {
        // Turn-off PD by setting the bit.
        self.syscon.sys_syscfg8().modify(|_, w| w.pll0_pd().off());

        self.syscon.sys_syscfg6().modify(|_, w| {
            w.pll0_dacpd().variant(f.dacpd.into());
            w.pll0_dsmpd().variant(f.dsmpd.into())
        });

        self.syscon
            .sys_syscfg9()
            .modify(|_, w| w.pll0_prediv().variant(f.prediv));

        self.syscon
            .sys_syscfg7()
            .modify(|_, w| w.pll0_fbdiv().variant(f.fbdiv));

        // From `oreboot` comments (`28d0eea9af3` - Daniel Maslowski <info@orangecms.org>):
        //
        // NOTE: Not sure why, but the original code does this shift, and defines
        // all postdiv values for all PLLs and config to be 1, effectively dropping
        // to 0 here.
        self.syscon.sys_syscfg8().modify(|_, w| {
            w.pll0_postdiv1().variant(f.postdiv1 >> 1);
            // Turn on PD by clearing the bit
            w.pll0_pd().on()
        });

        self
    }

    /// Sets the PLL1 clock frequency.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use jh71xx_hal::{pac, pll};
    ///
    /// let dp = pac::Peripherals::take().unwrap();
    /// let mut pll = pll::Pll::new(dp.sys_syscon);
    ///
    /// pll.set_pll1(pll::Freq::pll1_ddr2133_1066mhz());
    /// ```
    pub fn set_pll1(&mut self, f: Freq) -> &mut Self {
        // Turn-off PD by setting the bit.
        self.syscon.sys_syscfg10().modify(|_, w| w.pll1_pd().off());

        self.syscon.sys_syscfg9().modify(|_, w| {
            w.pll1_dacpd().variant(f.dacpd.into());
            w.pll1_dsmpd().variant(f.dsmpd.into())
        });

        self.syscon
            .sys_syscfg11()
            .modify(|_, w| w.pll1_prediv().variant(f.prediv));

        self.syscon
            .sys_syscfg9()
            .modify(|_, w| w.pll1_fbdiv().variant(f.fbdiv));

        // From `oreboot` comments (`28d0eea9af3` - Daniel Maslowski <info@orangecms.org>):
        //
        // NOTE: Not sure why, but the original code does this shift, and defines
        // all postdiv values for all PLLs and config to be 1, effectively dropping
        // to 0 here.
        self.syscon.sys_syscfg10().modify(|_, w| {
            w.pll1_postdiv1().variant(f.postdiv1 >> 1);
            // Turn on PD by clearing the bit
            w.pll1_pd().on()
        });

        self
    }

    /// Sets the PLL2 clock frequency.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use jh71xx_hal::{pac, pll};
    ///
    /// let dp = pac::Peripherals::take().unwrap();
    /// let mut pll = pll::Pll::new(dp.sys_syscon);
    ///
    /// pll.set_pll2(pll::Freq::pll2_1188mhz());
    /// ```
    pub fn set_pll2(&mut self, f: Freq) -> &mut Self {
        self.syscon.sys_syscfg12().modify(|_, w| w.pll2_pd().off());

        self.syscon.sys_syscfg11().modify(|_, w| {
            w.pll2_dacpd().variant(f.dacpd.into());
            w.pll2_dsmpd().variant(f.dsmpd.into())
        });

        self.syscon
            .sys_syscfg13()
            .modify(|_, w| w.pll2_prediv().variant(f.prediv));

        self.syscon
            .sys_syscfg11()
            .modify(|_, w| w.pll2_fbdiv().variant(f.fbdiv));

        // From `oreboot` comments (`28d0eea9af3` - Daniel Maslowski <info@orangecms.org>):
        //
        // NOTE: Not sure why, but the original code does this shift, and defines
        // all postdiv values for all PLLs and config to be 1, effectively dropping
        // to 0 here.
        self.syscon.sys_syscfg12().modify(|_, w| {
            w.pll2_postdiv1().variant(f.postdiv1 >> 1);
            // Turn on PD by clearing the bit
            w.pll2_pd().on()
        });

        self
    }

    /// Releases ownership of the [SysSyscon](pac::SysSyscon) peripheral to the caller.
    ///
    /// Gives ownership of the peripheral back to the caller, before calling [Pll] destructor.
    ///
    /// Otherwise, caller would have no safe way to regain ownership.
    pub const fn release(self) -> pac::SysSyscon {
        self.syscon
    }
}
