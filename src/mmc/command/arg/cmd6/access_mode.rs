use crate::mmc::Error;

hal_enum! {
    /// Represents access modes (Group 1) for `CMD6`.
    AccessMode: u32 {
        default: SDR12,
        error: Error,
        /// Default speed: 12.5 MB/s.
        SDR12 = 0x0,
        /// High speed: 25 MB/s.
        SDR25 = 0x1,
        /// UHS-I: 50 MB/s.
        SDR50 = 0x2,
        /// UHS-I: 104 MB/s.
        SDR104 = 0x3,
        /// UHS-I: 50 MB/s.
        DDR50 = 0x4,
    }
}
