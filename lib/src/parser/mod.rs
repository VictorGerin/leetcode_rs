mod error;
mod parser;
mod value;
mod iterator;
mod input;

pub use error::ProcessInputError;
pub use parser::parser;
pub use value::Value;
pub use iterator::ValueIterator;
pub use input::read_input; 