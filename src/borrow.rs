use LuaContext;
use ffi;

use read::Read;
use push::Push;
use size::Size;

#[derive(Debug, PartialEq, Eq)]
pub struct LuaRef<'a> {
    cxt: &'a LuaContext,
    key: i32,
}

impl<'a> Drop for LuaRef<'a> {
    fn drop(&mut self) {
        unsafe { ffi::luaL_unref(self.cxt.handle, ffi::LUA_REGISTRYINDEX, self.key) }
    }
}


impl<'a> Read<'a> for LuaRef<'a> {
    fn read(cxt: &'a LuaContext, idx: i32) -> Self {
        unsafe {
            let t = ffi::LUA_REGISTRYINDEX;

            match ffi::lua_isnil(cxt.handle, idx) {
                true => {
                    cxt.pop_discard(-1);
                    LuaRef {
                        cxt: cxt,
                        key: -1,
                    }
                }
                false => {
                    ffi::lua_rawgeti(cxt.handle, t, ffi::FREELIST_REF);
                    let r = match cxt.pop::<i32>() {
                        0 => {
                            ffi::lua_objlen(cxt.handle, t) as i32 + 1
                        },
                        a @ _ => {
                            ffi::lua_rawgeti(cxt.handle, t, a);
                            ffi::lua_rawseti(cxt.handle, t, ffi::FREELIST_REF);
                            a
                        }
                    };
                    ffi::lua_rawseti(cxt.handle, t, r);

                    LuaRef {
                        cxt: cxt,
                        key: r,
                    }
                }
            }
        }
    }

    fn check(cxt: &'a LuaContext, idx: i32) -> bool {
        true
    }
}

impl<'a> Push for LuaRef<'a> {
    fn push(&self, cxt: &LuaContext) {
        unsafe { ffi::lua_rawgeti(cxt.handle, ffi::LUA_REGISTRYINDEX, self.key) }
    }
}

impl<'a> Size for LuaRef<'a> {
    fn size(&self) -> i32 {
        0
    }
}