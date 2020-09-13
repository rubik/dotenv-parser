extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parser;
pub use parser::parse_dotenv;
