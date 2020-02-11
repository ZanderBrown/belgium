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
pub const LDI_INTERRUPT: u8 = 0b1101_0000;

// Flow
pub const OP_BRANCH: u8 = 0b1110_0000;

// Load Constant
pub const OP_LOAD_C: u8 = 0b1111_0000;

// Variants of `NOT_NEG_INC_DEC`
pub const NOT: u8 = 0b0000_0000;
pub const NEG: u8 = 0b0000_0100;
pub const DEC: u8 = 0b0000_1000;
pub const INC: u8 = 0b0000_1100;

// Variants of `LDI_INTERRUPT`
pub const OP_LDI_0: u8 = 0b0000_0000;
pub const OP_LDI_1: u8 = 0b0000_0001;
pub const OP_LDI_2: u8 = 0b0000_0010;
pub const OP_LDI_3: u8 = 0b0000_0011;
pub const OP_HALT: u8 = 0b0000_0100;
pub const OP_WAIT: u8 = 0b0000_0101;
pub const OP_JSR: u8 = 0b0000_0110;
pub const OP_RTS: u8 = 0b0000_0111;
pub const OP_IOI: u8 = 0b0000_1000;
pub const OP_RTI: u8 = 0b0000_1001;
pub const OP_CRC: u8 = 0b0000_1010;
pub const OP_OSIX: u8 = 0b0000_1011;
pub const OP_RAND: u8 = 0b0000_1111;

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

// Variants fo `OP_BRANCH`
pub const BEQ_BZ: u8 = 0b0000_0000;
pub const BNE_BNZ: u8 = 0b0000_0001;
pub const BHS_BCS: u8 = 0b0000_0010;
pub const BLO_BCC: u8 = 0b0000_0011;
pub const BMI: u8 = 0b0000_0100;
pub const BPL: u8 = 0b0000_0101;
pub const BVS: u8 = 0b0000_0110;
pub const BVC: u8 = 0b0000_0111;
pub const BHI: u8 = 0b0000_1000;
pub const BLS: u8 = 0b0000_1001;
pub const BGE: u8 = 0b0000_1010;
pub const BLT: u8 = 0b0000_1011;
pub const BGT: u8 = 0b0000_1100;
pub const BLE: u8 = 0b0000_1101;
pub const BR: u8 = 0b0000_1110;
pub const NOP: u8 = 0b0000_1111;

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
