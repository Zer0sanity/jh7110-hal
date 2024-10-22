bitfield! {
    /// Argument for CMD8.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Arg(u32);
    /// Programs the supply voltage of the card.
    pub vhs, set_vhs: 11, 8;
    /// Programs the check pattern of the card.
    pub check, set_check: 7, 0;
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

        assert_eq!(arg.check(), 0);

        let exp_check = 0b1010_1010;
        arg.set_check(exp_check);

        assert_eq!(arg.check(), exp_check);

        assert_eq!(arg.vhs(), 0);

        let exp_vhs = 0b0101;
        arg.set_vhs(exp_vhs);

        assert_eq!(arg.vhs(), exp_vhs);

        assert_eq!(arg.bits(), (exp_vhs << 8) | exp_check);
    }
}
