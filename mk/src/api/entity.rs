use crate::api::use_context;
use crate::codegen_traits::LuaApiTable;
use crate::component::{
    Camera, GlyphRenderer, GlyphRendererConfig, LuaComponentCamera, LuaComponentGlyphRenderer,
    LuaComponentNinePatchRenderer, LuaComponentSpriteRenderer, LuaComponentTilemapRenderer,
    NinePatchRenderer, Size, SpriteRenderer, TilemapRenderer, Transform, UIElement,
};
use crate::render::{
    Color, LuaRcFont, LuaRcShader, LuaRcSprite, LuaRcSpriteNinePatch, LuaRcTilemap,
};
use crate::structure::Vec2;
use codegen::LuaComponentNoWrapper;
use legion::world::Entry;
use mlua::prelude::*;
use std::marker::PhantomData;

#[derive(LuaComponentNoWrapper, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    #[lua_hidden]
    entity: legion::Entity,
    #[lua_readonly]
    #[lua_userfunc(get=lua_get_transform)]
    transform: PhantomData<Transform>,
    #[lua_readonly]
    #[lua_userfunc(get=lua_get_size)]
    size: PhantomData<Size>,
    #[lua_readonly]
    #[lua_userfunc(get=lua_get_ui_element)]
    ui_element: PhantomData<UIElement>,
    #[lua_readonly]
    #[lua_userfunc(get=lua_get_camera)]
    camera: PhantomData<LuaComponentCamera>,
    #[lua_readonly]
    #[lua_userfunc(get=lua_get_glyph_renderer)]
    glyph_renderer: PhantomData<LuaComponentGlyphRenderer>,
    #[lua_readonly]
    #[lua_userfunc(get=lua_get_sprite_renderer)]
    sprite_renderer: PhantomData<LuaComponentSpriteRenderer>,
    #[lua_readonly]
    #[lua_userfunc(get=lua_get_nine_patch_renderer)]
    nine_patch_renderer: PhantomData<LuaComponentNinePatchRenderer>,
    #[lua_readonly]
    #[lua_userfunc(get=lua_get_tilemap_renderer)]
    tilemap_renderer: PhantomData<LuaComponentTilemapRenderer>,
    #[lua_method]
    listen: PhantomData<()>,
    #[lua_method]
    unlisten: PhantomData<()>,
}

impl Entity {
    pub fn new(entity: legion::Entity) -> Self {
        Self {
            entity,
            transform: PhantomData,
            size: PhantomData,
            ui_element: PhantomData,
            camera: PhantomData,
            glyph_renderer: PhantomData,
            sprite_renderer: PhantomData,
            nine_patch_renderer: PhantomData,
            tilemap_renderer: PhantomData,
            listen: PhantomData,
            unlisten: PhantomData,
        }
    }

    pub fn entity(&self) -> legion::Entity {
        self.entity
    }

    pub fn with_entry<T>(&self, f: impl FnOnce(&Entry) -> T) -> Option<T> {
        let mut world = use_context().world_mut();
        let entry = match world.entry(self.entity) {
            Some(entry) => entry,
            None => return None,
        };
        Some(f(&entry))
    }

    pub fn with_entry_mut<T>(&self, f: impl FnOnce(&mut Entry) -> T) -> Option<T> {
        let mut world = use_context().world_mut();
        let mut entry = match world.entry(self.entity) {
            Some(entry) => entry,
            None => return None,
        };
        Some(f(&mut entry))
    }

    fn lua_get_transform<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        self.with_entry(|e| e.get_component::<Transform>().ok().cloned())
            .to_lua(lua)
    }

    fn lua_get_size<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        self.with_entry(|e| e.get_component::<Size>().ok().cloned())
            .to_lua(lua)
    }

    fn lua_get_ui_element<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        self.with_entry(|e| e.get_component::<UIElement>().ok().cloned())
            .to_lua(lua)
    }

    fn lua_get_camera<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        self.with_entry(|e| {
            e.get_component::<Camera>()
                .ok()
                .map(|_| LuaComponentCamera::from(self.entity))
        })
        .to_lua(lua)
    }

    fn lua_get_glyph_renderer<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        self.with_entry(|e| {
            e.get_component::<GlyphRenderer>()
                .ok()
                .map(|_| LuaComponentGlyphRenderer::from(self.entity))
        })
        .to_lua(lua)
    }

    fn lua_get_sprite_renderer<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        self.with_entry(|e| {
            e.get_component::<SpriteRenderer>()
                .ok()
                .map(|_| LuaComponentSpriteRenderer::from(self.entity))
        })
        .to_lua(lua)
    }

    fn lua_get_nine_patch_renderer<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        self.with_entry(|e| {
            e.get_component::<NinePatchRenderer>()
                .ok()
                .map(|_| LuaComponentNinePatchRenderer::from(self.entity))
        })
        .to_lua(lua)
    }

    fn lua_get_tilemap_renderer<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        self.with_entry(|e| {
            e.get_component::<TilemapRenderer>()
                .ok()
                .map(|_| LuaComponentTilemapRenderer::from(self.entity))
        })
        .to_lua(lua)
    }

    fn listen(&self, lua: &Lua, (event, function): (String, LuaFunction)) -> LuaResult<usize> {
        use_context()
            .entity_event_mgr_mut()
            .add_entity_listener(lua, function, event, self.entity)
    }

    fn unlisten(&self, _lua: &Lua, handler: usize) -> LuaResult<()> {
        use_context()
            .entity_event_mgr_mut()
            .remove_entity_listener(self.entity, handler);
        Ok(())
    }
}

