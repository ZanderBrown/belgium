use crate::alu::ALU;
use crate::op1;
use crate::op2;
use crate::opcodes::HALT;
use crate::opcodes::{OPERATION, OP_LOAD, OP_LOAD_I, OP_STACK, OP_STORE, POP, PUSH};
use crate::stream::Error;
use std::rc::Weak;

pub const MEM_SIZE: usize = 256;
// 4 General Purpose + 3 Special
pub const REG_SIZE: u8 = 4 + 3;

pub const COUNTER: u8 = 4;
pub const STATUS: u8 = 5;
pub const SP: u8 = 6;

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
            registers: [0; REG_SIZE as usize],
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
    pub fn set_reg(&mut self, i: u8, v: u8) -> Result<(), Error> {
        if i >= REG_SIZE {
            Err(Error::new(format!("Invalid register {}", i), None))
        } else {
            self.registers[i as usize] = v;
            Self::emit(&ChangeEvent { idx: i, val: v }, &self.reg_listeners);
            Ok(())
        }
    }

    /// # Errors
    ///
    /// Will return `Err` if `i` is an invalid register
    pub fn reg(&self, i: u8) -> Result<u8, Error> {
        if i >= REG_SIZE {
            Err(Error::new(format!("Invalid register {}", i), None))
        } else {
            Ok(self.registers[i as usize])
        }
    }

    fn advanace_counter(&mut self) -> Result<(), Error> {
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

    /// # Errors
    ///
    /// Will return `Err` on a malformed instruction
    pub fn step(&mut self) -> Result<bool, Error> {
        let instruction = self.mem(self.reg(COUNTER)?);
        let operation = instruction & OPERATION;

        self.advanace_counter()?;

        // HALT is a special case
        if instruction == HALT {
            return Ok(false);
        }

        // STORE is the first non-ALU operation
        if operation < OP_STORE {
            self.process_alu(instruction)?;
        } else {
            match operation {
                OP_LOAD => {
                    let address = op1!(instruction);
                    let target = op2!(instruction);
                    self.set_reg(target, self.mem(address))?;
                }
                OP_LOAD_I => {
                    let target = op2!(instruction);
                    let address = self.reg(COUNTER)?;

                    self.advanace_counter()?;

                    self.set_reg(target, self.mem(address))?;
                }
                OP_STORE => {
                    let address = op1!(instruction);
                    let source = op2!(instruction);

                    self.set_mem(address, self.reg(source)?);
                }
                OP_STACK => {
                    let mode = op1!(instruction);
                    let rn = op2!(instruction);
                    let sp = self.reg(SP)?;

                    match mode {
                        PUSH => {
                            self.set_reg(SP, sp + 1)?;
                            self.set_reg(sp + 1, rn)?;
                        }
                        POP => {
                            self.set_reg(SP, sp - 1)?;
                            self.set_reg(rn, self.reg(sp)?)?;
                        }
                        _ => {
                            return Err(Error::new(
                                format!("0x{:X} isn't an instruction", instruction),
                                None,
                            ))
                        }
                    }
                }
                _ => return Err(Error::new(String::from("Unknown instruction"), None)),
            }
        }
        Ok(true)
    }

    #[must_use]
    pub fn iter_mem(&self) -> MemIter {
        MemIter {
            machine: Box::new(self),
            pos: 0,
        }
    }
}

pub struct MemIter<'a> {
    machine: Box<&'a Machine>,
    pos: u8,
}

impl<'a> Iterator for MemIter<'a> {
    type Item = (u8, u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == 255 {
            None
        } else {
            let old_pos = self.pos;
            self.pos += 1;
            Some((old_pos, self.machine.mem(old_pos)))
        }
    }
}
