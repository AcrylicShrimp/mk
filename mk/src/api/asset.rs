use crate::api::use_context;
use crate::codegen_traits::LuaApiTable;
use crate::render::{LuaRcFont, LuaRcShader, LuaRcSprite, LuaRcSpriteAtlas, LuaRcSpriteAtlasGrid};
use mlua::prelude::*;

pub struct FontAsset;

impl LuaApiTable for FontAsset {
    fn api_name() -> &'static str {
        "Font"
    }

    fn fill_api_table(lua: &Lua, table: &LuaTable) -> LuaResult<()> {
        table.set(
            "load",
            lua.create_function(|_, name: String| {
                let asset_mgr = use_context().asset_mgr();
                let shader = asset_mgr.load(&name).map_err(|err| {
                    format!("unable to load font '{}' due to: {}", name, err).to_lua_err()
                })?;
                Ok(LuaRcFont::from(shader))
            })?,
        )?;
        Ok(())
    }
}

pub struct ShaderAsset;

impl LuaApiTable for ShaderAsset {
    fn api_name() -> &'static str {
        "Shader"
    }

    fn fill_api_table(lua: &Lua, table: &LuaTable) -> LuaResult<()> {
        table.set(
            "load",
            lua.create_function(|_, name: String| {
                let asset_mgr = use_context().asset_mgr();
                let shader = asset_mgr.load(&name).map_err(|err| {
                    format!("unable to load shader '{}' due to: {}", name, err).to_lua_err()
                })?;
                Ok(LuaRcShader::from(shader))
            })?,
        )?;
        Ok(())
    }
}

pub struct SpriteAsset;

impl LuaApiTable for SpriteAsset {
    fn api_name() -> &'static str {
        "Sprite"
    }

    fn fill_api_table(lua: &Lua, table: &LuaTable) -> LuaResult<()> {
        table.set(
            "load",
            lua.create_function(|_, name: String| {
                let asset_mgr = use_context().asset_mgr();
                let shader = asset_mgr.load(&name).map_err(|err| {
                    format!("unable to load sprite '{}' due to: {}", name, err).to_lua_err()
                })?;
                Ok(LuaRcSprite::from(shader))
            })?,
        )?;
        Ok(())
    }
}

pub struct SpriteAtlasAsset;

impl LuaApiTable for SpriteAtlasAsset {
    fn api_name() -> &'static str {
        "SpriteAtlas"
    }

    fn fill_api_table(lua: &Lua, table: &LuaTable) -> LuaResult<()> {
        table.set(
            "load",
            lua.create_function(|_, name: String| {
                let asset_mgr = use_context().asset_mgr();
                let shader = asset_mgr.load(&name).map_err(|err| {
                    format!("unable to load sprite atlas '{}' due to: {}", name, err).to_lua_err()
                })?;
                Ok(LuaRcSpriteAtlas::from(shader))
            })?,
        )?;
        Ok(())
    }
}

pub struct SpriteAtlasGridAsset;

impl LuaApiTable for SpriteAtlasGridAsset {
    fn api_name() -> &'static str {
        "SpriteAtlasGrid"
    }

    fn fill_api_table(lua: &Lua, table: &LuaTable) -> LuaResult<()> {
        table.set(
            "load",
            lua.create_function(|_, name: String| {
                let asset_mgr = use_context().asset_mgr();
                let shader = asset_mgr.load(&name).map_err(|err| {
                    format!(
                        "unable to load sprite atlas grid '{}' due to: {}",
                        name, err
                    )
                    .to_lua_err()
                })?;
                Ok(LuaRcSpriteAtlasGrid::from(shader))
            })?,
        )?;
        Ok(())
    }
}
