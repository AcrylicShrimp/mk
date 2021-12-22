use crate::api::use_context;
use codegen::LuaComponentNoWrapper;
use mlua::prelude::*;
use std::marker::PhantomData;

#[derive(LuaComponentNoWrapper, Debug, Clone, Copy, PartialEq)]
pub struct Size {
    #[lua_hidden]
    index: u32,
    #[lua_readonly]
    #[lua_userfunc(get=lua_get_width)]
    pub width: f32,
    #[lua_readonly]
    #[lua_userfunc(get=lua_get_height)]
    pub height: f32,
    #[lua_readonly]
    #[lua_userfunc(get=lua_get_local_width)]
    local_width: PhantomData<f32>,
    #[lua_readonly]
    #[lua_userfunc(get=lua_get_local_height)]
    local_height: PhantomData<f32>,
}

impl Size {
    pub fn new(index: u32) -> Self {
        Self {
            index,
            width: 0f32,
            height: 0f32,
            local_width: PhantomData,
            local_height: PhantomData,
        }
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    fn lua_get_width<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        (self.width
            * crate::transform::Transform::world_scale(self.index, &use_context().transform_mgr())
                .x)
            .to_lua(lua)
    }

    fn lua_get_height<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        (self.height
            * crate::transform::Transform::world_scale(self.index, &use_context().transform_mgr())
                .y)
            .to_lua(lua)
    }

    fn lua_get_local_width<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        self.width.to_lua(lua)
    }

    fn lua_get_local_height<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        self.height.to_lua(lua)
    }
}
