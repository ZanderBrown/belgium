use crate::op1;
use crate::op2;
use crate::opcodes::HALT;
use crate::opcodes::{
    ADD, AND, CMP, DEC, INC, LOAD, LOAD_I, MOVE, NEG, NOT, NOT_NEG_INC_DEC, OPERATION, OR, SHIFT,
    SHLA, SHRA, STORE, SUB,
};
use crate::stream::Error;
use std::rc::Weak;

pub const MEM_SIZE: usize = 256;
// 4 General Purpose + 3 Special
pub const REG_SIZE: u8 = 4 + 3;

pub const COUNTER: u8 = 4;
pub const STATUS: u8 = 5;
pub const STACK: u8 = 6;

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
        if current as usize + 1 < MEM_SIZE {
            self.set_reg(COUNTER, current + 1)
        } else {
            self.set_reg(COUNTER, 0)
        }
    }

    /// # Errors
    ///
    /// Will return `Err` on a malformed instruction
    pub fn step(&mut self) -> Result<bool, Error> {
        let instruction = self.mem(self.reg(COUNTER)?);

        self.advanace_counter()?;

        // HALT is a special case
        if instruction == HALT {
            return Ok(false);
        }

        match instruction & OPERATION {
            LOAD => {
                let address = op1!(instruction);
                let target = op2!(instruction);
                self.set_reg(target, self.mem(address))?;
            }
            LOAD_I => {
                let target = op2!(instruction);
                let address = self.reg(COUNTER)?;

                self.advanace_counter()?;

                self.set_reg(target, self.mem(address))?;
            }
            STORE => {
                let address = op1!(instruction);
                let source = op2!(instruction);

                self.set_mem(address, self.reg(source)?);
            }
            ADD => {
                let rn = op1!(instruction);
                let rm = op2!(instruction);

                // TODO: Update STATUS
                self.set_reg(rm, self.reg(rn)?.wrapping_add(self.reg(rm)?))?;
            }
            SUB => {
                let rn = op1!(instruction);
                let rm = op2!(instruction);

                // TODO: Update STATUS
                self.set_reg(self.reg(rm)?, self.reg(rn)?.wrapping_sub(self.reg(rm)?))?;
            }
            MOVE => {
                let rn = op1!(instruction);
                let rm = op2!(instruction);

                self.set_reg(rm, self.reg(rn)?)?;
            }
            CMP => {
                let rn = op1!(instruction);
                let rm = op2!(instruction);

                // TODO
            }
            AND => {
                let rn = op1!(instruction);
                let rm = op2!(instruction);

                self.set_reg(rm, self.reg(rn)? & self.reg(rm)?)?;
            }
            OR => {
                let rn = op1!(instruction);
                let rm = op2!(instruction);

                self.set_reg(rm, self.reg(rn)? | self.reg(rm)?)?;
            }
            NOT_NEG_INC_DEC => {
                let rn = op1!(instruction);
                let rm = op2!(instruction);

                let old = self.reg(rm)?;

                // TODO: Update STATUS

                match 42_u8 {
                    12 => println!("12"),
                    42 => println!("42"),
                    n => println!("{}", n),
                }

                self.set_reg(
                    rm,
                    match rn {
                        NOT => !old,
                        #[allow(clippy::cast_sign_loss)]
                        #[allow(clippy::cast_possible_wrap)]
                        NEG => -(old as i8) as u8,
                        INC => old + 1,
                        DEC => old - 1,
                        n => panic!("impossible {}", n),
                    },
                )?;
            }
            SHIFT => {
                let rn = op1!(instruction);
                let rm = op2!(instruction);

                let old = self.reg(rm)?;

                // TODO: Update STATUS
                self.set_reg(
                    rm,
                    match rn {
                        SHLA => old << 1,
                        SHRA => old >> 1,
                        n => panic!("wat {}", n),
                    },
                )?;
            }
            _ => return Err(Error::new(String::from("Unknown instruction"), None)),
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
