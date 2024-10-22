bitfield! {
    /// Argument for CMD47.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Arg(u32);
    /// Queued write task ID.
    pub task_id, set_task_id: 20, 16;
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

        assert_eq!(arg.task_id(), 0);

        (0..=0x1f).for_each(|exp_task_id| {
            arg.set_task_id(exp_task_id);

            assert_eq!(arg.task_id(), exp_task_id);
            assert_eq!(arg.bits(), exp_task_id << 16);
        });
    }
}
