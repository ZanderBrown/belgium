mod eval;
mod parse;
mod stream;

// Make enough public to easily run programs
pub use eval::Eval;
pub use eval::MainMemory;
pub use eval::Registers;
pub use eval::Storage;
pub use parse::Parser;
pub use stream::Input;
