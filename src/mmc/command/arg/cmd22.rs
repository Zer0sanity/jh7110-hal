bitfield! {
    /// Argument for CMD22.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Arg(u32);
    /// 6-bit extended address added to the upper part of a unit or sector address.
    pub extended_address, set_extended_address: 5, 0;
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

        assert_eq!(arg.extended_address(), 0);

        let exp_extended_address = 0x3f;
        arg.set_extended_address(exp_extended_address);

        assert_eq!(arg.extended_address(), exp_extended_address);
        assert_eq!(arg.bits(), exp_extended_address);
    }
}
