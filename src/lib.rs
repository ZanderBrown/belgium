#![deny(clippy::all)]
#![deny(clippy::pedantic)]

// mod assemble;
mod machine;
// mod parse;
mod opcodes;
mod stream;

// Make enough public to easily run programs
// pub use crate::assemble::Assemble;
pub use crate::machine::ChangeEvent;
pub use crate::machine::Observer;
// pub use crate::parse::Parser;
pub use crate::machine::Machine;
pub use crate::machine::{COUNTER, STACK, STATUS};
pub use crate::stream::Error;
pub use crate::stream::Input;
