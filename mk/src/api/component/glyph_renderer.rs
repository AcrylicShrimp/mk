use crate::api::*;
use crate::component::*;
use legion::*;
use mlua::prelude::*;
use mlua::{MetaMethod, UserData, UserDataMethods};
use std::cell::RefCell;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GlyphRendererUserData(Entity);

impl From<Entity> for GlyphRendererUserData {
    fn from(entity: Entity) -> Self {
        Self(entity)
    }
}

impl From<GlyphRendererUserData> for Entity {
    fn from(glyph_renderer: GlyphRendererUserData) -> Self {
        glyph_renderer.0
    }
}

impl UserData for GlyphRendererUserData {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_meta_method(MetaMethod::Index, |lua, this, index: String| {
            let context = use_context();

            let mut world = context.world_mut();
            let entry = world
                .entry(this.0)
                .ok_or_else(|| format!("invalid entity: {:#?}", this.0).to_lua_err())?;
            let glyph_renderer = entry
                .get_component::<GlyphRenderer>()
                .map_err(|err| err.to_lua_err())?;

            match index.as_str() {
                "shader" => ShaderUserData::from(glyph_renderer.shader.clone()).to_lua(lua),
                "font" => FontUserData::from(glyph_renderer.font().clone()).to_lua(lua),
                "font_size" => glyph_renderer.font_size().to_lua(lua),
                "order" => glyph_renderer.order.to_lua(lua),
                "color" => ColorUserData::from(RefCell::new(glyph_renderer.color)).to_lua(lua),
                "text" => glyph_renderer.text().to_lua(lua),
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
                let mut glyph_renderer = entry
                    .get_component_mut::<GlyphRenderer>()
                    .map_err(|err| err.to_lua_err())?;

                match index.as_str() {
                    "shader" => {
                        glyph_renderer.shader = ShaderUserData::from_lua(value, lua)?.into()
                    }
                    "font" => glyph_renderer.set_font(FontUserData::from_lua(value, lua)?.into()),
                    "font_size" => glyph_renderer.set_font_size(f32::from_lua(value, lua)?),
                    "order" => glyph_renderer.order = isize::from_lua(value, lua)?,
                    "color" => {
                        glyph_renderer.color = ColorUserData::from_lua(value, lua)?
                            .into_inner()
                            .borrow()
                            .clone()
                    }
                    "text" => glyph_renderer.set_text(&String::from_lua(value, lua)?),
                    _ => return Err(format!("no such field: {}", index).to_lua_err()),
                }

                Ok(())
            },
        );
    }
}
