//! Functions and types for handling DDR configuration.
//!
//! Implementations derived from [`oreboot`](https://github.com/oreboot/oreboot/blob/main/src/mainboard/starfive/visionfive2/bt0/src/ddrlib.rs).

use embedded_hal::delay::DelayNs;

use crate::{clocks, delay, pac, pll};

mod csr;
mod mem;
mod phy;

pub use mem::*;

#[cfg(not(any(feature = "2G", feature = "4G", feature = "8G")))]
core::compile_error!("unsupported DRAM size or none set");

/// Gets the configured size of the DRAM at build-time.
pub const fn dram_size() -> usize {
    if cfg!(feature = "2G") {
        2
    } else if cfg!(feature = "4G") {
        4
    } else if cfg!(feature = "8G") {
        8
    } else {
        0
    }
}

/// Contains registers for initializing and configuring the DDR peripheral.
#[repr(C)]
pub struct Ddr {
    dmc_ctrl: pac::DmcCtrl,
    dmc_phy: pac::DmcPhy,
    syscrg: clocks::ClockSyscrg,
    pll: pll::Pll,
}

impl Ddr {
    /// Creates a new [Ddr].
    ///
    /// # Example
    ///
    /// ```no_run
    /// use jh71xx_hal::{ddr, pac};
    ///
    /// let dp = pac::Peripherals::take().unwrap();
    /// let _clock = ddr::Ddr::new(dp.dmc_ctrl, dp.dmc_phy, dp.syscrg, dp.sys_syscon);
    /// ```
    pub const fn new(
        dmc_ctrl: pac::DmcCtrl,
        dmc_phy: pac::DmcPhy,
        syscrg: pac::Syscrg,
        syscon: pac::SysSyscon,
    ) -> Self {
        Self {
            dmc_ctrl,
            dmc_phy,
            syscrg: clocks::ClockSyscrg::new(syscrg),
            pll: pll::Pll::new(syscon),
        }
    }

    /// Selects the [Ddr] Bus clock.
    pub fn select_bus_clock(&mut self, mux_sel: clocks::ClkDdrBusMuxSel) {
        self.syscrg.select_ddr_bus(mux_sel);
    }

    /// Sets the [Pll] frequency for PLL1.
    pub fn set_pll1(&mut self, freq: pll::Freq) {
        self.pll.set_pll1(freq);
    }

    /// Initializes the DDR DRAM peripheral.
    pub fn init(&mut self) {
        let mut udelay = delay::u74_mdelay();

        self.syscrg
            .select_ddr_bus(clocks::ClkDdrBusMuxSel::ClkOscDiv2);

        self.pll.set_pll1(pll::Freq::pll1_ddr2133_1066mhz());

        udelay.delay_ns(500);

        self.syscrg
            .select_ddr_bus(clocks::ClkDdrBusMuxSel::ClkPll1Div2);
        udelay.delay_ns(200);

        // init the clocks.
        self.init_osc(&mut udelay);
        self.init_apb(&mut udelay);
        self.init_axi(&mut udelay);

        // init the OMC PHY.
        self.phy_train();
        self.phy_util();
        self.phy_start();

        self.syscrg
            .select_ddr_bus(clocks::ClkDdrBusMuxSel::ClkOscDiv2);

        // init the OMC (Orbit Memory Controller).
        self.omc_init();
    }

    /// Initialize the DDR OSC clock.
    pub fn init_osc(&mut self, udelay: &mut delay::McycleDelay) {
        self.syscrg
            .inner()
            .rst()
            .software_address_selector()
            .rst1()
            .modify(|_, w| w.u0_ddr_osc().set_bit());

        while self
            .syscrg
            .inner()
            .rst()
            .syscrg_status()
            .rst1()
            .read()
            .u0_ddr_osc()
            .is_reset()
        {
            udelay.delay_ns(1);
        }

        self.syscrg
            .inner()
            .rst()
            .software_address_selector()
            .rst1()
            .modify(|_, w| w.u0_ddr_osc().none());

        while self
            .syscrg
            .inner()
            .rst()
            .syscrg_status()
            .rst1()
            .read()
            .u0_ddr_osc()
            .is_none()
        {
            udelay.delay_ns(1);
        }
    }

    /// Initialize the DDR APB clock.
    pub fn init_apb(&mut self, udelay: &mut delay::McycleDelay) {
        self.syscrg
            .inner()
            .rst()
            .software_address_selector()
            .rst1()
            .modify(|_, w| w.u0_ddr_apb().reset());

        while self
            .syscrg
            .inner()
            .rst()
            .syscrg_status()
            .rst1()
            .read()
            .u0_ddr_apb()
            .is_reset()
        {
            udelay.delay_ns(1);
        }

        self.syscrg
            .inner()
            .rst()
            .software_address_selector()
            .rst1()
            .modify(|_, w| w.u0_ddr_apb().none());

        while self
            .syscrg
            .inner()
            .rst()
            .syscrg_status()
            .rst1()
            .read()
            .u0_ddr_apb()
            .is_none()
        {
            udelay.delay_ns(1);
        }
    }

    /// Initialize the DDR AXI clock.
    pub fn init_axi(&mut self, udelay: &mut delay::McycleDelay) {
        self.syscrg
            .inner()
            .rst()
            .software_address_selector()
            .rst1()
            .modify(|_, w| w.u0_ddr_axi().reset());

        while self
            .syscrg
            .inner()
            .rst()
            .syscrg_status()
            .rst1()
            .read()
            .u0_ddr_axi()
            .is_reset()
        {
            udelay.delay_ns(1);
        }

        self.syscrg
            .inner()
            .rst()
            .software_address_selector()
            .rst1()
            .modify(|_, w| w.u0_ddr_axi().none());

        while self
            .syscrg
            .inner()
            .rst()
            .syscrg_status()
            .rst1()
            .read()
            .u0_ddr_axi()
            .is_none()
        {
            udelay.delay_ns(1);
        }
    }

    /// Releases ownership of `DDR` peripherals, conuming the [Ddr].
    ///
    /// Gives ownership of the peripheral back to the caller, before calling [Ddr] destructor.
    ///
    /// Otherwise, caller would have no safe way to regain ownership.
    pub const fn release(self) -> (pac::DmcCtrl, pac::DmcPhy, pac::Syscrg, pac::SysSyscon) {
        (
            self.dmc_ctrl,
            self.dmc_phy,
            self.syscrg.release(),
            self.pll.release(),
        )
    }
}
