extern crate action;

use action::module::*;
use action::graph::*;

fn main() {
    let mut graph = Graph::new();
    let lfo    = graph.insert_module(Oscillator::default().freq(5.0).amp(0.5));
    let voice  = graph.insert_module(Oscillator::default().freq(110.0).amp(1.0));
    let output = graph.insert_module(PortAudioOut::default());

    graph.connect((lfo, 0),   (voice,  0)).unwrap();
    graph.connect((voice, 0), (output, 0)).unwrap();

    loop {
        graph.run()
    }
}
