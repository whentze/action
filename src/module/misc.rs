// modules for testing the graph implementation

use byteorder::{LittleEndian, WriteBytesExt};
use std::io::stdout;
use std::f64::consts::PI;

use definitions::*;
use graph::{Input, Output};
use module::Module;

#[derive(Default)]
pub struct Sine {
    phase: f64,
}
impl Module for Sine {
    fn num_inputs(&self)  -> usize { 0 }
    fn num_outputs(&self) -> usize { 1 }
    fn run(&mut self, _: &[Input], o: &[Output]) {
        let mut buf = Chunk::default();
        for i in 0..CHUNK_SIZE {
            buf[i] = (2.0*PI*self.phase).sin() as Sample;
            self.phase += 110.0/SAMPLE_RATE;
        }
        o[0].put(buf);
    }
}

#[derive(Default)]
pub struct Id {}
impl Module for Id {
    fn num_inputs(&self)  -> usize { 1 }
    fn num_outputs(&self) -> usize { 1 }
    fn run(&mut self, i: &[Input], o: &[Output]) {
        o[0].put(i[0].get())
    }
}

#[derive(Default)]
pub struct Adder {
    buf: Chunk,
}
impl Module for Adder {
    fn num_inputs(&self)  -> usize { 8 }
    fn num_outputs(&self) -> usize { 1 }
    fn run(&mut self, i: &[Input], o: &[Output]) {
        o[0].put(i.iter().map(|x| x.get()).fold(Chunk::default(), |l, r| {
            let mut buf = Chunk::default();
            for i in 0..CHUNK_SIZE {
                buf[i] = l[i] + r[i];
            }
            buf
        }));
    }
}

#[derive(Default)]
pub struct Splitter {}
impl Module for Splitter {
    fn num_inputs(&self)  -> usize { 1 }
    fn num_outputs(&self) -> usize { 8 }
    fn run(&mut self, i: &[Input], o: &[Output]) {
        for x in o.iter() {
            x.put(i[0].get())
        }
    }
}

#[derive(Default)]
pub struct Printer {}
impl Module for Printer {
    fn num_inputs(&self)  -> usize { 1 }
    fn num_outputs(&self) -> usize { 0 }
    fn run(&mut self, i: &[Input], _: &[Output]) {
        for f in i[0].get().iter() {
            stdout().write_f32::<LittleEndian>(*f).unwrap();
        }
    }
}