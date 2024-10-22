use crate::pac;

/// Contains configuration registers for SoC [`ISPCRG`](pac::Ispcrg) clock peripheral.
// FIXME: currently a place-holder, implement ISPCRG specific functionality
#[repr(C)]
pub struct ClockIspcrg {
    ispcrg: pac::Ispcrg,
}

impl ClockIspcrg {
    /// Creates a new [ClockIspcrg].
    ///
    /// # Example
    ///
    /// ```no_run
    /// use jh71xx_hal::pac;
    /// use jh71xx_hal::clocks::ClockIspcrg;
    ///
    /// let dp = pac::Peripherals::take().unwrap();
    /// let _clock = ClockIspcrg::new(dp.ispcrg);
    /// ```
    pub const fn new(ispcrg: pac::Ispcrg) -> Self {
        Self { ispcrg }
    }

    /// Releases ownership of [`ISPCRG`](pac::Ispcrg) clock configuration peripherals, conuming the [ClockIspcrg].
    ///
    /// Gives ownership of the peripheral back to the caller, before calling [ClockIspcrg] destructor.
    ///
    /// Otherwise, caller would have no safe way to regain ownership.
    pub const fn release(self) -> pac::Ispcrg {
        self.ispcrg
    }
}
