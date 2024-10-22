bitfield! {
    /// Argument for CMD23.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Arg(u32);
    /// Specifies the block count for CMD18 and CMD25.
    pub block_count, set_block_count: 31, 0;
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

        assert_eq!(arg.block_count(), 0);

        let exp_block_count = 0xaaaa_5555;
        arg.set_block_count(exp_block_count);

        assert_eq!(arg.block_count(), exp_block_count);
        assert_eq!(arg.bits(), exp_block_count);
    }
}
