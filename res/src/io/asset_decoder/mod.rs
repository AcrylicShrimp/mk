use crate::{BaseResource, DecoderError, ResourceDecoder};
use brotli::DecompressorWriter;
use std::io::Write;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BrotliDecoderOutput {
    pub content: Vec<u8>,
}

impl BaseResource for BrotliDecoderOutput {
    fn ty(&self) -> &str {
        return "text";
    }
}

pub struct BrotliDecoder;

impl ResourceDecoder for BrotliDecoder {
    fn ty(&self) -> &str {
        return "text";
    }

    fn decode(&self, content: Vec<u8>) -> Result<Arc<dyn BaseResource>, DecoderError> {
        let mut result = vec![];

        {
            let mut writer = DecompressorWriter::new(&mut result, 4096);
            writer.write_all(&content)?;
        }

        Ok(Arc::new(BrotliDecoderOutput { content: result }))
    }
}
