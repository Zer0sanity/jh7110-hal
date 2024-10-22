use crate::mmc::Error;

hal_enum! {
    /// Represents the argument to `CMD58` to set flag read/write operation.
    ReadWrite: u32 {
        default: Write,
        error: Error,
        /// Indicates writing data to the card.
        Write = 0,
        /// Indicates reading data from the card.
        Read = 1,
    }
}
