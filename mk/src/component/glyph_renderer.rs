use crate::render::{Color, Layer, LuaRcFont, LuaRcShader, Shader};
use codegen::{Animation, LuaComponent};
use fontdue::layout::{CoordinateSystem, Layout, TextStyle};
use fontdue::Font;
use mlua::prelude::*;
use std::sync::Arc;

#[derive(Animation, LuaComponent)]
pub struct GlyphRenderer {
    pub layer: Layer,
    pub order: isize,
    pub color: Color,
    #[lua_userdata(LuaRcShader)]
    pub shader: Arc<Shader>,
    #[lua_userdata(LuaRcFont)]
    #[lua_userfunc(set=lua_set_font)]
    // NOTE: Support the userfunc to the animation derive macro too.
    font: Arc<Font>,
    #[lua_userfunc(set=lua_set_font_size)]
    font_size: f32,
    #[lua_userfunc(set=lua_set_text)]
    text: String,
    #[lua_hidden]
    layout: Layout,
}

impl GlyphRenderer {
    pub fn new(shader: Arc<Shader>, font: Arc<Font>, font_size: f32) -> Self {
        Self {
            layer: Layer::default(),
            order: 0,
            color: Color::white(),
            shader,
            font,
            font_size,
            text: String::with_capacity(32),
            layout: Layout::new(CoordinateSystem::PositiveYUp),
        }
    }

    pub fn font(&self) -> &Arc<Font> {
        &self.font
    }

    pub fn font_size(&self) -> f32 {
        self.font_size
    }

    pub fn text(&self) -> &str {
        self.text.as_str()
    }

    pub fn layout(&self) -> &Layout {
        &self.layout
    }

    pub fn font_and_layout(&mut self) -> (&Arc<Font>, &mut Layout) {
        (&self.font, &mut self.layout)
    }

    pub fn set_font(&mut self, font: Arc<Font>) {
        self.font = font;
        self.layout.clear();
        self.layout.append(
            &[self.font.as_ref()],
            &TextStyle::new(self.text.as_str(), self.font_size, 0),
        );
    }

    pub fn set_font_size(&mut self, font_size: f32) {
        self.font_size = font_size;
        self.layout.clear();
        self.layout.append(
            &[self.font.as_ref()],
            &TextStyle::new(self.text.as_str(), self.font_size, 0),
        );
    }

    pub fn set_text(&mut self, text: String) {
        self.text = text;
        self.layout.clear();
        self.layout.append(
            &[self.font.as_ref()],
            &TextStyle::new(self.text.as_str(), self.font_size, 0),
        );
    }

    fn lua_set_font(&mut self, value: LuaValue, lua: &Lua) -> LuaResult<()> {
        self.set_font(<_>::from(LuaRcFont::from_lua(value, lua)?));
        Ok(())
    }

    fn lua_set_font_size(&mut self, value: LuaValue, lua: &Lua) -> LuaResult<()> {
        self.set_font_size(f32::from_lua(value, lua)?);
        Ok(())
    }

    fn lua_set_text(&mut self, value: LuaValue, lua: &Lua) -> LuaResult<()> {
        self.set_text(String::from_lua(value, lua)?);
        Ok(())
    }
}
