use crate::mmc::Error;

hal_enum! {
    /// MMC opcodes for Class 2.
    MmcOpcodeClass2: u32 {
        default: SetBlocklen,
        error: Error,
        SetBlocklen = 16,
        ReadSingleBlock = 17,
        ReadMultipleBlock = 18,
        SendTuningBlock = 19,
        SpeedClassControl = 20,
        AddressExtension = 22,
        SetBlockCount = 23,
    }
}
