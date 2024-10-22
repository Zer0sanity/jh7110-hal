use crate::mmc::Error;

hal_enum! {
    /// Represents the driver strength (Group 3) for `CMD6`.
    DriverStrength: u32 {
        default: TypeB,
        error: Error,
        /// Type B driver strength.
        TypeB = 0x0,
        /// Type A driver strength.
        TypeA = 0x1,
        /// Type C driver strength.
        TypeC = 0x2,
        /// Type D driver strength.
        TypeD = 0x3,
    }
}
