#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct Transform(u32);

impl Transform {
    pub fn new(index: u32) -> Self {
        Self(index)
    }
}

impl From<Transform> for u32 {
    fn from(this: Transform) -> Self {
        this.0
    }
}
