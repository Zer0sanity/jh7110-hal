//! Clock initialization and configuration.
//!
//! Some implementations derived from [`oreboot`](https://github.com/oreboot/oreboot/blob/main/src/mainboard/starfive/visionfive2/bt0/src/init.rs).

use crate::pac;

mod aoncrg;
mod ispcrg;
mod stgcrg;
mod syscrg;
mod voutcrg;

pub use aoncrg::*;
pub use ispcrg::*;
pub use stgcrg::*;
pub use syscrg::*;
pub use voutcrg::*;

/// Convenience alias for CPU Root clock mux selector.
pub type ClkCpuRootMuxSel = pac::syscrg::clk_cpu::root::ClkMuxSel;

/// Convenience alias for Bus Root clock mux selector.
pub type ClkBusRootMuxSel = pac::syscrg::clk_bus::root::ClkMuxSel;

/// Convenience alias for Peripheral Root clock mux selector.
pub type ClkPeripheralMuxSel = pac::syscrg::clk_peripheral::root::ClkMuxSel;

/// Convenience alias for AON APB clock mux selector.
pub type ClkAonApbMuxSel = pac::aoncrg::clk_aon_apb::ClkMuxSel;

/// Convenience alias for QSPI Reference clock mux selector.
pub type ClkQspiMuxSel = pac::syscrg::clk_qspi::clk_ref::ClkMuxSel;

/// Convenience alias for DDR Bus clock mux selector.
pub type ClkDdrBusMuxSel = pac::syscrg::clk_ddr::bus::ClkMuxSel;

/// Convenience alias for NOC clock ICG configuration.
pub type ClkNocIcg = pac::syscrg::clk_noc_stg_axi::ClkIcg;

/// Convenience alias for DDR AXI clock ICG configuration.
pub type ClkDdrAxiIcg = pac::syscrg::clk_ddr::axi::ClkIcg;

/// Contains configuration registers for SoC clock peripherals.
#[repr(C)]
pub struct Clocks {
    aoncrg: ClockAoncrg,
    ispcrg: ClockIspcrg,
    stgcrg: ClockStgcrg,
    syscrg: ClockSyscrg,
    voutcrg: ClockVoutcrg,
}

impl Clocks {
    /// Creates a new [Clocks].
    ///
    /// # Example
    ///
    /// ```no_run
    /// use jh71xx_hal::pac;
    /// use jh71xx_hal::clocks::Clocks;
    ///
    /// let dp = pac::Peripherals::take().unwrap();
    /// let _clocks = Clocks::new(dp.aoncrg, dp.ispcrg, dp.stgcrg, dp.syscrg, dp.voutcrg);
    /// ```
    pub const fn new(
        aoncrg: pac::Aoncrg,
        ispcrg: pac::Ispcrg,
        stgcrg: pac::Stgcrg,
        syscrg: pac::Syscrg,
        voutcrg: pac::Voutcrg,
    ) -> Self {
        Self {
            aoncrg: ClockAoncrg::new(aoncrg),
            ispcrg: ClockIspcrg::new(ispcrg),
            stgcrg: ClockStgcrg::new(stgcrg),
            syscrg: ClockSyscrg::new(syscrg),
            voutcrg: ClockVoutcrg::new(voutcrg),
        }
    }

    /// Selects the reference clock for the CPU Root peripheral.
    pub fn select_cpu_root(&mut self, mux_sel: ClkCpuRootMuxSel) -> &mut Self {
        self.syscrg.select_cpu_root(mux_sel);

        self
    }

    /// Selects the reference clock for the Bus Root peripheral.
    pub fn select_bus_root(&mut self, mux_sel: ClkBusRootMuxSel) -> &mut Self {
        self.syscrg.select_bus_root(mux_sel);

        self
    }

    /// Selects the reference clock for the Peripheral Root peripheral.
    pub fn select_peripheral_root(&mut self, mux_sel: ClkPeripheralMuxSel) -> &mut Self {
        self.syscrg.select_peripheral_root(mux_sel);

        self
    }

    /// Selects the reference clock for the AON APB peripheral.
    pub fn select_aon_apb(&mut self, mux_sel: ClkAonApbMuxSel) -> &mut Self {
        self.aoncrg.select_aon_apb(mux_sel);

        self
    }

    /// Selects the reference clock for the QSPI peripheral.
    pub fn select_qspi(&mut self, mux_sel: ClkQspiMuxSel) -> &mut Self {
        self.syscrg.select_qspi(mux_sel);

        self
    }

    /// Selects the reference clock for the DDR Bus peripheral.
    pub fn select_ddr_bus(&mut self, mux_sel: ClkDdrBusMuxSel) -> &mut Self {
        self.syscrg.select_ddr_bus(mux_sel);

        self
    }

    /// Sets whether to enable the DDR AXI clock.
    pub fn set_ddr_axi(&mut self, icg: ClkDdrAxiIcg) -> &mut Self {
        self.syscrg.set_ddr_axi(icg);

        self
    }

    /// Sets whether to enable the NOC STG AXI clock.
    pub fn set_noc_stg_axi(&mut self, icg: ClkNocIcg) -> &mut Self {
        self.syscrg.set_noc_stg_axi(icg);

        self
    }

    /// Attempts to reset the System APB0 clock.
    pub fn reset_apb0(&mut self) -> &mut Self {
        self.syscrg.reset_apb0();

        self
    }

    /// Releases ownership of clock configuration peripherals, conuming the [Clocks].
    ///
    /// Gives ownership of the peripherals back to the caller, before calling [Clocks] destructor.
    ///
    /// Otherwise, caller would have no safe way to regain ownership.
    pub const fn release(
        self,
    ) -> (
        pac::Aoncrg,
        pac::Ispcrg,
        pac::Stgcrg,
        pac::Syscrg,
        pac::Voutcrg,
    ) {
        (
            self.aoncrg.release(),
            self.ispcrg.release(),
            self.stgcrg.release(),
            self.syscrg.release(),
            self.voutcrg.release(),
        )
    }
}
