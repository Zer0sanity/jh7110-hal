use crate::mmc::Error;

hal_enum! {
    /// Represents the power limit (Group 4) for `CMD6`.
    PowerLimit: u32 {
        default: W072,
        error: Error,
        /// Default power limit: 0.72W.
        W072 = 0x0,
        /// Power limit: 1.44W.
        W144 = 0x1,
        /// Power limit: 2.16W.
        W216 = 0x2,
        /// Power limit: 2.88W.
        W288 = 0x3,
        /// Power limit: 1.80W.
        W180 = 0x4,
    }
}
