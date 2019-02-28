use crate::parse::Comparison;
use crate::parse::Node;
use crate::stream::Error;

use std::collections::HashMap;

pub type Labels = HashMap<String, usize>;

pub trait Storage {
    fn get(&self, i: u32) -> Result<u32, Error>;
    fn set(&mut self, i: u32, v: u32) -> Result<(), Error>;
    fn count(&self) -> usize;
}

impl Storage {
    pub fn iter<'a>(&'a self) -> StorageIterator<'a> {
        StorageIterator {
            mem: Box::new(self),
            pos: 0,
        }
    }
}

pub struct StorageIterator<'a> {
    mem: Box<&'a Storage>,
    pos: u32,
}

impl<'a> Iterator for StorageIterator<'a> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(itm) = (*self.mem).get(self.pos).ok() {
            self.pos += 1;
            Some((self.pos - 1, itm))
        } else {
            None
        }
    }
}

trait Jump {
    fn jump(&self, label: &String) -> Result<usize, Error>;
}

pub trait Eval {
    fn eval(
        &self,
        lbls: &HashMap<String, usize>,
        regs: &mut impl Storage,
        mem: &mut impl Storage,
    ) -> Result<(), Error>;
}

impl Jump for Labels {
    fn jump(&self, label: &String) -> Result<usize, Error> {
        if let Some(pos) = self.get(label) {
            Ok(pos - 1)
        } else {
            Err(Error::new(format!("Bad Label {}", label), None))
        }
    }
}

impl Eval for Vec<Node> {
    fn eval(
        &self,
        lbls: &Labels,
        regs: &mut impl Storage,
        mem: &mut impl Storage,
    ) -> Result<(), Error> {
        let mut last = Comparison::None;
        let mut idx = 0;
        loop {
            if idx >= self.len() {
                break Err(Error::new("Out of instructions".into(), None));
            }
            match &self[idx] {
                Node::Ldr(reg, memref) => {
                    regs.set(*reg, mem.get(*memref)?)?;
                    last = Comparison::None;
                }
                Node::Str(reg, memref) => {
                    mem.set(*memref, regs.get(*reg)?)?;
                    last = Comparison::None;
                }
                Node::Add(dest, source, other) => {
                    let val = regs.get(*source)?.wrapping_add(other.value(regs)?);
                    regs.set(*dest, val)?;
                    last = Comparison::None;
                }
                Node::Sub(dest, source, other) => {
                    let val = regs.get(*source)?.wrapping_sub(other.value(regs)?);
                    regs.set(*dest, val)?;
                    last = Comparison::None;
                }
                Node::Mov(dest, source) => {
                    let val = source.value(regs)?;
                    regs.set(*dest, val)?;
                    last = Comparison::None;
                }
                Node::Cmp(a, b) => {
                    let a = regs.get(*a)?;
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
                    let val = regs.get(*source)? & other.value(regs)?;
                    regs.set(*dest, val)?;
                    last = Comparison::None;
                }
                Node::Orr(dest, source, other) => {
                    let val = regs.get(*source)? | other.value(regs)?;
                    regs.set(*dest, val)?;
                    last = Comparison::None;
                }
                Node::Eor(dest, source, other) => {
                    let val = regs.get(*source)? ^ other.value(regs)?;
                    regs.set(*dest, val)?;
                    last = Comparison::None;
                }
                Node::Mvn(dest, source) => {
                    let val = !source.value(regs)?;
                    regs.set(*dest, val)?;
                    last = Comparison::None;
                }
                Node::Lsl(dest, source, other) => {
                    let val = regs.get(*source)? << other.value(regs)?;
                    regs.set(*dest, val)?;
                    last = Comparison::None;
                }
                Node::Lsr(dest, source, other) => {
                    let val = regs.get(*source)? >> other.value(regs)?;
                    regs.set(*dest, val)?;
                    last = Comparison::None;
                }
                Node::Halt => break Ok(()),
            }
            idx += 1;
        }
    }
}
