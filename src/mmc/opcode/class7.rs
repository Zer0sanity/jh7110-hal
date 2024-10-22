use crate::mmc::Error;

hal_enum! {
    /// MMC opcodes for Class 7.
    MmcOpcodeClass7: u32 {
        default: LockUnlock,
        error: Error,
        SetBlocklen = 16,
        SingleBlockRead = 40,
        LockUnlock = 42,
    }
}
