mod eval;
mod parse;
mod stream;

// Make enough public to easily run programs
pub use crate::eval::Eval;
pub use crate::eval::MainMemory;
pub use crate::eval::Registers;
pub use crate::eval::Storage;
pub use crate::parse::Parser;
pub use crate::stream::Input;
