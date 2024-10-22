#![no_std]

//! # JH71xx-HAL
//!
//! `jh71xx-hal` is a HAL (hardware abstraction layer) crate for `JH71xx`-based SoCs from StarFive.
//!
//! Currently, only the `JH7110` SoC is supported. `JH7100` appears to be discontinued, and the next SoC line from StarFive will be the `JH81xx` series.
//!
//! Please submit changes if you would like to add support for the `JH7100` SoC!
//!
//! See individual module documentation for more information and examples.

#[macro_use]
extern crate bitflags;

#[macro_use]
extern crate bitfield;

extern crate embedded_io as io;

pub extern crate jh7110_pac as pac;

pub mod clocks;
#[cfg(feature = "rt")]
pub mod critical_section;
pub mod ddr;
pub mod delay;
pub mod gpio;
pub mod i2c;
#[cfg(feature = "rt")]
pub mod interrupt;
#[macro_use]
mod macros;
pub mod mmc;
pub mod pll;
pub mod pwm;
pub mod register;
pub mod spi;
pub mod uart;
