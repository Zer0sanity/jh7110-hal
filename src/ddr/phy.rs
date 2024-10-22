//! Functions and types for DDR PHY intiialization, training and configuration;
//!
//! Implementations derived from [`oreboot`](https://github.com/oreboot/oreboot/blob/main/src/mainboard/starfive/visionfive2/bt0/src/ddrphy.rs).

use super::*;

mod cfg;
mod data;

pub use cfg::*;
pub use data::*;

impl Ddr {
    /// Performs DDR PHY training algorithm.
    ///
    /// # Safety
    ///
    /// Caller must have exclusive access to DDR MC PHY memory area.
    pub fn phy_train(&mut self) {
        TRAIN_DATA.iter().enumerate().for_each(|(reg_nr, &value)| {
            self.dmc_phy.base(reg_nr).write(|w| w.base().variant(value));
        });
    }

    /// Utility function for DDR PHY training.
    ///
    /// First, we write everything from register 1792 on, then everything up to 1792.
    /// We do not know why, just copied it from the reference implementation.
    ///
    /// # Safety
    ///
    /// Caller must have exclusive access to the DDR MC PHY memory area.
    pub fn phy_util(&mut self) {
        UTIL_DATA2.iter().enumerate().for_each(|(reg_nr, &value)| {
            self.dmc_phy
                .ac_base(1792 + reg_nr)
                .write(|w| w.ac_base().variant(value));
        });

        // 4 blocks of 256 each
        [0x1, 0x0, 0x1, 0x0]
            .into_iter()
            .enumerate()
            .for_each(|(i, val)| self.util_data1_a(256 * i, val));

        // 3 blocks of 256 each
        [
            (0x0a41_8820, 0x003f_0000, 0x013f),
            (0x0, 0x0, 0x0),
            (0x0, 0x1000_0000, 0x0),
        ]
        .into_iter()
        .enumerate()
        .for_each(|(i, (v1, v2, v3))| self.util_data1_b(256 * (i + 4), v1, v2, v3));
    }

    fn util_data1_a(&mut self, base: usize, v1: u32) {
        UTIL_DATA1_A1
            .iter()
            .enumerate()
            .for_each(|(reg_nr, &value)| {
                self.dmc_phy
                    .ac_base(base + reg_nr)
                    .write(|w| w.ac_base().variant(value));
            });

        self.dmc_phy
            .ac_base(base + 44)
            .write(|w| w.ac_base().variant(v1));

        UTIL_DATA1_A2
            .iter()
            .enumerate()
            .for_each(|(reg_nr, &value)| {
                self.dmc_phy
                    .ac_base(base + 45 + reg_nr)
                    .write(|w| w.ac_base().variant(value));
            });

        (131..256).for_each(|r| {
            self.dmc_phy
                .ac_base(base + r)
                .write(|w| w.ac_base().variant(0))
        })
    }

    fn util_data1_b(&mut self, base: usize, v1: u32, v2: u32, v3: u32) {
        UTIL_DATA1_B1
            .iter()
            .enumerate()
            .for_each(|(reg_nr, &value)| {
                self.dmc_phy
                    .ac_base(base + reg_nr)
                    .write(|w| w.ac_base().variant(value));
            });

        self.dmc_phy
            .ac_base(base + 30)
            .write(|w| w.ac_base().variant(v1));
        self.dmc_phy
            .ac_base(base + 31)
            .write(|w| w.ac_base().variant(v2));
        self.dmc_phy
            .ac_base(base + 32)
            .write(|w| w.ac_base().variant(v3));

        UTIL_DATA1_B2
            .iter()
            .enumerate()
            .for_each(|(reg_nr, &value)| {
                self.dmc_phy
                    .ac_base(base + 33 + reg_nr)
                    .write(|w| w.ac_base().variant(value));
            });

        (49..256).for_each(|r| {
            self.dmc_phy
                .ac_base(base + r)
                .write(|w| w.ac_base().variant(0));
        })
    }

