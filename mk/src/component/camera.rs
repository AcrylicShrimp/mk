use codegen::{Animation, LuaComponent};

#[derive(Animation, LuaComponent, Debug)]
pub struct Camera {
    pub layer: u64,
    pub order: isize,
}
