
use Context;
use ffi;
use std::ffi::CString;

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
            ffi::lua_getfield(cxt.handle, idx, unsafe { CString::new(*self).unwrap().as_ptr() as _ })
        }
    }

    fn set(&self, cxt: &Context, idx: i32) {
        unsafe {
            ffi::lua_setfield(cxt.handle, idx, unsafe { CString::new(*self).unwrap().as_ptr() as _ })
        }
    }
}
