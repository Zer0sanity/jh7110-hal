use crate::mmc::Error;

hal_enum! {
    /// MMC opcodes for Class 1.
    MmcOpcodeClass1: u32 {
        default: QueueManagement,
        error: Error,
        QueueManagement = 43,
        QueueTaskInfoA = 44,
        QueueTaskInfoB = 45,
        QueueReadTask = 46,
        QueueWriteTask = 47,
    }
}
