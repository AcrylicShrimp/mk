use crate::arc_user_data;
use crate::render::*;
use mlua::prelude::*;
use mlua::{MetaMethod, UserData, UserDataMethods};
use std::cell::RefCell;

arc_user_data!(RefCell<Color> => ColorUserData);

impl UserData for ColorUserData {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Index, |lua, this, index: String| {
            match index.as_str() {
                "r" => this.clone().into_inner().borrow().r.to_lua(lua),
                "g" => this.clone().into_inner().borrow().g.to_lua(lua),
                "b" => this.clone().into_inner().borrow().b.to_lua(lua),
                "a" => this.clone().into_inner().borrow().a.to_lua(lua),
                _ => Err(format!("no such field: {}", index).to_lua_err()),
            }
        });
        methods.add_meta_method(
            MetaMethod::NewIndex,
            |_lua, this, (index, value): (String, f32)| {
                match index.as_str() {
                    "r" => this.clone().into_inner().borrow_mut().r = value,
                    "g" => this.clone().into_inner().borrow_mut().g = value,
                    "b" => this.clone().into_inner().borrow_mut().b = value,
                    "a" => this.clone().into_inner().borrow_mut().a = value,
                    _ => return Err(format!("no such field: {}", index).to_lua_err()),
                }

                Ok(())
            },
        );
    }
}

pub fn lua_api_color(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;

    table.set(
        "RGB",
        LuaValue::Function(lua.create_function(lua_api_color_rgb)?),
    )?;
    table.set(
        "RGBA",
        LuaValue::Function(lua.create_function(lua_api_color_rgba)?),
    )?;

    Ok(table)
}

fn lua_api_color_rgb(_lua: &Lua, (r, g, b): (f32, f32, f32)) -> LuaResult<ColorUserData> {
    let color = Color { r, g, b, a: 1f32 };
    Ok(ColorUserData::from(RefCell::from(color)))
}

fn lua_api_color_rgba(_lua: &Lua, (r, g, b, a): (f32, f32, f32, f32)) -> LuaResult<ColorUserData> {
    let color = Color { r, g, b, a };
    Ok(ColorUserData::from(RefCell::from(color)))
}
