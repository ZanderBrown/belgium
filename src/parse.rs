use crate::node::{Node, Register, Type};
use crate::section::Section;
use crate::stream::Error;
use crate::stream::Input;
use crate::token::Range;
use crate::token::Type as TokenType;
use std::ops::Deref;

use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::rc::Rc;

pub struct Parser {
    input: Input,
    building: Rc<RefCell<Section>>,
    rsects: HashMap<String, Rc<RefCell<Section>>>,
    templates: HashMap<String, Rc<RefCell<Section>>>,
    asects: Vec<Rc<RefCell<Section>>>
}

macro_rules! two_register {
    ( $input:expr, $token:expr, $type:ident ) => {{
        let (mem, mem_range) = $input.register()?;
        let c = $input.comma()?;
        let (reg, reg_range) = $input.register()?;
        $input.building.borrow_mut().add(Node::new(
            Type::$type(mem, reg),
            $token.range() + mem_range + c + reg_range,
        ))?;
    }};
}

impl Parser {
    #[must_use]
    pub fn new(input: Input) -> Self {
        Self {
            input,
            building: Rc::new(RefCell::new(Section::None)),
            rsects: HashMap::new(),
            templates: HashMap::new(),
            asects: Vec::new()
        }
    }

    fn register(&mut self) -> Result<(Register, Range), Error> {
        let token = self.input.consume()?;
        if let TokenType::Register(reg) = *token {
            Ok((reg, token.range()))
        } else {
            Err(Error::new(
                format!("Expected a register, got {}", *token),
                token.range(),
            ))
        }
    }

    fn number(&mut self) -> Result<Node, Error> {
        let token = self.input.consume()?;
        match &*token {
            TokenType::Decimal(num) | TokenType::Hexadecimal(num) | TokenType::Binary(num) => {
                Ok(Node::new(Type::Unsigned(*num), token.range()))
            }
            TokenType::Minus => {
                let peek = self.input.peek()?;
                match *peek {
                    TokenType::Decimal(num) => {
                        self.input.consume()?;
                        if num <= 128 {
                            let num = i8::try_from(
                                0 - i16::try_from(num).expect("somehow still out of range"),
                            )
                            .expect("somehow still out of range");
                            Ok(Node::new(Type::Signed(num), token.range()))
                        } else {
                            Err(Error::new(
                                format!("Expected a number in range -128->127, got {}", *peek),
                                token.range(),
                            ))
                        }
                    }
                    TokenType::Hexadecimal(_) | TokenType::Binary(_) => Err(Error::new(
                        format!("Only decimal numbers can be signed, not {}", *peek),
                        token.range() + peek.range(),
                    )),
                    _ => Err(Error::new(
                        format!("Expected an number, got {}", *peek),
                        token.range(),
                    )),
                }
            }
            _ => Err(Error::new(
                format!("Expected an number, got {}", *token),
                token.range(),
            )),
        }
    }

    fn immediate(&mut self) -> Result<Node, Error> {
        let token = self.input.peek()?;
        match &*token {
            TokenType::Symbol(sym) => {
                self.input.consume()?;
                Ok(Node::new(Type::Label(sym.to_string()), token.range()))
            }
            TokenType::Text(txt) => {
                self.input.consume()?;
                let bytes = txt.as_bytes();
                if bytes.len() == 1 {
                    Ok(Node::new(Type::Unsigned(bytes[0]), token.range()))
                } else {
                    Err(Error::new(
                        format!(
                            "Expected a single byte, got {} ({} bytes)",
                            *token,
                            bytes.len()
                        ),
                        token.range(),
                    ))
                }
            }
            _ => self.number(),
        }
    }

    fn comma(&mut self) -> Result<Range, Error> {
        let token = self.input.consume()?;
        if let TokenType::Comma = *token {
            Ok(token.range())
        } else {
            Err(Error::new(
                format!("Expected a comma, got {}", *token),
                token.range(),
            ))
        }
    }

