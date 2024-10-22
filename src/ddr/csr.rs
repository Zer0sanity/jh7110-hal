//! DDR CSR training and initialization algorithms.
//!
//! Implementations derived from [`oreboot`](https://github.com/oreboot/oreboot/blob/main/src/mainboard/starfive/visionfive2/bt0/src/ddrcsr.rs).

use embedded_hal::delay::DelayNs;

use super::*;
use crate::delay;

mod cfg;

pub use cfg::*;

const FREQ_CHANGE: usize = 0x0001;
const FREQ_CHANGE_ACK: usize = 0x0002;

const REQ_RES_REG1: usize = 0x0504;
const REQ_RES_REG2: usize = 0x050c;
const REQ_RES_REG3: usize = 0x0514;

const TRAINING_STATUS_MAYBE: usize = 0x0518;

impl Ddr {
    /// Performs DDR training algorithm.
    ///
    /// # Safety
    ///
    /// Caller must have exclusive access to the DDR MC PHY and Control memory range.
    fn train(&mut self, udelay: &mut delay::McycleDelay, training_status_reg: usize) {
        let freq_change_req = FREQ_CHANGE;

        while (self.dmc_ctrl.csr(training_status_reg >> 2).read().bits() & 0x2) != 0x0 {
            let req_type = self.dmc_phy.csr(freq_change_req).read().bits();
            if (req_type & 0x0000_0020) == 0x0000_0020 {
                let freq_change_req = req_type & 0x0000_001f;
                match freq_change_req {
                    0 => {
                        // DDRC_CLOCK = 12.5M
                        self.syscrg
                            .select_ddr_bus(clocks::ClkDdrBusMuxSel::ClkOscDiv2);
                    }
                    1 => {
                        // DDRC_CLOCK = 200M
                        self.syscrg
                            .select_ddr_bus(clocks::ClkDdrBusMuxSel::ClkPll1Div8);
                    }
                    2 => {
                        // DDRC_CLOCK = 400M
                        self.syscrg
                            .select_ddr_bus(clocks::ClkDdrBusMuxSel::ClkPll1Div2);
                    }
                    _ => (),
                }

                self.dmc_phy
                    .csr(FREQ_CHANGE_ACK)
                    .write(|w| w.csr().variant(0x1));

                while (self.dmc_phy.csr(FREQ_CHANGE_ACK).read().bits() & 0x1) != 0x0 {
                    udelay.delay_ns(2);
                }
            }

            udelay.delay_ns(1);
        }
    }

