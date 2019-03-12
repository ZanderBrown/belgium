use crate::stream::Error;
use crate::stream::Input;
use crate::stream::Token;

use std::collections::HashMap;

pub type Register = u32;
pub type Memory = u32;
pub type Label = String;

pub enum Operand {
    Register(Register),
    Literal(u32),
}

#[derive(PartialEq)]
pub enum Comparison {
    None,
    Equal,
    NotEqual,
    Greater,
    Less,
}

pub enum Node {
    Ldr(Register, Memory),
    Str(Register, Memory),
    Add(Register, Register, Operand),
    Sub(Register, Register, Operand),
    Mov(Register, Operand),
    Cmp(Register, Operand),
    B(Comparison, Label),
    And(Register, Register, Operand),
    Orr(Register, Register, Operand),
    Eor(Register, Register, Operand),
    Mvn(Register, Operand),
    Lsl(Register, Register, Operand),
    Lsr(Register, Register, Operand),
    Halt,
}

pub trait Parser {
    fn parse(&mut self) -> Result<(Vec<Node>, HashMap<String, usize>), Error>;
    fn register(&mut self) -> Result<Register, Error>;
    fn memory(&mut self) -> Result<Memory, Error>;
    fn label(&mut self) -> Result<Label, Error>;
    fn operand(&mut self) -> Result<Operand, Error>;
    fn comma(&mut self) -> Result<(), Error>;
    fn colon(&mut self) -> Result<(), Error>;
}

fn encode(n: u32) -> Option<u32> {
    for i in 0..16 {
        let m = n.rotate_left(i * 2);
        if m < 256 {
            return Some((i << 8) | m);
        }
    }
    None
}

impl Parser for Input {
    fn register(&mut self) -> Result<Register, Error> {
        if let Some(tok) = self.next() {
            match tok? {
                Token::Register(r) => Ok(r),
                tok => Err(self.error(format!("Expected register got {}", tok))),
            }
        } else {
            Err(self.error("Expected register".into()))
        }
    }

    fn memory(&mut self) -> Result<Memory, Error> {
        if let Some(tok) = self.next() {
            match tok? {
                Token::Memory(r) => Ok(r),
                tok => Err(self.error(format!("Expected memory address got {}", tok))),
            }
        } else {
            Err(self.error("Expected memory address".into()))
        }
    }

    fn operand(&mut self) -> Result<Operand, Error> {
        if let Some(tok) = self.next() {
            match tok? {
                Token::Register(r) => Ok(Operand::Register(r)),
                Token::Number(r) => {
                    if let Some(r) = encode(r) {
                        Ok(Operand::Literal(r))
                    } else {
                        Err(self.error(format!("Can't encode {}", r)))
                    }
                }
                tok => Err(self.error(format!("Expected operand got {}", tok))),
            }
        } else {
            Err(self.error("Expected operand".into()))
        }
    }

    fn label(&mut self) -> Result<Label, Error> {
        if let Some(tok) = self.next() {
            match tok? {
                Token::Label(r) => Ok(r),
                tok => Err(self.error(format!("Expected label got {}", tok))),
            }
        } else {
            Err(self.error("Expected label".into()))
        }
    }

    fn comma(&mut self) -> Result<(), Error> {
        if let Some(tok) = self.next() {
            match tok? {
                Token::Comma => Ok(()),
                tok => Err(self.error(format!("Expected comma got {}", tok))),
            }
        } else {
            Err(self.error("Expected comma".into()))
        }
    }

    fn colon(&mut self) -> Result<(), Error> {
        if let Some(tok) = self.next() {
            match tok? {
                Token::Colon => Ok(()),
                tok => Err(self.error(format!("Expected colon got {}", tok))),
            }
        } else {
            Err(self.error("Expected colon".into()))
        }
    }

    fn parse(&mut self) -> Result<(Vec<Node>, HashMap<String, usize>), Error> {
        let mut prog = Vec::with_capacity(20);
        let mut labels = HashMap::new();
        while let Some(token) = self.next() {
            match token? {
                Token::Command(cmd) => match cmd.as_str() {
                    "LDR" => prog.push(Node::Ldr(self.register()?, {
                        self.comma()?;
                        self.memory()?
                    })),
                    "STR" => prog.push(Node::Str(self.register()?, {
                        self.comma()?;
                        self.memory()?
                    })),
                    "ADD" => prog.push(Node::Add(
                        self.register()?,
                        {
                            self.comma()?;
                            self.register()?
                        },
                        {
                            self.comma()?;
                            self.operand()?
                        },
                    )),
                    "SUB" => prog.push(Node::Sub(
                        self.register()?,
                        {
                            self.comma()?;
                            self.register()?
                        },
                        {
                            self.comma()?;
                            self.operand()?
                        },
                    )),
                    "MOV" => prog.push(Node::Mov(self.register()?, {
                        self.comma()?;
                        self.operand()?
                    })),
                    "CMP" => prog.push(Node::Cmp(self.register()?, {
                        self.comma()?;
                        self.operand()?
                    })),
                    "B" => prog.push(Node::B(Comparison::None, self.label()?)),
                    "BEQ" => prog.push(Node::B(Comparison::Equal, self.label()?)),
                    "BNE" => prog.push(Node::B(Comparison::NotEqual, self.label()?)),
                    "BGT" => prog.push(Node::B(Comparison::Greater, self.label()?)),
                    "BLT" => prog.push(Node::B(Comparison::Less, self.label()?)),
                    "AND" => prog.push(Node::And(
                        self.register()?,
                        {
                            self.comma()?;
                            self.register()?
                        },
                        {
                            self.comma()?;
                            self.operand()?
                        },
                    )),
                    "ORR" => prog.push(Node::Orr(
                        self.register()?,
                        {
                            self.comma()?;
                            self.register()?
                        },
                        {
                            self.comma()?;
                            self.operand()?
                        },
                    )),
                    "EOR" => prog.push(Node::Eor(
                        self.register()?,
                        {
                            self.comma()?;
                            self.register()?
                        },
                        {
                            self.comma()?;
                            self.operand()?
                        },
                    )),
                    "MVN" => prog.push(Node::Mvn(self.register()?, {
                        self.comma()?;
                        self.operand()?
                    })),
                    "LSL" => prog.push(Node::Lsl(
                        self.register()?,
                        {
                            self.comma()?;
                            self.register()?
                        },
                        {
                            self.comma()?;
                            self.operand()?
                        },
                    )),
                    "LSR" => prog.push(Node::Lsr(
                        self.register()?,
                        {
                            self.comma()?;
                            self.register()?
                        },
                        {
                            self.comma()?;
                            self.operand()?
                        },
                    )),
                    "HALT" => prog.push(Node::Halt),
                    _ => return Err(self.error(format!("Unexpected {}", cmd))),
                },
                Token::Label(lbl) => {
                    self.colon()?;
                    labels.insert(lbl, prog.len());
                }
                tok => return Err(self.error(format!("Unexpected {}", tok))),
            }
        }
        Ok((prog, labels))
    }
}
