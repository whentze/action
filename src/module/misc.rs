// example modules for testing the graph implementation

use byteorder::{LittleEndian, WriteBytesExt};
use std::io::stdout;
use std::f32::consts::PI;
use definitions::*;
use module::{Input, Output, Module};

#[derive(Default, Debug, Clone)]
pub struct Sine {
    freq  : f32,
    phase : f32,
}

impl Sine {
    pub fn with_freq(freq: f32) -> Self {
        Sine { freq, phase: 0.0 }
    }
}

impl Module for Sine {
    fn num_inputs(&self) -> usize { 0 }

    fn num_outputs(&self) -> usize { 1 }

    fn process_samples(&mut self, _: &Input, output: &mut Output) {
        output[0] = (self.phase * 2.0 * PI).sin();
        self.phase = (self.phase + self.freq/SAMPLE_RATE) % 1.0;
    }
}

#[derive(Default, Debug, Clone)]
pub struct Id {}
impl Module for Id {
    fn num_inputs(&self)  -> usize { 1 }
    fn num_outputs(&self) -> usize { 1 }
    fn process_samples(&mut self, i: &Input, o: &mut Output)  {
        o[0] = i[0];
    }
}

#[derive(Default, Debug, Clone)]
pub struct Mixer {}
impl Module for Mixer {
    fn num_inputs(&self)  -> usize { 8 }
    fn num_outputs(&self) -> usize { 1 }
    fn process_samples(&mut self, i: &Input, o: &mut Output)  {
        o[0] = (0..8).map(|x| i[x]).sum()
    }
}

#[derive(Default, Debug, Clone)]
pub struct Splitter {}
impl Module for Splitter {
    fn num_inputs(&self)  -> usize { 1 }
    fn num_outputs(&self) -> usize { 8 }
    fn process_samples(&mut self, i: &Input, o: &mut Output) {
        for x in 0..8 {
            o[x] = i[0];
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Printer {}
impl Module for Printer {
    fn num_inputs(&self)  -> usize { 1 }
    fn num_outputs(&self) -> usize { 0 }
    fn process_samples(&mut self, i: &Input, _: &mut Output) {
        stdout().write_f32::<LittleEndian>(i[0]).unwrap();
    }
}

#[derive(Default, Debug, Clone)]
pub struct Sink {}
impl Module for Sink {
    fn num_inputs(&self)  -> usize { 1 }
    fn num_outputs(&self) -> usize { 0 }
    fn process_samples(&mut self, _: &Input, _: &mut Output) {}
}