    /// Performs DDR MC CSR initialization algorithm.
    ///
    /// # Safety
    ///
    /// Caller must have exclusive access to the DDR MC PHY and Control memory range.
    pub fn omc_init(&mut self) {
        self.dmc_ctrl.csr(0).write(|w| w.csr().variant(0x1));

        DDR_CSR_CFG0.iter().for_each(|cfg| {
            self.dmc_ctrl
                .sec((cfg.reg_nr >> 2) as usize)
                .write(|w| w.sec().variant(cfg.value));
        });

        if cfg!(feature = "8G") || cfg!(feature = "4G") {
            self.dmc_ctrl
                .sec(0xf34 >> 2)
                .write(|w| w.sec().variant(0x1f00_0041));
        }

        self.dmc_ctrl
            .sec(0x0110 >> 2)
            .write(|w| w.sec().variant(0xc000_0001));

        self.dmc_ctrl
            .sec(0x0114 >> 2)
            .write(|w| w.sec().variant(0xffff_ffff));

        DDR_CSR_CFG1.iter().for_each(|cfg| {
            self.dmc_ctrl
                .csr((cfg.reg_nr >> 2) as usize)
                .write(|w| w.csr().variant(cfg.value));
        });

        let mut udelay = delay::u74_mdelay();

        // This seems to trigger some sort of readiness.
        // Memory frequency should be changed below 50MHz somewhere before here
        self.dmc_ctrl
            .csr(REQ_RES_REG1 >> 2)
            .write(|w| w.csr().variant(0x4000_0000));

        while self.dmc_ctrl.csr(REQ_RES_REG1 >> 2).read().bits() & 0x8000_0000 != 0x8000_0000 {
            udelay.delay_ns(1);
        }

        self.dmc_ctrl
            .csr(REQ_RES_REG1 >> 2)
            .write(|w| w.csr().variant(0x0000_0000));

        // tINIT0 is controlled by System
        self.dmc_ctrl
            .csr(REQ_RES_REG2 >> 2)
            .write(|w| w.csr().variant(0x0));

        // Waits tINIT1 (300 us): Minimum RESET_n LOW time after completion of
        // voltage ramp
        // NOTE: 200 us in VF1 code
        udelay.delay_ns(300);

        self.dmc_ctrl
            .csr(REQ_RES_REG2 >> 2)
            .write(|w| w.csr().variant(0x1));

        udelay.delay_ns(3000);

        // Drive CKE high (clock enable)
        let val = if cfg!(feature = "8G") || cfg!(feature = "4G") {
            0x000_0003c
        } else {
            0x0000_001c
        };

        self.dmc_ctrl
            .csr(0x0010 >> 2)
            .write(|w| w.csr().variant(val));

        self.dmc_ctrl
            .csr(0x0014 >> 2)
            .write(|w| w.csr().variant(0x0000_0001));

        // Waits tINIT5 (2 us): Minimum idle time before first MRW/MRR command
        udelay.delay_ns(4);
        DDR_CSR_CFG2.iter().for_each(|cfg| {
            self.dmc_ctrl
                .csr((cfg.reg_nr >> 2) as usize)
                .write(|w| w.csr().variant(cfg.value));
        });

        // Waits tZQCAL (1 us)
        udelay.delay_ns(4);

        self.dmc_ctrl
            .csr(0x0010 >> 2)
            .write(|w| w.csr().variant(0x0000_0011));

        self.dmc_ctrl
            .csr(0x0014 >> 2)
            .write(|w| w.csr().variant(0x0000_0001));

        if cfg!(feature = "8G") || cfg!(feature = "4G") {
            self.dmc_ctrl
                .csr(0x0010 >> 2)
                .write(|w| w.csr().variant(0x0000_0020));

            self.dmc_ctrl
                .csr(0x0014 >> 2)
                .write(|w| w.csr().variant(0x0000_0001));

            // Waits tZQCAL (1 us)
            udelay.delay_ns(4);

            self.dmc_ctrl
                .csr(0x0010 >> 2)
                .write(|w| w.csr().variant(0x0000_0021));

            self.dmc_ctrl
                .csr(0x0014 >> 2)
                .write(|w| w.csr().variant(0x0000_0001));
        }

        self.dmc_ctrl
            .csr(REQ_RES_REG3 >> 2)
            .write(|w| w.csr().variant(0x0000_0000));

        // This register seems to first indicate that we are ready for training,
        // and then, that training is done. See the train() function using the same
        // mask again.
        while self.dmc_ctrl.csr(TRAINING_STATUS_MAYBE >> 2).read().bits() & 0x2 != 0x2 {
            udelay.delay_ns(1);
        }

        self.train(&mut udelay, TRAINING_STATUS_MAYBE);

        // NOTE: This here even worked when I was accidentally off to 0x150 / 0x154.
        self.dmc_phy.base(0x14c >> 2).read().bits();
        let val = self.dmc_phy.base(85).read().bits();
        self.dmc_phy
            .base(81)
            .write(|w| w.base().variant(val & 0xf800_0000));

        DDR_CSR_CFG3.iter().for_each(|cfg| {
            self.dmc_phy
                .base((cfg.reg_nr >> 2) as usize)
                .modify(|r, w| w.base().variant((r.bits() & cfg.mask) | cfg.value));
        });

        DDR_CSR_CFG4.iter().for_each(|cfg| {
            self.dmc_ctrl
                .csr((cfg.reg_nr >> 2) as usize)
                .write(|w| w.csr().variant(cfg.value));
        });

        self.dmc_ctrl
            .sec(0x0704 >> 2)
            .write(|w| w.sec().variant(0x0000_0007));

        DDR_CSR_CFG5.iter().for_each(|cfg| {
            self.dmc_ctrl
                .csr((cfg.reg_nr >> 2) as usize)
                .write(|w| w.csr().variant(cfg.value));
        });

        self.dmc_ctrl
            .sec(0x0700 >> 2)
            .write(|w| w.sec().variant(0x0000_0003));

        self.dmc_ctrl
            .csr(0x0514 >> 2)
            .write(|w| w.csr().variant(0x0000_0600));

        self.dmc_ctrl
            .csr(0x0020 >> 2)
            .write(|w| w.csr().variant(0x0000_0001));
    }
}
