use crate::pac;

/// Contains configuration registers for SoC [`VOUTCRG`](pac::Voutcrg) clock peripheral.
// FIXME: currently a place-holder, implement VOUTCRG specific functionality
#[repr(C)]
pub struct ClockVoutcrg {
    voutcrg: pac::Voutcrg,
}

impl ClockVoutcrg {
    /// Creates a new [ClockVoutcrg].
    ///
    /// # Example
    ///
    /// ```no_run
    /// use jh71xx_hal::pac;
    /// use jh71xx_hal::clocks::ClockVoutcrg;
    ///
    /// let dp = pac::Peripherals::take().unwrap();
    /// let _clock = ClockVoutcrg::new(dp.voutcrg);
    /// ```
    pub const fn new(voutcrg: pac::Voutcrg) -> Self {
        Self { voutcrg }
    }

    /// Releases ownership of [`VOUTCRG`](pac::Voutcrg) clock configuration peripherals, conuming the [ClockVoutcrg].
    ///
    /// Gives ownership of the peripheral back to the caller, before calling [ClockVoutcrg] destructor.
    ///
    /// Otherwise, caller would have no safe way to regain ownership.
    pub const fn release(self) -> pac::Voutcrg {
        self.voutcrg
    }
}
