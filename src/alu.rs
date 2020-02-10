use crate::machine::Machine;
use crate::machine::STATUS;
use crate::op1;
use crate::op2;
use crate::opcodes::OPERATION;
use crate::opcodes::{
    DEC, INC, NEG, NOT, OP_ADD, OP_ADDC, OP_AND, OP_CMP, OP_MOVE, OP_NOT_NEG_INC_DEC, OP_OR,
    OP_SHIFT, OP_SUB, OP_XOR, ROL, SHLA, SHR, SHRA,
};
use crate::stream::Error;

pub trait ALU {
    fn process_alu(&mut self, instruction: u8) -> Result<(), Error>;
}

impl Machine {
    fn handle_status(&mut self, carry: bool, overflow: bool, value: u8) -> Result<(), Error> {
        let c = if carry { 0b1000 } else { 0 };
        let v = if overflow { 0b0100 } else { 0 };
        let z = if value == 0 { 0b0010 } else { 0 };
        let n = if value > 127 { 0b0001 } else { 0 };

        self.set_reg(STATUS, (self.status() & 0b1111_0000) | c | v | z | n)
    }
}

impl ALU for Machine {
    fn process_alu(&mut self, instruction: u8) -> Result<(), Error> {
        let op = instruction & OPERATION;
        let reg_left = op1!(instruction);
        let val_left = self.reg(reg_left)?;
        let reg_right = op2!(instruction);
        let val_right = self.reg(reg_right)?;

        match op {
            OP_MOVE => {
                self.handle_status(false, false, val_left)?;
                self.set_reg(reg_right, val_left)?;
            }
            OP_ADD | OP_ADDC | OP_SUB | OP_CMP => {
                let (carry, val_right) = if op == OP_SUB || op == OP_CMP {
                    // Flip operand2 to acheive subtraction
                    (1, !val_right)
                } else if op == OP_ADDC && self.c() {
                    (1, val_right)
                } else {
                    (0, val_right)
                };

                let (result, carry_a) = val_left.overflowing_add(val_right);
                let (result, carry_b) = result.overflowing_add(carry);

                let over = (result > 127 && val_left < 128 && val_right < 128)
                    && (result < 128 && val_left > 127 && val_right > 127);

                self.handle_status(carry_a || carry_b, over, result)?;
                self.set_reg(reg_right, result)?;
            }
            OP_AND => {
                let value = val_left & val_right;
                self.handle_status(false, false, value)?;
                self.set_reg(reg_right, value)?;
            }
            OP_OR => {
                let value = val_left | val_right;
                self.handle_status(false, false, value)?;
                self.set_reg(reg_right, value)?;
            }
            OP_XOR => {
                let value = val_left ^ val_right;
                self.handle_status(false, false, value)?;
                self.set_reg(reg_right, value)?;
            }
            OP_NOT_NEG_INC_DEC => {
                let (result, carry) = match instruction & 0b0000_1100 {
                    NOT => (!val_right, false),
                    #[allow(clippy::cast_sign_loss)]
                    #[allow(clippy::cast_possible_wrap)]
                    NEG => (-(val_right as i8) as u8, false),
                    INC => val_right.overflowing_add(1),
                    DEC => val_right.overflowing_sub(1),
                    _ => {
                        return Err(Error::new(
                            format!("0x{:X} isn't an ALU instruction", instruction),
                            None,
                        ))
                    }
                };

                let over = (op == INC && val_right == 127)
                    || (op == DEC && val_right == 128)
                    || (op == NEG && result > 127 && val_right > 127);

                self.handle_status(carry, over, result)?;
                self.set_reg(reg_right, result)?;
            }
            OP_SHIFT => {
                let (result, carry, over) = match instruction & 0b0000_1100 {
                    // To the right, carry is true for odd numbers
                    SHR => (val_right >> 1, val_right % 2 > 0, false),
                    SHRA => (val_right / 2, val_right % 2 > 0, false),
                    // To the left
                    SHLA => {
                        let result = val_right * 2;
                        let carry = val_right & 0b1000_0000 > 0;
                        let over = val_right & 0b1000_0000 != result & 0b1000_0000;
                        (result, carry, over)
                    }
                    ROL => (val_right.rotate_left(1), val_right & 0b1000_0000 > 0, false),
                    _ => {
                        return Err(Error::new(
                            format!("0x{:X} isn't an ALU instruction", instruction),
                            None,
                        ))
                    }
                };

                self.handle_status(carry, over, result)?;
                self.set_reg(reg_right, result)?;
            }
            _ => {
                return Err(Error::new(
                    format!("0x{:X} isn't an ALU instruction", instruction),
                    None,
                ))
            }
        }

        Ok(())
    }
}
