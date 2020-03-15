use std::fmt;
use std::ops::Add;
use std::ops::Deref;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Range {
    start: Point,
    end: Point,
}

impl Range {
    pub fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    pub fn start(&self) -> Point {
        self.start
    }

    pub fn end(&self) -> Point {
        self.end
    }
}

impl Add for Range {
    type Output = Range;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            start: self.start,
            end: rhs.end,
        }
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.start.line() == self.end.line() {
            write!(
                f,
                "[{}:{}-{}]",
                self.start.line(),
                self.start.column(),
                self.end.column()
            )
        } else {
            write!(
                f,
                "[{}-{}:{}-{}]",
                self.start.line(),
                self.end.line(),
                self.start.column(),
                self.end.column()
            )
        }
    }
}

/// Line, Column
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Point(usize, usize);

impl Point {
    pub fn new(line: usize, column: usize) -> Self {
        Self(line, column)
    }

    pub fn line(&self) -> usize {
        self.0
    }

    pub fn column(&self) -> usize {
        self.1
    }
}

#[derive(Clone, Debug)]
pub struct Token {
    data: Type,
    range: Range,
}

impl Token {
    #[must_use]
    pub fn new(data: Type, range: Range) -> Self {
        Self { data, range }
    }

    #[must_use]
    pub fn range(&self) -> Range {
        self.range
    }
}

impl Deref for Token {
    type Target = Type;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Type {
    Symbol(String),
    /// rAB
    Register(u8),
    /// 123
    Decimal(u8),
    /// 0xAB
    Hexadecimal(u8),
    /// 0b01010101
    Binary(u8),
    /// blah
    Text(String),
    /// # blah
    Comment(String),
    /// An "entry point"
    Entry(String),
    /// ,
    Comma,
    /// -
    Minus,
    /// +
    Add,
    /// _
    Underscore,
    /// :
    Colon,
    /// >
    Gt,
    /// '
    Apostrophy,
    /// /
    Slash,
    /// ?
    Question,
    /// !
    Exclame,
    /// .
    Dot,
    Eof,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Symbol(sym) => write!(f, "{}", sym),
            Self::Register(reg) => write!(f, "r{}", reg),
            Self::Decimal(num) => write!(f, "{}", num),
            Self::Hexadecimal(num) => write!(f, "0x{:X}", num),
            Self::Binary(num) => write!(f, "0b{:b}", num),
            Self::Text(txt) => write!(f, "\"{}\"", txt),
            Self::Entry(txt) => write!(f, "_{}", txt),
            Self::Comment(txt) => write!(f, "#{}", txt),
            Self::Comma => write!(f, ","),
            Self::Add => write!(f, "+"),
            Self::Minus => write!(f, "-"),
            Self::Underscore => write!(f, "_"),
            Self::Colon => write!(f, ":"),
            Self::Gt => write!(f, ">"),
            Self::Apostrophy => write!(f, "'"),
            Self::Slash => write!(f, "/"),
            Self::Question => write!(f, "?"),
            Self::Exclame => write!(f, "!"),
            Self::Dot => write!(f, "."),
            Self::Eof => Ok(()),
        }
    }
}
