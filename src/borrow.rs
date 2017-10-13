use Context;
use ffi;

use stack::Read;
use stack::Push;
use stack::Size;

#[derive(Debug, PartialEq, Eq)]
pub struct LuaRef<'a> {
    ctx: &'a Context,
    key: i32,
}

impl<'a> Drop for LuaRef<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::luaL_unref(self.ctx.handle, ffi::LUA_REGISTRYINDEX, self.key)
        }
    }
}

impl<'a> Read<'a> for LuaRef<'a> {
    fn read(ctx: &'a Context, idx: i32) -> Self {
        unsafe {
            let t = ffi::LUA_REGISTRYINDEX;

            match ffi::lua_isnil(ctx.handle, idx) {
                true => {
                    ctx.pop_discard(-1);
                    LuaRef { ctx: ctx, key: -1 }
                }
                false => {
                    ffi::lua_rawgeti(ctx.handle, t, ffi::FREELIST_REF);
                    let r = match ctx.pop::<i32>() {
                        0 => {
                            ffi::lua_objlen(ctx.handle, t) as i32 + 1
                        }
                        a @ _ => {
                            ffi::lua_rawgeti(ctx.handle, t, a);
                            ffi::lua_rawseti(ctx.handle, t, ffi::FREELIST_REF);
                            a
                        }
                    };
                    ffi::lua_rawseti(ctx.handle, t, r);

                    LuaRef { ctx: ctx, key: r }
                }
            }
        }
    }

    fn check(ctx: &'a Context, idx: i32) -> bool {
        true
    }
}

impl<'a> Push for LuaRef<'a> {
    fn push(&self, ctx: &Context) {
        unsafe {
            ffi::lua_rawgeti(ctx.handle, ffi::LUA_REGISTRYINDEX, self.key)
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
    let ctx = Context::new();

    ctx.push("Hello world!");

    {
        assert_eq!(ctx.size(), 1);

        let r = ctx.pop::<LuaRef>();

        assert_eq!(ctx.size(), 0);

        ctx.push(r);

        assert_eq!(ctx.pop::<&str>(), "Hello world!");
    }
}
