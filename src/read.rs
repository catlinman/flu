use ffi;
use LuaContext;

use std::ffi::CString;
use std::slice;
use std::str;
use std::mem;

pub trait Read {
    fn read(cxt: &mut LuaContext, idx: i32) -> Self;
}

macro_rules! integer_read {
    ($ty:ident) => (
        impl Read for $ty {
            fn read(cxt: &mut LuaContext, idx: i32) -> Self {
                unsafe { ffi::lua_tointeger(cxt.handle, idx) as Self }
            }
        }
    )
}

integer_read!(i8);
integer_read!(i16);
integer_read!(i32);

macro_rules! number_read {
    ($ty:ident) => (
        impl Read for $ty {
            fn read(cxt: &mut LuaContext, idx: i32) -> Self {
                unsafe { ffi::lua_tonumber(cxt.handle, idx) as Self }
            }
        }
    )
}

number_read!(f32);
number_read!(f64);

impl<'a> Read for &'a str {
    fn read(cxt: &mut LuaContext, idx: i32) -> Self {
        unsafe {
            let slice = {
                let mut size = 0;
                let cs = ffi::lua_tolstring(cxt.handle, idx, &mut size);
                slice::from_raw_parts(cs, size as usize)
            };
            str::from_utf8(mem::transmute(slice)).unwrap()
        }
    }
}

impl Read for String {
    fn read(cxt: &mut LuaContext, idx: i32) -> Self {
        unsafe {
            let slice = {
                let mut size = 0;
                let cs = ffi::lua_tolstring(cxt.handle, idx, &mut size);
                slice::from_raw_parts(cs, size as usize - 1)
            };
            String::from_utf8_lossy(mem::transmute(slice)).into_owned()
        }
    }
}

impl<T> Read for Option<T> where T: Read {
    fn read(cxt: &mut LuaContext, idx: i32) -> Self {
        unsafe {
            match ffi::lua_isnil(cxt.handle, idx) {
                false => Some(T::read(cxt, idx)),
                true  => None,
            }
        }
    }
}

/*impl<T> Read for (T) where T: Read {
    fn read(cxt: &mut LuaContext, idx: i32) -> Self {
        unsafe {
            
        }
    }
}*/
