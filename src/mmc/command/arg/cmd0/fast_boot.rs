use crate::mmc::Error;

hal_enum! {
    /// Represents the CA-Mode (CMD0 Arg mode) in `Fast Boot` mode.
    ///
    /// **NOTE**: only valid in `idle` state, otherwise considered `00000000h`.
    FastBootMode: u32 {
        default: Conventional,
        error: Error,
        /// Handled as a conventional CMD0.
        Conventional = 0x0000_0000,
        /// DS mode.
        DS = 0xf1f1_f1f1,
        /// HS mode.
        HS = 0xf2f2_f2f2,
        /// Preprogrammed mode stored in `Preprogrammed bus mode for Fast Boot` of the `Extension Register`.
        Preprogrammed = 0xf8f8_f8f8,
        /// SDR12 mode.
        SDR12 = 0xf9f9_f9f9,
        /// SDR25 mode.
        SDR25 = 0xfafa_fafa,
        /// SDR50 mode.
        SDR50 = 0xfbfb_fbfb,
        /// SDR104 mode.
        SDR104 = 0xfcfc_fcfc,
        /// DDR50 mode.
        DDR50 = 0xfdfd_fdfd,
    }
}