impl LuaApiTable for Entity {
    fn api_name() -> &'static str {
        "Entity"
    }

    #[allow(unused_variables)]
    fn fill_api_table(lua: &Lua, table: &LuaTable) -> LuaResult<()> {
        table.set(
            "build",
            lua.create_function(|lua, param: EntityBuildParam| {
                let context = use_context();
                let mut world = context.world_mut();
                let entity = world.push(());
                let mut entry = world.entry(entity).unwrap();

                let mut transform_mgr = context.transform_mgr_mut();
                let transform = transform_mgr.alloc(entity);

                entry.add_component(Transform::new(transform));
                entry.add_component(Size::new(transform));

                transform_mgr.set_name(transform, param.name);

                if let Some(param) = param.transform {
                    transform_mgr
                        .set_parent(transform, param.parent.map(|transform| transform.index()));

                    let transform = transform_mgr.transform_mut(transform);

                    if let Some(position) = param.position {
                        transform.position = position;
                    }

                    if let Some(scale) = param.scale {
                        transform.scale = scale;
                    }

                    if let Some(angle) = param.angle {
                        transform.angle = angle;
                    }
                }

                if let Some(param) = param.ui_element {
                    let mut ui_mgr = context.ui_mgr_mut();
                    let element = ui_mgr.alloc(entity);

                    if let Some(is_interactible) = param.is_interactible {
                        ui_mgr
                            .element_mut(element)
                            .set_interactible(is_interactible);
                    }

                    ui_mgr
                        .element_mut(element)
                        .set_order_index(param.order_index);

                    entry.add_component(UIElement::new(element));
                }

                if let Some(param) = param.camera {
                    entry.add_component(Camera {
                        layer: param.layer.unwrap_or_default(),
                        order: param.order.unwrap_or_default(),
                    });
                }

                if let Some(param) = param.glyph_renderer {
                    let mut glyph_renderer = GlyphRenderer::new(
                        <_>::from(param.shader),
                        <_>::from(param.font),
                        param.font_size,
                    );

                    if let Some(layer) = param.layer {
                        glyph_renderer.layer = layer;
                    }

                    if let Some(order) = param.order {
                        glyph_renderer.order = order;
                    }

                    if let Some(color) = param.color {
                        glyph_renderer.color = color;
                    }

                    if let Some(config) = param.config {
                        glyph_renderer.set_config(config);
                    }

                    if let Some(text) = param.text {
                        glyph_renderer.set_text(text);
                    }

                    entry.add_component(glyph_renderer);
                }

                if let Some(param) = param.sprite_renderer {
                    let mut sprite_renderer =
                        SpriteRenderer::new(<_>::from(param.shader), <_>::from(param.sprite));

                    if let Some(layer) = param.layer {
                        sprite_renderer.layer = layer;
                    }

                    if let Some(order) = param.order {
                        sprite_renderer.order = order;
                    }

                    if let Some(color) = param.color {
                        sprite_renderer.color = color;
                    }

                    entry.add_component(sprite_renderer);
                }

                if let Some(param) = param.nine_patch_renderer {
                    let mut nine_patch_renderer = NinePatchRenderer::new(
                        <_>::from(param.shader),
                        <_>::from(param.nine_patch),
                        <_>::from(param.width),
                        <_>::from(param.height),
                    );

                    if let Some(layer) = param.layer {
                        nine_patch_renderer.layer = layer;
                    }

                    if let Some(order) = param.order {
                        nine_patch_renderer.order = order;
                    }

                    if let Some(color) = param.color {
                        nine_patch_renderer.color = color;
                    }

                    entry.add_component(nine_patch_renderer);
                }

                if let Some(param) = param.tilemap_renderer {
                    let mut tilemap_renderer =
                        TilemapRenderer::new(<_>::from(param.shader), <_>::from(param.tilemap));

                    if let Some(layer) = param.layer {
                        tilemap_renderer.layer = layer;
                    }

                    if let Some(order) = param.order {
                        tilemap_renderer.order = order;
                    }

                    if let Some(color) = param.color {
                        tilemap_renderer.color = color;
                    }

                    entry.add_component(tilemap_renderer);
                }

                Ok(Entity::new(entity))
            })?,
        )?;
        table.set(
            "get_by_name",
            lua.create_function(|lua, name: String| {
                let transform_mgr = use_context().transform_mgr();
                Ok(transform_mgr.find_by_name(name).map(|indices| {
                    indices
                        .iter()
                        .map(|index| Entity::new(transform_mgr.entity(*index)))
                        .collect::<Vec<_>>()
                }))
            })?,
        )?;
        Ok(())
    }
}

