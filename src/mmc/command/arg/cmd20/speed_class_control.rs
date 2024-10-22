use crate::mmc::Error;

hal_enum! {
    /// Represents the `Speed Class Control` field for legacy and UHS speed mode.
    SpeedClassControl: u32 {
        default: StartRecording,
        error: Error,
        /// Indicates that the host starts stream recording.
        StartRecording = 0b0000,
        /// Indicates the following write is a directory entry write.
        UpdateDIR = 0b0001,
        /// Indicates the following write command is a write to a CI cluster.
        UpdateCI = 0b0100,
    }
}
