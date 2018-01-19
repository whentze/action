#![feature(nll, test)]

extern crate byteorder;
extern crate failure;
extern crate fnv;

mod definitions;
pub mod graph;
pub mod module;

#[cfg(test)]
mod tests;