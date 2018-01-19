extern crate test;
use self::test::Bencher;

use module::*;
use graph::*;
use definitions::*;

// The "throughput" benchmarks all construct some module graph and then
// measure its throughput when run at full speed.
// Every benchmark produces 1ms worth of samples per iteration.
// Therefore, the aim is to keep every result well below 1,000,000 ns/iter.

// How many samples do we have to produce per millisecond?
const SAMPLES_PER_MS: usize = (SAMPLE_RATE as usize)/1000;
// How many chunks do we have to produce per millisecond?
const CHUNKS_PER_MS:  usize = SAMPLES_PER_MS/CHUNK_SIZE;

// Sine -> Sink
#[bench]
fn throughput_simple(b: &mut Bencher) {
    let mut graph = Graph::new();
    let sine = graph.insert_module(Sine::default());
    let sink = graph.insert_module(Sink::default());
    graph.connect((sine, 0), (sink, 0)).unwrap();
    b.iter(|| {
        for _ in 0..CHUNKS_PER_MS {
            graph.run();
        }
    });
}

// Sine -> ID1 -> ... -> Id1000 -> Sink
#[bench]
fn throughput_long_chain(b: &mut Bencher) {
    let mut graph = Graph::new();
    let sine = graph.insert_module(Sine::default());
    let sink = graph.insert_module(Sink::default());
    let mut next = sine;
    for _ in 0..1000 {
        let last = next;
        next = graph.insert_module(Id::default());
        graph.connect((last, 0), (next, 0)).unwrap();
    }
    graph.connect((next, 0), (sink, 0)).unwrap();
    b.iter(|| {
        for _ in 0..CHUNKS_PER_MS {
            graph.run();
        }
    });
}

// Sine1  -\
// Sine2  -\
// ...      -> Mixer1 -\
// Sine7  -/          -\
// Sine8  -/          -\
//                    -\
// ...          ...     -> Mixer9 -> Sink
//                    -/
// Sine57 -\          -/
// Sine58 -\          -/
// ...      -> Mixer8 -/
// Sine63 -/
// Sine64 -/
#[bench]
fn throughput_mixtree(b: &mut Bencher) {
    let mut graph = Graph::new();
    let final_mixer = graph.insert_module(Mixer::default());
    for i in 0..8 {
        let mixer = graph.insert_module(Mixer::default());
        graph.connect((mixer, 0), (final_mixer, i)).unwrap();
        for j in 0..8 {
            let sine = graph.insert_module(Sine::default());
            graph.connect((sine, 0), (mixer, j)).unwrap();
        }
    }
    let sink = graph.insert_module(Sink::default());
    graph.connect((final_mixer, 0), (sink, 0)).unwrap();
    
    b.iter(|| {
        for _ in 0..CHUNKS_PER_MS {
            graph.run();
        }
    });
}