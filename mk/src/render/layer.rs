use mlua::prelude::*;

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

impl<'lua> FromLua<'lua> for Layer {
    fn from_lua(value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        match value {
            LuaValue::Integer(layer) => Ok(Self(layer as _)),
            _ => {
                return Err(format!("the type {} must be a {}", "Layer", "integer").to_lua_err());
            }
        }
    }
}

impl<'lua> ToLua<'lua> for Layer {
    fn to_lua(self, _lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        Ok(LuaValue::Integer(self.0 as _))
    }
}
