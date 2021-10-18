use crate::api::{use_context, SpriteUserData};
use crate::arc_user_data;
use crate::render::*;
use mlua::prelude::*;
use mlua::{MetaMethod, UserData, UserDataMethods};

arc_user_data!(SpriteAtlasGrid => SpriteAtlasGridUserData);

impl UserData for SpriteAtlasGridUserData {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Index, |_lua, this, index: i64| {
            if index < 1 {
                return Ok(None);
            }

            Ok(this
                .clone()
                .into_inner()
                .sprites()
                .get((index - 1) as usize)
                .map(|sprite| SpriteUserData::from(sprite.clone())))
        });
        methods.add_meta_function(MetaMethod::Len, |lua, this: Self| {
            this.clone().into_inner().sprites().len().to_lua(lua)
        });
        methods.add_meta_function(MetaMethod::Pairs, |lua, this: Self| {
            let stateless_iter = lua.create_function(|lua, (this, index): (Self, i64)| {
                let index = index + 1;

                if index < 1 {
                    return Ok(LuaMultiValue::new());
                }

                let uindex = index as usize;
                let this = this.clone().into_inner();
                let sprites = this.sprites();

                Ok(if uindex <= sprites.len() {
                    (index, SpriteUserData::from(sprites[uindex - 1].clone())).to_lua_multi(lua)?
                } else {
                    LuaMultiValue::new()
                })
            })?;
            Ok((stateless_iter, this.clone(), 0))
        });
    }
}

pub fn lua_api_sprite_atlas_grid(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;

    table.set(
        "load",
        LuaValue::Function(lua.create_function(move |lua, path: String| {
            let sprite_atlas_grid = use_context()
                .asset_mgr()
                .load::<SpriteAtlasGrid, _>(path)
                .map_err(|err| err.to_lua_err())?;

            SpriteAtlasGridUserData::from(sprite_atlas_grid).to_lua(lua)
        })?),
    )?;

    Ok(table)
}
