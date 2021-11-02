use downcast_rs::{impl_downcast, Downcast};
use std::collections::HashMap;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Error as IOError;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use crate::read_file_all;

#[derive(Debug)]
pub enum ResourceLoadError {
    UnknownResourceType,
    IOError(IOError),
    DecoderError(DecoderError),
    BasePathNotFound(IOError),
    CannotOpenResourceFile(IOError),
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

pub struct ResourceLoader {
    base_path: PathBuf,
    decoders: HashMap<String, Box<dyn ResourceDecoder>>,
}

impl ResourceLoader {
    pub fn new(base_path: impl AsRef<Path>) -> Result<Self, ResourceLoadError> {
        let base_path = base_path
            .as_ref()
            .canonicalize()
            .map_err(|err| ResourceLoadError::BasePathNotFound(err))?;

        Ok(Self {
            base_path,
            decoders: HashMap::new(),
        })
    }

    pub fn add_decoder(&mut self, decoder: Box<dyn ResourceDecoder>) {
        self.decoders.insert(decoder.ty().to_owned(), decoder);
    }

    pub fn load(
        &self,
        ty: impl AsRef<str>,
        path: impl AsRef<Path>,
    ) -> Result<Arc<dyn BaseResource>, ResourceLoadError> {
        let decoder = self
            .decoders
            .get(ty.as_ref())
            .ok_or(ResourceLoadError::UnknownResourceType)?;

        let file = OpenOptions::new()
            .read(true)
            .open(self.base_path.join(path))
            .map_err(|err| ResourceLoadError::CannotOpenResourceFile(err))?;
        let meta = file
            .metadata()
            .map_err(|err| ResourceLoadError::CannotOpenResourceFile(err))?;
        let mut content = vec![0; meta.len() as usize];

        read_file_all(&file, 0, &mut content)?;

        Ok(decoder.decode(content)?)
    }
}

pub type DecoderError = Box<dyn Error>;

pub trait ResourceDecoder {
    fn ty(&self) -> &str;
    fn decode(&self, content: Vec<u8>) -> Result<Arc<dyn BaseResource>, DecoderError>;
}

pub trait BaseResource: Downcast {
    fn ty(&self) -> &str;
}

impl_downcast!(BaseResource);
