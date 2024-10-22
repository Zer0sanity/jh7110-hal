use super::*;
use crate::pac;

/// Contains configuration registers for SoC [`AONCRG`](pac::Aoncrg) clock peripheral.
#[repr(C)]
pub struct ClockAoncrg {
    aoncrg: pac::Aoncrg,
}

impl ClockAoncrg {
    /// Creates a new [ClockAoncrg].
    ///
    /// # Example
    ///
    /// ```no_run
    /// use jh71xx_hal::pac;
    /// use jh71xx_hal::clocks::ClockAoncrg;
    ///
    /// let dp = pac::Peripherals::take().unwrap();
    /// let _clock = ClockAoncrg::new(dp.aoncrg);
    /// ```
    pub const fn new(aoncrg: pac::Aoncrg) -> Self {
        Self { aoncrg }
    }

    /// Selects the reference clock for the AON APB peripheral.
    pub fn select_aon_apb(&mut self, mux_sel: ClkAonApbMuxSel) -> &mut Self {
        self.aoncrg
            .clk_aon_apb()
            .modify(|_, w| w.clk_mux_sel().variant(mux_sel));

        self
    }

    /// Releases ownership of [`AONCRG`](pac::Aoncrg) clock configuration peripherals, conuming the [ClockAoncrg].
    ///
    /// Gives ownership of the peripheral back to the caller, before calling [ClockAoncrg] destructor.
    ///
    /// Otherwise, caller would have no safe way to regain ownership.
    pub const fn release(self) -> pac::Aoncrg {
        self.aoncrg
    }
}