    /// Performs DDR PHY startup algorithm.
    ///
    /// # Safety
    ///
    /// Caller must have exlusive access to the DDR memory range.
    ///
    /// Typically, this means calling from the monitor core in Supervisor mode during bootloader initialization.
    pub fn phy_start(&mut self) {
        START_CFG0.iter().for_each(|cfg| {
            self.dmc_phy
                .ac_base(cfg.reg_nr as usize)
                .modify(|r, w| w.ac_base().variant((r.bits() & cfg.mask) | cfg.value));
        });

        START_CFG1.iter().for_each(|cfg| {
            self.dmc_phy
                .base(cfg.reg_nr as usize)
                .modify(|r, w| w.base().variant((r.bits() & cfg.mask) | cfg.value));
        });

        // NOTE: Commented out in VF1 code
        if cfg!(feature = "2G") {
            self.dmc_phy
                .base(11)
                .modify(|r, w| w.base().variant((r.bits() & 0xffff_fff0) | 0x0000_0005));
        }

        START_CFG2.iter().for_each(|cfg| {
            self.dmc_phy
                .base(cfg.reg_nr as usize)
                .modify(|r, w| w.base().variant((r.bits() & cfg.mask) | cfg.value));
        });

        START_CFG3.iter().for_each(|cfg| {
            self.dmc_phy
                .ac_base(cfg.reg_nr as usize)
                .modify(|r, w| w.ac_base().variant((r.bits() & cfg.mask) | cfg.value));
        });

        // PHY_RPTR_UPDATE_x: bit[11:8]+=3
        // NOTE: Special handling: write back current val + val to register
        // That was the behavior in the previous implementation, too. No clue why...
        // The registers might be reset to 0 on cold boot. If they retain their
        // current value on hot reset, it could turn into weird behavior.
        // With `START_CFG0`, we set this to `0x00000400`, so it should now become
        // `0x00000700`.
        [96, 352, 608, 864].iter().for_each(|&reg| {
            self.dmc_phy
                .ac_base(reg)
                .modify(|r, w| w.ac_base().variant(r.bits() + 0x300));
        });

        // PHY_WRLVL_DLY_STEP_X: 8'hC -> 8'h12
        // NOTE: This is h18 in the JH7100 code
        // This is for G_SPEED_2133.
        //    G_SPEED_2666: 0x00140000
        //    G_SPEED_3200: 0x00180000
        // TODO: try lower speed?
        [96, 352, 608, 864].iter().for_each(|&reg| {
            self.dmc_phy
                .ac_base(reg)
                .modify(|r, w| w.ac_base().variant((r.bits() & 0xff00_ffff) | 0x0012_0000));
        });

        START_CFG4.iter().for_each(|cfg| {
            self.dmc_phy
                .ac_base(cfg.reg_nr as usize)
                .modify(|r, w| w.ac_base().variant((r.bits() & cfg.mask) | cfg.value));
        });

        START_CFG5.iter().for_each(|cfg| {
            self.dmc_phy
                .ac_base(cfg.reg_nr as usize)
                .write(|w| w.ac_base().variant(cfg.value));
        });

        START_CFG6.iter().for_each(|cfg| {
            self.dmc_phy
                .ac_base(cfg.reg_nr as usize)
                .modify(|r, w| w.ac_base().variant((r.bits() & cfg.mask) | cfg.value));
        });

        START_CFG7.iter().for_each(|cfg| {
            self.dmc_phy
                .ac_base(cfg.reg_nr as usize)
                .write(|w| w.ac_base().variant(cfg.value));
        });

        START_CFG8.iter().for_each(|cfg| {
            self.dmc_phy
                .ac_base(cfg.reg_nr as usize)
                .modify(|r, w| w.ac_base().variant((r.bits() & cfg.mask) | cfg.value));
        });

        self.dmc_phy.csr(0).write(|w| w.csr().variant(0x1));
    }
}
