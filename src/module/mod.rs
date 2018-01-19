mod misc;
pub use self::misc::*;
mod output;
pub use self::output::*;

use definitions::*;
use std::ops::{Index, IndexMut};

pub struct Input<'a> {
    chunks : &'a Vec<Chunk>,
    offset: usize,
}

impl<'a> Index<usize> for Input<'a> {
    type Output = Sample;
    fn index(&self, i: usize) -> &Sample {
        &self.chunks[i][self.offset]
    }
}

pub struct Output<'a> {
    chunks : &'a mut Vec<Chunk>,
    offset: usize,
}

impl<'a> Index<usize> for Output<'a> {
    type Output = Sample;
    fn index(&self, i: usize) -> &Sample {
        &self.chunks[i][self.offset]
    }
}
impl<'a> IndexMut<usize> for Output<'a> {
    fn index_mut(&mut self, i: usize) -> &mut Sample{
        &mut self.chunks[i][self.offset]
    }
}

pub trait Module : Duplicate {
    fn num_inputs(&self) -> usize;
    fn num_outputs(&self) -> usize;
    fn process_samples(&mut self, input: &Input, output: &mut Output);

    fn process_chunks(&mut self, input: &Vec<Chunk>, output: &mut Vec<Chunk>) {
        for i in 0..CHUNK_SIZE {
            self.process_samples(&Input {
                chunks: input,
                offset: i,
            }, &mut Output {
                chunks: output,
                offset: i,
            });
        }
    }
}

pub trait Duplicate {
    fn duplicate(&self) -> Box<Module>;
}

impl<T : Clone + Module + 'static> Duplicate for T {
    fn duplicate(&self) -> Box<Module> {
        Box::new(self.clone())
    }
}