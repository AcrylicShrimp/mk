use crate::api::use_context;
use crate::component::*;
use legion::*;
use mlua::prelude::*;
use mlua::{Error, MetaMethod, UserData, UserDataMethods};
use std::any::type_name;

#[derive(Clone)]
pub struct CameraUserData(Entity);

impl From<Entity> for CameraUserData {
    fn from(entity: Entity) -> Self {
        Self(entity)
    }
}

impl From<CameraUserData> for Entity {
    fn from(entity: CameraUserData) -> Self {
        entity.0
    }
}

impl UserData for CameraUserData {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Index, |lua, this, index: String| {
            let context = use_context();

            let mut world = context.world_mut();
            let entry = world
                .entry(this.0)
                .ok_or_else(|| format!("invalid entity: {:#?}", this.0).to_lua_err())?;
            let camera = entry
                .get_component::<Camera>()
                .map_err(|err| Error::external(err))?;

            match index.as_str() {
                "order" => camera.order.to_lua(lua),
                "layer" => camera.layer.to_lua(lua),
                _ => Err(format!(
                    "property '{}' is not exists on the '{}'",
                    index,
                    type_name::<Self>()
                )
                .to_lua_err()),
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
                let mut camera = entry
                    .get_component_mut::<Camera>()
                    .map_err(|err| Error::external(err))?;

                match index.as_str() {
                    "order" => camera.order = isize::from_lua(value, lua)?,
                    "layer" => {
                        camera.layer = match value {
                            LuaValue::Nil => 0xFFFFFFFFFFFFFFFF,
                            LuaValue::Boolean(boolean) => {
                                if boolean {
                                    0xFFFFFFFFFFFFFFFF
                                } else {
                                    0
                                }
                            }
                            LuaValue::Number(number) => number as u64,
                            LuaValue::Integer(integer) => integer as u64,
                            LuaValue::String(string) => match string.to_str()? {
                                "all" => 0xFFFFFFFFFFFFFFFF,
                                "none" => 0,
                                _ => return Err("only 'all' or 'none' allowed".to_lua_err()),
                            },
                            LuaValue::Table(table) => {
                                let mut layer = 0;

                                for index in 0..table.len()? {
                                    layer &= 1 << table.get::<_, LuaInteger>(index)?;
                                }

                                layer
                            }
                            _ => return Err("only 'all' or 'none' allowed".to_lua_err()),
                        }
                    }
                    _ => return Err(format!("no such field: {}", index).to_lua_err()),
                }

                Ok(())
            },
        );
    }
}
