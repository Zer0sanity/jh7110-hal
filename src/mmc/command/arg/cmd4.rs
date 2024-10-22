bitfield! {
    /// Argument for CMD4.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Arg(u32);
    /// Programs the DSR of all cards.
    pub dsr, set_dsr: 31, 16;
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

        assert_eq!(arg.dsr(), 0);

        let exp_dsr = 0b1010_1010_0101_0101;
        arg.set_dsr(exp_dsr);

        assert_eq!(arg.dsr(), exp_dsr);
        assert_eq!(arg.bits(), exp_dsr << 16);
    }
}
