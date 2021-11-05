use std::sync::Arc;

use crate::api::use_context;
use crate::arc_user_data;
use crate::render::*;
use mlua::prelude::*;
use mlua::UserData;

#[derive(Debug, Clone)]
pub struct ArcShaderWrapper(pub Arc<Shader>);

impl From<Arc<Shader>> for ArcShaderWrapper {
    fn from(shader: Arc<Shader>) -> Self {
        Self(shader)
    }
}

impl From<ArcShaderWrapper> for Arc<Shader> {
    fn from(shader: ArcShaderWrapper) -> Self {
        shader.0
    }
}

impl<'lua> mlua::ToLua<'lua> for ArcShaderWrapper {
    fn to_lua(self, _lua: &'lua Lua) -> mlua::Result<mlua::Value<'lua>> {
        Ok(mlua::Value::LightUserData(mlua::LightUserData(
            Arc::into_raw(self.0) as _,
        )))
    }
}

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
