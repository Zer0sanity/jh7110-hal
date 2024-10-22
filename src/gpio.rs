//! GPIO configuration and access.
//!
//! ## Examples
//!
//! ```no_run
//! use jh71xx_hal::{pac, gpio};
//! use embedded_hal::digital::{InputPin, OutputPin};
//!
//! let dp = pac::Peripherals::take().unwrap();
//! let gpio0 = gpio::get_gpio(dp.sys_pinctrl.padcfg().gpio0());
//!
//! // Configure as an enabled output
//! let mut gpio0_out = gpio0.into_enabled_output();
//!
//! // Drive pin high
//! gpio0_out.set_high();
//! // Drive pin low
//! gpio0_out.set_low();
//!
//! // Configure as an enabled input
//! let gpio0_in = gpio0_out.into_enabled_input();
//!
//! // Configure as high-impedance input
//! let gpio0_in_high_z = gpio0_in.into_input_high_z();
//! // Configure as pull-up input
//! let gpio0_in_pull_up = gpio0_in_high_z.into_input_pull_up();
//! // Configure as pull-down input
//! let mut gpio0_in_pull_down = gpio0_in_pull_up.into_input_pull_down();
//!
//! // Is pin low?
//! if gpio0_in_pull_down.is_low().unwrap() {
//!     // do interesting GPIO stuff
//! }
//!
//! // Is pin high?
//! if gpio0_in_pull_down.is_high().unwrap() {
//!     // do interesting GPIO stuff
//! }
//! ```
//!
//! ### WIP
//!
//! `JH7110` SoCs use a pin multiplexer to configure pins for specialized functionality (I2C, SPI, etc.).
//!
//! Work is on-going to provide high-level interfaces to configure specialized functions for GPIO pins.
//!
//! Low-level configuration can currently be achieved through the `jh71xx-pac` crate which is re-exported as `jh71xx_hal::pac`.

use embedded_hal::digital::{ErrorType, InputPin, OutputPin};

use crate::pac::SysPinctrl;

mod config;
mod error;
mod functions;
mod pad;

pub use config::*;
pub use error::*;
pub use functions::*;
pub use pad::*;

/// Configures the GPIO as enabled.
pub struct Enabled;
/// Configures the GPIO as disabled.
pub struct Disabled;

/// Configures the GPIO as an input.
pub struct Input;
/// Configures the GPIO as an output.
pub struct Output;

/// Configures the GPIO as high-impedance input.
pub struct HiZ;
/// Configures the GPIO as pull-down input.
pub struct PullDown;
/// Configures the GPIO as pull-up input.
pub struct PullUp;

/// Not-important placeholder
pub struct Nop;

/// Configures how to drive a GPIO.
#[repr(u8)]
pub enum OutputConfig {
    Low = 0b00,
    Neutral = 0b01,
    High = 0b11,
}

impl From<u8> for OutputConfig {
    fn from(val: u8) -> Self {
        match val {
            0b00 => Self::Low,
            0b01 => Self::Neutral,
            0b11 => Self::High,
            _ => Self::Neutral,
        }
    }
}

impl From<OutputConfig> for u8 {
    fn from(val: OutputConfig) -> Self {
        val as u8
    }
}

/// Options to drive the GPIO.
pub enum OutputDrive {
    Low = 0b0,
    High = 0b1,
    HighLow = 0b10,
    HighHigh = 0b11,
}

impl From<u8> for OutputDrive {
    fn from(val: u8) -> Self {
        match val & 0b11 {
            0b00 => Self::Low,
            0b01 => Self::High,
            0b10 => Self::HighLow,
            0b11 => Self::HighHigh,
            _ => Self::Low,
        }
    }
}

impl From<bool> for OutputDrive {
    fn from(val: bool) -> Self {
        (val as u8).into()
    }
}

impl From<OutputDrive> for u8 {
    fn from(val: OutputDrive) -> Self {
        val as u8
    }
}

