use std::error;
use std::fmt;

use crate::token::{Range, Token, Type};
use crate::token::Point;

#[derive(Debug)]
pub struct Error {
    message: String,
    at: Range,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.message, self.at)
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
}

impl Error {
    #[must_use]
    pub fn new(message: String, at: Range) -> Self {
        Self { message, at }
    }

    #[must_use]
    pub fn at(&self) -> Range {
        self.at
    }

    pub fn print(&self, src: Option<&Input>) {
        if let Some(src) = src {
            let line = format!("{}", self.at.start().line());
            let lines: Vec<&str> = src.input.split('\n').collect();
            if lines.len() >= self.at.start().line() {
                eprintln!("{} ❘{}", line, lines[self.at.start().line() - 1]);
            } else {
                eprintln!("{} ❘ [err]", line);
            }
            eprintln!(
                "{:idt$} ❘{:pad$}{:↑>num$}",
                " ",
                "",
                "↑",
                idt = line.len(),
                pad = self.at.start().column(),
                num = self.at.end().column() - self.at.start().column()
            );
            eprintln!(
                "{:idt$} ❘{:pad$}{}",
                " ",
                " ",
                self.message,
                idt = line.len(),
                pad = self.at.start().column()
            );
        } else {
            eprintln!("{}", self.message);
        }
    }
}

pub struct Input {
    input: String,
    line: usize,
    col: usize,
    pos: usize,
    current: Option<Token>,
}

impl Input {
    fn forward(&mut self) {
        if let Some(ch) = self.input.chars().nth(self.pos) {
            self.pos += 1;
            if ch == '\n' {
                self.line += 1;
                self.col = 0;
            } else {
                self.col += 1;
            }
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.input.chars().nth(self.pos)
    }

    fn here(&self) -> Point {
        Point::new(self.line, self.col)
    }

    fn read(&mut self, matcher: &dyn Fn(char) -> bool) -> String {
        let mut s = String::new();
        while let Some(ch) = self.peek_char() {
            if matcher(ch) {
                s.push(ch);
                self.forward();
            } else {
                break;
            }
        }
        s
    }

    fn read_next(&mut self) -> Result<Token, Error> {
        self.read(&|ch| ch.is_whitespace());
        let start = self.here();
        if let Some(ch) = self.peek_char() {
            match ch {
                'r' => {
                    self.forward();
                    if let Some(reg) = self.peek_char() {
                        self.forward();
                        match reg {
                            '1' => Ok(Token::new(Type::Register(1), Range::new(start, self.here()))),
                            '2' => Ok(Token::new(Type::Register(2), Range::new(start, self.here()))),
                            '3' => Ok(Token::new(Type::Register(3), Range::new(start, self.here()))),
                            '4' => Ok(Token::new(Type::Register(4), Range::new(start, self.here()))),
                            _ => Err(Error::new(format!("Invalid register {}", reg), Range::new(start, self.here())))
                        }
                    } else {
                        Err(Error::new("Unexpected end of file".to_string(), Range::new(start, self.here())))
                    }
                }
                '"' => {
                    let text = self.read(&|c| c != '"');
                    self.forward();
                    Ok(Token::new(Type::LiteralString(text), Range::new(start, self.here())))
                }
                ch if ch.is_alphabetic() => {
                    let text = self.read(&|c| c.is_alphanumeric());
                    Ok(Token::new(Type::Symbol(text), Range::new(start, self.here())))
                }
                ch => {
                    self.forward();
                    Err(Error::new(
                        format!("Unknown {}", ch),
                        Range::new(start, self.here()),
                    ))
                }
            }
        } else {
            Ok(Token::new(Type::Eof, Range::new(start, self.here())))
        }
    }

    /// # Errors
    /// 
    pub fn peek(&mut self) -> Result<Option<Token>, Error> {
        if self.current.is_none() {
            self.current = Some(self.read_next()?);
        }
        Ok(self.current.clone())
    }

    /// # Errors
    /// 
    pub fn consume(&mut self) -> Result<Token, Error> {
        if let Some(tok) = self.current.take() {
            Ok(tok)
        } else {
            self.read_next()
        }
    }
}

impl From<String> for Input {
    fn from(input: String) -> Self {
        Self {
            input,
            line: 1,
            col: 0,
            pos: 0,
            current: None,
        }
    }
}
