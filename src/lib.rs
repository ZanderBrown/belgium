#![deny(clippy::all)]
#![deny(clippy::pedantic)]

mod assemble;
mod cpu;
mod memory;
mod parse;
mod storage;
mod stream;
mod vm;

// Make enough public to easily run programs
pub use crate::assemble::Assemble;
pub use crate::cpu::{
    ADD, ADDRESS, AND, B, CIR, CMP, COND, COND_EQ, COND_GT, COND_LT, COND_NE, COND_NONE, COUNTER,
    DATA, DEST, EOR, HALT, IMMEDIATE, INDIRECT, LDR, LSL, LSR, MBUFF, MOV, MVN, OFFSET, OPERATION,
    ORR, SHIFT, SOURCE, STATUS, STR, SUB,
};
pub use crate::memory::ChangeEvent;
pub use crate::memory::Memory;
pub use crate::memory::Observer;
pub use crate::parse::Parser;
pub use crate::storage::Storage;
pub use crate::stream::Error;
pub use crate::stream::Input;
pub use crate::vm::execute;
