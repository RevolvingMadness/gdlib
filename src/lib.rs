#![doc = include_str!("../README.md")]
// #![recursion_limit = "256"]

pub mod core;
pub mod deserialiser;
pub mod gdlevel;
pub mod gdobj;
pub mod serialiser;

#[cfg(test)]
pub mod tests;
