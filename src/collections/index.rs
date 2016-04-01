
use Context;
use ffi;

pub trait LuaIndex {
    fn get(&self, cxt: &Context, idx: i32);
    fn set(&self, cxt: &Context, idx: i32);
}

macro_rules! integer_index {
    ($ty:ident) => (
        impl LuaIndex for $ty {
            fn get(&self, cxt: &Context, idx: i32) {
                unsafe { ffi::lua_rawgeti(cxt.handle, idx, *self as i32) }
            }

            fn set(&self, cxt: &Context, idx: i32) {
                unsafe { ffi::lua_rawseti(cxt.handle, idx, *self as i32) }
            }
        }
    )
}

integer_index!(i32);
integer_index!(usize);

impl<'a, 'b> LuaIndex for &'b str {
    fn get(&self, cxt: &Context, idx: i32) {
        unsafe {
            ffi::lua_getfield(cxt.handle, idx, self.as_ptr() as *const i8)
        }
    }

    fn set(&self, cxt: &Context, idx: i32) {
        unsafe {
            ffi::lua_setfield(cxt.handle, idx, self.as_ptr() as *const i8)
        }
    }
}
