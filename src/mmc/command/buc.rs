use crate::mmc::{Error, Result};

/// The maximum number of `Block Units`.
pub const BLOCK_UNIT_MAX: u32 = 512;

/// Represents the number of `Block Units`.
///
/// The number written to the register is the number of blocks minus one, e.g. `0 = 1 Block Unit`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BlockUnitCount(u32);

impl BlockUnitCount {
    /// Creates a new [BlockUnitCount].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Gets the number of `Block Units`.
    pub const fn count(&self) -> u32 {
        self.0 + 1
    }

    /// Sets the number of `Block Units`.
    ///
    /// The max value is [BLOCK_UNIT_MAX].
    ///
    /// **NOTE**: `val` should be the actual number of block units, conversion is handled internally.
    pub fn set_count(&mut self, val: u32) -> Result<()> {
        Self::check_max(val, false)?;
        self.0 = val.saturating_sub(1);
        Ok(())
    }

    /// Builder function that Sets the number of `Block Units`.
    ///
    /// The max value is [BLOCK_UNIT_MAX].
    ///
    /// **NOTE**: `val` should be the actual number of block units, conversion is handled internally.
    pub const fn with_count(self, val: u32) -> Result<Self> {
        match Self::check_max(val, false) {
            Ok(_) => Ok(Self(val.saturating_sub(1))),
            Err(err) => Err(err),
        }
    }

    /// Attempts to convert a raw [`u32`] into a [BlockUnitCount].
    pub const fn from_raw(val: u32) -> Result<Self> {
        match Self::check_max(val, true) {
            Ok(_) => Ok(Self(val)),
            Err(err) => Err(err),
        }
    }

    /// Converts a [BlockUnitCount] into a [`u32`].
    pub fn into_raw(self) -> u32 {
        self.0
    }

    const fn check_max(val: u32, raw: bool) -> Result<()> {
        let max = if raw {
            BLOCK_UNIT_MAX - 1
        } else {
            BLOCK_UNIT_MAX
        };

        if val <= max {
            Ok(())
        } else {
            Err(Error::InvalidValue {
                value: val as usize,
                min: 0,
                max: BLOCK_UNIT_MAX as usize,
            })
        }
    }
}

impl Default for BlockUnitCount {
    fn default() -> Self {
        Self::new()
    }
}

impl From<BlockUnitCount> for u32 {
    fn from(val: BlockUnitCount) -> Self {
        val.into_raw()
    }
}

impl TryFrom<u32> for BlockUnitCount {
    type Error = Error;

    fn try_from(val: u32) -> Result<Self> {
        Self::from_raw(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid() {
        (0..BLOCK_UNIT_MAX).for_each(|raw| {
            let exp_buc = BlockUnitCount::from_raw(raw).expect("invalid BlockUnitCount");
            let exp_count = raw + 1;

            assert_eq!(BlockUnitCount::from_raw(raw), Ok(exp_buc));
            assert_eq!(BlockUnitCount::new().with_count(exp_count), Ok(exp_buc));

            let mut buc = BlockUnitCount::new();

            assert_eq!(buc.set_count(exp_count), Ok(()));
            assert_eq!(buc.count(), exp_count);
        });
    }

    #[test]
    fn test_invalid() {
        let mut buc = BlockUnitCount::new();

        (BLOCK_UNIT_MAX + 1..=1024)
            .chain([u32::MAX])
            .for_each(|invalid| {
                assert_eq!(
                    buc.set_count(invalid),
                    Err(Error::InvalidValue {
                        value: invalid as usize,
                        min: 0,
                        max: BLOCK_UNIT_MAX as usize
                    })
                );

                let raw_invalid = invalid - 1;

                assert_eq!(
                    BlockUnitCount::from_raw(raw_invalid),
                    Err(Error::InvalidValue {
                        value: raw_invalid as usize,
                        min: 0,
                        max: BLOCK_UNIT_MAX as usize
                    })
                );
            });
    }
}
