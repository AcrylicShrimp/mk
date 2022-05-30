use crate::component::{
    GlyphRenderer, NinePatchRenderer, SpriteRenderer, TilemapRenderer, UIElement,
};
use crate::system::System;
use crate::EngineContextWithoutSystemManager;
use legion::*;

#[derive(Default, Debug)]
pub struct UIElementSystem;

impl UIElementSystem {
    pub fn new() -> Self {
        Self::default()
    }
}

impl System for UIElementSystem {
    fn run(&mut self, context: &EngineContextWithoutSystemManager) {
        let mut world = context.world_mut();

        <&UIElement>::query().for_each(&mut *world, |element| {
            element.with_element_mut(|e| {
                e.width = 0f32;
                e.height = 0f32;
            })
        });

        <(&mut GlyphRenderer, &UIElement)>::query().for_each_mut(
            &mut *world,
            |(glyph_renderer, element)| {
                let (_, layout) = glyph_renderer.font_and_layout();
                let glyphs = layout.glyphs();
                let (mut width, mut height) = element.with_element(|e| (e.width, e.height));

                for glyph in glyphs {
                    width = f32::max(width, glyph.x + glyph.width as f32);
                }

                height = f32::max(height, layout.height());

                element.with_element_mut(|e| {
                    e.width = width;
                    e.height = height;
                });
            },
        );

        <(&SpriteRenderer, &UIElement)>::query().for_each_mut(
            &mut *world,
            |(sprite_renderer, element)| {
                element.with_element_mut(|e| {
                    e.width = f32::max(e.width, sprite_renderer.sprite.width() as f32);
                    e.height = f32::max(e.height, sprite_renderer.sprite.height() as f32);
                });
            },
        );

        <(&NinePatchRenderer, &UIElement)>::query().for_each_mut(
            &mut *world,
            |(nine_patch_renderer, element)| {
                element.with_element_mut(|e| {
                    e.width = f32::max(e.width, nine_patch_renderer.width);
                    e.height = f32::max(e.height, nine_patch_renderer.height);
                });
            },
        );

        <(&TilemapRenderer, &UIElement)>::query().for_each_mut(
            &mut *world,
            |(tilemap_renderer, element)| {
                element.with_element_mut(|e| {
                    e.width = f32::max(
                        e.width,
                        tilemap_renderer.tilemap.tile_width
                            * tilemap_renderer.tilemap.tile_count_x as f32,
                    );
                    e.height = f32::max(
                        e.height,
                        tilemap_renderer.tilemap.tile_height
                            * tilemap_renderer.tilemap.tile_count_y as f32,
                    );
                });
            },
        );
    }
}
