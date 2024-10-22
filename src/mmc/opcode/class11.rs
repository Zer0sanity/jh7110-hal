use crate::mmc::Error;

hal_enum! {
    /// MMC opcodes for Class 11.
    MmcOpcodeClass11: u32 {
        default: SelectCardPartition,
        error: Error,
        SelectCardPartition = 39,
        ReadExtraSingle = 48,
        WriteExtraSingle = 49,
        ReadExtraMultiple = 58,
        WriteExtraMultiple = 59,
    }
}
