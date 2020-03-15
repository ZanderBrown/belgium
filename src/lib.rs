#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![allow(clippy::non_ascii_literal)]

// mod assemble;
mod alu;
mod machine;
mod node;
mod opcodes;
mod parse;
mod stack;
mod stream;
mod token;

// Make enough public to easily run programs
// pub use crate::assemble::Assemble;
pub use crate::machine::ChangeEvent;
pub use crate::machine::Observer;
// pub use crate::parse::Parser;
pub use crate::machine::Machine;
pub use crate::machine::{Response, COUNTER, SP, STATUS};
pub use crate::node::{Node, Type as NodeType};
pub use crate::stream::Error;
pub use crate::stream::Input;
pub use crate::token::{Token, Type};
