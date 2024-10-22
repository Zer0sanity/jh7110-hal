use crate::mmc::Error;

hal_enum! {
    /// MMC opcodes for Class 4.
    MmcOpcodeClass4: u32 {
        default: SetBlockCount,
        error: Error,
        SetBlocklen = 16,
        SpeedClassControl = 20,
        AddressExtension = 22,
        SetBlockCount = 23,
        WriteBlock = 24,
        WriteMutlipleBlock = 25,
        ProgramCID = 26,
        ProgramCSD = 27,
    }
}
