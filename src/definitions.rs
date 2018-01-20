pub use failure::{err_msg, Error};

pub const SAMPLE_RATE: f32 = 96_000.0;
pub type Sample = f32;
pub const CHUNK_SIZE: usize = 30;
pub type Chunk = [Sample; CHUNK_SIZE];

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ModuleId(pub usize);
pub type Port = usize;
pub type PortAddr = (ModuleId, Port);
pub type Result<T> = ::std::result::Result<T, Error>;
