#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Layer(u64);

impl Default for Layer {
    fn default() -> Self {
        Self(0xFFFFFFFFFFFFFFFF)
    }
}

impl From<u64> for Layer {
    fn from(layer: u64) -> Self {
        Self(layer)
    }
}

impl From<Layer> for u64 {
    fn from(layer: Layer) -> Self {
        layer.0
    }
}