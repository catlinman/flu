
use Context;
use ffi;
use std::ffi::CString;

pub trait LuaIndex {
    fn get(&self, ctx: &Context, idx: i32);
    fn set(&self, ctx: &Context, idx: i32);
}

macro_rules! integer_index {
    ($ty:ident) => (
        impl LuaIndex for $ty {
            fn get(&self, ctx: &Context, idx: i32) {
                unsafe { ffi::lua_rawgeti(ctx.handle, idx, *self as i32) }
            }

            fn set(&self, ctx: &Context, idx: i32) {
                unsafe { ffi::lua_rawseti(ctx.handle, idx, *self as i32) }
            }
        }
    )
}

integer_index!(i32);
integer_index!(usize);

impl<'a, 'b> LuaIndex for &'b str {
    fn get(&self, ctx: &Context, idx: i32) {
        unsafe {
            ffi::lua_getfield(ctx.handle, idx, unsafe { CString::new(*self).unwrap().as_ptr() as _ })
        }
    }

    fn set(&self, ctx: &Context, idx: i32) {
        unsafe {
            ffi::lua_setfield(ctx.handle, idx, unsafe { CString::new(*self).unwrap().as_ptr() as _ })
        }
    }
}
