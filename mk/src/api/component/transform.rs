use crate::api::use_context;
use crate::structure::Vec2;
use mlua::prelude::*;
use mlua::{MetaMethod, UserData, UserDataMethods};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TransformUserData(u32);

impl From<u32> for TransformUserData {
    fn from(index: u32) -> Self {
        Self(index)
    }
}

impl From<TransformUserData> for u32 {
    fn from(this: TransformUserData) -> Self {
        this.0
    }
}

impl UserData for TransformUserData {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Index, |lua, this, index: String| {
            let context = use_context();
            let transform_mgr = context.transform_mgr();
            let transform = transform_mgr.transform(u32::from(*this));

            // TODO: Split the properties into dedicated vec2 types.
            match index.as_str() {
                "position_x" => transform.position.x.to_lua(lua),
                "position_y" => transform.position.y.to_lua(lua),
                "scale_x" => transform.scale.x.to_lua(lua),
                "scale_y" => transform.scale.y.to_lua(lua),
                "angle" => transform.angle.to_lua(lua),
                "parent" => transform
                    .parent_index()
                    .map(|parent_index| TransformUserData::from(parent_index))
                    .to_lua(lua),
                _ => Err(format!("no such field: {}", index).to_lua_err()),
            }
        });
        methods.add_meta_method(
            MetaMethod::NewIndex,
            |lua, this, (index, value): (String, LuaValue)| {
                let context = use_context();
                let mut transform_mgr = context.transform_mgr_mut();
                let transform_index = u32::from(*this);

                match index.as_str() {
                    "position_x" => {
                        transform_mgr.transform_mut(transform_index).position.x =
                            f32::from_lua(value, lua)?
                    }
                    "position_y" => {
                        transform_mgr.transform_mut(transform_index).position.y =
                            f32::from_lua(value, lua)?
                    }
                    "scale_x" => {
                        transform_mgr.transform_mut(transform_index).scale.x =
                            f32::from_lua(value, lua)?
                    }
                    "scale_y" => {
                        transform_mgr.transform_mut(transform_index).scale.y =
                            f32::from_lua(value, lua)?
                    }
                    "angle" => {
                        transform_mgr.transform_mut(transform_index).angle =
                            f32::from_lua(value, lua)?
                    }
                    "parent" => {
                        transform_mgr.set_parent(
                            u32::from(*this),
                            if let LuaValue::Nil = value {
                                None
                            } else {
                                let parent_index = TransformUserData::from_lua(value, lua)?;
                                Some(u32::from(parent_index))
                            },
                        );
                    }
                    _ => return Err(format!("no such field: {}", index).to_lua_err()),
                }

                transform_mgr.transform_mut(transform_index).mark_as_dirty();
                Ok(())
            },
        );
        methods.add_method("translate", |_lua, this, (x, y): (f32, f32)| {
            let context = use_context();
            let mut transform_mgr = context.transform_mgr_mut();
            let transform = transform_mgr.transform_mut(u32::from(*this));

            transform.translate(Vec2::new(x, y));
            Ok(())
        });
        methods.add_method("translate_world", |_lua, this, (x, y): (f32, f32)| {
            let context = use_context();
            let mut transform_mgr = context.transform_mgr_mut();
            let transform = transform_mgr.transform_mut(u32::from(*this));

            transform.translate_world(Vec2::new(x, y));
            Ok(())
        });
    }
}
