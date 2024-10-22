use crate::mmc::Error;

bitfield! {
    /// Argument for CMD5.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Arg(u32);
    /// Switching 1.8V request.
    pub switching_18v_req, set_switching_18v_req: 24;
}

impl Arg {
    /// Creates a new [Arg].
    pub const fn new() -> Self {
        Self(1)
    }

    /// Gets the bit value of [Arg].
    pub const fn bits(&self) -> u32 {
        self.0
    }

    /// Gets the I/O Operation Conditions Register (OCR) value.
    pub const fn io_ocr(&self) -> Result<IoOCR, Error> {
        IoOCR::from_raw(self.0 & 0xff_ffff)
    }

    /// Sets the I/O Operation Conditions Register (OCR) value.
    pub fn set_io_ocr(&mut self, val: IoOCR) {
        self.0 = (self.0 & !0xff_ffff) | val.into_raw();
    }
}

impl Default for Arg {
    fn default() -> Self {
        Self::new()
    }
}

hal_enum! {
    /// I/O Operation Conditions Register.
    ///
    /// Represents the minimum and maximum VDD voltage range.
    IoOCR: u32 {
        default: Vdd2021,
        error: Error,
        /// VDD Voltage Window: 2.0 - 2.1 V
        Vdd2021 = 0b0000_0000_0000_0001,
        /// VDD Voltage Window: 2.1 - 2.2 V
        Vdd2122 = 0b0000_0000_0000_0010,
        /// VDD Voltage Window: 2.2 - 2.3 V
        Vdd2223 = 0b0000_0000_0000_0100,
        /// VDD Voltage Window: 2.3 - 2.4 V
        Vdd2324 = 0b0000_0000_0000_1000,
        /// VDD Voltage Window: 2.4 - 2.5 V
        Vdd2425 = 0b0000_0000_0001_0000,
        /// VDD Voltage Window: 2.5 - 2.6 V
        Vdd2526 = 0b0000_0000_0010_0000,
        /// VDD Voltage Window: 2.6 - 2.7 V
        Vdd2627 = 0b0000_0000_0100_0000,
        /// VDD Voltage Window: 2.7 - 2.8 V
        Vdd2728 = 0b0000_0000_1000_0000,
        /// VDD Voltage Window: 2.8 - 2.9 V
        Vdd2829 = 0b0000_0001_0000_0000,
        /// VDD Voltage Window: 2.9 - 3.0 V
        Vdd2930 = 0b0000_0010_0000_0000,
        /// VDD Voltage Window: 3.0 - 3.1 V
        Vdd3031 = 0b0000_0100_0000_0000,
        /// VDD Voltage Window: 3.1 - 3.2 V
        Vdd3132 = 0b0000_1000_0000_0000,
        /// VDD Voltage Window: 3.2 - 3.3 V
        Vdd3233 = 0b0001_0000_0000_0000,
        /// VDD Voltage Window: 3.3 - 3.4 V
        Vdd3334 = 0b0010_0000_0000_0000,
        /// VDD Voltage Window: 3.4 - 3.5 V
        Vdd3435 = 0b0100_0000_0000_0000,
        /// VDD Voltage Window: 3.5 - 3.6 V
        Vdd3536 = 0b1000_0000_0000_0000,
    }
}

#[cfg(test)]
mod ext_tests {
    use super::*;

    #[test]
    fn test_fields() {
        let mut arg = Arg::new();

        assert_eq!(arg.switching_18v_req(), false);
        arg.set_switching_18v_req(true);
        assert_eq!(arg.switching_18v_req(), true);

        assert_eq!(arg.io_ocr(), Ok(IoOCR::Vdd2021));

        arg.set_io_ocr(IoOCR::Vdd2122);
        assert_eq!(arg.io_ocr(), Ok(IoOCR::Vdd2122));
    }
}
