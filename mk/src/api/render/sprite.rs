use crate::api::use_context;
use crate::arc_user_data;
use crate::render::*;
use mlua::prelude::*;
use mlua::{MetaMethod, UserData, UserDataMethods};
use std::convert::TryFrom;

#[derive(Debug)]
pub struct SpriteChannelUserData(SpriteChannel);

impl From<SpriteChannel> for SpriteChannelUserData {
    fn from(channel: SpriteChannel) -> Self {
        Self(channel)
    }
}

impl From<SpriteChannelUserData> for SpriteChannel {
    fn from(channel: SpriteChannelUserData) -> Self {
        channel.0
    }
}

impl From<SpriteChannelUserData> for &str {
    fn from(channel: SpriteChannelUserData) -> Self {
        match channel.0 {
            SpriteChannel::R => "R",
            SpriteChannel::RG => "RG",
            SpriteChannel::RGB => "RGB",
            SpriteChannel::RGBA => "RGBA",
        }
    }
}

impl TryFrom<&str> for SpriteChannelUserData {
    type Error = String;

    fn try_from(channel: &str) -> Result<Self, Self::Error> {
        Ok(Self(match channel {
            "R" => SpriteChannel::R,
            "RG" => SpriteChannel::RG,
            "RGB" => SpriteChannel::RGB,
            "RGBA" => SpriteChannel::RGBA,
            _ => return Err(format!("invalid channel: {}", channel)),
        }))
    }
}

impl<'lua> FromLua<'lua> for SpriteChannelUserData {
    fn from_lua(lua_value: LuaValue<'lua>, _lua: &'lua Lua) -> LuaResult<Self> {
        Ok(match lua_value {
            LuaValue::String(string) => SpriteChannelUserData::try_from(string.to_str()?)
                .map_err(|err| LuaError::external(err))?,
            _ => {
                return Err(LuaError::external(format!(
                    "invalid channel: {:?}",
                    lua_value
                )))
            }
        })
    }
}

impl<'lua> ToLua<'lua> for SpriteChannelUserData {
    fn to_lua(self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        Ok(<&str>::from(self).to_lua(lua)?)
    }
}

arc_user_data!(Sprite => SpriteUserData);

impl UserData for SpriteUserData {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Index, |lua, this, index: String| {
            match index.as_str() {
                "width" => {
                    let this = this.clone().into_inner();
                    let texel_mapping = this.texel_mapping();
                    (texel_mapping.max().0 - texel_mapping.min().0).to_lua(lua)
                }
                "height" => {
                    let this = this.clone().into_inner();
                    let texel_mapping = this.texel_mapping();
                    (texel_mapping.max().1 - texel_mapping.min().1).to_lua(lua)
                }
                "channel" => {
                    SpriteChannelUserData::from(this.clone().into_inner().channel()).to_lua(lua)
                }
                _ => Err(format!("no such field: {}", index).to_lua_err()),
            }
        });
    }
}

pub fn lua_api_sprite(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;

    table.set(
        "load",
        LuaValue::Function(lua.create_function(move |lua, path: String| {
            let sprite = use_context()
                .asset_mgr()
                .load(path)
                .map_err(|err| err.to_lua_err())?;

            SpriteUserData::from(sprite).to_lua(lua)
        })?),
    )?;

    Ok(table)
}
