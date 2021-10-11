use byteorder::{ByteOrder, LittleEndian};
use crc32fast::Hasher as Crc32Hasher;
use memmap2::Mmap;
use rayon::prelude::*;
use sha256::digest_bytes as sha256_digest_bytes;
use std::cmp::min;
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::fs::OpenOptions;
use std::io::{Error as IOError, Write};
use std::iter::FromIterator;
use std::mem::size_of;
use std::path::Path;

use crate::io::read_file_all;
use crate::{
    chunk_to_filename, Resource, ResourceChunk, ResourceHash, ResourceHashAlgorithm, ResourceMeta,
    ResourceUUID, ResourcesMeta,
};

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
            .map(|(ty, path)| {
                let encoder = self
                    .encoders
                    .get(ty.as_ref())
                    .ok_or(ResourceWriteError::UnknownResourceType)?;

                let file = OpenOptions::new().read(true).open(path.as_ref())?;
                let content = unsafe { Mmap::map(&file)? };

                let hash = ResourceHash {
                    hash: {
                        let mut hasher = Crc32Hasher::new();
                        hasher.update(&content);

                        let buffer = &mut [0u8; size_of::<u32>()];
                        LittleEndian::write_u32(buffer, hasher.finalize());

                        sha256_digest_bytes(buffer)
                    },
                    algorithm: ResourceHashAlgorithm::CRC32SHA256,
                };
                let encoded = encoder.encode(&content)?;

                Ok((hash, encoded))
            })
            .collect::<Result<Vec<_>, ResourceWriteError>>()?;

        let mut chunk = 0;
        let mut chunk_offset = 0;
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(base_path.as_ref().join(chunk_to_filename(chunk)))?;
        const CHUNK_SIZE: usize = 1024 * 1024 * 512;

        let resources = resources
            .into_iter()
            .enumerate()
            .map(|(index, (meta, content, hash))| {
                let mut chunks = vec![];
                let mut content_offset = 0;

                while content_offset < content.len() {
                    let len = min(content.len() - content_offset, CHUNK_SIZE - chunk_offset);
                    file.write_all(&content[content_offset..content_offset + len])?;
                    chunks.push(ResourceChunk {
                        id: chunk,
                        offset: chunk_offset as _,
                        size: len as _,
                    });
                    chunk_offset += len;
                    content_offset += len;

                    if chunk_offset == CHUNK_SIZE {
                        chunk += 1;
                        chunk_offset = 0;
                        file = OpenOptions::new()
                            .write(true)
                            .create(true)
                            .truncate(true)
                            .open(base_path.as_ref().join(chunk_to_filename(chunk)))?;
                    }
                }

                Ok(Resource {
                    parent: None,
                    uuid: unsafe { ResourceUUID::new_unchecked((resources.len() + 1) as u64) },
                    ty: res[index].0.as_ref().to_owned(),
                    hash,
                    chunks,
                    meta: Some(meta),
                    deps: vec![],
                    subs: vec![],
                })
            })
            .collect::<Result<Vec<_>, ResourceWriteError>>()?;
        let mut resource_uuids = BTreeMap::from_iter(
            resources
                .iter()
                .enumerate()
                .map(|(index, resource)| (resource.uuid, index as _)),
        );

        Ok(ResourcesMeta {
            version: 1,
            resources,
            resource_uuids,
        })
    }
}

pub type EncoderError = Box<dyn Error + Send + Sync>;

pub trait ResourceEncoder: Send + Sync {
    fn encode(&self, src: &[u8]) -> Result<EncodedResource, EncoderError>;
}

pub struct EncodedResource {
    pub meta: ResourceMeta,
    pub content: Mmap,
    pub subs: Vec<EncodedSubResource>,
}

pub struct EncodedSubResource {
    pub ty: String,
    pub meta: ResourceMeta,
}