/// Represents a GPIO peripheral on a JH71xx-based board.
pub struct Gpio<'g, GPIO: GpioCfg, ENABLED, DIRECTION, MODE> {
    periph: &'g GPIO,
    _enabled: ENABLED,
    _direction: DIRECTION,
    _mode: MODE,
}

impl<'g, GPIO: GpioCfg, ENABLED, DIRECTION, MODE> Gpio<'g, GPIO, ENABLED, DIRECTION, MODE> {
    /// Converts the [Gpio] into a disabled pin.
    pub fn into_disabled(mut self) -> Gpio<'g, GPIO, Disabled, Nop, Nop> {
        self.periph.input_enable(false);
        self.disable_output();

        Gpio {
            periph: self.periph,
            _enabled: Disabled,
            _direction: Nop,
            _mode: Nop,
        }
    }

    /// Converts the [Gpio] into an enabled input.
    pub fn into_enabled_input(mut self) -> Gpio<'g, GPIO, Enabled, Input, HiZ> {
        self.periph.input_enable(true);
        self.periph.set_high_z();
        self.disable_output();

        Gpio {
            periph: self.periph,
            _enabled: Enabled,
            _direction: Input,
            _mode: HiZ,
        }
    }

    /// Converts the [Gpio] into an enabled output.
    pub fn into_enabled_output(mut self) -> Gpio<'g, GPIO, Enabled, Output, Nop> {
        self.periph.input_enable(false);
        self.enable_output();

        Gpio {
            periph: self.periph,
            _enabled: Enabled,
            _direction: Output,
            _mode: Nop,
        }
    }

    fn enable_output(&mut self) {
        self.config_output(OutputConfig::Low);
    }

    fn disable_output(&mut self) {
        self.config_output(OutputConfig::Neutral);
    }

    fn config_output(&mut self, config: OutputConfig) {
        let pinctrl = unsafe { &*SysPinctrl::ptr() };
        let pad = GPIO::pad();

        // StarFive uses a GPIO muxer, the lower two bits of the DOEN registers configure
        // SET_LOW and SET_HIGH, respectively
        //
        // Somewhat counter-intuitively:
        // - setting SET_LOW to zero enables the GPIO driven low
        // - setting SET_HIGH to one enables the GPIO driven high
        // - setting SET_LOW(1) SET_HIGH(0) brings the GPIO to driven neutral
        let cfg: u8 = config.into();
        match pad {
            0 => pinctrl
                .gpo_doen()
                .gpo_doen0()
                .modify(|_, w| w.doen0().variant(cfg)),
            1 => pinctrl
                .gpo_doen()
                .gpo_doen0()
                .modify(|_, w| w.doen1().variant(cfg)),
            2 => pinctrl
                .gpo_doen()
                .gpo_doen0()
                .modify(|_, w| w.doen2().variant(cfg)),
            3 => pinctrl
                .gpo_doen()
                .gpo_doen0()
                .modify(|_, w| w.doen3().variant(cfg)),
            4 => pinctrl
                .gpo_doen()
                .gpo_doen1()
                .modify(|_, w| w.doen4().variant(cfg)),
            5 => pinctrl
                .gpo_doen()
                .gpo_doen1()
                .modify(|_, w| w.doen5().variant(cfg)),
            6 => pinctrl
                .gpo_doen()
                .gpo_doen1()
                .modify(|_, w| w.doen6().variant(cfg)),
            7 => pinctrl
                .gpo_doen()
                .gpo_doen1()
                .modify(|_, w| w.doen7().variant(cfg)),
            8 => pinctrl
                .gpo_doen()
                .gpo_doen2()
                .modify(|_, w| w.doen8().variant(cfg)),
            9 => pinctrl
                .gpo_doen()
                .gpo_doen2()
                .modify(|_, w| w.doen9().variant(cfg)),
            10 => pinctrl
                .gpo_doen()
                .gpo_doen2()
                .modify(|_, w| w.doen10().variant(cfg)),
            11 => pinctrl
                .gpo_doen()
                .gpo_doen2()
                .modify(|_, w| w.doen11().variant(cfg)),
            12 => pinctrl
                .gpo_doen()
                .gpo_doen3()
                .modify(|_, w| w.doen12().variant(cfg)),
            13 => pinctrl
                .gpo_doen()
                .gpo_doen3()
                .modify(|_, w| w.doen13().variant(cfg)),
            14 => pinctrl
                .gpo_doen()
                .gpo_doen3()
                .modify(|_, w| w.doen14().variant(cfg)),
            15 => pinctrl
                .gpo_doen()
                .gpo_doen3()
                .modify(|_, w| w.doen15().variant(cfg)),
            16 => pinctrl
                .gpo_doen()
                .gpo_doen4()
                .modify(|_, w| w.doen16().variant(cfg)),
            17 => pinctrl
                .gpo_doen()
                .gpo_doen4()
                .modify(|_, w| w.doen17().variant(cfg)),
            18 => pinctrl
                .gpo_doen()
                .gpo_doen4()
                .modify(|_, w| w.doen18().variant(cfg)),
            19 => pinctrl
                .gpo_doen()
                .gpo_doen4()
                .modify(|_, w| w.doen19().variant(cfg)),
            20 => pinctrl
                .gpo_doen()
                .gpo_doen5()
                .modify(|_, w| w.doen20().variant(cfg)),
            21 => pinctrl
                .gpo_doen()
                .gpo_doen5()
                .modify(|_, w| w.doen21().variant(cfg)),
            22 => pinctrl
                .gpo_doen()
                .gpo_doen5()
                .modify(|_, w| w.doen22().variant(cfg)),
            23 => pinctrl
                .gpo_doen()
                .gpo_doen5()
                .modify(|_, w| w.doen23().variant(cfg)),
            24 => pinctrl
                .gpo_doen()
                .gpo_doen6()
                .modify(|_, w| w.doen24().variant(cfg)),
            25 => pinctrl
                .gpo_doen()
                .gpo_doen6()
                .modify(|_, w| w.doen25().variant(cfg)),
            26 => pinctrl
                .gpo_doen()
                .gpo_doen6()
                .modify(|_, w| w.doen26().variant(cfg)),
            27 => pinctrl
                .gpo_doen()
                .gpo_doen6()
                .modify(|_, w| w.doen27().variant(cfg)),
            28 => pinctrl
                .gpo_doen()
                .gpo_doen7()
                .modify(|_, w| w.doen28().variant(cfg)),
            29 => pinctrl
                .gpo_doen()
                .gpo_doen7()
                .modify(|_, w| w.doen29().variant(cfg)),
            30 => pinctrl
                .gpo_doen()
                .gpo_doen7()
                .modify(|_, w| w.doen30().variant(cfg)),
            31 => pinctrl
                .gpo_doen()
                .gpo_doen7()
                .modify(|_, w| w.doen31().variant(cfg)),
            32 => pinctrl
                .gpo_doen()
                .gpo_doen8()
                .modify(|_, w| w.doen32().variant(cfg)),
            33 => pinctrl
                .gpo_doen()
                .gpo_doen8()
                .modify(|_, w| w.doen33().variant(cfg)),
            34 => pinctrl
                .gpo_doen()
                .gpo_doen8()
                .modify(|_, w| w.doen34().variant(cfg)),
            35 => pinctrl
                .gpo_doen()
                .gpo_doen8()
                .modify(|_, w| w.doen35().variant(cfg)),
            36 => pinctrl
                .gpo_doen()
                .gpo_doen9()
                .modify(|_, w| w.doen36().variant(cfg)),
            37 => pinctrl
                .gpo_doen()
                .gpo_doen9()
                .modify(|_, w| w.doen37().variant(cfg)),
            38 => pinctrl
                .gpo_doen()
                .gpo_doen9()
                .modify(|_, w| w.doen38().variant(cfg)),
            39 => pinctrl
                .gpo_doen()
                .gpo_doen9()
                .modify(|_, w| w.doen39().variant(cfg)),
            40 => pinctrl
                .gpo_doen()
                .gpo_doen10()
                .modify(|_, w| w.doen40().variant(cfg)),
            41 => pinctrl
                .gpo_doen()
                .gpo_doen10()
                .modify(|_, w| w.doen41().variant(cfg)),
            42 => pinctrl
                .gpo_doen()
                .gpo_doen10()
                .modify(|_, w| w.doen42().variant(cfg)),
            43 => pinctrl
                .gpo_doen()
                .gpo_doen10()
                .modify(|_, w| w.doen43().variant(cfg)),
            44 => pinctrl
                .gpo_doen()
                .gpo_doen11()
                .modify(|_, w| w.doen44().variant(cfg)),
            45 => pinctrl
                .gpo_doen()
                .gpo_doen11()
                .modify(|_, w| w.doen45().variant(cfg)),
            46 => pinctrl
                .gpo_doen()
                .gpo_doen11()
                .modify(|_, w| w.doen46().variant(cfg)),
            47 => pinctrl
                .gpo_doen()
                .gpo_doen11()
                .modify(|_, w| w.doen47().variant(cfg)),
            48 => pinctrl
                .gpo_doen()
                .gpo_doen12()
                .modify(|_, w| w.doen48().variant(cfg)),
            49 => pinctrl
                .gpo_doen()
                .gpo_doen12()
                .modify(|_, w| w.doen49().variant(cfg)),
            50 => pinctrl
                .gpo_doen()
                .gpo_doen12()
                .modify(|_, w| w.doen50().variant(cfg)),
            51 => pinctrl
                .gpo_doen()
                .gpo_doen12()
                .modify(|_, w| w.doen51().variant(cfg)),
            52 => pinctrl
                .gpo_doen()
                .gpo_doen13()
                .modify(|_, w| w.doen52().variant(cfg)),
            53 => pinctrl
                .gpo_doen()
                .gpo_doen13()
                .modify(|_, w| w.doen53().variant(cfg)),
            54 => pinctrl
                .gpo_doen()
                .gpo_doen13()
                .modify(|_, w| w.doen54().variant(cfg)),
            55 => pinctrl
                .gpo_doen()
                .gpo_doen13()
                .modify(|_, w| w.doen55().variant(cfg)),
            56 => pinctrl
                .gpo_doen()
                .gpo_doen14()
                .modify(|_, w| w.doen56().variant(cfg)),
            57 => pinctrl
                .gpo_doen()
                .gpo_doen14()
                .modify(|_, w| w.doen57().variant(cfg)),
            58 => pinctrl
                .gpo_doen()
                .gpo_doen14()
                .modify(|_, w| w.doen58().variant(cfg)),
            59 => pinctrl
                .gpo_doen()
                .gpo_doen14()
                .modify(|_, w| w.doen59().variant(cfg)),
            60 => pinctrl
                .gpo_doen()
                .gpo_doen15()
                .modify(|_, w| w.doen60().variant(cfg)),
            61 => pinctrl
                .gpo_doen()
                .gpo_doen15()
                .modify(|_, w| w.doen61().variant(cfg)),
            62 => pinctrl
                .gpo_doen()
                .gpo_doen15()
                .modify(|_, w| w.doen62().variant(cfg)),
            63 => pinctrl
                .gpo_doen()
                .gpo_doen15()
                .modify(|_, w| w.doen63().variant(cfg)),
            _ => (),
        }
    }
}

