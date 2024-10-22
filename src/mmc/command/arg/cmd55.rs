bitfield! {
    /// Argument for CMD55.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Arg(u32);
    /// Indicates to the card that the next command is application specific (`ACMD*`).
    pub rca, set_rca: 31, 16;
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

        assert_eq!(arg.rca(), 0);

        let exp_rca = 0xaa55;
        arg.set_rca(exp_rca);

        assert_eq!(arg.rca(), exp_rca);
        assert_eq!(arg.bits(), exp_rca << 16);
    }
}
