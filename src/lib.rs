#![deny(clippy::all)]
#![deny(clippy::pedantic)]

// mod assemble;
mod machine;
// mod parse;
mod alu;
mod opcodes;
mod stack;
mod stream;

// Make enough public to easily run programs
// pub use crate::assemble::Assemble;
pub use crate::machine::ChangeEvent;
pub use crate::machine::Observer;
// pub use crate::parse::Parser;
pub use crate::machine::Machine;
pub use crate::machine::{Response, COUNTER, SP, STATUS};
pub use crate::stream::Error;
pub use crate::stream::Input;
