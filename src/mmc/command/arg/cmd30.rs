bitfield! {
    /// Argument for CMD30.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Arg(u32);
    /// If the card has write protection, this asks the card to send the status of write protection for the address group.
    pub data_address, set_data_address: 31, 0;
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

        assert_eq!(arg.data_address(), 0);

        let exp_data_address = 0xaaaa_5555;
        arg.set_data_address(exp_data_address);

        assert_eq!(arg.data_address(), exp_data_address);
        assert_eq!(arg.bits(), exp_data_address);
    }
}
