use crate::api::component::util::OnetimeCell;
use crate::api::*;
use crate::component::*;
use legion::*;
use mlua::prelude::*;
use mlua::{MetaMethod, UserData, UserDataMethods};
use std::cell::RefCell;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpriteRendererAnimationUserData(Entity);

impl From<Entity> for SpriteRendererAnimationUserData {
    fn from(entity: Entity) -> Self {
        Self(entity)
    }
}

impl From<SpriteRendererAnimationUserData> for Entity {
    fn from(animation: SpriteRendererAnimationUserData) -> Self {
        animation.0
    }
}

impl UserData for SpriteRendererAnimationUserData {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Index, |lua, this, index: String| {
            let context = use_context();

            let mut world = context.world_mut();
            let entry = world
                .entry(this.0)
                .ok_or_else(|| format!("invalid entity: {:#?}", this.0).to_lua_err())?;
            let sprite_renderer = entry
                .get_component::<SpriteRenderer>()
                .map_err(|err| err.to_lua_err())?;

            match index.as_str() {
                "current_sprite" => Ok(
                    if let SpriteType::Animated(animation) = &sprite_renderer.sprite {
                        SpriteUserData::from(animation.sprite().clone()).to_lua(lua)?
                    } else {
                        LuaValue::Nil
                    },
                ),
                _ => Err(format!("no such field: {}", index).to_lua_err()),
            }
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SpriteRendererUserData(Entity);

impl From<Entity> for SpriteRendererUserData {
    fn from(entity: Entity) -> Self {
        Self(entity)
    }
}

impl From<SpriteRendererUserData> for Entity {
    fn from(sprite_renderer: SpriteRendererUserData) -> Self {
        sprite_renderer.0
    }
}

impl UserData for SpriteRendererUserData {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Index, |lua, this, index: String| {
            let context = use_context();

            let mut world = context.world_mut();
            let entry = world
                .entry(this.0)
                .ok_or_else(|| format!("invalid entity: {:#?}", this.0).to_lua_err())?;
            let sprite_renderer = entry
                .get_component::<SpriteRenderer>()
                .map_err(|err| err.to_lua_err())?;

            match index.as_str() {
                "shader" => ShaderUserData::from(sprite_renderer.shader.clone()).to_lua(lua),
                "sprite" => Ok(
                    if let SpriteType::Sprite(sprite) = &sprite_renderer.sprite {
                        SpriteUserData::from(sprite.clone()).to_lua(lua)?
                    } else {
                        LuaValue::Nil
                    },
                ),
                "animation" => Ok(if let SpriteType::Animated(..) = &sprite_renderer.sprite {
                    SpriteRendererAnimationUserData::from(this.0).to_lua(lua)?
                } else {
                    LuaValue::Nil
                }),
                "order" => sprite_renderer.order.to_lua(lua),
                "color" => ColorUserData::from(RefCell::new(sprite_renderer.color)).to_lua(lua),
                _ => Err(format!("no such field: {}", index).to_lua_err()),
            }
        });
        methods.add_meta_method(
            MetaMethod::NewIndex,
            |lua, this, (index, value): (String, LuaValue)| {
                let context = use_context();

                let mut world = context.world_mut();
                let mut entry = world
                    .entry(this.0)
                    .ok_or_else(|| format!("invalid entity: {:#?}", this.0).to_lua_err())?;
                let mut sprite_renderer = entry
                    .get_component_mut::<SpriteRenderer>()
                    .map_err(|err| err.to_lua_err())?;

                match index.as_str() {
                    "shader" => {
                        sprite_renderer.shader =
                            ShaderUserData::from_lua(value, lua)?.clone().into()
                    }
                    "sprite" => {
                        sprite_renderer.sprite =
                            SpriteType::Sprite(SpriteUserData::from_lua(value, lua)?.into());
                    }
                    "animation" => {
                        sprite_renderer.sprite =
                            <Arc<OnetimeCell<crate::api::SpriteAnimation>>>::from(
                                SpriteAnimationUserData::from_lua(value, lua)?,
                            )
                            .take()
                            .unwrap()
                            .into();
                    }
                    "order" => sprite_renderer.order = isize::from_lua(value, lua)?,
                    "color" => {
                        sprite_renderer.color = ColorUserData::from_lua(value, lua)?
                            .into_inner()
                            .borrow()
                            .clone()
                    }
                    _ => return Err(format!("no such field: {}", index).to_lua_err()),
                }

                Ok(())
            },
        );
    }
}
