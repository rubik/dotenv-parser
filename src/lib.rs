//! The parser handles comments, strings and the `export` syntax automatically.
//!
//! # Example
//! ```
//! use dotenv_parser::parse_dotenv;
//!
//! fn main() {
//!     let source = r#"
//!         ## main comment
//!         ENV_FOR_HYDRO='testing 2' # another one here
//!         export USER_ID=5gpPN5rcv5G41U_S
//!         API_TOKEN=30af563ccc668bc8ced9e24e  # relax! these values are fake
//!         APP_SITE_URL=https://my.example.com
//!     "#;
//!
//!     let map = vec![
//!         ("ENV_FOR_HYDRO", "testing 2"),
//!         ("USER_ID", "5gpPN5rcv5G41U_S"),
//!         ("API_TOKEN", "30af563ccc668bc8ced9e24e"),
//!         ("APP_SITE_URL", "https://my.example.com"),
//!     ]
//!     .into_iter()
//!     .map(|(a, b)| (a.into(), b.into()))
//!     .collect();
//!
//!     let res = parse_dotenv(source).unwrap();
//!     assert_eq!(res, map);
//! }

#![deny(missing_docs)]

extern crate pest;
#[macro_use]
extern crate pest_derive;

mod parser;
pub use parser::parse_dotenv;
