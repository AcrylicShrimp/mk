use crate::{resource_uuid_to_filename, Resource, ResourcesMeta};
use bincode::{options, Error as BincodeError, Options};
use downcast_rs::{impl_downcast, Downcast};
use std::collections::HashMap;
use std::error::Error;
use std::io::Error as IOError;
use std::path::Path;
use std::sync::Arc;

pub enum MetaLoadError {
    BincodeError(BincodeError),
    UnsupportedVersion,
}

impl From<BincodeError> for MetaLoadError {
    fn from(err: BincodeError) -> Self {
        Self::BincodeError(err)
    }
}

pub fn load_resource_meta(meta: &[u8]) -> Result<ResourcesMeta, MetaLoadError> {
    let meta: ResourcesMeta = options()
        .with_no_limit()
        .with_little_endian()
        .with_varint_encoding()
        .reject_trailing_bytes()
        .deserialize(meta)?;

    if meta.version != 1 {
        return Err(MetaLoadError::UnsupportedVersion);
    }

    Ok(meta)
}

pub enum ResourceLoadError {
    UnknownResourceType,
    IOError(IOError),
    DecoderError(DecoderError),
}

impl From<IOError> for ResourceLoadError {
    fn from(err: IOError) -> Self {
        Self::IOError(err)
    }
}

impl From<DecoderError> for ResourceLoadError {
    fn from(err: DecoderError) -> Self {
        Self::DecoderError(err)
    }
}

#[derive(Default)]
pub struct ResourceLoader {
    decoders: HashMap<String, Box<dyn ResourceDecoder>>,
}

impl ResourceLoader {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_decoder(&mut self, ty: String, decoder: Box<dyn ResourceDecoder>) {
        self.decoders.insert(ty, decoder);
    }

    pub async fn load(
        &self,
        base_path: impl AsRef<Path>,
        res: &Resource,
    ) -> Result<Arc<dyn BaseResource>, ResourceLoadError> {
        let decoder = self
            .decoders
            .get(&res.ty)
            .ok_or_else(|| ResourceLoadError::UnknownResourceType)?;

        let path = base_path.as_ref().join(resource_uuid_to_filename(res.uuid));
        let content = read(&path)?;

        Ok(decoder.decode(&content)?)
    }
}

pub type DecoderError = Box<dyn Error>;

pub trait ResourceDecoder {
    fn decode(&self, content: &[u8]) -> Result<Arc<dyn BaseResource>, DecoderError>;
}

pub trait BaseResource: Downcast {
    fn ty(&self) -> &'static str;
}

impl_downcast!(BaseResource);
