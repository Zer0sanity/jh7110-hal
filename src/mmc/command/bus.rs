use crate::mmc::Error;

hal_enum! {
    /// Represents the number of block in a `Block Unit`.
    BlockUnitSelect: u32 {
        default: Bytes512,
        error: Error,
        /// Selects 512 byte `Block Unit`.
        Bytes512 = 0,
        /// Selects 32 kilobyte `Block Unit`.
        Bytes32k = 1,
    }
}
