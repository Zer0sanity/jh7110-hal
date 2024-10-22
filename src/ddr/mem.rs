/// Represents memory configuration values used to train DDR.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemCfg {
    pub reg_nr: u32,
    pub mask: u32,
    pub value: u32,
}

impl MemCfg {
    /// Creates a new [MemCfg].
    pub const fn new() -> Self {
        Self {
            reg_nr: 0,
            mask: 0,
            value: 0,
        }
    }

    /// Creates a new [MemCfg] from the provided parameters.
    pub const fn create(reg_nr: u32, mask: u32, value: u32) -> Self {
        Self {
            reg_nr,
            mask,
            value,
        }
    }
}

impl Default for MemCfg {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents memory setting values used to train DDR.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemSet {
    pub reg_nr: u32,
    pub value: u32,
}

impl MemSet {
    /// Creates a new [MemSet].
    pub const fn new() -> Self {
        Self {
            reg_nr: 0,
            value: 0,
        }
    }

    /// Creates a new [MemSet] from the provided parameters.
    pub const fn create(reg_nr: u32, value: u32) -> Self {
        Self { reg_nr, value }
    }
}

impl Default for MemSet {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience macro for defining [MemCfg] constant arrays.
#[macro_export]
macro_rules! mem_cfg_arr {
    ($({$reg_nr:expr, $mask:expr, $value:expr}),* $(,)?) => {
        [
            $(MemCfg::create($reg_nr, $mask, $value)),*
        ]
    };
}

/// Convenience macro for defining [MemSet] constant arrays.
#[macro_export]
macro_rules! mem_set_arr {
    ($({$reg_nr:expr, $value:expr}),* $(,)?) => {
        [
            $(MemSet::create($reg_nr, $value)),*
        ]
    };
}
