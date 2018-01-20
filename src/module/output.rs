use portaudio as pa;
use definitions::*;
use module::*;

const BUFFER_SIZE: usize = 32;

pub struct PortAudioOut {
    buffer: [Chunk; BUFFER_SIZE],
    chunks_stored: usize,
    stream: pa::Stream<pa::Blocking<pa::stream::Buffer>, pa::Output<f32>>,
}

impl Duplicate for PortAudioOut {
    fn duplicate(&self) -> Box<Module> {
        Box::new(Self::default())
    }
}

impl Default for PortAudioOut {
    fn default() -> Self {
        let pa = pa::PortAudio::new().unwrap();
        let def_output = pa.default_output_device().unwrap();

        let output_params = pa::StreamParameters::<f32>::new(def_output, 1, true, 0.005);
        let settings = pa::stream::OutputSettings::new(
            output_params,
            SAMPLE_RATE as f64,
            (BUFFER_SIZE * CHUNK_SIZE) as u32,
        );

        let mut stream = pa.open_blocking_stream(settings).unwrap();
        stream.start().unwrap();
        PortAudioOut {
            buffer: <[Chunk; BUFFER_SIZE]>::default(),
            chunks_stored: 0,
            stream,
        }
    }
}

impl Module for PortAudioOut {
    fn num_inputs(&self) -> usize {
        1
    }
    fn num_outputs(&self) -> usize {
        0
    }

    fn process_samples(&mut self, _: &Input, _: &mut Output) {}
    fn process_chunks(&mut self, input: &Vec<Chunk>, _: &mut Vec<Chunk>) {
        self.buffer[self.chunks_stored] = input[0];
        self.chunks_stored += 1;
        if self.chunks_stored == BUFFER_SIZE {
            self.chunks_stored = 0;
            let buf = self.buffer;
            self.stream
                .write((CHUNK_SIZE * BUFFER_SIZE) as u32, |output| {
                    for i in 0..BUFFER_SIZE {
                        for j in 0..CHUNK_SIZE {
                            output[i * CHUNK_SIZE + j] = buf[i][j];
                        }
                    }
                })
                .unwrap();
        }
    }
}
