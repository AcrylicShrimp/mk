use mlua::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Layer(pub u64);

impl Layer {
    pub fn has_overlap(lhs: Self, rhs: Self) -> bool {
        lhs.0 & rhs.0 != 0
    }
}

impl Default for Layer {
    fn default() -> Self {
        Self(0xFFFFFFFFFFFFFFFF)
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
