use crate::api::component::util::OnetimeCell;
use crate::api::*;
use crate::arc_user_data;
use crate::component::*;
use crate::render::*;
use mlua::prelude::*;
use mlua::{Result as LuaResult, UserData};
use std::sync::Arc;

pub enum SpriteAnimation {
    Sequential(Vec<(Arc<Sprite>, f32)>),
}

impl From<SpriteAnimation> for SpriteType {
    fn from(animation: SpriteAnimation) -> Self {
        match animation {
            SpriteAnimation::Sequential(sprites) => {
                Self::Animated(Box::new(SequentialSpriteAnimation::new(sprites)))
            }
        }
    }
}

arc_user_data!(OnetimeCell<SpriteAnimation> => SpriteAnimationUserData);

impl UserData for SpriteAnimationUserData {}

pub fn lua_api_sprite_animation(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;

    table.set(
        "sequential",
        LuaValue::Function(lua.create_function(lua_api_sprite_animation_sequential)?),
    )?;

    Ok(table)
}

fn lua_api_sprite_animation_sequential<'lua>(
    lua: &'lua Lua,
    frames: LuaMultiValue<'lua>,
) -> LuaResult<SpriteAnimationUserData> {
    Ok(SpriteAnimationUserData::from(OnetimeCell::new(
        SpriteAnimation::Sequential(
            frames
                .into_iter()
                .map(|item| {
                    let table = LuaTable::from_lua(item, lua)?;
                    Ok((
                        <Arc<Sprite>>::from(table.get::<_, SpriteUserData>(1)?),
                        table.get::<_, f32>(2)?,
                    ))
                })
                .collect::<LuaResult<Vec<_>>>()?,
        ),
    )))
}
