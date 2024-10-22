bitfield! {
    /// Argument for CMD39.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Arg(u32);
    /// Selects an active partition.
    pub partition_id, set_partition_id: 31, 24;
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

        assert_eq!(arg.partition_id(), 0);

        let exp_partition_id = 0xa5;
        arg.set_partition_id(exp_partition_id);

        assert_eq!(arg.partition_id(), exp_partition_id);
        assert_eq!(arg.bits(), exp_partition_id << 24);
    }
}
