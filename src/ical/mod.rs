//! This module handles conversion between iCal files and internal representations
//!
//! It is a wrapper around different Rust third-party libraries, since I haven't find any complete library that is able to parse _and_ generate iCal files

mod parser;
pub use parser::parse;
mod builder;
pub use builder::build_from;