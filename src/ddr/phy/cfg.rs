use crate::ddr::{MemCfg, MemSet};
use crate::{mem_cfg_arr, mem_set_arr};

#[cfg(feature = "8G")]
pub const VAL_X2: u32 = 0x3600_0000;
#[cfg(not(feature = "8G"))]
pub const VAL_X2: u32 = 0x6600_0000;

#[cfg(feature = "8G")]
pub const VAL_X3: u32 = 0xff;
#[cfg(not(feature = "8G"))]
pub const VAL_X3: u32 = 0xfb;

#[rustfmt::skip]
pub const START_CFG0: [MemCfg; 41] = mem_cfg_arr![
    {89, 0xffff_ff00, 0x0000_0051},
    //disable RDLVL VREF
    {78,  0xffff_fcff, 0x0},
    {345, 0xffff_ff00, 0x0000_0051},
    //disable RDLVL VREF
    {334, 0xffff_fcff, 0x0},
    {601, 0xffff_ff00, 0x0000_0051},
    //disable RDLVL VREF
    {590, 0xffff_fcff, 0x0},
    {857, 0xffff_ff00, 0x0000_0051},
    //disable RDLVL VREF
    {846, 0xffff_fcff, 0x0},

    //turn off multicast
    {1793, 0xffff_feff, 0x0},
    //set to freq copy 0
    {1793, 0xfffc_ffff, 0x0},

    //data slice registers
    {125, 0xfff0_ffff, 0x0001_0000},
    {102, 0xffff_fffc, 0x0000_0001},
    {105, 0xffff_ffe0, 0x0000_0001},
    {92,  0xffff_fffe, 0x0000_0001},
    {94,  0xffff_e0ff, 0x0000_0200},
    {96,  0xffff_f0ff, 0x0000_0400},
    {89,  0xffff_ff00, 0x0000_0051},

    {381, 0xfff0_ffff, 0x0001_0000},
    {358, 0xffff_fffc, 0x0000_0001},
    {361, 0xffff_ffe0, 0x0000_0001},
    {348, 0xffff_fffe, 0x0000_0001},
    {350, 0xffff_e0ff, 0x0000_0200},
    {352, 0xffff_f0ff, 0x0000_0400},
    {345, 0xffff_ff00, 0x0000_0051},

    {637, 0xfff0_ffff, 0x0001_0000},
    {614, 0xffff_fffc, 0x0000_0001},
    {617, 0xffff_ffe0, 0x0000_0001},
    {604, 0xffff_fffe, 0x0000_0001},
    {606, 0xffff_e0ff, 0x0000_0200},
    {608, 0xffff_f0ff, 0x0000_0400},
    {601, 0xffff_ff00, 0x0000_0051},

    {893, 0xfff0_ffff, 0x0001_0000},
    {870, 0xffff_fffc, 0x0000_0001},
    {873, 0xffff_ffe0, 0x0000_0001},
    {860, 0xffff_fffe, 0x0000_0001},
    {862, 0xffff_e0ff, 0x0000_0200},
    {864, 0xffff_f0ff, 0x0000_0400},
    {857, 0xffff_ff00, 0x0000_0051},

    //phy level registers
    {1895, 0xffff_e000, 0x0000_1342},
    {1835, 0xffff_f0ff, 0x0000_0200},
    //turn on multicast
    {1793, 0xffff_feff, 0x0000_0100},
];

/* PI config */
#[rustfmt::skip]
pub const START_CFG1: [MemCfg; 28] = mem_cfg_arr![
    {62,  0xffff_feff, 0x0},
    {66,  0xffff_feff, 0x0},
    {166, 0xffff_ff80, 0x0000_0001},
    {62,  0xfff0_ffff, 0x0001_0000},
    {62,  0xf0ff_ffff, 0x0100_0000},
    {166, 0xffff_80ff, 0x0000_0100},

    {179, 0xff80_ffff, 0x0001_0000},
    {67,  0xffe0_ffff, 0x0001_0000},
    {67,  0xe0ff_ffff, 0x0100_0000},
    {179, 0x80ff_ffff, 0x0100_0000},

    {166, 0xff80_ffff, 0x0001_0000},
    {62,  0xfff0_ffff, 0x0001_0000},
    {62,  0xf0ff_ffff, 0x0100_0000},
    {166, 0x80ff_ffff, 0x0100_0000},

    {182, 0xff80_ffff, 0x0001_0000},
    {67,  0xffe0_ffff, 0x0001_0000},
    {67,  0xe0ff_ffff, 0x0100_0000},
    {182, 0x80ff_ffff, 0x0100_0000},

    {167, 0xffff_ff80, 0x0000_0017},
    {62,  0xfff0_ffff, 0x0001_0000},
    {62,  0xf0ff_ffff, 0x0100_0000},
    {167, 0xffff_80ff, 0x0000_1700},
    {185, 0xff80_ffff, 0x0020_0000},
    {67,  0xffe0_ffff, 0x0001_0000},
    {67,  0xe0ff_ffff, 0x0100_0000},
    {185, 0x80ff_ffff, 0x2000_0000},
    {10,  0xffff_ffe0, 0x0000_0002},

    {0,   0xffff_fffe, 0x0000_0001},
];

