pub trait Component {
    fn ty(&self) -> &'static str;
    fn animate(
        &mut self,
        _time_line: &crate::animation::AnimationTimeLine,
        _key_frame: &crate::animation::AnimationKeyFrame,
        _normalized_time_in_key_frame: f32,
    ) {
    }
}

use component_macros::*;

#[derive(Component)]
pub struct Struct {
    #[lua(get = "x", set = "x")]
    #[animate(field = "position.x", ty = "integer")]
    pub x: f32,
    #[lua(get = "y", set = "y")]
    #[animate(field = "position.y", ty = "float")]
    pub y: f32,
    #[lua(get = "z", set = "z")]
    #[animate(field = "position.z", ty = "float")]
    pub z: f32,
}

mod camera;
mod diagnostic;
mod glyph_renderer;
mod layer;
mod not_yet_complete;
mod single_animator;
mod sprite_renderer;
mod tilemap_renderer;
mod transform;

pub use camera::*;
pub use diagnostic::*;
pub use glyph_renderer::*;
pub use layer::*;
pub use not_yet_complete::*;
pub use single_animator::*;
pub use sprite_renderer::*;
pub use tilemap_renderer::*;
pub use transform::*;
