use crate::api::use_context;
use crate::structure::Vec2;
use codegen::LuaComponentNoWrapper;
use mlua::prelude::*;
use std::marker::PhantomData;

#[derive(LuaComponentNoWrapper, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UIElement {
    #[lua_hidden]
    index: u32,
    #[lua_readonly]
    #[lua_userfunc(get=lua_get_size)]
    size: PhantomData<Vec2>,
    #[lua_userfunc(get=lua_get_is_interactible, set=lua_set_is_interactible)]
    is_interactible: PhantomData<bool>,
    #[lua_userfunc(get=lua_get_order_index, set=lua_set_order_index)]
    order_index: PhantomData<u32>,
}

impl UIElement {
    pub fn new(index: u32) -> Self {
        Self {
            index,
            size: PhantomData,
            is_interactible: PhantomData,
            order_index: PhantomData,
        }
    }

    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn with_element<T>(&self, f: impl FnOnce(&crate::ui::UIElement) -> T) -> T {
        let ui_mgr = use_context().ui_mgr();
        f(ui_mgr.element(self.index))
    }

    pub fn with_element_mut<T>(&self, f: impl FnOnce(&mut crate::ui::UIElement) -> T) -> T {
        let mut ui_mgr = use_context().ui_mgr_mut();
        f(ui_mgr.element_mut(self.index))
    }

    fn lua_get_size<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        self.with_element(|e| e.size).to_lua(lua)
    }

    fn lua_get_is_interactible<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        self.with_element(|e| e.is_interactible()).to_lua(lua)
    }

    fn lua_set_is_interactible(&mut self, value: LuaValue, lua: &Lua) -> LuaResult<()> {
        self.with_element_mut(move |e| {
            e.mark_as_dirty();
            e.set_interactible(bool::from_lua(value, lua)?);
            Ok(())
        })
    }

    fn lua_get_order_index<'lua>(&self, lua: &'lua Lua) -> LuaResult<LuaValue<'lua>> {
        self.with_element(|e| e.order_index()).to_lua(lua)
    }

    fn lua_set_order_index(&mut self, value: LuaValue, lua: &Lua) -> LuaResult<()> {
        self.with_element_mut(move |e| {
            e.mark_as_dirty();
            e.set_order_index(u32::from_lua(value, lua)?);
            Ok(())
        })
    }
}
