use crate::api::use_context;
use crate::arc_user_data;
use mlua::prelude::*;
use mlua::{MetaMethod, UserData, UserDataMethods};

#[derive(Debug, Clone, Copy)]
pub struct Time;

arc_user_data!(Time => TimeUserData);

impl UserData for TimeUserData {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Index, |lua, _this, index: String| {
            match index.as_str() {
                "dt" => use_context().time_mgr().dt().to_lua(lua),
                _ => Err(format!("no such field: {}", index).to_lua_err()),
            }
        });
    }
}

pub fn lua_api_time() -> LuaResult<TimeUserData> {
    Ok(TimeUserData::from(Time))
}
