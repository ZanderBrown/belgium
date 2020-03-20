use crate::token::Range;

use std::fmt;
use std::ops::Deref;

pub type Register = u8;
pub type Literal = u8;
pub type Label = String;

#[derive(Debug, Clone)]
pub enum Type {
    Move(Register, Register),
    Add(Register, Register),
    Addc(Register, Register),
    Sub(Register, Register),
    And(Register, Register),
    Or(Register, Register),
    Xor(Register, Register),
    Cmp(Register, Register),
    Not(Register),
    Neg(Register),
    Inc(Register),
    Dec(Register),
    Shr(Register),
    Shla(Register),
    Shra(Register),
    Rol(Register),
    St(Register, Register),
    Ld(Register, Register),
    Push(Register),
    Pop(Register),
    Ldsa(Register, Literal),
    Addsp(Literal),
    Setsp(Literal),
    Pushall,
    Popall,
    Ldi(Register, Box<Node>),
    Halt,
    Wait,
    Jsr(Label),
    Rts,
    Ioi,
    Rti,
    Crc,
    Osix,
    Rand,
    BeqBz(Label),
    BneBnz(Label),
    BhsBcs(Label),
    BloBcc(Label),
    Bmi(Label),
    Bpl(Label),
    Bvs(Label),
    Bvc(Label),
    Bhi(Label),
    Bls(Label),
    Bge(Label),
    Blt(Label),
    Bgt(Label),
    Ble(Label),
    Br(Label),
    Nop(Label),
    Ldc(Register, Register),
    Label(String),
    Entry(String),
    Signed(i8),
    Unsigned(u8),
    Asect(Literal),
    Dc(Vec<Node>),
    Ds(u8),
    End,
}

#[derive(Debug, Clone)]
pub struct Node {
    data: Type,
    range: Range,
}

impl Node {
    #[must_use]
    pub fn new(data: Type, range: Range) -> Self {
        Self { data, range }
    }

    #[must_use]
    pub fn range(&self) -> Range {
        self.range
    }
}

impl Deref for Node {
    type Target = Type;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Move(from, to) => write!(f, "move r{}, r{}", from, to),
            Self::Add(a, b) => write!(f, "add r{}, r{}", a, b),
            Self::Addc(a, b) => write!(f, "addc r{}, r{}", a, b),
            Self::Sub(a, b) => write!(f, "sub r{}, r{}", a, b),
            Self::And(a, b) => write!(f, "and r{}, r{}", a, b),
            Self::Or(a, b) => write!(f, "or r{}, r{}", a, b),
            Self::Xor(a, b) => write!(f, "xor r{}, r{}", a, b),
            Self::Cmp(a, b) => write!(f, "cmp r{}, r{}", a, b),
            Self::Not(a) => write!(f, "not r{}", a),
            Self::Neg(a) => write!(f, "neg r{}", a),
            Self::Inc(a) => write!(f, "inc r{}", a),
            Self::Dec(a) => write!(f, "dec r{}", a),
            Self::Shr(a) => write!(f, "shr r{}", a),
            Self::Shla(a) => write!(f, "shla r{}", a),
            Self::Shra(a) => write!(f, "shra r{}", a),
            Self::Rol(a) => write!(f, "rol r{}", a),
            Self::St(a, b) => write!(f, "st r{}, r{}", a, b),
            Self::Ld(a, b) => write!(f, "ld r{}, r{}", a, b),
            Self::Push(a) => write!(f, "push r{}", a),
            Self::Pop(a) => write!(f, "pop r{}", a),
            Self::Ldsa(r, o) => write!(f, "ldsa r{}, {}", r, o),
            Self::Addsp(o) => write!(f, "addsp {}", o),
            Self::Setsp(o) => write!(f, "setsp {}", o),
            Self::Pushall => write!(f, "pushall"),
            Self::Popall => write!(f, "popall"),
            Self::Ldi(reg, i) => write!(f, "ldi r{}, {}", reg, i.data),
            Self::Halt => write!(f, "halt"),
            Self::Wait => write!(f, "wait"),
            Self::Jsr(l) => write!(f, "jsr {}", l),
            Self::Rts => write!(f, "rts"),
            Self::Ioi => write!(f, "ioi"),
            Self::Rti => write!(f, "rti"),
            Self::Crc => write!(f, "crc"),
            Self::Osix => write!(f, "osix"),
            Self::Rand => write!(f, "rand"),
            Self::BeqBz(l) => write!(f, "beq {}", l),
            Self::BneBnz(l) => write!(f, "bne {}", l),
            Self::BhsBcs(l) => write!(f, "bhs {}", l),
            Self::BloBcc(l) => write!(f, "blo {}", l),
            Self::Bmi(l) => write!(f, "bmi {}", l),
            Self::Bpl(l) => write!(f, "pls {}", l),
            Self::Bvs(l) => write!(f, "bvs {}", l),
            Self::Bvc(l) => write!(f, "bvc {}", l),
            Self::Bhi(l) => write!(f, "bhi {}", l),
            Self::Bls(l) => write!(f, "bls {}", l),
            Self::Bge(l) => write!(f, "bge {}", l),
            Self::Blt(l) => write!(f, "blt {}", l),
            Self::Bgt(l) => write!(f, "bgt {}", l),
            Self::Ble(l) => write!(f, "ble {}", l),
            Self::Br(l) => write!(f, "br {}", l),
            Self::Nop(l) => write!(f, "nop {}", l),
            Self::Ldc(a, b) => write!(f, "ldc r{}, r{}", a, b),
            Self::Label(l) | Self::Entry(l) => write!(f, "{}", l),
            Self::Signed(num) => write!(f, "{}", num),
            Self::Unsigned(num) => write!(f, "{}", num),
            Self::Asect(p) => write!(f, "asect {}", p),
            Self::Dc(d) => write!(
                f,
                "dc {}",
                d.iter()
                    .map(|n| format!("{}", **n))
                    .collect::<Vec<_>>()
                    .join(",")
            ),
            Self::Ds(s) => write!(f, "ds {}", s),
            Self::End => write!(f, "end"),
        }
    }
}
