#[cfg(feature = "reader")]
mod loader;

#[cfg(feature = "reader")]
pub use loader::*;

#[cfg(feature = "writer")]
mod writer;

#[cfg(feature = "writer")]
pub use writer::*;

use crate::ResourceUUID;

pub fn resource_uuid_to_filename(uuid: ResourceUUID) -> String {
    format!("res{}.res", uuid)
}
