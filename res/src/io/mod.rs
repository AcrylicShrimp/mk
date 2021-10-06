#[cfg(feature = "reader")]
mod loader;

#[cfg(feature = "reader")]
pub use loader::*;

#[cfg(feature = "writer")]
mod writer;

#[cfg(feature = "writer")]
pub use writer::*;

use crate::ResourceChunkID;

pub fn chunk_to_filename(chunk: ResourceChunkID) -> String {
    format!("assets{}.res", chunk)
}
