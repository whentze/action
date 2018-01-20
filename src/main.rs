extern crate action;

use action::module::*;
use action::graph::*;

fn main() {
    let mut graph = Graph::new();
    let sine = graph.insert_module(Sine::with_freq(220.0));
    let output = graph.insert_module(PortAudioOut::default());

    graph.connect((sine, 0), (output, 0)).unwrap();

    loop {
        graph.run()
    }
}
