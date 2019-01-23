use crate::parse::Comparison;
use crate::parse::Node;

use std::collections::HashMap;
use std::fmt;

pub type MainMemory = Vec<usize>;
pub type Registers = Vec<usize>;
pub type Labels = HashMap<String, usize>;

pub trait Storage {
    fn get(&self, i: usize, n: &str) -> Result<usize, Runtime>;
    fn set(&mut self, i: usize, v: usize, n: &str) -> Result<(), Runtime>;
    fn create(count: usize) -> Self;
}

trait Jump {
    fn jump(&self, label: &String) -> Result<usize, Runtime>;
}

pub struct Runtime(String);

impl Runtime {
    pub fn new(m: String) -> Self {
        Runtime(m)
    }
}

impl fmt::Display for Runtime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Runtime Error: {}", self.0)
    }
}

pub trait Eval {
    fn eval(
        &self,
        lbls: &HashMap<String, usize>,
        regs: &mut Registers,
        mem: &mut MainMemory,
    ) -> Result<(), Runtime>;
}

impl Storage for Vec<usize> {
    fn get(&self, i: usize, n: &str) -> Result<usize, Runtime> {
        if i >= self.len() {
            Err(Runtime::new(format!("Invalid {} {}", i, n)))
        } else {
            Ok(self[i])
        }
    }

    fn set(&mut self, i: usize, v: usize, n: &str) -> Result<(), Runtime> {
        if i >= self.len() {
            Err(Runtime::new(format!("Invalid {} {}", n, i)))
        } else {
            self[i] = v;
            Ok(())
        }
    }

    fn create(count: usize) -> Self {
        let mut regs = Vec::with_capacity(count);
        for _ in 0..count {
            regs.push(0);
        }
        regs
    }
}

impl Jump for Labels {
    fn jump(&self, label: &String) -> Result<usize, Runtime> {
        if let Some(pos) = self.get(label) {
            Ok(pos - 1)
        } else {
            Err(Runtime::new(format!("Bad Label {}", label)))
        }
    }
}

impl Eval for Vec<Node> {
    fn eval(
        &self,
        lbls: &Labels,
        regs: &mut Registers,
        mem: &mut MainMemory,
    ) -> Result<(), Runtime> {
        let regstr = "Register";
        let memstr = "Memory Address";
        let mut last = Comparison::None;
        let mut idx = 0;
        loop {
            if idx >= self.len() {
                break Err(Runtime::new("Out of instructions".into()));
            }
            match &self[idx] {
                Node::Ldr(reg, memref) => {
                    regs.set(*reg, mem.get(*memref, memstr)?, regstr)?;
                    last = Comparison::None;
                }
                Node::Str(reg, memref) => {
                    mem.set(*memref, regs.get(*reg, regstr)?, memstr)?;
                    last = Comparison::None;
                }
                Node::Add(dest, source, other) => {
                    let val = regs.get(*source, regstr)?.wrapping_add(other.value(regs)?);
                    regs.set(*dest, val, regstr)?;
                    last = Comparison::None;
                }
                Node::Sub(dest, source, other) => {
                    let val = regs.get(*source, regstr)?.wrapping_sub(other.value(regs)?);
                    regs.set(*dest, val, regstr)?;
                    last = Comparison::None;
                }
                Node::Mov(dest, source) => {
                    let val = source.value(regs)?;
                    regs.set(*dest, val, regstr)?;
                    last = Comparison::None;
                }
                Node::Cmp(a, b) => {
                    let a = regs.get(*a, regstr)?;
                    let b = b.value(regs)?;
                    last = if a == b {
                        Comparison::Equal
                    } else if a < b {
                        Comparison::Less
                    } else {
                        Comparison::Greater
                    };
                }
                Node::B(cond, target) => {
                    match cond {
                        Comparison::None => {
                            idx = lbls.jump(target)?;
                        }
                        Comparison::Equal => {
                            if Comparison::Equal == last {
                                idx = lbls.jump(target)?;
                            }
                        }
                        Comparison::NotEqual => {
                            if Comparison::Equal != last {
                                idx = lbls.jump(target)?;
                            }
                        }
                        Comparison::Less => {
                            if Comparison::Less == last {
                                idx = lbls.jump(target)?;
                            }
                        }
                        Comparison::Greater => {
                            if Comparison::Greater == last {
                                idx = lbls.jump(target)?;
                            }
                        }
                    }
                    last = Comparison::None;
                }
                Node::And(dest, source, other) => {
                    let val = regs.get(*source, regstr)? & other.value(regs)?;
                    regs.set(*dest, val, regstr)?;
                    last = Comparison::None;
                }
                Node::Orr(dest, source, other) => {
                    let val = regs.get(*source, regstr)? | other.value(regs)?;
                    regs.set(*dest, val, regstr)?;
                    last = Comparison::None;
                }
                Node::Eor(dest, source, other) => {
                    let val = regs.get(*source, regstr)? ^ other.value(regs)?;
                    regs.set(*dest, val, regstr)?;
                    last = Comparison::None;
                }
                Node::Mvn(dest, source) => {
                    let val = !source.value(regs)?;
                    regs.set(*dest, val, regstr)?;
                    last = Comparison::None;
                }
                Node::Lsl(dest, source, other) => {
                    let val = regs.get(*source, regstr)? << other.value(regs)?;
                    regs.set(*dest, val, regstr)?;
                    last = Comparison::None;
                }
                Node::Lsr(dest, source, other) => {
                    let val = regs.get(*source, regstr)? >> other.value(regs)?;
                    regs.set(*dest, val, regstr)?;
                    last = Comparison::None;
                }
                Node::Halt => break Ok(()),
            }
            idx += 1;
        }
    }
}
