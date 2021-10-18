use crate::api::use_context;
use crate::arc_user_data;
use fontdue::Font;
use mlua::prelude::*;
use mlua::UserData;

arc_user_data!(Font => FontUserData);

impl UserData for FontUserData {}

pub fn lua_api_font(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;

    table.set(
        "load",
        LuaValue::Function(lua.create_function(lua_api_font_load)?),
    )?;

    Ok(table)
}

fn lua_api_font_load(_lua: &Lua, path: String) -> LuaResult<FontUserData> {
    let font = use_context()
        .asset_mgr()
        .load::<Font, _>(path)
        .map_err(|err| err.to_lua_err())?;

    Ok(FontUserData::from(font))
}
