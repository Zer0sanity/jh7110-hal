use crate::mmc::Result;

mod read_write;

pub use read_write::*;

bitfield! {
    /// Argument for CMD55.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Arg(u32);
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

    /// Gets the [ReadWrite] field of the `CMD58` [Arg].
    pub const fn read_write(&self) -> Result<ReadWrite> {
        ReadWrite::from_raw(self.0 & 1)
    }

    /// Sets the [ReadWrite] field of the `CMD58` [Arg].
    pub fn set_read_write(&mut self, val: ReadWrite) {
        self.0 = val.into_raw();
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

        assert_eq!(arg.read_write(), Ok(ReadWrite::new()));

        [ReadWrite::Write, ReadWrite::Read]
            .into_iter()
            .for_each(|exp_rw| {
                arg.set_read_write(exp_rw);

                assert_eq!(arg.read_write(), Ok(exp_rw));
                assert_eq!(arg.bits(), exp_rw.into_raw());
            });
    }
}
