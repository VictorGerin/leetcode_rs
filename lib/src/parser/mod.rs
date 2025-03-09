mod error;
mod parser;
mod value;
mod iterator;
mod input;

pub use error::ProcessInputError;
pub use parser::parser;
pub use parser::parser_str;
pub use value::Val;
pub use iterator::ValIter;
pub use input::read_input; 