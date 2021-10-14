use aes::cipher::generic_array::GenericArray;
use aes::cipher::{FromBlockCipher, NewBlockCipher, StreamCipher};
use aes::{Aes256, Aes256Ctr};
use byteorder::{ByteOrder, LittleEndian};
use crc32fast::Hasher as Crc32Hasher;
use memmap2::{Mmap, MmapOptions};
use rayon::prelude::*;
use sha256::digest_bytes as sha256_digest_bytes;
use std::cmp::min;
use std::collections::HashMap;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Error as IOError;
use std::mem::size_of;
use std::path::Path;

use crate::{
    chunk_to_filename, Resource, ResourceChunk, ResourceHash, ResourceHashAlgorithm, ResourceMeta,
    ResourceUUID, ResourcesMeta,
};

#[derive(Debug)]
pub enum ResourceWriteError {
    UnknownResourceType,
    IOError(IOError),
    EncoderError(EncoderError),
}

impl From<IOError> for ResourceWriteError {
    fn from(err: IOError) -> Self {
        Self::IOError(err)
    }
}

impl From<EncoderError> for ResourceWriteError {
    fn from(err: EncoderError) -> Self {
        Self::EncoderError(err.into())
    }
}

#[derive(Default)]
pub struct ResourceWriter {
    encoders: HashMap<String, Box<dyn ResourceEncoder>>,
}

impl ResourceWriter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_encoder(&mut self, ty: String, encoder: Box<dyn ResourceEncoder>) {
        self.encoders.insert(ty, encoder);
    }

    pub fn write(
        &self,
        chunk_size: Option<u64>,
        base_path: impl AsRef<Path>,
        res: &[(
            impl AsRef<str> + Sync + Send,
            impl AsRef<Path> + Sync + Send,
        )],
    ) -> Result<ResourcesMeta, ResourceWriteError> {
        let resources = res
            .par_iter()
            .enumerate()
            .map(|(index, (ty, path))| {
                let encoder = self
                    .encoders
                    .get(ty.as_ref())
                    .ok_or(ResourceWriteError::UnknownResourceType)?;

                let file = OpenOptions::new().read(true).open(path.as_ref())?;
                let content = unsafe { Mmap::map(&file)? };

                let uuid = unsafe { ResourceUUID::new_unchecked((index + 1) as u64) };
                let hash = ResourceHash {
                    hash: {
                        let mut hasher = Crc32Hasher::new();
                        hasher.update(&content);

                        let buffer = &mut [0u8; size_of::<u32>()];
                        LittleEndian::write_u32(buffer, hasher.finalize());

                        sha256_digest_bytes(buffer)
                    },
                    algorithm: ResourceHashAlgorithm::CRC32LESHA256,
                };
                let encoded = encoder.encode(uuid, content)?;

                Ok((uuid, hash, encoded))
            })
            .collect::<Result<Vec<_>, ResourceWriteError>>()?;

        let key = GenericArray::from_slice(&[0u8; 32]);
        let nonce = GenericArray::from_slice(&[0u8; 16]);
        let mut cipher = Aes256Ctr::from_block_cipher(Aes256::new(key), nonce);

        let mut chunk = 0;
        let mut chunk_offset = 0;

        let total_size = {
            let mut size = 0;

            for (_, _, encoded) in resources.iter() {
                size += encoded.content.len() as u64;
            }

            size
        };
        let chunk_size = chunk_size.unwrap_or(total_size);

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(base_path.as_ref().join(chunk_to_filename(chunk)))?;
        file.set_len(chunk_size)?;
        let mut chunk_content = unsafe { MmapOptions::new().map_mut(&file) }?;

        let resources = resources
            .into_iter()
            .enumerate()
            .map(|(index, (uuid, hash, encoded))| {
                let mut chunks = vec![];
                let mut content_offset = 0;

                while content_offset < encoded.content.len() {
                    if chunk_offset == chunk_size {
                        chunk += 1;
                        chunk_offset = 0;
                        file = OpenOptions::new()
                            .read(true)
                            .write(true)
                            .create(true)
                            .truncate(true)
                            .open(base_path.as_ref().join(chunk_to_filename(chunk)))?;
                        file.set_len(min(total_size - chunk as u64 * chunk_size, chunk_size))?;
                        chunk_content = unsafe { MmapOptions::new().map_mut(&file) }?;
                    }

                    let len = min(
                        encoded.content.len() - content_offset,
                        (chunk_size - chunk_offset) as usize,
                    );
                    let range = &mut chunk_content[{
                        let chunk_offset = chunk_offset as usize;
                        chunk_offset..chunk_offset + len
                    }];

                    range.copy_from_slice(&encoded.content[..len]);
                    cipher.apply_keystream(range);

                    chunks.push(ResourceChunk {
                        id: chunk,
                        offset: chunk_offset,
                        size: len as _,
                    });
                    chunk_offset += len as u64;
                    content_offset += len;
                }

                Ok(Resource {
                    uuid,
                    ty: res[index].0.as_ref().to_owned(),
                    hash,
                    chunks,
                    meta: Some(encoded.meta),
                })
            })
            .collect::<Result<Vec<_>, ResourceWriteError>>()?;

        Ok(ResourcesMeta {
            version: 1,
            resources,
        })
    }
}

pub type EncoderError = Box<dyn Error + Send + Sync>;

pub trait ResourceEncoder: Send + Sync {
    fn encode(&self, uuid: ResourceUUID, src: Mmap) -> Result<EncodedResource, EncoderError>;
}

pub struct EncodedResource {
    pub meta: ResourceMeta,
    pub content: Mmap,
}
