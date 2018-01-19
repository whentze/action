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

//                                    /-> Sink1
//                                    /-> Sink2
//                    /-> Splitter2 --    ...
//                    /->             \-> Sink7
//                    /->             \-> Sink8
//                    /->
//Sine -> Splitter1 --    ...          ...
//                    \->
//                    \->             /-> Sink56
//                    \->             /-> Sink57
//                    \-> Splitter9 -- ...
//                                    \-> Sink63
//                                    \-> Sink64
#[bench]
fn throughput_splittree(b: &mut Bencher) {
    let mut graph = Graph::new();
    let sine = graph.insert_module(Sine::default());
    let first_splitter = graph.insert_module(Splitter::default());
    graph.connect((sine, 0), (first_splitter, 0)).unwrap();
    for i in 0..8 {
        let splitter = graph.insert_module(Splitter::default());
        graph.connect((first_splitter, i), (splitter, 0)).unwrap();
        for j in 0..8 {
            let sink = graph.insert_module(Sink::default());
            graph.connect((splitter, j), (sink, 0)).unwrap();
        }
    }

    b.iter(|| {
        for _ in 0..CHUNKS_PER_MS {
            graph.run();
        }
    });
}

// Sine -> Mixer00 -> ... -> Mixer90
//          v                 v
//         ...               ...
//          v                 v
//         Mixer09 -> ... -> Mixer99 -> Sink
#[bench]
fn throughput_grid(b: &mut Bencher) {
    let mut graph = Graph::new();

    let mut mixers = vec![Vec::new(); 10];
    for x in 0..10 {
        for _ in 0..10 {
            mixers[x].push(graph.insert_module(Mixer::default()));
        }
        for pair in mixers[x].windows(2) {
            graph.connect((pair[0], 0), (pair[1], 0)).unwrap();
        }
    }
    for pair in mixers.windows(2) {
        for y in 0..10 {
            graph.connect((pair[0][y], 0), (pair[1][y], 1)).unwrap();
        }
    }
    let sine = graph.insert_module(Sine::default());
    let sink = graph.insert_module(Sink::default());
    graph.connect((sine, 0), (mixers[0][0], 0)).unwrap();
    graph.connect((mixers[9][9], 0), (sink, 0)).unwrap();

    b.iter(|| {
        for _ in 0..CHUNKS_PER_MS {
            graph.run();
        }
    });
}
