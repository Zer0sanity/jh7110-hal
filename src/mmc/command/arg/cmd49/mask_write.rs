use crate::mmc::Error;

hal_enum! {
    /// Indicate whether the `LEN/MASK` field specifies a data mask.
    MaskWrite: u32 {
        default: Disable,
        error: Error,
        /// Indicates the `LEN/MASK` field specifies the data length.
        Disable = 0,
        /// Indicates the `LEN/MASK` field specifies the data mask.
        Enable = 1,
    }
}
