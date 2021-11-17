use crate::Buffer;
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub struct BufferId(pub u32);

pub trait GraphicsManager {
    fn create_buffer(&self) -> BufferId;
    fn delete_buffer(&self, buffer: BufferId);
    fn query_buffer(&self, buffer: BufferId);
}
