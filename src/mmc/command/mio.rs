use crate::mmc::Error;

hal_enum! {
    /// Memory and I/O media type selection.
    MIO: u32 {
        default: Memory,
        error: Error,
        /// Selects the Memory media type (MMC).
        Memory = 0,
        /// Selects the I/O media type (SDIO).
        IO = 1,
    }
}
