use crate::api::render::SpriteAtlasGridUserData;
use crate::api::use_context;
use crate::arc_user_data;
use crate::render::Tilemap;
use mlua::prelude::*;
use mlua::{MetaMethod, UserData, UserDataMethods};

arc_user_data!(Tilemap => TilemapUserData);

impl UserData for TilemapUserData {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Index, |lua, this, index: String| {
            match index.as_str() {
                "tile_width" => {
                    let this = this.clone().into_inner();
                    this.tile_width.to_lua(lua)
                }
                "tile_height" => {
                    let this = this.clone().into_inner();
                    this.tile_height.to_lua(lua)
                }
                "tile_count_x" => {
                    let this = this.clone().into_inner();
                    this.tile_count_x.to_lua(lua)
                }
                "tile_count_y" => {
                    let this = this.clone().into_inner();
                    this.tile_count_y.to_lua(lua)
                }
                "layers" => {
                    let this = this.clone().into_inner();
                    this.layers.clone().to_lua(lua)
                }
                "palette" => {
                    let this = this.clone().into_inner();
                    SpriteAtlasGridUserData::from(this.palette.clone()).to_lua(lua)
                }
                _ => Err(format!("no such field: {}", index).to_lua_err()),
            }
        });
    }
}

pub fn lua_api_tilemap(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;

    table.set(
        "load",
        LuaValue::Function(lua.create_function(move |lua, path: String| {
            let tilemap = use_context()
                .asset_mgr()
                .load(path)
                .map_err(|err| err.to_lua_err())?;

            TilemapUserData::from(tilemap).to_lua(lua)
        })?),
    )?;

    Ok(table)
}
