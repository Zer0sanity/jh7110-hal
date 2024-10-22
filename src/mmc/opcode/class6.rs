use crate::mmc::Error;

hal_enum! {
    /// MMC opcodes for Class 6.
    MmcOpcodeClass6: u32 {
        default: SetWriteProtection,
        error: Error,
        SetWriteProtection = 28,
        ClearWriteProtection = 29,
        SendWriteProtection = 30,
    }
}
