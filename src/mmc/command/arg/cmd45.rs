bitfield! {
    /// Argument for CMD45.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Arg(u32);
    /// Starting block address for a queued task.
    pub start_block_address, set_start_block_address: 31, 0;
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

        assert_eq!(arg.start_block_address(), 0);

        (0..=1024).chain([u32::MAX]).for_each(|exp_addr| {
            arg.set_start_block_address(exp_addr);

            assert_eq!(arg.start_block_address(), exp_addr);
            assert_eq!(arg.bits(), exp_addr);
        });
    }
}
