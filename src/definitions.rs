pub use failure::{err_msg, Error};

pub const SAMPLE_RATE: f32 = 96_000.0;
pub type Sample = f32;
pub const CHUNK_SIZE: usize = 30;
pub type Chunk = [Sample; CHUNK_SIZE];

pub type Result<T> = ::std::result::Result<T, Error>;
