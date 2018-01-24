//! Modules with functionality related to testing.
//! Most of these are useless for actual audio.

use byteorder::{LittleEndian, WriteBytesExt};
use std::f32::consts::PI;
use std::io::stdout;
use module::{Input, Module, Output};
use definitions::SAMPLE_RATE;

/// A very simple sine wave generator
#[derive(Default, Debug, Clone)]
pub struct Sine {
    freq: f32,
    phase: f32,
}

impl Module for Sine {
    fn num_inputs(&self) -> usize {
        0
    }

    fn num_outputs(&self) -> usize {
        1
    }

    fn process_samples(&mut self, _: &Input, output: &mut Output) {
        output[0] = (self.phase * 2.0 * PI).sin();
        self.phase = (self.phase + self.freq / SAMPLE_RATE) % 1.0;
    }
}

/// An identity module.
/// It implements the function |x| x.
#[derive(Default, Debug, Clone)]
pub struct Id {}
impl Module for Id {
    fn num_inputs(&self) -> usize {
        1
    }
    fn num_outputs(&self) -> usize {
        1
    }
    fn process_samples(&mut self, i: &Input, o: &mut Output) {
        o[0] = i[0];
    }
}

/// A module that takes 8 inputs and outputs their sum.
#[derive(Default, Debug, Clone)]
pub struct Mixer {}
impl Module for Mixer {
    fn num_inputs(&self) -> usize {
        8
    }
    fn num_outputs(&self) -> usize {
        1
    }
    fn process_samples(&mut self, i: &Input, o: &mut Output) {
        o[0] = (0..8).map(|x| i[x]).sum()
    }
}

/// A module that takes 1 input and outputs it 8 times.
#[derive(Default, Debug, Clone)]
pub struct Splitter {}
impl Module for Splitter {
    fn num_inputs(&self) -> usize {
        1
    }
    fn num_outputs(&self) -> usize {
        8
    }
    fn process_samples(&mut self, i: &Input, o: &mut Output) {
        for x in 0..8 {
            o[x] = i[0];
        }
    }
}

/// A module that takes 1 input and outputs it on stdout as raw, little endian 32-bit floats.
#[derive(Default, Debug, Clone)]
pub struct Printer {}
impl Module for Printer {
    fn num_inputs(&self) -> usize {
        1
    }
    fn num_outputs(&self) -> usize {
        0
    }
    fn process_samples(&mut self, i: &Input, _: &mut Output) {
        stdout().write_f32::<LittleEndian>(i[0]).unwrap();
    }
}

/// A module that takes 1 input and does nothing with it.
#[derive(Default, Debug, Clone)]
pub struct Sink {}
impl Module for Sink {
    fn num_inputs(&self) -> usize {
        1
    }
    fn num_outputs(&self) -> usize {
        0
    }
    fn process_samples(&mut self, _: &Input, _: &mut Output) {}
}
