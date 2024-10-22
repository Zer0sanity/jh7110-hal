use crate::mmc::Error;

hal_enum! {
    /// Represents the `Speed Class Control` field for legacy and UHS speed mode.
    VideoSpeedClass: u32 {
        default: LegacyUHS,
        error: Error,
        /// Indicates that the card is in legacy or UHS speed mode.
        LegacyUHS = 0b0,
        /// Indicates that the card is in Video speed mode.
        Video = 0b1,
    }
}
