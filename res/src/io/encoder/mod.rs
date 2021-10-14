use crate::{
    EncodedResource, EncoderError, ResourceEncoder, ResourceMeta, ResourceMetaValue, ResourceUUID,
};
use brotli::CompressorWriter;
use memmap2::Mmap;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BrotliEncoder;

impl ResourceEncoder for BrotliEncoder {
    fn encode(&self, uuid: ResourceUUID, src: Mmap) -> Result<EncodedResource, EncoderError> {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(Path::new(".tmp").join(format!("{}.tmp", uuid)))?;

        {
            let mut writer = CompressorWriter::new(&mut file, 4096, 11, 22);
            writer.write_all(&src)?;
        }

        let content = unsafe { Mmap::map(&file) }?;
        let mut meta = ResourceMeta::new();
        meta.insert("compression".to_owned(), ResourceMetaValue::Boolean(true));
        meta.insert(
            "algorithm".to_owned(),
            ResourceMetaValue::String("brotli".to_owned()),
        );
        meta.insert("quality".to_owned(), ResourceMetaValue::Integer(11));
        meta.insert("lg_window_size".to_owned(), ResourceMetaValue::Integer(22));
        meta.insert(
            "size_before".to_owned(),
            ResourceMetaValue::Integer(src.len() as _),
        );
        meta.insert(
            "size_after".to_owned(),
            ResourceMetaValue::Integer(content.len() as _),
        );

        Ok(EncodedResource { meta, content })
    }
}