struct TransformBuildParam {
    pub parent: Option<Transform>,
    pub position: Option<Vec2>,
    pub scale: Option<Vec2>,
    pub angle: Option<f32>,
}

impl<'lua> FromLua<'lua> for TransformBuildParam {
    #[allow(unused_variables)]
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        let table = match value {
            LuaValue::Table(table) => table,
            _ => {
                return Err(
                    format!("the type {} must be a {}", "TransformBuildParam", "table")
                        .to_lua_err(),
                );
            }
        };

        Ok(Self {
            parent: if table.contains_key("parent")? {
                Some(table.get("parent")?)
            } else {
                None
            },
            position: if table.contains_key("position")? {
                Some(table.get("position")?)
            } else {
                None
            },
            scale: if table.contains_key("scale")? {
                Some(table.get("scale")?)
            } else {
                None
            },
            angle: if table.contains_key("angle")? {
                Some(table.get("angle")?)
            } else {
                None
            },
        })
    }
}

struct UIElementBuildParam {
    pub is_interactible: Option<bool>,
    pub order_index: u32,
}

impl<'lua> FromLua<'lua> for UIElementBuildParam {
    #[allow(unused_variables)]
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        let table = match value {
            LuaValue::Table(table) => table,
            _ => {
                return Err(
                    format!("the type {} must be a {}", "UIElementBuildParam", "table")
                        .to_lua_err(),
                );
            }
        };

        Ok(Self {
            is_interactible: if table.contains_key("is_interactible")? {
                Some(table.get("is_interactible")?)
            } else {
                None
            },
            order_index: table.get("order_index")?,
        })
    }
}

struct CameraBuildParam {
    pub layer: Option<crate::render::Layer>,
    pub order: Option<isize>,
}

impl<'lua> FromLua<'lua> for CameraBuildParam {
    #[allow(unused_variables)]
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        let table = match value {
            LuaValue::Table(table) => table,
            _ => {
                return Err(
                    format!("the type {} must be a {}", "CameraBuildParam", "table").to_lua_err(),
                );
            }
        };

        Ok(Self {
            layer: if table.contains_key("layer")? {
                Some(table.get("layer")?)
            } else {
                None
            },
            order: if table.contains_key("order")? {
                Some(table.get("order")?)
            } else {
                None
            },
        })
    }
}

struct GlyphRendererBuildParam {
    pub layer: Option<crate::render::Layer>,
    pub order: Option<isize>,
    pub color: Option<Color>,
    pub shader: LuaRcShader,
    pub font: LuaRcFont,
    pub font_size: f32,
    pub config: Option<GlyphRendererConfig>,
    pub text: Option<String>,
}

impl<'lua> FromLua<'lua> for GlyphRendererBuildParam {
    #[allow(unused_variables)]
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        let table = match value {
            LuaValue::Table(table) => table,
            _ => {
                return Err(format!(
                    "the type {} must be a {}",
                    "GlyphRendererBuildParam", "table"
                )
                .to_lua_err());
            }
        };

        Ok(Self {
            layer: if table.contains_key("layer")? {
                Some(table.get("layer")?)
            } else {
                None
            },
            order: if table.contains_key("order")? {
                Some(table.get("order")?)
            } else {
                None
            },
            color: if table.contains_key("color")? {
                Some(table.get("color")?)
            } else {
                None
            },
            shader: table.get("shader")?,
            font: table.get("font")?,
            font_size: table.get("font_size")?,
            config: table.get("config")?,
            text: if table.contains_key("text")? {
                Some(table.get("text")?)
            } else {
                None
            },
        })
    }
}

struct SpriteRendererBuildParam {
    pub layer: Option<crate::render::Layer>,
    pub order: Option<isize>,
    pub color: Option<Color>,
    pub shader: LuaRcShader,
    pub sprite: LuaRcSprite,
}

impl<'lua> FromLua<'lua> for SpriteRendererBuildParam {
    #[allow(unused_variables)]
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        let table = match value {
            LuaValue::Table(table) => table,
            _ => {
                return Err(format!(
                    "the type {} must be a {}",
                    "SpriteRendererBuildParam", "table"
                )
                .to_lua_err());
            }
        };

