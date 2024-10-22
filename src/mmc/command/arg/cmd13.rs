bitfield! {
    /// Argument for CMD13.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Arg(u32);
    /// Programs the RCA value for send status on the CMD line.
    pub rca, set_rca: 31, 16;
    /// Addressed card sends its status register, or task status register.
    ///
    /// CQ not enabled:
    ///
    /// - `0` or `1` addressed card sends status register.
    ///
    /// CQ enabled:
    ///
    /// - `0` addressed card sends status register
    /// - `1` addressed card sends task status register
    pub send_task_status, set_send_task_status: 15;
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

        assert_eq!(arg.send_task_status(), false);
        arg.set_send_task_status(true);

        assert_eq!(arg.send_task_status(), true);
        arg.set_send_task_status(false);

        assert_eq!(arg.rca(), 0);

        let exp_rca = 0xaa55;
        arg.set_rca(exp_rca);

        assert_eq!(arg.rca(), exp_rca);
        assert_eq!(arg.bits(), exp_rca << 16);
    }
}
