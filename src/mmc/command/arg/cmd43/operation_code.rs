use crate::mmc::Error;

hal_enum! {
    /// Represents the operation code field for the `Q_MANAGEMENT` (`CMD43`) command.
    OperationCode: u32 {
        default: AbortEntireQueue,
        error: Error,
        /// Abort the entire queue.
        AbortEntireQueue = 0b0001,
        /// Abort a queued task by ID.
        AbortTaskID = 0b0010,
    }
}
