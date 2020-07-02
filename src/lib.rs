//! Library used to access the YouTrack API with Rust
#![allow(dead_code)]
#![deny(//missing_docs,
    unsafe_code,
    unused_import_braces,
    unused_qualifications)]

#[macro_use]
extern crate error_chain;

#[macro_use]
mod macros;
mod util;

pub mod client;
pub mod errors;
pub mod issues;
pub mod users;

pub use hyper::{HeaderMap, StatusCode};
