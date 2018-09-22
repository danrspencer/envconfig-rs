//! Envconfig is a Rust library that helps to initialize configuration structure
//! from environment variables.
//! It makes use of custom derive macros to reduce boilerplate.
//!
//! Example
//!
//! ```
//! #[macro_use]
//! extern crate envconfig_derive;
//! extern crate envconfig;
//!
//! use std::env;
//! use envconfig::Envconfig;
//!
//! #[derive(Envconfig)]
//! struct Config {
//!     #[envconfig(from = "DB_HOST")]
//!     pub db_host: String,
//!
//!     #[envconfig(from = "DB_PORT")]
//!     pub db_port: u16,
//! }
//!
//! fn main() {
//!     // We assume that those environment variables are set somewhere outside
//!     env::set_var("DB_HOST", "localhost");
//!     env::set_var("DB_PORT", "5432");
//!
//!     // Initialize config from environment variables
//!     let config = Config::init().unwrap();
//!
//!     assert_eq!(config.db_host, "localhost");
//!     assert_eq!(config.db_port, 5432);
//! }
//! ```
//!
//! The library uses `std::str::FromStr` trait to convert environment variables into custom
//! data type. So, if your data type does not implement `std::str::FromStr` the program
//! will not compile.

#[macro_use]
extern crate failure;

mod error;
mod traits;
mod utils;

pub use error::Error;
pub use traits::Envconfig;
pub use utils::{load_optional_var, load_var};
