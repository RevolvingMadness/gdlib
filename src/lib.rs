#![doc = include_str!("../README.md")]
#![deny(missing_docs)]

pub mod core;
pub mod deserialiser;
pub mod gdlevel;
pub mod gdobj;
pub mod serialiser;

#[cfg(test)]
pub mod tests;
