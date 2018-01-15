mod misc;
pub use self::misc::*;

use graph::{Input, Output};

pub trait Module {
    fn num_inputs(&self) -> usize;
    fn num_outputs(&self) -> usize;
    fn run(&mut self, &[Input], &[Output]);
}