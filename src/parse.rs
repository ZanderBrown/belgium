use crate::node::{Node, Register, Type};
use crate::stream::Error;
use crate::stream::Input;
use crate::token::Range;
use crate::token::Type as TokenType;

use std::convert::TryFrom;

macro_rules! two_register {
    ( $input:expr, $token:expr, $type:ident ) => {{
        let (mem, mem_range) = $input.register()?;
        let c = $input.comma()?;
        let (reg, reg_range) = $input.register()?;
        Ok(Node::new(
            Type::$type(mem, reg),
            $token.range() + mem_range + c + reg_range,
        ))
    }};
}

impl Input {
    fn register(&mut self) -> Result<(Register, Range), Error> {
        let token = self.consume()?;
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
        let token = self.consume()?;
        match &*token {
            TokenType::Decimal(num) | TokenType::Hexadecimal(num) | TokenType::Binary(num) => {
                Ok(Node::new(Type::Unsigned(*num), token.range()))
            }
            TokenType::Minus => {
                let peek = self.peek()?;
                match *peek {
                    TokenType::Decimal(num) => {
                        self.consume()?;
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
        let token = self.peek()?;
        match &*token {
            TokenType::Symbol(sym) => {
                self.consume()?;
                Ok(Node::new(Type::Label(sym.to_string()), token.range()))
            }
            TokenType::Text(txt) => {
                self.consume()?;
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
        let token = self.consume()?;
        if let TokenType::Comma = *token {
            Ok(token.range())
        } else {
            Err(Error::new(
                format!("Expected a comma, got {}", *token),
                token.range(),
            ))
        }
    }

    /// # Errors
    ///
    pub fn node(&mut self) -> Result<Node, Error> {
        let token = self.consume()?;
        match &*token {
            TokenType::Symbol(ref symbol) => match symbol.as_ref() {
                "asect" => {
                    let pos = self.number()?;
                    if let Type::Unsigned(idx) = *pos {
                        Ok(Node::new(Type::Asect(idx), token.range() + pos.range()))
                    } else {
                        Err(Error::new(
                            format!("Expected address, got {}", *pos),
                            pos.range(),
                        ))
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
                    Ok(Node::new(
                        Type::Ldi(rn, Box::new(lit)),
                        token.range() + r + c + l,
                    ))
                }
                "halt" => Ok(Node::new(Type::Halt, token.range())),
                "wait" => Ok(Node::new(Type::Wait, token.range())),
                "dc" => self.immediate(),
                "ds" => {
                    let pos = self.number()?;
                    if let Type::Unsigned(idx) = *pos {
                        Ok(Node::new(Type::Ds(idx), token.range() + pos.range()))
                    } else {
                        Err(Error::new(
                            format!("Expected amount, got {}", *pos),
                            pos.range(),
                        ))
                    }
                }
                "end" => Ok(Node::new(Type::End, token.range())),
                symbol => {
                    let peek = self.peek()?;
                    match *peek {
                        TokenType::Colon => {
                            self.consume()?;
                            Ok(Node::new(
                                Type::Label(symbol.to_string()),
                                token.range() + peek.range(),
                            ))
                        }
                        TokenType::Gt => {
                            self.consume()?;
                            Ok(Node::new(
                                Type::Entry(symbol.to_string()),
                                token.range() + peek.range(),
                            ))
                        }
                        _ => Err(Error::new(format!("Unexpected {}", *token), token.range())),
                    }
                }
            },
            TokenType::Comment(_) => self.node(),
            _ => Err(Error::new(format!("Unexpected {}", *token), token.range())),
        }
    }
}
