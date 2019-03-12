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

fn encode(n: &Operand) -> u32 {
    match n {
        Operand::Literal(n) => IMMEDIATE | n,
        Operand::Register(n) => *n,
    }
}

pub trait Assemble {
    fn assemble(&mut self, mem: &mut impl Storage) -> Result<(), Error>;
}

impl Assemble for Input {
    fn assemble(&mut self, storage: &mut impl Storage) -> Result<(), Error> {
        let (nodes, labels) = self.parse()?;
        for (i, node) in nodes.iter().enumerate() {
            #[allow(clippy::cast_possible_truncation)]
            let i = i as u32;
            match node {
                Node::Ldr(src, mem) => {
                    let ins = LDR | (src << 16) | mem;
                    storage.set(i, ins)?;
                }
                Node::Str(src, mem) => {
                    let ins = STR | (src << 16) | mem;
                    storage.set(i, ins)?;
                }
                Node::Add(dest, src, operand) => {
                    let ins = ADD | (src << 16) | (dest << 12) | encode(operand);
                    storage.set(i, ins)?;
                }
                Node::Sub(dest, src, operand) => {
                    let ins = SUB | (src << 16) | (dest << 12) | encode(operand);
                    storage.set(i, ins)?;
                }
                Node::Mov(dest, operand) => {
                    let ins = MOV | (dest << 12) | encode(operand);
                    storage.set(i, ins)?;
                }
                Node::Cmp(src, operand) => {
                    let ins = CMP | (src << 16) | encode(operand);
                    storage.set(i, ins)?;
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
                        #[allow(clippy::cast_possible_truncation)]
                        let ins = cond | B | (*pos as u32);
                        storage.set(i, ins)?;
                    } else {
                        return Err(Error::new(format!("Unkown label {}", label), None));
                    }
                }
                Node::And(dest, src, operand) => {
                    let ins = AND | (src << 16) | (dest << 12) | encode(operand);
                    storage.set(i, ins)?;
                }
                Node::Orr(dest, src, operand) => {
                    let ins = ORR | (src << 16) | (dest << 12) | encode(operand);
                    storage.set(i, ins)?;
                }
                Node::Eor(dest, src, operand) => {
                    let ins = EOR | (src << 16) | (dest << 12) | encode(operand);
                    storage.set(i, ins)?;
                }
                Node::Mvn(dest, operand) => {
                    let ins = MVN | (dest << 12) | encode(operand);
                    storage.set(i, ins)?;
                }
                Node::Lsl(dest, src, operand) => {
                    let ins = LSL | (src << 16) | (dest << 12) | encode(operand);
                    storage.set(i, ins)?;
                }
                Node::Lsr(dest, src, operand) => {
                    let ins = LSR | (src << 16) | (dest << 12) | encode(operand);
                    storage.set(i, ins)?;
                }
                Node::Halt => {
                    storage.set(i, HALT)?;
                }
            }
        }
        Ok(())
    }
}
