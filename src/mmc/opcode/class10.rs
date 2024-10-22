use crate::mmc::Error;

hal_enum! {
    /// MMC opcodes for Class 10.
    MmcOpcodeClass10: u32 {
        default: SwitchFunc,
        error: Error,
        SwitchFunc = 6,
    }
}
