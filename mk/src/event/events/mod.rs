macro_rules! impl_event_type_lua_api {
    ($name:ident) => {
        impl $name {
            pub fn generate_lua_table<'lua>(
                lua: &'lua mlua::Lua,
            ) -> mlua::Result<(&str, mlua::Table<'lua>)> {
                let table = lua.create_table()?;
                table.set(
                    "eventType",
                    crate::event::EventType::from(std::any::TypeId::of::<Self>()),
                )?;
                table.set(
                    "listen",
                    lua.create_function(|lua, handler: mlua::Function| {
                        Ok(crate::api::use_context()
                            .event_mgr()
                            .dispatcher()
                            .add_listener::<Self>(crate::event::TypedEventListener::LuaFunction(
                                crate::util::BoxId::new(lua.create_registry_value(handler)?),
                            )))
                    })?,
                )?;
                table.set(
                    "unlisten",
                    lua.create_function(|_lua, handler: usize| {
                        Ok(crate::api::use_context()
                            .event_mgr()
                            .dispatcher()
                            .remove_listener::<Self>(handler))
                    })?,
                )?;
                Ok((stringify!($name), table))
            }
        }
    };
}

mod diagnostic;
mod input;
mod lifecycles;

pub use diagnostic::*;
pub use input::*;
pub use lifecycles::*;
