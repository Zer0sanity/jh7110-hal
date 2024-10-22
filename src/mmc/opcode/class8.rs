use crate::mmc::Error;

hal_enum! {
    /// MMC opcodes for Class 8.
    MmcOpcodeClass8: u32 {
        default: AppCommand,
        error: Error,
        SetBlockCount = 23,
        AppCommand = 55,
        GenCommand = 56,
    }
}
