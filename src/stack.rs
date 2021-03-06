use crate::machine::{Machine, Response, COUNTER, SP};
use crate::op2;
use crate::opcodes::{ADDSP, ADDSP_SETSP_PUSHALL_POPALL, LDSA, POP, POPALL, PUSH, PUSHALL, SETSP};

impl Machine {
    pub(crate) fn stack_push(&mut self, v: u8) -> Result<(), Response> {
        let sp = self.reg(SP)?;
        let sp = sp.wrapping_sub(1);
        self.set_reg(SP, sp)?;
        self.set_mem(sp, v);
        Ok(())
    }

    pub(crate) fn stack_pop(&mut self) -> Result<u8, Response> {
        let sp = self.reg(SP)?;
        let v = self.mem(sp);
        self.set_reg(SP, sp.wrapping_add(1))?;
        Ok(v)
    }

    pub(crate) fn handle_stack(&mut self, instruction: u8) -> Result<(), Response> {
        let rn = op2!(instruction);
        let sp = self.reg(SP)?;

        match instruction & 0b0000_1100 {
            PUSH => {
                self.stack_push(self.reg(rn)?)?;
            }
            POP => {
                let v = self.stack_pop()?;
                self.set_reg(rn, v)?;
            }
            LDSA => {
                self.advanace_counter()?;

                let offset = self.mem(self.reg(COUNTER)?);

                self.set_reg(rn, sp.wrapping_add(offset))?;
            }
            ADDSP_SETSP_PUSHALL_POPALL => match instruction & 0b0000_0011 {
                ADDSP => {
                    self.advanace_counter()?;

                    let offset = self.mem(self.reg(COUNTER)?);

                    self.set_reg(SP, sp.wrapping_add(offset))?;
                }
                SETSP => {
                    self.advanace_counter()?;

                    self.set_reg(SP, self.mem(self.reg(COUNTER)?))?;
                }
                PUSHALL => {
                    let mut sp = sp;
                    for i in (0..=3).rev() {
                        sp = sp.wrapping_sub(1);
                        self.set_reg(SP, sp)?;
                        self.set_mem(sp, self.reg(i)?);
                    }
                }
                POPALL => {
                    let mut sp = sp;
                    for i in 0..4 {
                        self.set_reg(i, self.mem(sp))?;
                        sp = sp.wrapping_add(1);
                        self.set_reg(SP, sp)?;
                    }
                }
                _ => return Err(Response::UnknownInstruction),
            },
            _ => return Err(Response::UnknownInstruction),
        }

        Ok(())
    }
}
