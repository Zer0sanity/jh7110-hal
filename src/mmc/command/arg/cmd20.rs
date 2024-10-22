use crate::mmc::Result;

mod speed_class_control;
mod video_speed_class;
mod video_speed_class_control;

pub use speed_class_control::*;
pub use video_speed_class::*;
pub use video_speed_class_control::*;

bitfield! {
    /// Argument for CMD20.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Arg(u32);
    /// Count/ID field (based on function).
    ///
    /// **NOTE**: only valid for Video speed mode.
    pub cnt_id, set_cnt_id: 26, 24;
    /// Data block address.
    ///
    /// **NOTE**: only valid for Video speed mode.
    pub address, set_address: 21, 0;
}

impl Arg {
    /// Creates a new [Arg].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Gets the [VideoSpeedClass] in Video speed mode.
    pub const fn vsc(&self) -> Result<VideoSpeedClass> {
        VideoSpeedClass::from_raw((self.0 & 0x800_0000) >> 27)
    }

    /// Gets the [VideoSpeedClass] in Video speed mode.
    pub fn set_vsc(&mut self, val: VideoSpeedClass) {
        self.0 = (self.0 & !0x800_0000) | (val.into_raw() << 27);
    }

    /// Gets the [SpeedClassControl] in legacy and UHS mode.
    pub const fn scc(&self) -> Result<SpeedClassControl> {
        SpeedClassControl::from_raw((self.0 & 0xf000_0000) >> 28)
    }

    /// Sets the [SpeedClassControl] in legacy and UHS mode.
    pub fn set_scc(&mut self, val: SpeedClassControl) {
        self.0 = (self.0 & !0xf000_0000) | (val.into_raw() << 28);
    }

    /// Gets the [VideoSpeedClassControl] for Video speed mode.
    pub const fn vscc(&self) -> Result<VideoSpeedClassControl> {
        VideoSpeedClassControl::from_raw((self.0 & 0xf000_0000) >> 28)
    }

    /// Gets the [VideoSpeedClassControl] for Video speed mode.
    pub fn set_vscc(&mut self, val: VideoSpeedClassControl) {
        self.0 = (self.0 & !0xf000_0000) | (val.into_raw() << 28);
    }

    /// Gets the bit value of [Arg].
    pub const fn bits(&self) -> u32 {
        self.0
    }
}

impl Default for Arg {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fields() {
        let mut arg = Arg::new();

        assert_eq!(arg.address(), 0);
        assert_eq!(arg.cnt_id(), 0);

        assert_eq!(arg.vsc(), Ok(VideoSpeedClass::LegacyUHS));
        arg.set_vsc(VideoSpeedClass::Video);
        assert_eq!(arg.vsc(), Ok(VideoSpeedClass::Video));
        arg.set_vsc(VideoSpeedClass::LegacyUHS);

        assert_eq!(arg.scc(), Ok(SpeedClassControl::StartRecording));
        arg.set_scc(SpeedClassControl::UpdateDIR);
        assert_eq!(arg.scc(), Ok(SpeedClassControl::UpdateDIR));
        assert_eq!(arg.bits(), SpeedClassControl::UpdateDIR.into_raw() << 28);
        arg.set_scc(SpeedClassControl::StartRecording);

        assert_eq!(arg.vscc(), Ok(VideoSpeedClassControl::StartRecording));
        [
            VideoSpeedClassControl::UpdateDIR,
            VideoSpeedClassControl::UpdateCI,
            VideoSpeedClassControl::SuspendRecording,
            VideoSpeedClassControl::ResumeRecording,
            VideoSpeedClassControl::SetFreeAU,
            VideoSpeedClassControl::ReleaseDIR,
        ]
        .into_iter()
        .for_each(|exp_vscc| {
            arg.set_vscc(exp_vscc);
            assert_eq!(arg.vscc(), Ok(exp_vscc));
            assert_eq!(arg.bits(), exp_vscc.into_raw() << 28);
        });
        arg.set_vscc(VideoSpeedClassControl::StartRecording);

        let exp_address = 0x3a_5555;
        arg.set_address(exp_address);

        assert_eq!(arg.address(), exp_address);
        assert_eq!(arg.bits(), exp_address);
    }
}
