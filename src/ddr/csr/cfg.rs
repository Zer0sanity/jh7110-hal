use crate::ddr::{MemCfg, MemSet};
use crate::{mem_cfg_arr, mem_set_arr};

#[cfg(any(feature = "8G", feature = "4G"))]
pub const CFG0_X1: u32 = 0x0000_0001;
#[cfg(not(any(feature = "8G", feature = "4G")))]
pub const CFG0_X1: u32 = 0x0080_0001;

#[cfg(any(feature = "8G", feature = "4G"))]
pub const CFG1_X1: u32 = 0x3001_0006;
#[cfg(feature = "2G")]
pub const CFG1_X1: u32 = 0x1001_0006;

#[cfg(any(feature = "8G", feature = "4G"))]
pub const CFG1_X2: u32 = 0x3002_0000;
#[cfg(feature = "2G")]
pub const CFG1_X2: u32 = 0x1002_0000;

#[cfg(any(feature = "8G", feature = "4G"))]
pub const CFG1_X3: u32 = 0x3003_0031;
#[cfg(feature = "2G")]
pub const CFG1_X3: u32 = 0x1003_0031;

#[cfg(any(feature = "8G", feature = "4G"))]
pub const CFG1_X4: u32 = 0x300b_0033;
#[cfg(feature = "2G")]
pub const CFG1_X4: u32 = 0x100b_0033;

#[cfg(any(feature = "8G", feature = "4G"))]
pub const CFG1_X5: u32 = 0x3016_0016;
#[cfg(feature = "2G")]
pub const CFG1_X5: u32 = 0x1016_0016;

#[cfg(any(feature = "8G", feature = "4G"))]
pub const CFG3_X1: u32 = 0x3001_0036;
#[cfg(feature = "2G")]
pub const CFG3_X1: u32 = 0x10010036;

#[cfg(any(feature = "8G", feature = "4G"))]
pub const CFG3_X2: u32 = 0x3002_001b;
#[cfg(feature = "2G")]
pub const CFG3_X2: u32 = 0x1001_0036;

#[cfg(any(feature = "8G", feature = "4G"))]
pub const CFG3_X3: u32 = 0x3003_0031;
#[cfg(feature = "2G")]
pub const CFG3_X3: u32 = 0x1003_0031;

#[cfg(any(feature = "8G", feature = "4G"))]
pub const CFG3_X4: u32 = 0x300b_0036;
#[cfg(feature = "2G")]
pub const CFG3_X4: u32 = 0x300b_0066;

#[cfg(any(feature = "8G", feature = "4G"))]
pub const CFG3_X5: u32 = 0x3016_0016;
#[cfg(feature = "2G")]
pub const CFG3_X5: u32 = 0x1016_0016;

#[cfg(any(feature = "8G", feature = "4G"))]
pub const CFG3_X6: u32 = 0x0931_3fff;
#[cfg(feature = "2G")]
pub const CFG3_X6: u32 = 0x0931_1fff;

#[cfg(any(feature = "8G", feature = "4G"))]
pub const CFG3_X7: u32 = 0x0000_0033;
#[cfg(feature = "2G")]
pub const CFG3_X7: u32 = 0x0000_0013;

// see U-Boot drivers/ram/starfive/ddrcsr_boot.c
pub const DDR_CSR_CFG0: [MemSet; 6] = mem_set_arr![
    // TODO: same value used in original code for 2G/4G and 8G, what is this?
    {0xf00, 0x4000_1030},
    {0xf04, CFG0_X1},
    {0xf10, 0x0040_0000},
    {0xf14, 0x043f_ffff},
    {0xf18, 0x0000_0000},
    {0xf30, 0x1f00_0041},
];

#[rustfmt::skip]
pub const DDR_CSR_CFG1: [MemSet; 6] = mem_set_arr![
    {0x10c, 0x0000_0505},
    {0x11c, 0x0000_0000},
    {0x500, 0x0000_0201},
    {0x514, 0x0000_0100},
    {0x6a8, 0x0004_0000},
    {0xea8, 0x0004_0000},
];

#[rustfmt::skip]
pub const DDR_CSR_CFG2: [MemSet; 50] = mem_set_arr![
    {0x310, 0x0002_0000},
    {0x310, 0x0002_0001},
    // Write down RCLK-related CRs
    {0x600, 0x002e_0176},
    {0x604, 0x002e_0176},
    {0x608, 0x0017_00bb},
    {0x60c, 0x000b_005d},
    {0x610, 0x0005_002e},
    {0x614, 0x0002_0017},
    {0x618, 0x0002_0017},
    {0x61c, 0x0002_0017},
    {0x678, 0x0000_0019},
    {0x100, 0x0000_00f8},
    {0x620, 0x0303_0404},
    {0x624, 0x0403_0505},
    {0x628, 0x0703_0884},
    {0x62c, 0x1315_0401},
    {0x630, 0x1715_0604},
    {0x634, 0x0011_0000},
    {0x638, 0x200a_0a08},
    {0x63c, 0x1730_f803},
    {0x640, 0x000a_0c00},
    {0x644, 0xa005_000a},
    {0x648, 0x0000_0000},
    {0x64c, 0x0008_1306},
    {0x650, 0x0407_0304},
    {0x654, 0x0000_0404},
    {0x658, 0x0000_0060},
    {0x65c, 0x0003_0008},
    {0x660, 0x0000_0000},
    {0x680, 0x0000_0603},
    {0x684, 0x0100_0202},
    {0x688, 0x0413_040d},
    {0x68c, 0x2000_2420},
    {0x690, 0x0014_0000},
    {0x69c, 0x0124_0074},
    {0x6a0, 0x0000_0000},
    {0x6a4, 0x2024_0c00},
    {0x6a8, 0x0004_0000},

    {0x4, CFG1_X1},
    {0xc, 0x00000002},
    {0x4, CFG1_X2},
    {0xc, 0x00000002},
    {0x4, CFG1_X3},
    {0xc, 0x00000002},
    {0x4, CFG1_X4},
    {0xc, 0x00000002},
    {0x4, CFG1_X5},
    {0xc, 0x00000002},

    {0x10, 0x0000_0010},
    {0x14, 0x0000_0001},
];

