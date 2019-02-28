use crate::cpu::{
    ADD, AND, B, CMP, COND_EQ, COND_GT, COND_LT, COND_NE, COND_NONE, EOR, HALT, IMMEDIATE, LDR,
    LSL, LSR, MOV, MVN, ORR, STR, SUB,
};
use crate::parse::Comparison;
use crate::parse::Node;
use crate::parse::Operand;
use crate::parse::Parser;
use crate::storage::Storage;
use crate::stream::Error;
use crate::stream::Input;

fn encode(n: &Operand) -> Result<u32, Error> {
    match n {
        Operand::Literal(n) => {
            for i in 0..16 {
                let m = n.rotate_left(i * 2);
                if m < 256 {
                    return Ok(IMMEDIATE | (i << 8) | m);
                }
            }
            Err(Error::new(String::from("Bad number"), None))
        }
        Operand::Register(n) => Ok(*n),
    }
}

pub trait Assemble {
    fn assemble(&mut self, mem: &mut impl Storage) -> Result<(), Error>;
}

impl Assemble for Input {
    fn assemble(&mut self, storage: &mut impl Storage) -> Result<(), Error> {
        let (nodes, labels) = self.parse()?;
        for (i, node) in nodes.iter().enumerate() {
            match node {
                Node::Ldr(src, mem) => {
                    let ins = LDR | (src << 16) | mem;
                    storage.set(i as u32, ins)?;
                }
                Node::Str(src, mem) => {
                    let ins = STR | (src << 16) | mem;
                    storage.set(i as u32, ins)?;
                }
                Node::Add(dest, src, operand) => {
                    let ins = ADD | (src << 16) | (dest << 12) | encode(operand)?;
                    storage.set(i as u32, ins)?;
                }
                Node::Sub(dest, src, operand) => {
                    let ins = SUB | (src << 16) | (dest << 12) | encode(operand)?;
                    storage.set(i as u32, ins)?;
                }
                Node::Mov(dest, operand) => {
                    let ins = MOV | (dest << 12) | encode(operand)?;
                    storage.set(i as u32, ins)?;
                }
                Node::Cmp(src, operand) => {
                    let ins = CMP | (src << 16) | encode(operand)?;
                    storage.set(i as u32, ins)?;
                }
                Node::B(cond, label) => {
                    let cond = match cond {
                        Comparison::None => COND_NONE,
                        Comparison::Equal => COND_EQ,
                        Comparison::NotEqual => COND_NE,
                        Comparison::Less => COND_LT,
                        Comparison::Greater => COND_GT,
                    };
                    if let Some(pos) = labels.get(label) {
                        let ins = cond | B | (*pos as u32);
                        storage.set(i as u32, ins)?;
                    } else {
                        return Err(Error::new(format!("Unkown label {}", label), None));
                    }
                }
                Node::And(dest, src, operand) => {
                    let ins = AND | (src << 16) | (dest << 12) | encode(operand)?;
                    storage.set(i as u32, ins)?;
                }
                Node::Orr(dest, src, operand) => {
                    let ins = ORR | (src << 16) | (dest << 12) | encode(operand)?;
                    storage.set(i as u32, ins)?;
                }
                Node::Eor(dest, src, operand) => {
                    let ins = EOR | (src << 16) | (dest << 12) | encode(operand)?;
                    storage.set(i as u32, ins)?;
                }
                Node::Mvn(dest, operand) => {
                    let ins = MVN | (dest << 12) | encode(operand)?;
                    storage.set(i as u32, ins)?;
                }
                Node::Lsl(dest, src, operand) => {
                    let ins = LSL | (src << 16) | (dest << 12) | encode(operand)?;
                    storage.set(i as u32, ins)?;
                }
                Node::Lsr(dest, src, operand) => {
                    let ins = LSR | (src << 16) | (dest << 12) | encode(operand)?;
                    storage.set(i as u32, ins)?;
                }
                Node::Halt => {
                    storage.set(i as u32, HALT)?;
                }
            }
        }
        Ok(())
    }
}
