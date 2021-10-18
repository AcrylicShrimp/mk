use crate::event::events::*;
use mlua::prelude::*;

macro_rules! register_event {
    ($lua:ident, $table:ident, $event_type:ident) => {
        let (name, table) = $event_type::generate_lua_table($lua)?;
        $table.set(name, table)?;
    };
}

pub fn lua_api_event(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;

    register_event!(lua, table, Diagnostic);
    register_event!(lua, table, PreUpdate);
    register_event!(lua, table, Update);
    register_event!(lua, table, PostUpdate);
    register_event!(lua, table, PreRender);
    register_event!(lua, table, PostRender);
    register_event!(lua, table, KeyDown);
    register_event!(lua, table, KeyUp);

    Ok(table)
}
