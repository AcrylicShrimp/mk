mod component;
mod coroutine;
mod event;
mod lua_manager;
mod render;
mod time;

pub use self::render::*;
pub use component::*;
pub use coroutine::*;
pub use event::*;
pub use lua_manager::*;
pub use time::*;

use mlua::prelude::*;

#[macro_export]
macro_rules! arc_user_data {
    ($ty:ty => $name:ident) => {
        #[derive(Debug, Hash)]
        pub struct $name(*const $ty);

        impl $name {
            pub fn into_inner(self) -> std::sync::Arc<$ty> {
                std::sync::Arc::<$ty>::from(self)
            }
        }

        impl Clone for $name {
            fn clone(&self) -> Self {
                unsafe {
                    std::sync::Arc::increment_strong_count(self.0);
                }

                Self(self.0)
            }
        }

        impl Drop for $name {
            fn drop(&mut self) {
                unsafe {
                    std::sync::Arc::decrement_strong_count(self.0);
                }
            }
        }

        impl From<$ty> for $name {
            fn from(value: $ty) -> Self {
                Self(std::sync::Arc::into_raw(std::sync::Arc::new(value)))
            }
        }

        impl From<std::sync::Arc<$ty>> for $name {
            fn from(value: std::sync::Arc<$ty>) -> Self {
                Self(std::sync::Arc::into_raw(value))
            }
        }

        impl From<$name> for std::sync::Arc<$ty> {
            fn from(value: $name) -> Self {
                let arc = unsafe { std::sync::Arc::from_raw(value.0) };
                std::mem::forget(value);
                arc
            }
        }
    };
}

pub fn lua_api(lua: &Lua) -> LuaResult<LuaTable> {
    let table = lua.create_table()?;

    table.set("Color", lua_api_color(lua)?)?;
    // table.set("Coroutine", lua_api_coroutine(lua)?)?;
    table.set("Entity", lua_api_entity(lua)?)?;
    table.set("Event", lua_api_event(lua)?)?;
    table.set("Font", lua_api_font(lua)?)?;
    table.set("Shader", lua_api_shader(lua)?)?;
    table.set("Sprite", lua_api_sprite(lua)?)?;
    table.set("SpriteAnimation", lua_api_sprite_animation(lua)?)?;
    table.set("SpriteAtlas", lua_api_sprite_atlas(lua)?)?;
    table.set("SpriteAtlasGrid", lua_api_sprite_atlas_grid(lua)?)?;
    table.set("Tilemap", lua_api_tilemap(lua)?)?;
    table.set("Time", lua_api_time()?)?;

    Ok(table)
}
