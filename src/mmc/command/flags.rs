/// Card is present.
pub const RSP_PRESENT: u16 = 1 << 0;
/// 136-bit response.
pub const RSP_136: u16 = 1 << 1;
/// Expect valid CRC.
pub const RSP_CRC: u16 = 1 << 2;
/// Card may send busy.
pub const RSP_BUSY: u16 = 1 << 3;
/// Response contains opcode.
pub const RSP_OPCODE: u16 = 1 << 4;
/// Non-SPI AC command.
pub const CMD_AC: u16 = 0 << 5;
/// Non-SPI ADTC command.
pub const CMD_ADTC: u16 = 1 << 5;
/// Non-SPI BC command.
pub const CMD_BC: u16 = 2 << 5;
/// Non-SPI BCR command.
pub const CMD_BCR: u16 = 3 << 5;

bitflags! {
    /// Command expected response type flags for basic MMC commands.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
    pub struct MmcCommandFlags: u16 {
        const NONE = 0;
        /// Card is present.
        const RSP_PRESENT = RSP_PRESENT;
        /// 136-bit response.
        const RSP_136 = RSP_136;
        /// Expect valid CRC.
        const RSP_CRC = RSP_CRC;
        /// Card may send busy.
        const RSP_BUSY = RSP_BUSY;
        /// Response contains opcode.
        const RSP_OPCODE = RSP_OPCODE;
        /// Non-SPI AC command.
        const CMD_AC = CMD_AC;
        /// Non-SPI ADTC command.
        const CMD_ADTC = CMD_ADTC;
        /// Non-SPI BC command.
        const CMD_BC = CMD_BC;
        /// Non-SPI BCR command.
        const CMD_BCR = CMD_BCR;
        /// No response expected.
        const RSP_NONE = 0;
        /// Native R1 response type.
        const RSP_R1 = RSP_PRESENT | RSP_CRC | RSP_OPCODE;
        /// Native R1B response type.
        const RSP_R1B = RSP_PRESENT | RSP_CRC | RSP_OPCODE | RSP_BUSY;
        /// Native R2 response type.
        const RSP_R2 = RSP_PRESENT | RSP_136 | RSP_CRC;
        /// Native R3 response type.
        const RSP_R3 = RSP_PRESENT;
        /// Native R4 response type.
        const RSP_R4 = RSP_PRESENT;
        /// Native R5 response type.
        const RSP_R5 = RSP_PRESENT | RSP_CRC | RSP_OPCODE;
        /// Native R6 response type.
        const RSP_R6 = RSP_PRESENT | RSP_CRC | RSP_OPCODE;
        /// Native R7 response type.
        const RSP_R7 = RSP_PRESENT | RSP_CRC | RSP_OPCODE;
        /// Native R1 response with no CRC.
        ///
        /// Can be used to poll after switch to MMC HS mode.
        const RSP_R1_NO_CRC = RSP_PRESENT | RSP_OPCODE;
    }
}

impl MmcCommandFlags {
    /// Gets the response type for the [MmcCommandFlags].
    pub fn response_type(self) -> Self {
        self & (Self::RSP_PRESENT
            | Self::RSP_136
            | Self::RSP_CRC
            | Self::RSP_BUSY
            | Self::RSP_OPCODE)
    }

    /// Gets the command type for the [MmcCommandFlags].
    pub fn command_type(self) -> Self {
        self & (Self::CMD_ADTC | Self::CMD_BC | Self::CMD_BCR)
    }
}

bitflag_is_set!(MmcCommandFlags);
