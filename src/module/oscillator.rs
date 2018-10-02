use definitions::*;
use module::{Input, Module, Output};
use std::f32::consts::PI;

#[derive(Default, Debug, Clone)]
pub struct Oscillator {
    freq: f32,
    phase: f32,
    amp: f32,
}

impl Oscillator {
    pub fn freq(self, freq: f32) -> Self {
        Self { freq, ..self }
    }
    pub fn phase(self, phase: f32) -> Self {
        Self { phase, ..self }
    }
    pub fn amp(self, amp: f32) -> Self {
        Self { amp, ..self }
    }
}

impl Module for Oscillator {
    fn num_inputs(&self) -> usize {
        2
    }

    fn num_outputs(&self) -> usize {
        1
    }

    fn process_samples(&mut self, input: &Input, output: &mut Output) {
        let amp = self.amp * (input[0] + 1.0);
        let freq = self.freq * input[1].exp2();
        output[0] = amp * (self.phase * 2.0 * PI).sin();
        self.phase = (self.phase + freq / SAMPLE_RATE) % 1.0;
    }
}
