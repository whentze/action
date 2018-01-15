pub use failure::{Error, err_msg};

pub const SAMPLE_RATE : f64 = 96_000.0;
pub const CHUNK_SIZE : usize = 2;
pub type Sample = f32;
pub type Chunk = [Sample; CHUNK_SIZE];
pub type PortSpec = (usize, usize);
pub type Result<T> = ::std::result::Result<T, Error>;