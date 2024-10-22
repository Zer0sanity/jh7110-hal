use crate::mmc::Error;

hal_enum! {
    /// MMC opcodes for Class 9.
    MmcOpcodeClass9: u32 {
        default: IOReadWriteDirect,
        error: Error,
        IOReadWriteDirect = 52,
        IOReadWriteExtended = 53,
    }
}
