use crate::mmc::Result;

mod operation_code;

pub use operation_code::*;

bitfield! {
    /// Argument for CMD43.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Arg(u32);
    /// Queued task ID to abort, see [OperationCode].
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

    /// Gets the `Queue Management` [OperationCode] for [Arg].
    pub const fn operation_code(&self) -> Result<OperationCode> {
        OperationCode::from_raw(self.0 & 0xf)
    }

    /// Sets the `Queue Management` [OperationCode] for [Arg].
    pub fn set_operation_code(&mut self, val: OperationCode) {
        self.0 = (self.0 & !0xf) | val.into_raw();
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

        arg.set_task_id(0);

        [OperationCode::AbortEntireQueue, OperationCode::AbortTaskID]
            .into_iter()
            .for_each(|exp_op_code| {
                arg.set_operation_code(exp_op_code);
                assert_eq!(arg.operation_code(), Ok(exp_op_code));
                assert_eq!(arg.bits(), exp_op_code.into_raw());
            });
    }
}
