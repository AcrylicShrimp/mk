use crate::api::*;
use crate::component::*;
use legion::*;
use mlua::prelude::*;
use mlua::{MetaMethod, UserData, UserDataMethods};
use std::any::type_name;

#[derive(Clone)]
pub struct EntityUserData(Entity);

impl From<Entity> for EntityUserData {
    fn from(entity: Entity) -> Self {
        Self(entity)
    }
}

impl From<EntityUserData> for Entity {
    fn from(entity: EntityUserData) -> Self {
        entity.0
    }
}

impl UserData for EntityUserData {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("remove", |lua, this: EntityUserData| {
            let context = use_context();
            let mut world = context.world_mut();

            world.remove(this.0).to_lua(lua)
        });
        methods.add_meta_method(MetaMethod::Index, |lua, this, index: String| {
            let context = use_context();

            let mut world = context.world_mut();
            let entry = world
                .entry(this.0)
                .ok_or_else(|| format!("invalid entity: {:#?}", this.0).to_lua_err())?;

            match index.as_str() {
                "transform" => entry
                    .get_component::<Transform>()
                    .map_or(LuaValue::Nil.to_lua(lua), |transform| {
                        TransformUserData::from(u32::from(*transform)).to_lua(lua)
                    }),
                "camera" => entry
                    .get_component::<Camera>()
                    .map_or(LuaValue::Nil.to_lua(lua), |_| {
                        CameraUserData::from(this.0).to_lua(lua)
                    }),
                "glyph_renderer" => entry
                    .get_component::<GlyphRenderer>()
                    .map_or(LuaValue::Nil.to_lua(lua), |_| {
                        GlyphRendererUserData::from(this.0).to_lua(lua)
                    }),
                "sprite_renderer" => entry
                    .get_component::<SpriteRenderer>()
                    .map_or(LuaValue::Nil.to_lua(lua), |_| {
                        SpriteRendererUserData::from(this.0).to_lua(lua)
                    }),
                "tilemap_renderer" => entry
                    .get_component::<TilemapRenderer>()
                    .map_or(LuaValue::Nil.to_lua(lua), |_| {
                        TilemapRendererUserData::from(this.0).to_lua(lua)
                    }),
                _ => Err(format!(
                    "property '{}' is not exists on the '{}'",
                    index,
                    type_name::<Self>()
                )
                .to_lua_err()),
            }
        });
    }
}

pub fn lua_api_entity(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;

    table.set(
        "builder",
        LuaValue::Function(lua.create_function(|_lua, ()| {
            Ok(EntityBuilderUserData::from(EntityBuilder::default()))
        })?),
    )?;
    table.set(
        "getByName",
        LuaValue::Function(lua.create_function(move |_lua, name: String| {
            let transform_mgr = use_context().transform_mgr();

            Ok(transform_mgr.find_by_name(&name).map(|indices| {
                indices
                    .iter()
                    .map(|&index| EntityUserData::from(transform_mgr.entity(index)))
                    .collect::<Vec<_>>()
            }))
        })?),
    )?;

    Ok(table)
}