    fn symbol(&mut self) -> Result<String, Error> {
        let token = self.input.consume()?;
        if let TokenType::Symbol(sym) = &*token {
            Ok(sym.to_string())
        } else {
            Err(Error::new(
                format!("Expected a symbol, got {}", *token),
                token.range(),
            ))
        }
    }

    #[must_use]
    pub fn sections(&self) -> Vec<Rc<RefCell<Section>>> {
        let mut sects = Vec::new();
        for sect in self.rsects.values() {
            sects.push(Rc::clone(sect));
        }
        for sect in &self.asects {
            sects.push(Rc::clone(sect));
        }
        sects
    }

    /// # Errors
    ///
    pub fn node(&mut self) -> Result<(), Error> {
        loop {
            let token = self.input.consume()?;
            match &*token {
                TokenType::Symbol(ref symbol) => match symbol.as_ref() {
                    "asect" => {
                        let pos = self.number()?;
                        if let Type::Unsigned(idx) = *pos {
                            self.building = Rc::new(RefCell::new(Section::absolute(idx)));
                            self.asects.push(Rc::clone(&self.building));
                        } else {
                            return Err(Error::new(
                                format!("Expected address, got {}", *pos),
                                pos.range(),
                            ));
                        }
                    }
                    "rsect" => {
                        let name = self.symbol()?;
                        if let Some(existing) = self.rsects.get(&name) {
                            self.building = Rc::clone(existing);
                        } else {
                            let new = Rc::new(RefCell::new(Section::rsect(name.clone())));
                            self.rsects.insert(name, Rc::clone(&new));
                            self.building = new;
                        }
                    }
                    "st" => two_register!(self, token, St),
                    "ld" => two_register!(self, token, Ld),
                    "add" => two_register!(self, token, Add),
                    "sub" => two_register!(self, token, Sub),
                    "or" => two_register!(self, token, Or),
                    "xor" => two_register!(self, token, Xor),
                    "and" => two_register!(self, token, And),
                    "ldi" => {
                        let (rn, r) = self.register()?;
                        let c = self.comma()?;
                        let lit = self.immediate()?;
                        let l = lit.range();
                        self.building.borrow_mut().add(Node::new(
                            Type::Ldi(rn, Box::new(lit)),
                            token.range() + r + c + l,
                        ))?;
                    }
                    "halt" => self
                        .building
                        .borrow_mut()
                        .add(Node::new(Type::Halt, token.range()))?,
                    "wait" => self
                        .building
                        .borrow_mut()
                        .add(Node::new(Type::Wait, token.range()))?,
                    "dc" => {
                        let data = self.immediate()?;
                        self.building.borrow_mut().add(data)?;
                    },
                    "ds" => {
                        let pos = self.number()?;
                        if let Type::Unsigned(idx) = *pos {
                            self.building
                                .borrow_mut()
                                .add(Node::new(Type::Ds(idx), token.range() + pos.range()))?;
                        } else {
                            return Err(Error::new(
                                format!("Expected amount, got {}", *pos),
                                pos.range(),
                            ));
                        }
                    }
                    "end" => break,
                    symbol => {
                        let peek = self.input.peek()?;
                        match *peek {
                            TokenType::Colon => {
                                self.input.consume()?;
                                self.building.borrow_mut().add(Node::new(
                                    Type::Label(symbol.to_string()),
                                    token.range() + peek.range(),
                                ))?;
                            }
                            TokenType::Gt => {
                                self.input.consume()?;
                                self.building.borrow_mut().add(Node::new(
                                    Type::Entry(symbol.to_string()),
                                    token.range() + peek.range(),
                                ))?;
                            }
                            _ => return Err(Error::new(format!("Unexpected {}", *token), token.range())),
                        }
                    }
                },
                TokenType::Comment(_) => (),
                _ => return Err(Error::new(format!("Unexpected {}", *token), token.range())),
            }
        }
        Ok(())
    }
}

impl Deref for Parser {
    type Target = Input;

    fn deref(&self) -> &Self::Target {
        &self.input
    }
}
