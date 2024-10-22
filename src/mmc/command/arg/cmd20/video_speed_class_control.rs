use crate::mmc::Error;

hal_enum! {
    /// Represents the `Speed Class Control` field for Video speed mode.
    VideoSpeedClassControl: u32 {
        default: StartRecording,
        error: Error,
        /// Indicates that the host starts stream recording.
        StartRecording = 0b0000,
        /// Indicates the following write is a directory entry write.
        UpdateDIR = 0b0001,
        /// Indicates the following write command is a write to a CI cluster.
        UpdateCI = 0b0100,
        /// Indicates to suspend recording.
        SuspendRecording = 0b0101,
        /// Indicates to resume recording.
        ResumeRecording = 0b0110,
        /// Indicates that the host assigns one or more sequential AUs for recording.
        SetFreeAU = 0b0111,
        /// Indicates the DIR address stored in the specified slot shall be cleared.
        ReleaseDIR = 0b1000,
    }
}
