use crate::pac;

/// Contains configuration registers for SoC [`STGCRG`](pac::Stgcrg) clock peripheral.
// FIXME: currently a place-holder, implement STGCRG specific functionality
#[repr(C)]
pub struct ClockStgcrg {
    stgcrg: pac::Stgcrg,
}

impl ClockStgcrg {
    /// Creates a new [ClockStgcrg].
    ///
    /// # Example
    ///
    /// ```no_run
    /// use jh71xx_hal::pac;
    /// use jh71xx_hal::clocks::ClockStgcrg;
    ///
    /// let dp = pac::Peripherals::take().unwrap();
    /// let _clock = ClockStgcrg::new(dp.stgcrg);
    /// ```
    pub const fn new(stgcrg: pac::Stgcrg) -> Self {
        Self { stgcrg }
    }

    /// Releases ownership of [`STGCRG`](pac::Stgcrg) clock configuration peripherals, conuming the [ClockStgcrg].
    ///
    /// Gives ownership of the peripheral back to the caller, before calling [ClockStgcrg] destructor.
    ///
    /// Otherwise, caller would have no safe way to regain ownership.
    pub const fn release(self) -> pac::Stgcrg {
        self.stgcrg
    }
}
