use crate::api::use_context;
use crate::arc_user_data;
use crate::render::*;
use mlua::prelude::*;
use mlua::UserData;

arc_user_data!(Shader => ShaderUserData);

impl UserData for ShaderUserData {}

pub fn lua_api_shader(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;

    table.set(
        "load",
        LuaValue::Function(lua.create_function(move |lua, path: String| {
            let shader = use_context()
                .asset_mgr()
                .load(path)
                .map_err(|err| err.to_lua_err())?;

            ShaderUserData::from(shader).to_lua(lua)
        })?),
    )?;

    Ok(table)
}
