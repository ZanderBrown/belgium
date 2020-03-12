use crate::alu::ALU;
use crate::op1;
use crate::op2;
use crate::opcodes::{
    BEQ_BZ, BGE, BGT, BHI, BHS_BCS, BLE, BLO_BCC, BLS, BLT, BMI, BNE_BNZ, BPL, BR, BVC, BVS,
    LDI_INTERRUPT, NOP, OPERATION, OP_BRANCH, OP_CRC, OP_HALT, OP_IOI, OP_JSR, OP_LDI_0, OP_LDI_1,
    OP_LDI_2, OP_LDI_3, OP_LOAD, OP_LOAD_C, OP_OSIX, OP_RAND, OP_RTI, OP_RTS, OP_STACK, OP_STORE,
    OP_WAIT,
};
use std::rc::Weak;

pub const MEM_SIZE: usize = 256;
// 4 General Purpose + 3 Special
pub const REG_SIZE: u8 = 4 + 3;

pub const COUNTER: u8 = 4;
pub const STATUS: u8 = 5;
pub const SP: u8 = 6;

pub enum Response {
    Normal,
    Halt,
    Wait,
    UnknownInstruction,
    BadRegister,
}

#[derive(Clone)]
pub struct ChangeEvent {
    pub idx: u8,
    pub val: u8,
}

pub trait Observer<T> {
    fn notify(&self, evt: T);
}

pub struct Machine {
    memory: [u8; MEM_SIZE],
    registers: [u8; REG_SIZE as usize],
    mem_listeners: Vec<Weak<dyn Observer<ChangeEvent>>>,
    reg_listeners: Vec<Weak<dyn Observer<ChangeEvent>>>,
}

impl Default for Machine {
    fn default() -> Self {
        Self::new()
    }
}

impl Machine {
    #[must_use]
    pub fn new() -> Self {
        Self {
            memory: [0; MEM_SIZE],
            registers: [0, 0, 0, 0, 0, 0, 0],
            mem_listeners: Vec::new(),
            reg_listeners: Vec::new(),
        }
    }

    pub fn add_mem_observer(&mut self, obs: Weak<dyn Observer<ChangeEvent>>) {
        self.mem_listeners.push(obs);
    }

    pub fn add_reg_observer(&mut self, obs: Weak<dyn Observer<ChangeEvent>>) {
        self.reg_listeners.push(obs);
    }

    fn emit(evt: &ChangeEvent, to: &[Weak<dyn Observer<ChangeEvent>>]) {
        for l in to {
            if let Some(ref l) = l.upgrade() {
                l.notify(evt.clone());
            }
        }
    }

    pub fn set_mem(&mut self, i: u8, v: u8) {
        self.memory[i as usize] = v;
        Self::emit(&ChangeEvent { idx: i, val: v }, &self.mem_listeners);
    }

    #[must_use]
    pub fn mem(&self, i: u8) -> u8 {
        self.memory[i as usize]
    }

    /// Set a registers values
    ///
    /// # Errors
    ///
    /// Will return `Err` if `i` is an invalid register
    pub fn set_reg(&mut self, i: u8, v: u8) -> Result<(), Response> {
        if i >= REG_SIZE {
            Err(Response::BadRegister)
        } else {
            self.registers[i as usize] = v;
            Self::emit(&ChangeEvent { idx: i, val: v }, &self.reg_listeners);
            Ok(())
        }
    }

    /// # Errors
    ///
    /// Will return `Err` if `i` is an invalid register
    pub fn reg(&self, i: u8) -> Result<u8, Response> {
        if i >= REG_SIZE {
            Err(Response::BadRegister)
        } else {
            Ok(self.registers[i as usize])
        }
    }

    pub(crate) fn advanace_counter(&mut self) -> Result<(), Response> {
        let current = self.reg(COUNTER)?;
        self.set_reg(COUNTER, current.wrapping_add(1))
    }