impl<'g, GPIO: GpioCfg> Gpio<'g, GPIO, Enabled, Output, Nop> {
    /// Sets whether the [Gpio] is driven high.
    pub fn set_pin(&mut self, high: bool) {
        self.drive_output(high.into())
    }

    fn drive_output(&mut self, drive: OutputDrive) {
        let pinctrl = unsafe { &*SysPinctrl::ptr() };
        let pad = GPIO::pad();

        let val: u8 = drive.into();
        match pad {
            0 => pinctrl
                .gpo_dout()
                .gpo_dout0()
                .modify(|_, w| w.dout0().variant(val)),
            1 => pinctrl
                .gpo_dout()
                .gpo_dout0()
                .modify(|_, w| w.dout1().variant(val)),
            2 => pinctrl
                .gpo_dout()
                .gpo_dout0()
                .modify(|_, w| w.dout2().variant(val)),
            3 => pinctrl
                .gpo_dout()
                .gpo_dout0()
                .modify(|_, w| w.dout3().variant(val)),
            4 => pinctrl
                .gpo_dout()
                .gpo_dout1()
                .modify(|_, w| w.dout4().variant(val)),
            5 => pinctrl
                .gpo_dout()
                .gpo_dout1()
                .modify(|_, w| w.dout5().variant(val)),
            6 => pinctrl
                .gpo_dout()
                .gpo_dout1()
                .modify(|_, w| w.dout6().variant(val)),
            7 => pinctrl
                .gpo_dout()
                .gpo_dout1()
                .modify(|_, w| w.dout7().variant(val)),
            8 => pinctrl
                .gpo_dout()
                .gpo_dout2()
                .modify(|_, w| w.dout8().variant(val)),
            9 => pinctrl
                .gpo_dout()
                .gpo_dout2()
                .modify(|_, w| w.dout9().variant(val)),
            10 => pinctrl
                .gpo_dout()
                .gpo_dout2()
                .modify(|_, w| w.dout10().variant(val)),
            11 => pinctrl
                .gpo_dout()
                .gpo_dout2()
                .modify(|_, w| w.dout11().variant(val)),
            12 => pinctrl
                .gpo_dout()
                .gpo_dout3()
                .modify(|_, w| w.dout12().variant(val)),
            13 => pinctrl
                .gpo_dout()
                .gpo_dout3()
                .modify(|_, w| w.dout13().variant(val)),
            14 => pinctrl
                .gpo_dout()
                .gpo_dout3()
                .modify(|_, w| w.dout14().variant(val)),
            15 => pinctrl
                .gpo_dout()
                .gpo_dout3()
                .modify(|_, w| w.dout15().variant(val)),
            16 => pinctrl
                .gpo_dout()
                .gpo_dout4()
                .modify(|_, w| w.dout16().variant(val)),
            17 => pinctrl
                .gpo_dout()
                .gpo_dout4()
                .modify(|_, w| w.dout17().variant(val)),
            18 => pinctrl
                .gpo_dout()
                .gpo_dout4()
                .modify(|_, w| w.dout18().variant(val)),
            19 => pinctrl
                .gpo_dout()
                .gpo_dout4()
                .modify(|_, w| w.dout19().variant(val)),
            20 => pinctrl
                .gpo_dout()
                .gpo_dout5()
                .modify(|_, w| w.dout20().variant(val)),
            21 => pinctrl
                .gpo_dout()
                .gpo_dout5()
                .modify(|_, w| w.dout21().variant(val)),
            22 => pinctrl
                .gpo_dout()
                .gpo_dout5()
                .modify(|_, w| w.dout22().variant(val)),
            23 => pinctrl
                .gpo_dout()
                .gpo_dout5()
                .modify(|_, w| w.dout23().variant(val)),
            24 => pinctrl
                .gpo_dout()
                .gpo_dout6()
                .modify(|_, w| w.dout24().variant(val)),
            25 => pinctrl
                .gpo_dout()
                .gpo_dout6()
                .modify(|_, w| w.dout25().variant(val)),
            26 => pinctrl
                .gpo_dout()
                .gpo_dout6()
                .modify(|_, w| w.dout26().variant(val)),
            27 => pinctrl
                .gpo_dout()
                .gpo_dout6()
                .modify(|_, w| w.dout27().variant(val)),
            28 => pinctrl
                .gpo_dout()
                .gpo_dout7()
                .modify(|_, w| w.dout28().variant(val)),
            29 => pinctrl
                .gpo_dout()
                .gpo_dout7()
                .modify(|_, w| w.dout29().variant(val)),
            30 => pinctrl
                .gpo_dout()
                .gpo_dout7()
                .modify(|_, w| w.dout30().variant(val)),
            31 => pinctrl
                .gpo_dout()
                .gpo_dout7()
                .modify(|_, w| w.dout31().variant(val)),
            32 => pinctrl
                .gpo_dout()
                .gpo_dout8()
                .modify(|_, w| w.dout32().variant(val)),
            33 => pinctrl
                .gpo_dout()
                .gpo_dout8()
                .modify(|_, w| w.dout33().variant(val)),
            34 => pinctrl
                .gpo_dout()
                .gpo_dout8()
                .modify(|_, w| w.dout34().variant(val)),
            35 => pinctrl
                .gpo_dout()
                .gpo_dout8()
                .modify(|_, w| w.dout35().variant(val)),
            36 => pinctrl
                .gpo_dout()
                .gpo_dout9()
                .modify(|_, w| w.dout36().variant(val)),
            37 => pinctrl
                .gpo_dout()
                .gpo_dout9()
                .modify(|_, w| w.dout37().variant(val)),
            38 => pinctrl
                .gpo_dout()
                .gpo_dout9()
                .modify(|_, w| w.dout38().variant(val)),
            39 => pinctrl
                .gpo_dout()
                .gpo_dout9()
                .modify(|_, w| w.dout39().variant(val)),
            40 => pinctrl
                .gpo_dout()
                .gpo_dout10()
                .modify(|_, w| w.dout40().variant(val)),
            41 => pinctrl
                .gpo_dout()
                .gpo_dout10()
                .modify(|_, w| w.dout41().variant(val)),
            42 => pinctrl
                .gpo_dout()
                .gpo_dout10()
                .modify(|_, w| w.dout42().variant(val)),
            43 => pinctrl
                .gpo_dout()
                .gpo_dout10()
                .modify(|_, w| w.dout43().variant(val)),
            44 => pinctrl
                .gpo_dout()
                .gpo_dout11()
                .modify(|_, w| w.dout44().variant(val)),
            45 => pinctrl
                .gpo_dout()
                .gpo_dout11()
                .modify(|_, w| w.dout45().variant(val)),
            46 => pinctrl
                .gpo_dout()
                .gpo_dout11()
                .modify(|_, w| w.dout46().variant(val)),
            47 => pinctrl
                .gpo_dout()
                .gpo_dout11()
                .modify(|_, w| w.dout47().variant(val)),
            48 => pinctrl
                .gpo_dout()
                .gpo_dout12()
                .modify(|_, w| w.dout48().variant(val)),
            49 => pinctrl
                .gpo_dout()
                .gpo_dout12()
                .modify(|_, w| w.dout49().variant(val)),
            50 => pinctrl
                .gpo_dout()
                .gpo_dout12()
                .modify(|_, w| w.dout50().variant(val)),
            51 => pinctrl
                .gpo_dout()
                .gpo_dout12()
                .modify(|_, w| w.dout51().variant(val)),
            52 => pinctrl
                .gpo_dout()
                .gpo_dout13()
                .modify(|_, w| w.dout52().variant(val)),
            53 => pinctrl
                .gpo_dout()
                .gpo_dout13()
                .modify(|_, w| w.dout53().variant(val)),
            54 => pinctrl
                .gpo_dout()
                .gpo_dout13()
                .modify(|_, w| w.dout54().variant(val)),
            55 => pinctrl
                .gpo_dout()
                .gpo_dout13()
                .modify(|_, w| w.dout55().variant(val)),
            56 => pinctrl
                .gpo_dout()
                .gpo_dout14()
                .modify(|_, w| w.dout56().variant(val)),
            57 => pinctrl
                .gpo_dout()
                .gpo_dout14()
                .modify(|_, w| w.dout57().variant(val)),
            58 => pinctrl
                .gpo_dout()
                .gpo_dout14()
                .modify(|_, w| w.dout58().variant(val)),
            59 => pinctrl
                .gpo_dout()
                .gpo_dout14()
                .modify(|_, w| w.dout59().variant(val)),
            60 => pinctrl
                .gpo_dout()
                .gpo_dout15()
                .modify(|_, w| w.dout60().variant(val)),
            61 => pinctrl
                .gpo_dout()
                .gpo_dout15()
                .modify(|_, w| w.dout61().variant(val)),
            62 => pinctrl
                .gpo_dout()
                .gpo_dout15()
                .modify(|_, w| w.dout62().variant(val)),
            63 => pinctrl
                .gpo_dout()
                .gpo_dout15()
                .modify(|_, w| w.dout63().variant(val)),
            _ => (),
        }
    }
}

