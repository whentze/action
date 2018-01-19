#![feature(nll, test)]

extern crate byteorder;
extern crate failure;
extern crate fnv;
extern crate rayon;
extern crate rayon_hash;

mod definitions;
pub mod graph;
pub mod module;

#[cfg(test)]
mod tests;