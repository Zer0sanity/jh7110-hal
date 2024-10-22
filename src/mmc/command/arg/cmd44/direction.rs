use crate::mmc::Error;

hal_enum! {
    /// Represents the direction of the queued task operation.
    Direction: u32 {
        default: Write,
        error: Error,
        /// Queued write task.
        Write = 0,
        /// Queued read task.
        Read = 1,
    }
}
