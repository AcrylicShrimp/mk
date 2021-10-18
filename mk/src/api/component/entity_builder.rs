use crate::api::component::util::OnetimeCell;
use crate::api::*;
use crate::arc_user_data;
use crate::component::*;
use crate::structure::Vec2;
use mlua::prelude::*;
use mlua::{UserData, UserDataMethods};

#[derive(Default)]
pub struct EntityBuilder {
    name: OnetimeCell<String>,
    parent_transform: OnetimeCell<TransformUserData>,
    position: OnetimeCell<Vec2>,
    scale: OnetimeCell<Vec2>,
    angle: OnetimeCell<f32>,
    layer: OnetimeCell<Layer>,
    camera: OnetimeCell<Camera>,
    glyph_renderer: OnetimeCell<GlyphRenderer>,
    sprite_renderer: OnetimeCell<SpriteRenderer>,
    tilemap_renderer: OnetimeCell<TilemapRenderer>,
}

arc_user_data!(EntityBuilder => EntityBuilderUserData);

impl UserData for EntityBuilderUserData {
    fn add_methods<'lua, M: UserDataMethods<'lua, Self>>(methods: &mut M) {
        methods.add_function("name", |_, (this, name): (Self, String)| {
            this.clone().into_inner().name.replace(name);
            Ok(this)
        });
        methods.add_function(
            "parentTransform",
            |_, (this, transform): (Self, TransformUserData)| {
                this.clone()
                    .into_inner()
                    .parent_transform
                    .replace(transform);
                Ok(this)
            },
        );
        methods.add_function("position", |_, (this, x, y): (Self, f32, f32)| {
            this.clone().into_inner().position.replace(Vec2::new(x, y));
            Ok(this)
        });
        methods.add_function("scale", |_, (this, x, y): (Self, f32, f32)| {
            this.clone().into_inner().scale.replace(Vec2::new(x, y));
            Ok(this)
        });
        methods.add_function("angle", |_, (this, angle): (Self, f32)| {
            this.clone().into_inner().angle.replace(angle);
            Ok(this)
        });
        methods.add_function("layer", |_, (this, layer): (Self, LuaValue)| {
            this.clone()
                .into_inner()
                .layer
                .replace(Layer::from(match layer {
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
                }));

            Ok(this)
        });
        methods.add_function(
            "camera",
            |_, (this, order, layer): (Self, isize, LuaValue)| {
                this.clone().into_inner().camera.replace(Camera {
                    order,
                    layer: match layer {
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
                    },
                });

                Ok(this)
            },
        );
        methods.add_function(
            "glyphRenderer",
            |_,
             (this, shader, font, font_size, order, color): (
                Self,
                ShaderUserData,
                FontUserData,
                f32,
                isize,
                ColorUserData,
            )| {
                this.clone().into_inner().glyph_renderer.replace({
                    let mut renderer =
                        GlyphRenderer::new(shader.into_inner(), font.into_inner(), font_size);
                    renderer.order = order;
                    renderer.color = color.into_inner().borrow().clone();
                    renderer
                });

                Ok(this)
            },
        );
        methods.add_function(
            "spriteRenderer",
            |_,
             (this, shader, sprite, order, color): (
                Self,
                ShaderUserData,
                SpriteUserData,
                isize,
                ColorUserData,
            )| {
                this.clone()
                    .into_inner()
                    .sprite_renderer
                    .replace(SpriteRenderer {
                        layer: crate::render::Layer::default(),
                        order,
                        color: color.into_inner().borrow().clone(),
                        shader: shader.into(),
                        sprite: SpriteType::Sprite(sprite.into()),
                    });
                Ok(this)
            },
        );
        methods.add_function(
            "tilemapRenderer",
            |_,
             (this, shader, tilemap, order, color): (
                Self,
                ShaderUserData,
                TilemapUserData,
                isize,
                ColorUserData,
            )| {
                this.clone()
                    .into_inner()
                    .tilemap_renderer
                    .replace(TilemapRenderer {
                        layer: crate::render::Layer::default(),
                        order,
                        color: color.into_inner().borrow().clone(),
                        shader: shader.into(),
                        tilemap: tilemap.into(),
                    });
                Ok(this)
            },
        );
        methods.add_method("build", |lua, this, _: ()| {
            let context = use_context();

            let this = this.clone().into_inner();

            let mut world = context.world_mut();
            let entity = world.push((NotYetComplete,));
            let mut entry = world.entry(entity).unwrap();

            let mut transform_mgr = context.transform_mgr_mut();
            let transform_index = transform_mgr.alloc(entity);
            entry.add_component(Transform::new(transform_index));

            if let Some(name) = this.name.take() {
                transform_mgr.set_name(transform_index, Some(name));
            }
            if let Some(component) = this.layer.take() {
                entry.add_component(component);
            }

            let mut transform = transform_mgr.transform_mut(transform_index);

            if let Some(position) = this.position.take() {
                transform.position = position;
            }
            if let Some(scale) = this.scale.take() {
                transform.scale = scale;
            }
            if let Some(angle) = this.angle.take() {
                transform.angle = angle;
            }
            if let Some(parent_transform) = this.parent_transform.take() {
                transform_mgr.set_parent(transform_index, Some(u32::from(parent_transform)));
            }
            if let Some(component) = this.camera.take() {
                entry.add_component(component);
            }
            if let Some(component) = this.glyph_renderer.take() {
                entry.add_component(component);
            }
            if let Some(component) = this.sprite_renderer.take() {
                entry.add_component(component);
            }
            if let Some(component) = this.tilemap_renderer.take() {
                entry.add_component(component);
            }

            entry.remove_component::<NotYetComplete>();

            EntityUserData::from(entity).to_lua(lua)
        });
    }
}
