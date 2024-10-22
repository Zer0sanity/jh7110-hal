//! MMC abstraction for the DesignWare MMC controllers used in JH7110 SoCs.
//!
//! Based on the `dw_mmc` driver in Linux: `drivers/mmc/host/dw_mmc*`.

use embedded_hal::delay::DelayNs;

use crate::delay;

mod command;
mod data;
mod error;
mod opcode;

pub use command::*;
pub use data::*;
pub use error::*;
pub use opcode::*;

pub const MAX_DELAY_CHAIN: u8 = 32;
pub const INT_LEN: usize = 32;

/// Common operations for DesignWare MMC controllers on JH7110 SoCs.
pub trait MmcOps {
    /// Sets the sample phase for the MMC controller.
    fn set_sample_phase(&mut self, sample_phase: u8);

    /// Executes MMC tuning.
    fn execute_tuning(&mut self, opcode: MmcOpcode);

    /// Clear all interrupts.
    fn clear_all_int(&mut self);
}

/// Represents the DesignWare MMC controller used in JH7110 SoCs.
pub struct Mmc<MMC: MmcOps> {
    mmc: MMC,
}

impl<MMC: MmcOps> Mmc<MMC> {
    /// Creates a new [Mmc].
    pub const fn new(mmc: MMC) -> Self {
        Self { mmc }
    }

    /// Releases the inner [MmcOps] instance.
    pub fn into_raw(self) -> MMC {
        self.mmc
    }
}

impl MmcOps for pac::Mmc1 {
    fn set_sample_phase(&mut self, sample_phase: u8) {
        self.uhs_reg_ext()
            .modify(|_, w| w.smpl_phase().variant(sample_phase));
        // We should delay 1ms wait for timing setting finished.
        if cfg!(feature = "rt") {
            delay::u74_mdelay().delay_ms(1);
        } else if cfg!(feature = "rts") {
            delay::u74_udelay().delay_ms(1);
        }
    }

    fn execute_tuning(&mut self, _opcode: MmcOpcode) {
        for smpl_phase in 0..MAX_DELAY_CHAIN {
            self.set_sample_phase(smpl_phase);
            self.clear_all_int();
        }
    }

    fn clear_all_int(&mut self) {
        self.rintsts().reset();
    }
}
