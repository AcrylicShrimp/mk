mod interpolatable;

pub use interpolatable::*;

#[derive(Debug, Clone)]
pub struct Animation {
    pub time_lines: Vec<AnimationTimeLine>,
    pub duration: f32,
    pub looping: bool,
    pub pingpong: bool,
}

#[derive(Debug, Clone)]
pub struct AnimationTimeLine {
    pub key_frames: Vec<AnimationKeyFrame>,
    pub transform: Option<Vec<String>>,
    pub component: String,
    pub field: String,
}

#[derive(Debug, Clone)]
pub struct AnimationKeyFrame {
    pub begin: f32,
    pub end: f32,
    pub from: AnimationValue,
    pub to: AnimationValue,
    pub easing: AnimationEasing,
}

#[derive(Debug, Clone)]
pub enum AnimationValue {
    Bool(bool),
    Integer(i64),
    Float(f64),
    String(String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AnimationEasing {
    Linear,
}
