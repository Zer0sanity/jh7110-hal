mod erase_function;

pub use erase_function::*;

bitfield! {
    /// Argument for CMD38.
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

    /// Gets the [EraseFunction] for the `CMD38` [Arg].
    pub const fn erase_function(&self) -> EraseFunction {
        EraseFunction::from_raw(self.0)
    }

    /// Gets the [EraseFunction] for the `CMD38` [Arg].
    pub fn set_erase_function(&mut self, val: EraseFunction) {
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

        assert_eq!(arg.erase_function(), EraseFunction::new());

        [
            EraseFunction::Erase,
            EraseFunction::Discard,
            EraseFunction::FULE,
        ]
        .into_iter()
        .for_each(|exp_erase_fn| {
            arg.set_erase_function(exp_erase_fn);

            assert_eq!(arg.erase_function(), exp_erase_fn);
            assert_eq!(arg.bits(), exp_erase_fn.into_raw());
        });
    }
}
