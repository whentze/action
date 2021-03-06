#![feature(nll, test)]

extern crate byteorder;
extern crate failure;
extern crate petgraph;
extern crate portaudio;

mod definitions;
pub mod graph;
pub mod module;

#[cfg(test)]
mod tests;
