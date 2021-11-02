#[cfg(feature = "asset_loader")]
pub mod asset_decoder;
#[cfg(feature = "asset_loader")]
pub mod asset_loader;

#[cfg(feature = "fs_loader")]
pub mod fs_decoder;
#[cfg(feature = "fs_loader")]
pub mod fs_loader;

#[cfg(feature = "meta_loader")]
pub mod meta_loader;

#[cfg(feature = "writer")]
pub mod encoder;
#[cfg(feature = "writer")]
pub mod writer;

use crate::ResourceChunkID;
use std::fs::File;
use std::io::Error as IOError;

pub(crate) fn chunk_to_filename(chunk: ResourceChunkID) -> String {
    format!("assets{}.res", chunk)
}

pub(crate) fn read_file_all(file: &File, offset: u64, buffer: &mut [u8]) -> Result<(), IOError> {
    #[cfg(target_family = "unix")]
    {
        use std::os::unix::prelude::FileExt;
        file.read_exact_at(buffer, offset)?;
    }
    #[cfg(target_family = "windows")]
    {
        use std::io::ErrorKind;
        use std::os::windows::prelude::FileExt;
        if file.seek_read(buffer, offset)? != buffer.len() {
            return Err(IOError::from(ErrorKind::UnexpectedEof));
        }
    }
    #[cfg(target_family = "wasm")]
    {
        use std::os::unix::prelude::FileExt;
        file.read_exact_at(buffer, offset)?;
    }

    Ok(())
}
