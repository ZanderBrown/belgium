pub const COND: u32 = 0xF000_0000;
pub const INDIRECT: u32 = 0x0200_0000;
pub const IMMEDIATE: u32 = 0x0100_0000;
pub const OPERATION: u32 = 0x00F0_0000;
pub const SOURCE: u32 = 0x000F_0000;
pub const DEST: u32 = 0x0000_F000;
pub const SHIFT: u32 = 0x0000_0F00;
pub const DATA: u32 = 0x0000_00FF;
pub const OFFSET: u32 = 0x0000_FFFF;

pub const COND_NONE: u32 = 0x0000_0000;
pub const COND_EQ: u32 = 0x1000_0000;
pub const COND_NE: u32 = 0x2000_0000;
pub const COND_GT: u32 = 0x4000_0000;
pub const COND_LT: u32 = 0x8000_0000;

pub const LDR: u32 = 0x0010_0000;
pub const STR: u32 = 0x0020_0000;
pub const ADD: u32 = 0x0040_0000;
pub const SUB: u32 = 0x0050_0000;
pub const MOV: u32 = 0x0060_0000;
pub const CMP: u32 = 0x0070_0000;
pub const B: u32 = 0x0080_0000;
pub const AND: u32 = 0x0090_0000;
pub const ORR: u32 = 0x00A0_0000;
pub const EOR: u32 = 0x00B0_0000;
pub const MVN: u32 = 0x00C0_0000;
pub const LSR: u32 = 0x00D0_0000;
pub const LSL: u32 = 0x00E0_0000;
pub const HALT: u32 = 0x00F0_0000;

pub const CIR: u32 = 13;
pub const MBUFF: u32 = 14;
pub const COUNTER: u32 = 15;
pub const ADDRESS: u32 = 16;
pub const STATUS: u32 = 17;
