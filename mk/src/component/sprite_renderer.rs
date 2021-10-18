use crate::render::{Color, Layer, Shader, Sprite};
use crate::time::TimeManager;
use std::sync::Arc;
use std::time::*;

// TODO: Split the SpriteRenderer into dedicated animation system.
pub trait SpriteAnimation
where
    Self: Send + Sync,
{
    fn sprite(&self) -> &Arc<Sprite>;
    fn update(&mut self, time_manager: &TimeManager);
}

pub enum SpriteType {
    Sprite(Arc<Sprite>),
    Animated(Box<dyn SpriteAnimation>),
}

pub struct SpriteRenderer {
    pub layer: Layer,
    pub order: isize,
    pub color: Color,
    pub shader: Arc<Shader>,
    pub sprite: SpriteType,
}

impl SpriteRenderer {
    pub fn new(shader: Arc<Shader>, sprite: SpriteType) -> Self {
        Self {
            layer: Layer::default(),
            order: 0,
            sprite,
            shader,
            color: Color::white(),
        }
    }
}

pub struct SequentialSpriteAnimation {
    time: Instant,
    index: usize,
    sprites: Vec<(Arc<Sprite>, f32)>,
}

impl SequentialSpriteAnimation {
    pub fn new(sprites: Vec<(Arc<Sprite>, f32)>) -> Self {
        Self {
            time: Instant::now(),
            index: 0,
            sprites,
        }
    }
}

impl SpriteAnimation for SequentialSpriteAnimation {
    fn sprite(&self) -> &Arc<Sprite> {
        &self.sprites[self.index].0
    }

    fn update(&mut self, time_manager: &TimeManager) {
        while self.time + Duration::from_secs_f32(self.sprites[self.index].1) <= time_manager.time()
        {
            self.time += Duration::from_secs_f32(self.sprites[self.index].1);
            self.index = (self.index + 1) % self.sprites.len();
        }
    }
}