#[rustfmt::skip]
pub const DDR_CSR_CFG3: [MemCfg; 29] = mem_cfg_arr![
    // cdns_rdlvl_gate_tr_init( 3,0,0,0,0);
    {0xb8,  0xf0ffffff,  0x3000000},
    {0x84,  0xFEFFFFFF,  0x0},
    {0xb0,  0xFFFEFFFF,  0x0},
    {0xb0,  0xFEFFFFFF,  0x0},
    {0xb4,  0xffffffff,  0x1},
    {0x248,  0xffffffff,  0x3000000},
    {0x24c,  0xffffffff,  0x300},
    {0x24c,  0xffffffff,  0x3000000},
    {0xb0,  0xffffffff,  0x100},
    // cdns_rdlvl_tr_init( 3,0,0,0,0);
    {0xb8,  0xFFF0FFFF,  0x30000},
    {0x84,  0xFFFEFFFF,  0x0},
    {0xac,  0xFFFEFFFF,  0x0},
    {0xac,  0xFEFFFFFF,  0x0},
    {0xb0,  0xffffffff,  0x1},
    {0x248,  0xffffffff,  0x30000},
    {0x24c,  0xffffffff,  0x3},
    {0x24c,  0xffffffff,  0x30000},
    {0x250,  0xffffffff,  0x3000000},
    {0x254,  0xffffffff,  0x3000000},
    {0x258,  0xffffffff,  0x3000000},
    {0xac,  0xffffffff,  0x100},
    // cdns_wdqlvl_tr_init( 3,0,0,0,0);
    {0x10c,  0xFFFFF0FF,  0x300},
    {0x110,  0xFFFFFEFF,  0x0},
    {0x11c,  0xFFFEFFFF,  0x0},
    {0x11c,  0xFEFFFFFF,  0x0},
    {0x120,  0xffffffff,  0x100},
    {0x2d0,  0xffffffff,  0x300},
    {0x2dc,  0xffffffff,  0x300},
    {0x2e8,  0xffffffff,  0x300},
];

#[rustfmt::skip]
pub const DDR_CSR_CFG4: [MemSet; 43] = mem_set_arr![
    {0x100, 0x000000e0},
    {0x620, 0x04041417},
    {0x624, 0x09110609},
    {0x628, 0x442d0994},
    {0x62c, 0x271e102b},
    {0x630, 0x291b140a},
    {0x634, 0x001c0000},
    {0x638, 0x200f0f08},
    {0x63c, 0x29420a06},
    {0x640, 0x019e1fc1},
    {0x644, 0x10cb0196},
    {0x648, 0x00000000},
    {0x64c, 0x00082714},
    {0x650, 0x16442f0d},
    {0x654, 0x00001916},
    {0x658, 0x00000060},
    {0x65c, 0x00600020},
    {0x660, 0x00000000},
    {0x680, 0x0c00040f},
    {0x684, 0x03000604},
    {0x688, 0x0515040d},
    {0x68c, 0x20002c20},
    {0x690, 0x00140000},
    {0x69c, 0x01240074},
    {0x6a0, 0x00000000},
    {0x6a4, 0x202c0c00},
    {0x6a8, 0x00040000},

    {0x4, CFG3_X1},
    {0xc, 0x00000002},
    {0x4, CFG3_X2},
    {0xc, 0x00000002},
    {0x4, CFG3_X3},
    {0xc, 0x00000002},
    {0x4, CFG3_X4},
    {0xc, 0x00000002},
    {0x4, CFG3_X5},
    {0xc, 0x00000002},

    {0x410, 0x00101010},
    {0x420, 0x0c181006},
    {0x424, 0x20200820},
    {0x428, 0x80000020},
    {0x0,   0x00000001},
    {0x108, 0x00003000},
];

#[rustfmt::skip]
pub const DDR_CSR_CFG5: [MemSet; 6] = mem_set_arr![
    {0x330, CFG3_X6},
    {0x508, CFG3_X7},
    {0x324, 0x0000_2000},
    {0x104, 0x9000_0000},
    {0x510, 0x0000_0100},
    {0x514, 0x0000_0000},
];
