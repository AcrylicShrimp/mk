use mlua::prelude::*;

#[derive(Debug)]
pub struct RegistryKey {
    key: Box<LuaRegistryKey>,
}

impl RegistryKey {
    pub fn wrap(key: LuaRegistryKey) -> (usize, Self) {
        let ptr = Box::into_raw(Box::new(key));
        let hash = ptr as usize;

        (
            hash,
            Self {
                key: unsafe { Box::from_raw(ptr) },
            },
        )
    }

    pub fn key(&self) -> &LuaRegistryKey {
        &self.key
    }
}