    #[must_use]
    pub fn status(&self) -> u8 {
        // No need for the bound checking for reg()
        self.registers[STATUS as usize]
    }

    /// Get the 'Carry'
    #[must_use]
    pub fn c(&self) -> bool {
        self.status() & 0b0000_1000 != 0
    }

    /// Get the 'Overflow' flag
    #[must_use]
    pub fn v(&self) -> bool {
        self.status() & 0b0000_0100 != 0
    }

    /// Get the 'Zero' flag
    #[must_use]
    pub fn z(&self) -> bool {
        self.status() & 0b0000_0010 != 0
    }

    /// Get the 'Negative' flag
    #[must_use]
    pub fn n(&self) -> bool {
        self.status() & 0b0000_0001 != 0
    }

    #[must_use]
    pub fn interrupt_enable(&self) -> bool {
        self.status() & 0b1000_0000 != 0
    }

    /// # Errors
    ///
    /// Will return `Err` on a malformed instruction
    pub fn step(&mut self, interrupt: Option<u8>) -> Result<Response, Response> {
        let instruction = self.mem(self.reg(COUNTER)?);
        let operation = instruction & OPERATION;

        // STORE is the first non-ALU operation
        if operation < OP_STORE {
            self.process_alu(instruction)?;
        } else {
            match operation {
                OP_LOAD => {
                    let address = op1!(instruction);
                    let target = op2!(instruction);
                    self.set_reg(target, self.mem(self.reg(address)?))?;
                }
                LDI_INTERRUPT => {
                    match instruction & 0b0000_1111 {
                        OP_LDI_0 | OP_LDI_1 | OP_LDI_2 | OP_LDI_3 => {
                            self.advanace_counter()?;

                            let target = op2!(instruction);
                            let addr = self.reg(COUNTER)?;

                            self.set_reg(target, self.mem(addr))?;
                        }
                        OP_HALT => return Ok(Response::Halt),
                        OP_WAIT => return Ok(Response::Wait),
                        OP_JSR => {
                            self.advanace_counter()?;

                            let address = self.mem(self.reg(COUNTER)?);

                            // Push "here" to the stack
                            self.stack_push(self.reg(COUNTER)?.wrapping_add(1))?;

                            // FIXME: counteract the advanace_counter at the end
                            self.set_reg(COUNTER, address.wrapping_sub(1))?;
                        }
                        OP_RTS => {
                            // Pop the return address
                            let address = self.stack_pop()?;

                            // FIXME: counteract the advanace_counter at the end
                            self.set_reg(COUNTER, address.wrapping_sub(1))?;
                        }
                        OP_CRC => {
                            let temp = self.reg(COUNTER)?.wrapping_add(1);
                            let counter = self.stack_pop()?;
                            self.set_reg(COUNTER, counter)?;
                            self.stack_push(temp)?;
                        }
                        OP_RAND => {
                            // Yikes now we need the rand crate
                        }
                        OP_IOI | OP_RTI | OP_OSIX => {
                            self.handle_interrupt(instruction, interrupt)?
                        }
                        _ => return Err(Response::UnknownInstruction),
                    }
                }
                OP_STORE => {
                    let address = op1!(instruction);
                    let source = op2!(instruction);

                    self.set_mem(address, self.reg(source)?);
                }
                OP_STACK => self.handle_stack(instruction)?,
                OP_BRANCH => self.handle_branch(instruction)?,
                OP_LOAD_C => {
                    // TODO: This is kinda wrong
                    let address = op1!(instruction);
                    let target = op2!(instruction);
                    #[allow(unused_must_use)]
                    #[allow(clippy::no_effect)]
                    {
                        1 + 1; // Just to shut rustc up about OP_LOAD
                    }
                    self.set_reg(target, self.mem(self.reg(address)?))?;
                }
                _ => return Err(Response::UnknownInstruction),
            }
        }

        self.advanace_counter()?;

        Ok(Response::Normal)
    }

