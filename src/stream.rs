use std::convert::TryFrom;
use std::error;
use std::fmt;

use crate::token::Point;
use crate::token::{Range, Token, Type};

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

macro_rules! token {
    ( $input:expr, $start:expr, $type:expr ) => {{
        Ok(Token::new($type, Range::new($start, $input.here())))
    }};
}

macro_rules! char_token {
    ( $input:expr, $start:expr, $type:expr ) => {{
        $input.forward();
        Ok(Token::new($type, Range::new($start, $input.here())))
    }};
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

    fn read_hex(&mut self, start: Point) -> Result<Token, Error> {
        self.forward();
        let hex: Vec<_> = self
            .read(&|c| c.is_digit(16))
            .chars()
            .map(|ch| {
                u8::try_from(ch.to_digit(16).expect("Should have been hex"))
                    .expect("Somehow still out of range hex")
            })
            .collect();
        if hex.len() == 2 {
            let byte = (hex[0] << 4) | hex[1];
            token!(self, start, Type::Hexadecimal(byte))
        } else {
            Err(Error::new(
                format!("Expected 2 digits, got {}", hex.len()),
                Range::new(start, self.here()),
            ))
        }
    }

    fn read_bin(&mut self, start: Point) -> Result<Token, Error> {
        self.forward();
        let bin = self.read(&|c| c.is_digit(2));
        if bin.len() == 8 {
            let byte = bin
                .chars()
                .map(|ch| {
                    u8::try_from(ch.to_digit(2).expect("Should have been bin"))
                        .expect("Somehow still out of range binary")
                })
                .rev()
                .enumerate()
                .fold(0, |byte, (n, bit)| byte | (bit << n));
            token!(self, start, Type::Binary(byte))
        } else {
            Err(Error::new(
                format!("Expected 8 bits, got {}", bin.len()),
                Range::new(start, self.here()),
            ))
        }
    }

    fn read_register(&mut self, start: Point) -> Result<Token, Error> {
        self.forward();
        if let Some(reg) = self.peek_char() {
            self.forward();
            match reg {
                '0' => token!(self, start, Type::Register(0)),
                '1' => token!(self, start, Type::Register(1)),
                '2' => token!(self, start, Type::Register(2)),
                '3' => token!(self, start, Type::Register(3)),
                _ => Err(Error::new(
                    format!("Invalid register r{}", reg),
                    Range::new(start, self.here()),
                )),
            }
        } else {
            Err(Error::new(
                "Unexpected end of file".to_string(),
                Range::new(start, self.here()),
            ))
        }
    }

    fn read_zero_prefix(&mut self, start: Point) -> Result<Token, Error> {
        self.forward();
        match self.peek_char() {
            Some('x') => self.read_hex(start),
            Some('b') => self.read_bin(start),
            Some(_) => {
                let num = format!("0{}", self.read(&|ch| ch.is_digit(10)));
                let num = num
                    .chars()
                    .map(|ch| ch.to_digit(10).expect("Should have been decimal"))
                    .rev()
                    .enumerate()
                    .fold(0, |num, (n, digit)| num + (digit * (10_u32.pow(n as u32))));
                if num > 255 {
                    Err(Error::new(
                        format!("Expected number in range 0-255, got {}", num),
                        Range::new(start, self.here()),
                    ))
                } else {
                    token!(
                        self,
                        start,
                        Type::Decimal(u8::try_from(num).expect("Somehow still out of range"))
                    )
                }
            }
            None => Err(Error::new(
                "Unexpected end of file".to_string(),
                Range::new(start, self.here()),
            )),
        }
    }

    fn read_next(&mut self) -> Result<Token, Error> {
        self.read(&|ch| ch.is_whitespace());
        let start = self.here();
        if let Some(ch) = self.peek_char() {
            match ch {
                'r' => self.read_register(start),
                '0' => self.read_zero_prefix(start),
                ',' => char_token!(self, start, Type::Comma),
                '-' => char_token!(self, start, Type::Minus),
                '+' => char_token!(self, start, Type::Add),
                ':' => char_token!(self, start, Type::Colon),
                '_' => char_token!(self, start, Type::Underscore),
                '>' => char_token!(self, start, Type::Gt),
                '\'' => char_token!(self, start, Type::Apostrophy),
                '/' => char_token!(self, start, Type::Slash),
                '?' => char_token!(self, start, Type::Question),
                '!' => char_token!(self, start, Type::Exclame),
                '.' => char_token!(self, start, Type::Dot),
                '"' => {
                    self.forward();
                    let text = self.read(&|c| c != '"');
                    self.forward();
                    token!(self, start, Type::Text(text))
                }
                '#' => {
                    self.forward();
                    let text = self.read(&|c| c != '\n');
                    self.forward();
                    token!(self, start, Type::Comment(text))
                }
                ch if ch.is_digit(10) => {
                    let num = self.read(&|c| c.is_digit(10));
                    let num = num
                        .chars()
                        .map(|ch| ch.to_digit(10).expect("Should have been deciman"))
                        .rev()
                        .enumerate()
                        .fold(0, |num, (n, digit)| num + (digit * (10_u32.pow(n as u32))));
                    if num > 255 {
                        Err(Error::new(
                            format!("Expected number in range 0-255, got {}", num),
                            Range::new(start, self.here()),
                        ))
                    } else {
                        token!(
                            self,
                            start,
                            Type::Decimal(u8::try_from(num).expect("Somehow still out of range"))
                        )
                    }
                }
                ch if ch.is_alphabetic() => {
                    let text = self.read(&|c| c.is_alphanumeric());
                    token!(self, start, Type::Symbol(text))
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
            token!(self, start, Type::Eof)
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
