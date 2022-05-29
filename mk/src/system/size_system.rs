use crate::component::{GlyphRenderer, NinePatchRenderer, Size, SpriteRenderer, TilemapRenderer};
use crate::system::System;
use crate::EngineContextWithoutSystemManager;
use legion::*;

#[derive(Default, Debug)]
pub struct SizeSystem;

impl SizeSystem {
    pub fn new() -> Self {
        Self::default()
    }
}

impl System for SizeSystem {
    fn run(&mut self, context: &EngineContextWithoutSystemManager) {
        let mut world = context.world_mut();

        <&mut Size>::query().for_each_mut(&mut *world, |size| {
            size.width = 0f32;
            size.height = 0f32;
        });

        <(&mut GlyphRenderer, &mut Size)>::query().for_each_mut(
            &mut *world,
            |(glyph_renderer, size)| {
                let (_, layout) = glyph_renderer.font_and_layout();
                let glyphs = layout.glyphs();

                for glyph in glyphs {
                    size.width = f32::max(size.width, glyph.x + glyph.width as f32);
                }

                size.height = f32::max(size.height, layout.height());
            },
        );

        <(&SpriteRenderer, &mut Size)>::query().for_each_mut(
            &mut *world,
            |(sprite_renderer, size)| {
                size.width = f32::max(size.width, sprite_renderer.sprite.width() as f32);
                size.height = f32::max(size.height, sprite_renderer.sprite.height() as f32);
            },
        );

        <(&NinePatchRenderer, &mut Size)>::query().for_each_mut(
            &mut *world,
            |(nine_patch_renderer, size)| {
                size.width = f32::max(size.width, nine_patch_renderer.width);
                size.height = f32::max(size.height, nine_patch_renderer.height);
            },
        );

        <(&TilemapRenderer, &mut Size)>::query().for_each_mut(
            &mut *world,
            |(tilemap_renderer, size)| {
                size.width = f32::max(
                    size.width,
                    tilemap_renderer.tilemap.tile_width
                        * tilemap_renderer.tilemap.tile_count_x as f32,
                );
                size.height = f32::max(
                    size.height,
                    tilemap_renderer.tilemap.tile_height
                        * tilemap_renderer.tilemap.tile_count_y as f32,
                );
            },
        );
    }
}