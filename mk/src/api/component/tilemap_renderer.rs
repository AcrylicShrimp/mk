use crate::api::{use_context, ColorUserData, ShaderUserData, TilemapUserData};
use crate::component::TilemapRenderer;
use legion::*;
use mlua::prelude::*;
use mlua::{MetaMethod, UserData, UserDataMethods};
use std::cell::RefCell;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TilemapRendererUserData(Entity);

impl From<Entity> for TilemapRendererUserData {
    fn from(entity: Entity) -> Self {
        Self(entity)
    }
}

impl From<TilemapRendererUserData> for Entity {
    fn from(tilemap_renderer: TilemapRendererUserData) -> Self {
        tilemap_renderer.0
    }
}

impl UserData for TilemapRendererUserData {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Index, |lua, this, index: String| {
            let context = use_context();

            let mut world = context.world_mut();
            let entry = world
                .entry(this.0)
                .ok_or_else(|| format!("invalid entity: {:#?}", this.0).to_lua_err())?;
            let tilemap_renderer = entry
                .get_component::<TilemapRenderer>()
                .map_err(|err| err.to_lua_err())?;

            match index.as_str() {
                "shader" => ShaderUserData::from(tilemap_renderer.shader.clone()).to_lua(lua),
                "tilemap" => TilemapUserData::from(tilemap_renderer.tilemap.clone()).to_lua(lua),
                "order" => tilemap_renderer.order.to_lua(lua),
                "color" => ColorUserData::from(RefCell::new(tilemap_renderer.color)).to_lua(lua),
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
                let mut tilemap_renderer = entry
                    .get_component_mut::<TilemapRenderer>()
                    .map_err(|err| err.to_lua_err())?;

                match index.as_str() {
                    "shader" => {
                        tilemap_renderer.shader =
                            ShaderUserData::from_lua(value, lua)?.clone().into()
                    }
                    "tilemap" => {
                        tilemap_renderer.tilemap = TilemapUserData::from_lua(value, lua)?.into();
                    }
                    "order" => tilemap_renderer.order = isize::from_lua(value, lua)?,
                    "color" => {
                        tilemap_renderer.color = ColorUserData::from_lua(value, lua)?
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
