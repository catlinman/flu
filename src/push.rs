use ffi;
use LuaContext;
use nil;

use std::ffi::CString;

pub trait Push {
    fn push(self, cxt: &mut LuaContext);
}

impl Push for nil {
    fn push(self, cxt: &mut LuaContext) {
        unsafe { ffi::lua_pushnil(cxt.handle) }
    }
}

impl Push for bool {
    fn push(self, cxt: &mut LuaContext) {
        unsafe { ffi::lua_pushboolean(cxt.handle, self as i32) }
    }
}

macro_rules! integer_push {
    ($ty:ident) => (
        impl Push for $ty {
            fn push(self, cxt: &mut LuaContext) {
                unsafe { ffi::lua_pushinteger(cxt.handle, self as ffi::lua_Integer) }
            }
        }
    )
}

integer_push!(i8);
integer_push!(i16);
integer_push!(i32);

macro_rules! number_push {
    ($ty:ident) => (
        impl Push for $ty {
            fn push(self, cxt: &mut LuaContext) {
                unsafe { ffi::lua_pushnumber(cxt.handle, self as ffi::lua_Number) }
            }
        }
    )
}

number_push!(f32);
number_push!(f64);

impl Push for &'static str {
    fn push(self, cxt: &mut LuaContext) {
        unsafe { ffi::lua_pushliteral(cxt.handle, self) }
    }
}

impl Push for String {
    fn push(self, cxt: &mut LuaContext) {
        let value = CString::new(&self[..]).unwrap();
        unsafe { ffi::lua_pushlstring(cxt.handle, value.as_ptr(), self.len() as u64 + 1) };
    }
}

impl<T> Push for Option<T> where T: Push {
    fn push(self, cxt: &mut LuaContext) {
        match self {
            Some(p) => { p.push(cxt) },
            None => {
                unsafe { ffi::lua_pushnil(cxt.handle) }
            }
        }
    }
}
