use crate::render::{Color, Layer, Shader};
use fontdue::layout::{CoordinateSystem, Layout, TextStyle};
use fontdue::Font;
use std::sync::Arc;

pub struct GlyphRenderer {
    pub layer: Layer,
    pub order: isize,
    pub color: Color,
    pub shader: Arc<Shader>,
    font: Arc<Font>,
    font_size: f32,
    text: String,
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

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_owned();
        self.layout.clear();
        self.layout.append(
            &[self.font.as_ref()],
            &TextStyle::new(text, self.font_size, 0),
        );
    }
}