    fn handle_branch(&mut self, instruction: u8) -> Result<(), Response> {
        self.advanace_counter()?;

        let address = self.mem(self.reg(COUNTER)?);

        let jump = match instruction & 0b0000_1111 {
            BEQ_BZ => self.z(),
            BNE_BNZ => !self.z(),
            BHS_BCS => self.c(),
            BLO_BCC => !self.c(),
            BMI => self.n(),
            BPL => !self.n(),
            BVS => self.v(),
            BVC => !self.v(),
            BHI => self.c() && !self.z(),
            BLS => !self.c() || self.z(),
            BGE => (self.v() && self.n()) || (!self.v() && !self.z() && !self.n()) || self.z(),
            BLT => (!self.v() && self.n()) || (self.v() && !self.z() && !self.n()),
            BGT => (self.v() && self.n()) || (!self.v() && !self.z() && !self.n()),
            BLE => self.z() || (!self.v() && self.n()) || (self.v() && !self.z() && !self.n()),
            BR => true,
            NOP => false,
            _ => return Err(Response::UnknownInstruction),
        };

        if jump {
            // FIXME: -1 to absorb the advanace_counter
            self.set_reg(COUNTER, address.wrapping_sub(1))?;
        }

        Ok(())
    }

    fn handle_interrupt(&mut self, instruction: u8, interrupt: Option<u8>) -> Result<(), Response> {
        match instruction & 0b0000_1111 {
            OP_IOI => {
                if self.interrupt_enable() {
                    let vector = if let Some(vector) = interrupt {
                        vector
                    } else {
                        self.advanace_counter()?;
                        0
                    };
                    self.stack_push(self.reg(COUNTER)?)?;
                    self.set_reg(
                        COUNTER,
                        self.mem(0xF0_u8.wrapping_add(vector.wrapping_mul(2))),
                    )?;

                    self.stack_push(self.reg(STATUS)?)?;
                    self.set_reg(STATUS, 0xF1_u8.wrapping_add(vector.wrapping_mul(2)))?;

                    self.set_reg(COUNTER, self.reg(COUNTER)?.wrapping_sub(1))?;
                }
            }
            OP_RTI => {
                let status = self.stack_pop()?;
                self.set_reg(STATUS, status)?;

                let counter = self.stack_pop()?;
                self.set_reg(COUNTER, counter)?;

                self.set_reg(COUNTER, self.reg(COUNTER)?.wrapping_sub(1))?;
            }
            OP_OSIX => {
                if self.interrupt_enable() {
                    self.advanace_counter()?;
                    let new_ps = self.mem(self.reg(COUNTER)?) | (self.mem(0xF1) & 0b1000_0000);
                    self.advanace_counter()?;

                    self.stack_push(self.reg(COUNTER)?)?;
                    self.set_reg(COUNTER, self.mem(0xF0))?;

                    let enable = self.mem(0xF1) & 0b1000_0000;
                    self.stack_push(self.reg(STATUS)?)?;
                    self.set_reg(STATUS, new_ps | enable)?;
                } else {
                    self.advanace_counter()?;
                    self.advanace_counter()?;
                }

                self.set_reg(COUNTER, self.reg(COUNTER)?.wrapping_sub(1))?;
            }
            _ => return Err(Response::UnknownInstruction),
        }

        Ok(())
    }

    #[must_use]
    pub fn iter_mem(&self) -> MemIter {
        MemIter {
            machine: Box::new(self),
            pos: 0,
            done: false,
        }
    }
}

pub struct MemIter<'a> {
    machine: Box<&'a Machine>,
    pos: u8,
    done: bool,
}

impl<'a> Iterator for MemIter<'a> {
    type Item = (u8, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let old_pos = self.pos;
            if old_pos == 255 {
                self.done = true;
            } else {
                self.pos += 1;
            }
            Some((old_pos, self.machine.mem(old_pos)))
        }
    }
}
