use Context;
use ffi;

use stack::Read;
use stack::Push;
use stack::Size;

#[derive(Debug, PartialEq, Eq)]
pub struct LuaRef<'a> {
    cxt: &'a Context,
    key: i32,
}

impl<'a> Drop for LuaRef<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::luaL_unref(self.cxt.handle, ffi::LUA_REGISTRYINDEX, self.key)
        }
    }
}

impl<'a> Read<'a> for LuaRef<'a> {
    fn read(cxt: &'a Context, idx: i32) -> Self {
        unsafe {
            let t = ffi::LUA_REGISTRYINDEX;

            match ffi::lua_isnil(cxt.handle, idx) {
                true => {
                    cxt.pop_discard(-1);
                    LuaRef { cxt: cxt, key: -1 }
                }
                false => {
                    ffi::lua_rawgeti(cxt.handle, t, ffi::FREELIST_REF);
                    let r = match cxt.pop::<i32>() {
                        0 => {
                            ffi::lua_objlen(cxt.handle, t) as i32 + 1
                        }
                        a @ _ => {
                            ffi::lua_rawgeti(cxt.handle, t, a);
                            ffi::lua_rawseti(cxt.handle, t, ffi::FREELIST_REF);
                            a
                        }
                    };
                    ffi::lua_rawseti(cxt.handle, t, r);

                    LuaRef { cxt: cxt, key: r }
                }
            }
        }
    }

    fn check(cxt: &'a Context, idx: i32) -> bool {
        true
    }
}

impl<'a> Push for LuaRef<'a> {
    fn push(&self, cxt: &Context) {
        unsafe {
            ffi::lua_rawgeti(cxt.handle, ffi::LUA_REGISTRYINDEX, self.key)
        }
    }
}

impl<'a> Size for LuaRef<'a> {
    fn size() -> i32 {
        0
    }
}

#[test]
fn read_ref() {
    let cxt = Context::new();

    cxt.push("Hello world!");

    {
        assert_eq!(cxt.size(), 1);

        let r = cxt.pop::<LuaRef>();

        assert_eq!(cxt.size(), 0);

        cxt.push(r);

        assert_eq!(cxt.pop::<&str>(), "Hello world!");
    }
}
