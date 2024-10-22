use crate::mmc::Error;

hal_enum! {
    /// MMC opcodes for Class 5.
    MmcOpcodeClass5: u32 {
        default: EraseBlockStart,
        error: Error,
        EraseBlockStart = 32,
        EraseBlockEnd = 33,
        Erase = 38,
    }
}
