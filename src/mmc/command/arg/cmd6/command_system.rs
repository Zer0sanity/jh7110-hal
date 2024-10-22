use crate::mmc::Error;

hal_enum! {
    /// Represents the command system (Group 2) for `CMD6`.
    CommandSystem: u32 {
        default: SystemDefault,
        error: Error,
        /// Default command system function after boot.
        SystemDefault = 0x0,
        /// Command system function for eC.
        ForEC = 0x1,
        /// OTP command system function.
        OTP = 0x3,
        /// ASSD command system function.
        ASSD = 0x4,
        /// Vendor-specific command system function.
        Vendor = 0xe,
    }
}
