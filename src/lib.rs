extern crate libc;

pub mod ffi;

pub struct LuaContext {
    handle: *mut ffi::lua_State,
    owner: bool
}

impl LuaContext {
    pub fn new() -> Self {
        LuaContext {
            handle: unsafe { ffi::luaL_newstate() },
            owner: true,
        }
    }

    pub fn from_state(state: *mut ffi::lua_State) -> Self {
        LuaContext {
            handle: state,
            owner: true,
        }
    }

    pub fn from_state_weak(state: *mut ffi::lua_State) -> Self {
        LuaContext {
            handle: state,
            owner: false,
        }
    }

    // TODO: more stuff

}

impl Drop for LuaContext {
    fn drop(&mut self) {
        if self.owner {
            unsafe { ffi::lua_close(self.handle) }
        }
    }
}
