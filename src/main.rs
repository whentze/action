extern crate action;

use action::module::*;
use action::graph::*;

fn main() {
    let mut graph = Graph::new();
    let sine    = graph.insert_module(Sine::default());
    let printer = graph.insert_module(PortAudioOut::default());

    graph.connect((sine, 0), (printer, 0)).unwrap();

    loop { graph.run() };
}