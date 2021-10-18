use crate::api::{use_context, SpriteUserData};
use crate::arc_user_data;
use crate::render::*;
use mlua::prelude::*;
use mlua::{MetaMethod, UserData, UserDataMethods};
use std::cell::RefCell;

pub type SpriteAtlasIter = (RefCell<usize>, Vec<String>, SpriteAtlasUserData);

arc_user_data!(SpriteAtlasIter => SpriteAtlasIterUserData);

impl UserData for SpriteAtlasIterUserData {}

arc_user_data!(SpriteAtlas => SpriteAtlasUserData);

impl UserData for SpriteAtlasUserData {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Index, |_lua, this, index: String| {
            Ok(this
                .clone()
                .into_inner()
                .sprites()
                .get(&index)
                .map(|sprite| SpriteUserData::from(sprite.clone())))
        });
        methods.add_meta_function(MetaMethod::Len, |lua, this: Self| {
            this.clone().into_inner().sprites().len().to_lua(lua)
        });
        methods.add_meta_function(MetaMethod::Pairs, |lua, this: Self| {
            let stateless_iter = lua.create_function(
                |lua, (iter, _): (SpriteAtlasIterUserData, Option<String>)| {
                    let iter = iter.into_inner();
                    let this = iter.2.clone().into_inner();
                    let mut index = iter.0.borrow_mut();

                    Ok(if *index < iter.1.len() {
                        let key = &iter.1[*index];
                        let pair = (
                            key.clone(),
                            SpriteUserData::from(this.sprites().get(key).unwrap().clone()),
                        )
                            .to_lua_multi(lua)?;
                        *index += 1;
                        pair
                    } else {
                        LuaMultiValue::new()
                    })
                },
            )?;

            let this_inner = this.clone().into_inner();
            let iter = SpriteAtlasIterUserData::from((
                RefCell::from(0),
                this_inner.sprites().keys().cloned().collect(),
                this.clone(),
            ));

            Ok((
                stateless_iter,
                iter,
                this_inner.sprites().keys().next().map(|key| key.clone()),
            ))
        });
    }
}

pub fn lua_api_sprite_atlas(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;

    table.set(
        "load",
        LuaValue::Function(lua.create_function(move |lua, path: String| {
            let sprite_atlas = use_context()
                .asset_mgr()
                .load::<SpriteAtlas, _>(path)
                .map_err(|err| err.to_lua_err())?;

            SpriteAtlasUserData::from(sprite_atlas).to_lua(lua)
        })?),
    )?;

    Ok(table)
}