impl<'g, GPIO: GpioCfg, MODE> Gpio<'g, GPIO, Enabled, Input, MODE> {
    /// Gets whether the input pin is set.
    pub fn bit_is_set(&self) -> bool {
        // [`IOIRQ_15`] and [`IOIRQ_16`] are the GPIO sync registers, for GPIO 0-31 and 32-63
        // respectively.
        //
        // SAFETY:
        //
        // It is safe to access to IOIRQ15/16 because they are `read-only`.
        // Their values are only changed by the hardware.
        let pinctrl = unsafe { &*SysPinctrl::ptr() };

        let pad = GPIO::pad();
        let pad_per_reg = u32::from(Pad::Gpio32);

        if pad < pad_per_reg {
            (pinctrl.ioirq().ioirq15().read().bits() >> pad) & 0x1 != 0
        } else if pad < u32::from(Pad::Gpio63) {
            let idx = pad.saturating_sub(pad_per_reg);
            (pinctrl.ioirq().ioirq16().read().bits() >> idx) & 0x1 != 0
        } else {
            false
        }
    }

    /// Converts the [Gpio] into a high-impedance input.
    pub fn into_input_high_z(self) -> Gpio<'g, GPIO, Enabled, Input, HiZ> {
        self.periph.set_high_z();

        Gpio {
            periph: self.periph,
            _enabled: Enabled,
            _direction: Input,
            _mode: HiZ,
        }
    }

    /// Converts the [Gpio] into a pull-down input.
    pub fn into_input_pull_down(self) -> Gpio<'g, GPIO, Enabled, Input, PullDown> {
        self.periph.set_pull_down();

        Gpio {
            periph: self.periph,
            _enabled: Enabled,
            _direction: Input,
            _mode: PullDown,
        }
    }

    /// Converts the [Gpio] into a pull-up input.
    pub fn into_input_pull_up(self) -> Gpio<'g, GPIO, Enabled, Input, PullUp> {
        self.periph.set_pull_up();

        Gpio {
            periph: self.periph,
            _enabled: Enabled,
            _direction: Input,
            _mode: PullUp,
        }
    }
}

