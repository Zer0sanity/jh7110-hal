use crate::bitflag_is_set;

bitflags! {
    /// Data flags for performing basic MMC commands.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct MmcDataFlags: u16 {
        const NONE = 0;
        const WRITE = 1 << 8;
        const READ = 1 << 9;
        const CQE_QUEUE_BARRIER = 1 << 10;
        const CQE_HIGH_PRIORITY = 1 << 11;
        const RELIABLE_WRITE = 1 << 12;
        const TAG_REQUEST = 1 << 13;
        const FORCED_PROGRAMMING = 1 << 14;
    }
}

bitflag_is_set!(MmcDataFlags);

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct MmcData {
    timeout_ns: u32,
    timeout_clks: u32,
    blksz: u32,
    blocks: u32,
    blk_addr: u32,
    error: i32,
    flags: MmcDataFlags,
    bytes_xfered: u32,
}
