/// Convenience alias for the MMC [`Result`](core::result::Result) type.
pub type Result<T> = core::result::Result<T, Error>;

pub const EINVAL: i32 = 22;
pub const NEINVAL: i32 = -EINVAL;
pub const EILSEQ: i32 = 84;
pub const NEILSEQ: i32 = -EILSEQ;
pub const ETIMEDOUT: i32 = 110;
pub const NETIMEDOUT: i32 = -ETIMEDOUT;
pub const ENOMEDIUM: i32 = 123;
pub const NENOMEDIUM: i32 = -ENOMEDIUM;

/// Represents MMC error types.
#[repr(i32)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum Error {
    #[default]
    None = 0,
    /// Invalid argument.
    Invalid = 22,
    /// Illegal byte sequence.
    IllegalSequence = 84,
    /// Connection timed out.
    TimedOut = 110,
    /// No medium found.
    NoMedium = 123,
    /// Invalid Variant.
    InvalidVariant(usize),
    /// Invalid value.
    InvalidValue {
        value: usize,
        min: usize,
        max: usize,
    },
}

impl Error {
    /// Attempts to convert an [`i32`] into an [Error].
    pub const fn from_i32(val: i32) -> Option<Self> {
        match val {
            EINVAL | NEINVAL => Some(Self::Invalid),
            EILSEQ | NEILSEQ => Some(Self::IllegalSequence),
            ETIMEDOUT | NETIMEDOUT => Some(Self::TimedOut),
            ENOMEDIUM | NENOMEDIUM => Some(Self::NoMedium),
            _ => None,
        }
    }
}
