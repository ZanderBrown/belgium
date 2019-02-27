mod eval;
mod parse;
mod storage;
mod stream;
mod cpu;
mod assemble;

// Make enough public to easily run programs
pub use crate::eval::Eval;
pub use crate::stream::Error;
pub use crate::eval::Storage;
pub use crate::parse::Parser;
pub use crate::storage::ChangeEvent;
pub use crate::storage::Memory;
pub use crate::storage::Observer;
pub use crate::stream::Input;
pub use crate::assemble::Assemble;