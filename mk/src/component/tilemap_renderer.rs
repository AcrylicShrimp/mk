use crate::render::{Color, Layer, Shader, Tilemap};
use std::sync::Arc;

pub struct TilemapRenderer {
    pub layer: Layer,
    pub order: isize,
    pub color: Color,
    pub shader: Arc<Shader>,
    pub tilemap: Arc<Tilemap>,
}

impl TilemapRenderer {
    pub fn new(shader: Arc<Shader>, tilemap: Arc<Tilemap>) -> TilemapRenderer {
        TilemapRenderer {
            layer: Layer::default(),
            order: 0,
            color: Color::white(),
            shader,
            tilemap,
        }
    }
}