#[rustfmt::skip]
pub const START_CFG2: [MemCfg; 36] = mem_cfg_arr![
    //set CS0 MR13.VRCG=1
    {247, 0xffff_ffff, 0x0000_0008},
    //set CS1 MR13.VRCG=1
    {249, 0xffff_ffff, 0x0000_0800},
    //set CS2 MR13.VRCG=1
    {252, 0xffff_ffff, 0x0000_0008},
    //set CS3 MR13.VRCG=1
    {254, 0xffff_ffff, 0x0000_0800},

    //PI_MR11_DATA_F1_X
    {281, 0xffff_ffff, 0x3300_0000},
    {305, 0xffff_ffff, 0x3300_0000},
    {329, 0xffff_ffff, 0x3300_0000},
    {353, 0xffff_ffff, 0x3300_0000},

    //PI_MR11_DATA_F2_X
    {289, 0xffff_ffff, VAL_X2},
    {313, 0xffff_ffff, VAL_X2},
    {337, 0xffff_ffff, VAL_X2},
    {361, 0xffff_ffff, VAL_X2},

    //PI_MR22_DATA_F1_X
    {282, 0xffff_ffff, 0x0016_0000},
    {306, 0xffff_ffff, 0x0016_0000},
    {330, 0xffff_ffff, 0x0016_0000},
    {354, 0xffff_ffff, 0x0016_0000},
    //PI_MR22_DATA_F2_X
    {290, 0xffff_ffff, 0x0016_0000},
    {314, 0xffff_ffff, 0x0016_0000},
    {338, 0xffff_ffff, 0x0016_0000},
    {362, 0xffff_ffff, 0x0016_0000},

    {282, 0xffff_ff00, 0x17},
    {306, 0xffff_ff00, 0x17},
    {330, 0xffff_ff00, 0x17},
    {354, 0xffff_ff00, 0x17},
    {290, 0xffff_ff00, 0x17},
    {314, 0xffff_ff00, 0x17},
    {338, 0xffff_ff00, 0x17},
    {362, 0xffff_ff00, 0x17},

    {282, 0xffff_00ff, 0x2000},
    {306, 0xffff_00ff, 0x2000},
    {330, 0xffff_00ff, 0x2000},
    {354, 0xffff_00ff, 0x2000},
    {290, 0xffff_00ff, 0x2000},
    {314, 0xffff_00ff, 0x2000},
    {338, 0xffff_00ff, 0x2000},
    {362, 0xffff_00ff, 0x2000},
];

#[rustfmt::skip]
pub const START_CFG3: [MemCfg; 4] = mem_cfg_arr![
    {65,  0xffff_ffff, 0x0000_0100},
    {321, 0xffff_ffff, 0x0000_0100},
    {577, 0xffff_ffff, 0x0000_0100},
    {833, 0xffff_ffff, 0x0000_0100},
];

#[rustfmt::skip]
pub const START_CFG4: [MemCfg; 24] = mem_cfg_arr![
    //PHY_WDQLVL_CLK_JITTER_TOLERANCE_X: 8'h20 -> 8'h40
    {33,  0xffff_ff00, 0x0040},
    {289, 0xffff_ff00, 0x0040},
    {545, 0xffff_ff00, 0x0040},
    {801, 0xffff_ff00, 0x0040},

    {1038, 0xfcff_ffff, 0x0300_0000},
    {1294, 0xfcff_ffff, 0x0300_0000},
    {1550, 0xfcff_ffff, 0x0300_0000},

    //PHY_PAD_DSLICE_IO_CFG_x:0->7
    {83,  0xffc0_ffff, 0x70000},
    {339, 0xffc0_ffff, 0x70000},
    {595, 0xffc0_ffff, 0x70000},
    {851, 0xffc0_ffff, 0x70000},

    //PHY_PAD_ADR_IO_CFG_x:0->7
    {1062, 0xf800_ffff, 0x70000},
    {1318, 0xf800_ffff, 0x70000},
    {1574, 0xf800_ffff, 0x70000},

    //PHY_PAD_CAL_IO_CFG_0:0->0x15547
    // NOTE: was set to 0x7 in JH7100 code
    {1892, 0xfffc_0000, 0x15547},
    //PHY_PAD_ACS_IO_CFG:0->7
    {1893, 0xfffc_0000, 0x7},
    //PHY_CAL_MODE_0
    //NOTE: comment in JH7100 code says "TODO" and sets to 0x078
    {1852, 0xffff_e000, 0x07a},
    {1853, 0xffff_ffff, 0x0100},
    //PHY_PLL_WAIT
    {1822, 0xffff_ffff, 0xff},
    //PHY_PAD_VREF_CTRL_AC:10'h0100->10'h3d5
    {1896, 0xffff_fc00, 0x03d5},

    //PHY_PAD_VREF_CTRL_DQ_x:10'h11f->10'h3d5
    {91,  0xfc00_ffff, 0x03d50000},
    {347, 0xfc00_ffff, 0x03d50000},
    {603, 0xfc00_ffff, 0x03d50000},
    {859, 0xfc00_ffff, 0x03d50000},
];

