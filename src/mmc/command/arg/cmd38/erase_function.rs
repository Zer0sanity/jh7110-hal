const ERASE_FN_ERASE: u32 = 0;
const ERASE_FN_DISCARD: u32 = 1;
const ERASE_FN_FULE: u32 = 2;

/// Represents the erase function sent in `CMD38`.
#[repr(u32)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EraseFunction {
    Erase = ERASE_FN_ERASE,
    Discard = ERASE_FN_DISCARD,
    FULE = ERASE_FN_FULE,
}

impl EraseFunction {
    /// Creates a new [EraseFunction].
    pub const fn new() -> Self {
        Self::Erase
    }

    /// Attempts to convert a [`u32`] into a [EraseFunction].
    pub const fn from_raw(val: u32) -> Self {
        match val {
            ERASE_FN_DISCARD => Self::Discard,
            ERASE_FN_FULE => Self::FULE,
            _ => Self::Erase,
        }
    }

    /// Converts a [EraseFunction] into a [`u32`].
    pub const fn into_raw(self) -> u32 {
        self as u32
    }
}

impl Default for EraseFunction {
    fn default() -> Self {
        Self::new()
    }
}

impl From<u32> for EraseFunction {
    fn from(val: u32) -> Self {
        Self::from_raw(val)
    }
}

impl From<EraseFunction> for u32 {
    fn from(val: EraseFunction) -> Self {
        val.into_raw()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_variants() {
        let raw = [ERASE_FN_DISCARD, ERASE_FN_FULE];

        let exp = [EraseFunction::Discard, EraseFunction::FULE];

        raw.into_iter().zip(exp).for_each(|(r, e)| {
            assert_eq!(EraseFunction::from_raw(r), e);
            assert_eq!(EraseFunction::from(r), e);

            assert_eq!(e.into_raw(), r);
            assert_eq!(u32::from(e), r);
        });

        let e = EraseFunction::Erase;
        assert_eq!(e.into_raw(), ERASE_FN_ERASE);
        assert_eq!(u32::from(e), ERASE_FN_ERASE);

        (0..1024)
            .chain([u32::MAX])
            .filter(|r| !raw.iter().any(|e| e == r))
            .for_each(|r| {
                assert_eq!(EraseFunction::from_raw(r), e);
                assert_eq!(EraseFunction::from(r), e);
            });
    }
}
