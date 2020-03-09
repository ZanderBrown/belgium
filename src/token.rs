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
    pub fn new(data: Type, range: Range) -> Self {
        Self { data, range }
    }

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
    Label(String),
    Register(u8),
    LiteralNumber(u8),
    LiteralString(String),
    Eof
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Symbol(sym) => write!(f, "{}", sym),
            Self::Label(lbl) => write!(f, "{}:", lbl),
            Self::Register(reg) => write!(f, "r{}", reg),
            Self::LiteralNumber(num) => write!(f, "{}", num),
            Self::LiteralString(txt) => write!(f, "\"{}\"", txt),
            Self::Eof => Ok(())
        }
    }
}