#[rustfmt::skip]
pub const START_CFG5: [MemSet; 21] = mem_set_arr![
    {1912, 0x0cc3_bfc7},
    {1913, 0x0000_ff8f},
    {1914, 0x033f_07ff},
    {1915, 0x0c3c_37ff},
    {1916, 0x1fff_ff10},
    {1917, 0x0023_0070},

    // TODO: same value used in original code for 2G/4G and 8G, what is this?
    {1918, 0x3ff7_ffff},

    {1919, 0x0000_0e10},
    {1920, 0x1fff_ffff},
    {1921, 0x0018_8411},
    {1922, 0x1fff_ffff},
    {1923, 0x0018_0400},
    {1924, 0x1fff_ffff},
    {1925, 0x0018_0400},
    {1926, 0x1fff_ffcf},
    {1927, 0x0018_8400},
    {1928, 0x1fff_ffff},
    {1929, 0x0418_8411},
    {1837, 0x0002_4410},
    {1840, 0x0002_4410},
    {1842, 0x0002_ffff},
];

#[rustfmt::skip]
pub const START_CFG6: [MemCfg; 14] = mem_cfg_arr![
    {76,  0xff00_00f8, 0x00ff_8f07},
    {332, 0xff00_00f8, 0x00ff_8f07},
    {588, 0xff00_00f8, 0x00ff_8f07},
    {844, 0xff00_00f8, 0x00ff_8f07},

    {77,  0xffff_0000, 0xff8f},
    {333, 0xffff_0000, 0xff8f},
    {589, 0xffff_0000, 0xff8f},
    {845, 0xffff_0000, 0xff8f},

    //PHY_ADR_TSEL_SELECT_X:bit[7:0]:{ENSLICEP_ODT/DRV,PENSLICEN_ODT/DRV}
    {1062, 0xfff_fff00, VAL_X3}, // addr5-0
    {1318, 0xfff_fff00, VAL_X3}, // addr11-6
    {1574, 0xfff_fff00, VAL_X3}, // addr15-12

    //PHY_TST_CLK_PAD_CTRL_x
    {1028, 0xffff_ffff, 0x0100_0000},
    {1284, 0xffff_ffff, 0x0100_0000},
    {1540, 0xffff_ffff, 0x0100_0000},
];

// PHY_TST_CLK_PAD_CTRL_x
#[rustfmt::skip]
pub const START_CFG7: [MemSet; 4] = mem_set_arr![
    {1848, 0x03cf_07f8},
    {1849, 0x0000_003f},
    {1850, 0x001f_ffff},
    {1851, 0x0006_0000},
];

#[rustfmt::skip]
pub const START_CFG8: [MemCfg; 25] = mem_cfg_arr![
    // PHY_DSLICE_PAD_BOOSTPN_SETTING_x
    {130, 0x0000_ffff, 0xffff_0000},
    {386, 0x0000_ffff, 0xffff_0000},
    {642, 0x0000_ffff, 0xffff_0000},
    {898, 0x0000_ffff, 0xffff_0000},
    // ???
    {131, 0xffff_fff0, 0xf},
    {387, 0xffff_fff0, 0xf},
    {643, 0xffff_fff0, 0xf},
    {899, 0xffff_fff0, 0xf},
    //PHY_WRLVL_CAPTURE_CNT_X
    {29,  0xc0ff_ffff, 0x1000_0000},
    {285, 0xc0ff_ffff, 0x1000_0000},
    {541, 0xc0ff_ffff, 0x1000_0000},
    {797, 0xc0ff_ffff, 0x1000_0000},
    // PHY_GTLVL_CAPTURE_CNT_X
    {30,  0xffff_ffff, 0x0008_0000},
    {286, 0xffff_ffff, 0x0008_0000},
    {542, 0xffff_ffff, 0x0008_0000},
    {798, 0xffff_ffff, 0x0008_0000},
    // PHY_RDLVL_CAPTURE_CNT_X
    {31,  0xffff_ffc0, 0x0000_0010},
    {287, 0xffff_ffc0, 0x0000_0010},
    {543, 0xffff_ffc0, 0x0000_0010},
    {799, 0xffff_ffc0, 0x0000_0010},
    // PHY_ADRLVL_CAPTURE_CNT_X
    {1071, 0xffff_fff0, 0x0000_0008},
    {1327, 0xffff_fff0, 0x0000_0008},
    {1583, 0xffff_fff0, 0x0000_0008},
    // PHY_CSLVL_COARSECAPTURE_CNT
    {1808, 0xffff_fff0, 0x0000_0008},
    // PHY_CSLVL_CAPTURE_CNT_X
    {1896, 0xfff0_ffff, 0x0008_0000},
];
