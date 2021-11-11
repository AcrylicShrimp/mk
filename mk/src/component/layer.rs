use codegen::{Animation, LuaComponent};

#[derive(Animation, LuaComponent, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Layer(#[lua_field("value")] u64);

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
