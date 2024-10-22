use crate::mmc::Error;

hal_enum! {
    /// Represents modes for the `SWITCH_FUNC` (`CMD6`) command.
    Mode: u32 {
        default: Check,
        error: Error,
        /// Checks if a card supports a switchable function.
        Check = 0,
        /// Switches card function.
        Switch = 1,
    }
}
