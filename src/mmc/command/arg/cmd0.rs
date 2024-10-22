use crate::mmc::Result;

mod fast_boot;

pub use fast_boot::*;

bitfield! {
    /// Argument for CMD0.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Arg(u32);
}

impl Arg {
    /// Creates a new [Arg].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Gets the bit value of [Arg].
    pub const fn bits(&self) -> u32 {
        self.0
    }

    /// Gets the [FastBootMode] for [Arg].
    ///
    /// Only valid in the `idle` state, when the card supports boot functionality.
    pub const fn fast_boot_mode(&self) -> Result<FastBootMode> {
        FastBootMode::from_raw(self.0)
    }

    /// Sets the [FastBootMode] for [Arg].
    ///
    /// Only valid in the `idle` state, when the card supports boot functionality.
    pub fn set_fast_boot_mode(&mut self, val: FastBootMode) {
        self.0 = val.into_raw();
    }
}

impl Default for Arg {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fields() {
        let mut arg = Arg::new();

        assert_eq!(arg.fast_boot_mode(), Ok(FastBootMode::new()));

        [
            FastBootMode::Conventional,
            FastBootMode::DS,
            FastBootMode::HS,
            FastBootMode::Preprogrammed,
            FastBootMode::SDR12,
            FastBootMode::SDR25,
            FastBootMode::SDR50,
            FastBootMode::SDR104,
            FastBootMode::DDR50,
        ]
        .into_iter()
        .for_each(|exp_fast_boot_mode| {
            arg.set_fast_boot_mode(exp_fast_boot_mode);

            assert_eq!(arg.fast_boot_mode(), Ok(exp_fast_boot_mode));
            assert_eq!(arg.bits(), exp_fast_boot_mode.into_raw());
        });
    }
}
