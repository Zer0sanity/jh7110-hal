use crate::mmc::Result;

mod access_mode;
mod command_system;
mod driver_strength;
mod mode;
mod power_limit;

pub use access_mode::*;
pub use command_system::*;
pub use driver_strength::*;
pub use mode::*;
pub use power_limit::*;

bitfield! {
    /// Argument for CMD6.
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct Arg(u32);
}

impl Arg {
    /// Creates a new [Arg].
    pub const fn new() -> Self {
        Self(0)
    }

    /// Gets the bit value of [Arg].
    pub const fn bits(&self) -> u32 {
        self.0
    }

    /// Gets the [AccessMode] for the `CMD6` [Arg].
    pub const fn access_mode(&self) -> Result<AccessMode> {
        AccessMode::from_raw(self.0 & 0xf)
    }

    /// Sets the [AccessMode] for the `CMD6` [Arg].
    pub fn set_access_mode(&mut self, val: AccessMode) {
        self.0 = (self.0 & !0xf) | val.into_raw();
    }

    /// Gets the [CommandSystem] for the `CMD6` [Arg].
    pub const fn command_system(&self) -> Result<CommandSystem> {
        CommandSystem::from_raw((self.0 & 0xf0) >> 0x4)
    }

    /// Sets the [CommandSystem] for the `CMD6` [Arg].
    pub fn set_command_system(&mut self, val: CommandSystem) {
        self.0 = (self.0 & !0xf0) | (val.into_raw() << 0x4);
    }

    /// Gets the [DriverStrength] for the `CMD6` [Arg].
    pub const fn driver_strength(&self) -> Result<DriverStrength> {
        DriverStrength::from_raw((self.0 & 0xf00) >> 0x8)
    }

    /// Sets the [DriverStrength] for the `CMD6` [Arg].
    pub fn set_driver_strength(&mut self, val: DriverStrength) {
        self.0 = (self.0 & !0xf00) | (val.into_raw() << 0x8);
    }

    /// Gets the [PowerLimit] for the `CMD6` [Arg].
    pub const fn power_limit(&self) -> Result<PowerLimit> {
        PowerLimit::from_raw((self.0 & 0xf000) >> 0xc)
    }

    /// Sets the [PowerLimit] for the `CMD6` [Arg].
    pub fn set_power_limit(&mut self, val: PowerLimit) {
        self.0 = (self.0 & !0xf000) | (val.into_raw() << 0xc);
    }

    /// Gets the [Mode] for the `CMD6` [Arg].
    pub const fn mode(&self) -> Result<Mode> {
        Mode::from_raw(self.0 >> 31)
    }

    /// Sets the [Mode] for the `CMD6` [Arg].
    pub fn set_mode(&mut self, val: Mode) {
        self.0 = (self.0 & !0x8000_0000) | (val.into_raw() << 31);
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

        assert_eq!(arg.access_mode(), Ok(AccessMode::new()));
        assert_eq!(arg.command_system(), Ok(CommandSystem::new()));
        assert_eq!(arg.driver_strength(), Ok(DriverStrength::new()));
        assert_eq!(arg.power_limit(), Ok(PowerLimit::new()));

        [
            AccessMode::SDR12,
            AccessMode::SDR25,
            AccessMode::SDR50,
            AccessMode::SDR104,
            AccessMode::DDR50,
        ]
        .into_iter()
        .for_each(|exp_access_mode| {
            arg.set_access_mode(exp_access_mode);
            assert_eq!(arg.access_mode(), Ok(exp_access_mode));
        });

        [
            CommandSystem::SystemDefault,
            CommandSystem::ForEC,
            CommandSystem::OTP,
            CommandSystem::ASSD,
            CommandSystem::Vendor,
        ]
        .into_iter()
        .for_each(|exp_command_system| {
            arg.set_command_system(exp_command_system);
            assert_eq!(arg.command_system(), Ok(exp_command_system));
        });

        [
            DriverStrength::TypeB,
            DriverStrength::TypeA,
            DriverStrength::TypeC,
            DriverStrength::TypeD,
        ]
        .into_iter()
        .for_each(|exp_driver_strength| {
            arg.set_driver_strength(exp_driver_strength);
            assert_eq!(arg.driver_strength(), Ok(exp_driver_strength));
        });

        [
            PowerLimit::W072,
            PowerLimit::W144,
            PowerLimit::W216,
            PowerLimit::W288,
            PowerLimit::W180,
        ]
        .into_iter()
        .for_each(|exp_power_limit| {
            arg.set_power_limit(exp_power_limit);
            assert_eq!(arg.power_limit(), Ok(exp_power_limit));
        });

        [Mode::Check, Mode::Switch]
            .into_iter()
            .for_each(|exp_mode| {
                arg.set_mode(exp_mode);
                assert_eq!(arg.mode(), Ok(exp_mode));
            });
    }
}
