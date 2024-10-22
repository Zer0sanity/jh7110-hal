use crate::mmc::Error;

hal_enum! {
    /// MMC opcodes for Class 0.
    MmcOpcodeClass0: u32 {
        default: GoIdleState,
        error: Error,
        GoIdleState = 0,
        AllSendCID = 2,
        SetRelativeAddress = 3,
        SetDSR = 4,
        SendOpCond = 5,
        SelectCard = 7,
        SendIFCond = 8,
        SendCSD = 9,
        SendCID = 10,
        VoltageSwitch = 11,
        StopTransmission = 12,
        SendStatus = 13,
        GoInactiveState = 15,
    }
}
