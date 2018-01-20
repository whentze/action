// example modules for testing the graph implementation

use std::f32::consts::PI;
use definitions::*;
use module::{Input, Module, Output};

#[derive(Default, Debug, Clone)]
pub struct Sine {
    freq: f32,
    phase: f32,
}

impl Sine {
    pub fn with_freq(freq: f32) -> Self {
        Sine { freq, phase: 0.0 }
    }
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
