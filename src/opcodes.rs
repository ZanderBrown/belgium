pub const OPERATION: u8 = 0b1111_0000;

pub const MOVE: u8 = 0b0000_0000;
pub const ADD: u8 = 0b0001_0000;
pub const UNKNOWNA: u8 = 0b0010_0000;
pub const SUB: u8 = 0b0011_0000;
pub const AND: u8 = 0b0100_0000;
pub const OR: u8 = 0b0101_0000;
pub const UNKNOWNB: u8 = 0b0110_0000;
pub const CMP: u8 = 0b0111_0000;
pub const NOT_NEG_INC_DEC: u8 = 0b1000_0000;
pub const SHIFT: u8 = 0b1001_0000;
pub const STORE: u8 = 0b1010_0000;
pub const LOAD: u8 = 0b1011_0000;
pub const UNKNOWNC: u8 = 0b1100_0000;
pub const LOAD_I: u8 = 0b1101_0000;
pub const UNKNOWND: u8 = 0b1110_0000;
pub const UNKNOWNE: u8 = 0b1111_0000;

pub const NOT: u8 = 0b0000_0000;
pub const NEG: u8 = 0b0000_0100;
pub const DEC: u8 = 0b0000_1000;
pub const INC: u8 = 0b0000_1100;

pub const SHLA: u8 = 0b0000_0100;
pub const SHRA: u8 = 0b0000_1000;

pub const HALT: u8 = 0b1101_0100;

#[macro_export]
macro_rules! op1 {
    ( $instruction:expr ) => {
        ($instruction & 0b0000_1100) >> 2
    };
}

#[macro_export]
macro_rules! op2 {
    ( $instruction:expr ) => {
        ($instruction & 0b0000_0011)
    };
}
