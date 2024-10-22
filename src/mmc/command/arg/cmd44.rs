use crate::mmc::Result;

mod direction;

pub use direction::*;

bitfield! {
    /// Argument for CMD44.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Arg(u32);
    /// Upper 6-bits address of 38-bit 512B unit address, for `SDUC` cards.
    pub extended_address, set_extended_address: 29, 24;
    /// Priority task.
    pub priority, set_priority: 23;
    /// Queued task ID.
    pub task_id, set_task_id: 20, 16;
    /// Queued number of blocks.
    pub number_of_blocks, set_number_of_blocks: 15, 0;
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

    /// Gets the `Queue Task Info` [Direction] for [Arg].
    pub const fn direction(&self) -> Result<Direction> {
        Direction::from_raw((self.0 & 0x4000_0000) >> 30)
    }

    /// Sets the `Queue Task Info` [Direction] for [Arg].
    pub fn set_direction(&mut self, val: Direction) {
        self.0 = (self.0 & !0x4000_0000) | (val.into_raw() << 30);
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

        assert_eq!(arg.number_of_blocks(), 0);
        assert_eq!(arg.task_id(), 0);
        assert!(!arg.priority());
        assert_eq!(arg.extended_address(), 0);
        assert_eq!(arg.direction(), Ok(Direction::new()));

        (0..=1024).chain([u16::MAX as u32]).for_each(|exp_num| {
            arg.set_number_of_blocks(exp_num);

            assert_eq!(arg.number_of_blocks(), exp_num);
            assert_eq!(arg.bits(), exp_num);
        });

        arg.set_number_of_blocks(0);

        (0..=0x1f).for_each(|exp_task_id| {
            arg.set_task_id(exp_task_id);
            assert_eq!(arg.task_id(), exp_task_id);
            assert_eq!(arg.bits(), exp_task_id << 16);
        });

        arg.set_task_id(0);

        arg.set_priority(true);
        assert!(arg.priority());
        assert_eq!(arg.bits(), 1 << 23);
        arg.set_priority(false);

        (0..=0x3f).for_each(|exp_addr| {
            arg.set_extended_address(exp_addr);

            assert_eq!(arg.extended_address(), exp_addr);
            assert_eq!(arg.bits(), exp_addr << 24);
        });

        arg.set_extended_address(0);

        [Direction::Write, Direction::Read]
            .into_iter()
            .for_each(|exp_direction| {
                arg.set_direction(exp_direction);
                assert_eq!(arg.direction(), Ok(exp_direction));
                assert_eq!(arg.bits(), exp_direction.into_raw() << 30);
            });
    }
}
