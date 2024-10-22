use super::*;
use crate::pac;

/// Contains configuration registers for SoC [`SYSCRG`](pac::Syscrg) clock peripheral.
#[repr(C)]
pub struct ClockSyscrg {
    syscrg: pac::Syscrg,
}

impl ClockSyscrg {
    /// Creates a new [ClockSyscrg].
    ///
    /// # Example
    ///
    /// ```no_run
    /// use jh71xx_hal::pac;
    /// use jh71xx_hal::clocks::ClockSyscrg;
    ///
    /// let dp = pac::Peripherals::take().unwrap();
    /// let _clock = ClockSyscrg::new(dp.syscrg);
    /// ```
    pub const fn new(syscrg: pac::Syscrg) -> Self {
        Self { syscrg }
    }

    /// Gets a reference to the inner regsister.
    pub(crate) fn inner(&mut self) -> &pac::Syscrg {
        &self.syscrg
    }

    /// Selects the reference clock for the CPU Root peripheral.
    pub fn select_cpu_root(&mut self, mux_sel: ClkCpuRootMuxSel) -> &mut Self {
        self.syscrg
            .clk_cpu()
            .root()
            .modify(|_, w| w.clk_mux_sel().variant(mux_sel));

        self
    }

    /// Selects the reference clock for the Bus Root peripheral.
    pub fn select_bus_root(&mut self, mux_sel: ClkBusRootMuxSel) -> &mut Self {
        self.syscrg
            .clk_bus()
            .root()
            .modify(|_, w| w.clk_mux_sel().variant(mux_sel));

        self
    }

    /// Selects the reference clock for the Peripheral Root peripheral.
    pub fn select_peripheral_root(&mut self, mux_sel: ClkPeripheralMuxSel) -> &mut Self {
        self.syscrg
            .clk_peripheral()
            .root()
            .modify(|_, w| w.clk_mux_sel().variant(mux_sel));

        self
    }

    /// Selects the reference clock for the QSPI peripheral.
    pub fn select_qspi(&mut self, mux_sel: ClkQspiMuxSel) -> &mut Self {
        self.syscrg
            .clk_qspi()
            .clk_ref()
            .modify(|_, w| w.clk_mux_sel().variant(mux_sel));

        self
    }

    /// Selects the reference clock for the DDR Bus peripheral.
    pub fn select_ddr_bus(&mut self, mux_sel: ClkDdrBusMuxSel) -> &mut Self {
        self.syscrg
            .clk_ddr()
            .bus()
            .modify(|_, w| w.clk_mux_sel().variant(mux_sel));

        self
    }

    /// Sets whether to enable the DDR AXI clock.
    pub fn set_ddr_axi(&mut self, icg: ClkDdrAxiIcg) -> &mut Self {
        self.syscrg
            .clk_ddr()
            .axi()
            .modify(|_, w| w.clk_icg().variant(icg));

        self
    }

    /// Sets whether to enable the NOC STG AXI clock.
    pub fn set_noc_stg_axi(&mut self, icg: ClkNocIcg) -> &mut Self {
        self.syscrg
            .clk_noc_stg_axi()
            .modify(|_, w| w.clk_icg().variant(icg));

        self
    }

    /// Attempts to reset the System APB0 clock.
    pub fn reset_apb0(&mut self) -> &mut Self {
        self.syscrg.clk_apb0().modify(|_, w| {
            w.clk_icg().disable();
            w.clk_icg().enable()
        });

        self
    }

    /// Releases ownership of [`SYSCRG`](pac::Syscrg) clock configuration peripherals, conuming the [ClockSyscrg].
    ///
    /// Gives ownership of the peripheral back to the caller, before calling [ClockSyscrg] destructor.
    ///
    /// Otherwise, caller would have no safe way to regain ownership.
    pub const fn release(self) -> pac::Syscrg {
        self.syscrg
    }
}