        Ok(Self {
            layer: if table.contains_key("layer")? {
                Some(table.get("layer")?)
            } else {
                None
            },
            order: if table.contains_key("order")? {
                Some(table.get("order")?)
            } else {
                None
            },
            color: if table.contains_key("color")? {
                Some(table.get("color")?)
            } else {
                None
            },
            shader: table.get("shader")?,
            sprite: table.get("sprite")?,
        })
    }
}

struct NinePatchRendererBuildParam {
    pub layer: Option<crate::render::Layer>,
    pub order: Option<isize>,
    pub color: Option<Color>,
    pub shader: LuaRcShader,
    pub nine_patch: LuaRcSpriteNinePatch,
    pub width: f32,
    pub height: f32,
}

impl<'lua> FromLua<'lua> for NinePatchRendererBuildParam {
    #[allow(unused_variables)]
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        let table = match value {
            LuaValue::Table(table) => table,
            _ => {
                return Err(format!(
                    "the type {} must be a {}",
                    "NinePatchRendererBuildParam", "table"
                )
                .to_lua_err());
            }
        };

        Ok(Self {
            layer: if table.contains_key("layer")? {
                Some(table.get("layer")?)
            } else {
                None
            },
            order: if table.contains_key("order")? {
                Some(table.get("order")?)
            } else {
                None
            },
            color: if table.contains_key("color")? {
                Some(table.get("color")?)
            } else {
                None
            },
            shader: table.get("shader")?,
            nine_patch: table.get("nine_patch")?,
            width: table.get("width")?,
            height: table.get("height")?,
        })
    }
}

struct TilemapRendererBuildParam {
    pub layer: Option<crate::render::Layer>,
    pub order: Option<isize>,
    pub color: Option<Color>,
    pub shader: LuaRcShader,
    pub tilemap: LuaRcTilemap,
}

impl<'lua> FromLua<'lua> for TilemapRendererBuildParam {
    #[allow(unused_variables)]
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        let table = match value {
            LuaValue::Table(table) => table,
            _ => {
                return Err(format!(
                    "the type {} must be a {}",
                    "TilemapRendererBuildParam", "table"
                )
                .to_lua_err());
            }
        };

        Ok(Self {
            layer: if table.contains_key("layer")? {
                Some(table.get("layer")?)
            } else {
                None
            },
            order: if table.contains_key("order")? {
                Some(table.get("order")?)
            } else {
                None
            },
            color: if table.contains_key("color")? {
                Some(table.get("color")?)
            } else {
                None
            },
            shader: table.get("shader")?,
            tilemap: table.get("tilemap")?,
        })
    }
}

struct EntityBuildParam {
    name: Option<String>,
    transform: Option<TransformBuildParam>,
    ui_element: Option<UIElementBuildParam>,
    camera: Option<CameraBuildParam>,
    glyph_renderer: Option<GlyphRendererBuildParam>,
    sprite_renderer: Option<SpriteRendererBuildParam>,
    nine_patch_renderer: Option<NinePatchRendererBuildParam>,
    tilemap_renderer: Option<TilemapRendererBuildParam>,
}

impl<'lua> FromLua<'lua> for EntityBuildParam {
    #[allow(unused_variables)]
    fn from_lua(value: LuaValue<'lua>, lua: &'lua Lua) -> LuaResult<Self> {
        let table = match value {
            LuaValue::Table(table) => table,
            _ => {
                return Err(
                    format!("the type {} must be a {}", "EntityBuildParam", "table").to_lua_err(),
                );
            }
        };

        Ok(Self {
            name: if table.contains_key("name")? {
                Some(table.get("name")?)
            } else {
                None
            },
            ui_element: if table.contains_key("ui_element")? {
                Some(table.get("ui_element")?)
            } else {
                None
            },
            transform: if table.contains_key("transform")? {
                Some(table.get("transform")?)
            } else {
                None
            },
            camera: if table.contains_key("camera")? {
                Some(table.get("camera")?)
            } else {
                None
            },
            glyph_renderer: if table.contains_key("glyph_renderer")? {
                Some(table.get("glyph_renderer")?)
            } else {
                None
            },
            sprite_renderer: if table.contains_key("sprite_renderer")? {
                Some(table.get("sprite_renderer")?)
            } else {
                None
            },
            nine_patch_renderer: if table.contains_key("nine_patch_renderer")? {
                Some(table.get("nine_patch_renderer")?)
            } else {
                None
            },
            tilemap_renderer: if table.contains_key("tilemap_renderer")? {
                Some(table.get("tilemap_renderer")?)
            } else {
                None
            },
        })
    }
}
