use super::{Error, MmcOpcode};

pub(crate) mod buc;
pub(crate) mod bus;
pub(crate) mod mio;

mod arg;
mod flags;

pub use arg::*;
pub use flags::*;

/// MMC command response length.
pub const MMC_CMD_RSP_LEN: usize = 4;

/// Represents a native command to the MMC controller.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MmcCommand {
    opcode: MmcOpcode,
    arg: MmcCommandArg,
    resp: [u32; MMC_CMD_RSP_LEN],
    flags: MmcCommandFlags,
    retries: u16,
    error: Error,
    busy_timeout: u16,
}

impl MmcCommand {
    /// Creates a new [MmcCommand].
    pub const fn new() -> Self {
        Self {
            opcode: MmcOpcode::new(),
            arg: MmcCommandArg::new(),
            resp: [0; MMC_CMD_RSP_LEN],
            flags: MmcCommandFlags::NONE,
            retries: 0,
            error: Error::None,
            busy_timeout: 0,
        }
    }

    /// Gets the opcode for the [MmcCommand].
    pub const fn opcode(&self) -> MmcOpcode {
        self.opcode
    }
}

impl Default for MmcCommand {
    fn default() -> Self {
        Self::new()
    }
}