impl<'g, GPIO: GpioCfg, ENABLED, DIRECTION, MODE> ErrorType
    for Gpio<'g, GPIO, ENABLED, DIRECTION, MODE>
{
    type Error = Error;
}

impl<'g, GPIO: GpioCfg> OutputPin for Gpio<'g, GPIO, Enabled, Output, Nop> {
    fn set_low(&mut self) -> Result<()> {
        self.set_pin(false);
        Ok(())
    }

    fn set_high(&mut self) -> Result<()> {
        self.set_pin(true);
        Ok(())
    }
}

impl<'g, GPIO: GpioCfg, MODE> InputPin for Gpio<'g, GPIO, Enabled, Input, MODE> {
    fn is_low(&mut self) -> Result<bool> {
        self.is_high().map(|v| !v)
    }

    fn is_high(&mut self) -> Result<bool> {
        Ok(self.bit_is_set())
    }
}

/// Creates a new [Gpio].
///
/// Example:
///
/// ```no_run
/// use jh71xx_hal::{gpio, pac};
///
/// let dp = pac::Peripherals::take().unwrap();
/// let gpio0 = gpio::get_gpio(dp.sys_pinctrl.padcfg().gpio0());
/// ```
pub fn get_gpio<GPIO: GpioCfg>(periph: &GPIO) -> Gpio<GPIO, Disabled, Nop, Nop> {
    Gpio {
        periph,
        _enabled: Disabled,
        _direction: Nop,
        _mode: Nop,
    }
}
