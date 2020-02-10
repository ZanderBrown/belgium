pub const OPERATION: u8 = 0b1111_0000;

// ALU Instructions - Binary
pub const OP_MOVE: u8 = 0b0000_0000;
pub const OP_ADD: u8 = 0b0001_0000;
pub const OP_ADDC: u8 = 0b0010_0000;
pub const OP_SUB: u8 = 0b0011_0000;
pub const OP_AND: u8 = 0b0100_0000;
pub const OP_OR: u8 = 0b0101_0000;
pub const OP_XOR: u8 = 0b0110_0000;
pub const OP_CMP: u8 = 0b0111_0000;

// ALU Instructions - Unary
pub const OP_NOT_NEG_INC_DEC: u8 = 0b1000_0000;
pub const OP_SHIFT: u8 = 0b1001_0000;

// Memory
pub const OP_STORE: u8 = 0b1010_0000;
pub const OP_LOAD: u8 = 0b1011_0000;
pub const OP_STACK: u8 = 0b1100_0000;
pub const OP_LOAD_I: u8 = 0b1101_0000;

// Flow
pub const OP_BRANCH: u8 = 0b1110_0000;

// ???
pub const OP_LOAD_C: u8 = 0b1111_0000;

// Magic instruction for stopping the clock
pub const HALT: u8 = 0b1101_0100;

// Variants of `NOT_NEG_INC_DEC`
pub const NOT: u8 = 0b0000_0000;
pub const NEG: u8 = 0b0000_0100;
pub const DEC: u8 = 0b0000_1000;
pub const INC: u8 = 0b0000_1100;

// Variants of `SHIFT`
pub const SHR: u8 = 0b0000_0000;
pub const SHLA: u8 = 0b0000_0100;
pub const SHRA: u8 = 0b0000_1000;
pub const ROL: u8 = 0b0000_1100;

// Variants of `STACK`
pub const PUSH: u8 = 0b0000_0000;
pub const POP: u8 = 0b0000_0100;
pub const LDSA: u8 = 0b0000_1000;
pub const ADDSP_SETSP_PUSHALL_POPALL: u8 = 0b0000_1100;

// Variants within variants
pub const ADDSP: u8 = 0b0000_0000;
pub const SETSP: u8 = 0b0000_0001;
pub const PUSHALL: u8 = 0b0000_0010;
pub const POPALL: u8 = 0b0000_0011;

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
