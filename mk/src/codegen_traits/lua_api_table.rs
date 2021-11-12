use mlua::prelude::*;

pub trait LuaApiTable {
    fn name() -> &'static str;
    fn fill_table(lua: &Lua, table: &LuaTable) -> LuaResult<()>;
}
