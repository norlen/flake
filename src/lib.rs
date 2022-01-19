//! Flake
//!
//! A tiny library to generate snowflakes.
//!
//! Each snowflake consists of a timestamp, datacenter id, machine id, and a sequence number.
//! With the bit pattern
//!
//! | Timestamp | Datacenter ID | Machine ID | Sequence |
//! | --------- | ------------- | ---------- | -------- |
//! | 42 bits   | 5 bits        | 5 bits     | 12 bits  |
//!
//! The timestamp can be from a custom epoch.
//!
//!
//! ## Example
//!
//! ```rust
//! use flake::IdGenerator;
//! use std::time::UNIX_EPOCH;
//!
//! let mut generator = IdGenerator::new(UNIX_EPOCH, 1, 1).unwrap();
//! let snowflake = generator.id();
//! ```
//!
#![warn(missing_docs)]

mod generator;
mod id;

pub use generator::IdGenerator;
pub use id::Flake;
