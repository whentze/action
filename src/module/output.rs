use crate::definitions::*;
use crate::module::*;
use portaudio as pa;
use std::fmt;

const BUFFER_SIZE: usize = 32;

pub struct PortAudioOut {
    buffer: [Chunk; BUFFER_SIZE],
    chunks_stored: usize,
    stream: pa::Stream<pa::Blocking<pa::stream::Buffer>, pa::Output<f32>>,
}

// No Debug impl on pa::Stream so we have to do this manually, grr
impl fmt::Debug for PortAudioOut {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("PortAudioOut")
            .field("buffer", &self.buffer)
            .field("chunks_stored", &self.chunks_stored)
            .field("stream", &(&self.stream as *const _))
            .finish()
    }
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
    fn process_chunks(&mut self, input: &[Chunk], _: &mut [Chunk]) {
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
