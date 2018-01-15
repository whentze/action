extern crate action;

use action::module::*;
use action::graph::*;

fn main() {
    let mut graph = Graph::new();
    let sine    = graph.add_module(Sine::default());
    let printer = graph.add_module(Printer::default());

    let mut last : usize;
    let mut next = sine;
    for _ in 0..1000 {
        last = next;
        next = graph.add_module(Id::default());
        let mid = graph.add_module(Id::default());
        graph.connect((last, 0), (mid, 0)).unwrap();
        graph.connect((mid, 0), (next, 0)).unwrap();
    }
    graph.connect((next, 0), (printer, 0)).unwrap();

    loop { graph.run() };
}