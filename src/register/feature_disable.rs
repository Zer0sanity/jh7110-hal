//! Representation of the custom SiFive CSR for disabling U74 (MC) SoC features.
//!
//! Ss. 7.6 SiFive Feature Disable CSR, "U74MC Core Complex Manual 21 G1"
//!
//! <https://starfivetech.com/uploads/u74_core_complex_manual_21G1.pdf>

use crate::{clear_csr, read_as_csr, set_clear_csr, set_csr, write_as_csr};

/// Bit-field mask that represents the settable fields in the [FeatureDisable] CSR.
pub const FIELD_MASK: usize = 0b11_0000_0010_0000_1111;

/// Represents the custom SiFive CSR for disabling U74 (MC) SoC features.
#[derive(Clone, Copy, Debug)]
pub struct FeatureDisable {
    bits: usize,
}

impl FeatureDisable {
    /// Returns the contents of the register as raw bits.
    #[inline]
    pub fn bits(&self) -> usize {
        self.bits
    }

    /// Disable data cache clock gating.
    #[inline]
    pub fn dccg(&self) -> bool {
        self.bits & (1 << 0) != 0
    }

    /// Disable instruction cache clock gating.
    #[inline]
    pub fn iccg(&self) -> bool {
        self.bits & (1 << 1) != 0
    }

    /// Disable pipeline clock gating.
    #[inline]
    pub fn pcg(&self) -> bool {
        self.bits & (1 << 2) != 0
    }

    /// Disable speculative instruction cache refill.
    #[inline]
    pub fn sicr(&self) -> bool {
        self.bits & (1 << 3) != 0
    }

    /// Suppress corrupt signal on GrantData messages.
    #[inline]
    pub fn csgdm(&self) -> bool {
        self.bits & (1 << 9) != 0
    }

    /// Disable short forward branch optimization.
    #[inline]
    pub fn sfbo(&self) -> bool {
        self.bits & (1 << 16) != 0
    }

    /// Disable instruction cache next-line prefetcher.
    #[inline]
    pub fn icnlp(&self) -> bool {
        self.bits & (1 << 17) != 0
    }
}

read_as_csr!(
    /// Reads the [FeatureDisable] from the platform CSR.
    , FeatureDisable, 0x7c1);

write_as_csr!(
    /// Writes the [FeatureDisable] in-memory value to the platform CSR.
    , FeatureDisable, 0x7c1);

set_csr!(0x7c1);
clear_csr!(0x7c1);

set_clear_csr!(
    /// Disable data cache clock gating.
    , set_dccg, clear_dccg, 1 << 0);
set_clear_csr!(
    /// Disable instruction cache clock gating.
    , set_iccg, clear_iccg, 1 << 1);
set_clear_csr!(
    /// Disable pipeline clock gating.
    , set_pcg, clear_pcg, 1 << 2);
set_clear_csr!(
    /// Disable speculative instruction cache refill.
    , set_sicr, clear_sicr, 1 << 3);
set_clear_csr!(
    /// Suppress corrupt signal on GrantData messages.
    , set_csgdm, clear_csgdm, 1 << 9);
set_clear_csr!(
    /// Disable short forward branch optimization.
    , set_sfbo, clear_sfbo, 1 << 16);
set_clear_csr!(
    /// Disable instruction cache next-line prefetcher.
    , set_icnlp, clear_icnlp, 1 << 17);
set_clear_csr!(
    /// Disable all features.
    , set_all, clear_all, FIELD_MASK);
