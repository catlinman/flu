
use LuaContext;
use LuaRef;
use ffi;
use nil;

use std::ffi::CString;

pub trait Push {
    fn push(&self, cxt: &LuaContext);
}

impl Push for nil {
    fn push(&self, cxt: &LuaContext) {
        unsafe {
            ffi::lua_pushnil(cxt.handle)
        }
    }
}

impl Push for bool {
    fn push(&self, cxt: &LuaContext) {
        unsafe {
            ffi::lua_pushboolean(cxt.handle, *self as i32)
        }
    }
}

macro_rules! integer_push {
    ($ty:ident) => (
        impl Push for $ty {
            fn push(&self, cxt: &LuaContext) {
                unsafe { ffi::lua_pushinteger(cxt.handle, *self as ffi::lua_Integer) }
            }
        }
    )
}

integer_push!(i8);
integer_push!(i16);
integer_push!(i32);

integer_push!(u8);
integer_push!(u16);
integer_push!(u32);

macro_rules! number_push {
    ($ty:ident) => (
        impl Push for $ty {
            fn push(&self, cxt: &LuaContext) {
                unsafe { ffi::lua_pushnumber(cxt.handle, *self as ffi::lua_Number) }
            }
        }
    )
}

number_push!(f32);
number_push!(f64);

impl<'a> Push for &'a str {
    fn push(&self, cxt: &LuaContext) {
        unsafe {
            ffi::lua_pushlstring(cxt.handle, self.as_ptr() as *const i8, self.len());
        }
    }
}

impl Push for String {
    fn push(&self, cxt: &LuaContext) {
        let value = CString::new(&self[..]).unwrap();
        unsafe {
            ffi::lua_pushlstring(cxt.handle, value.as_ptr(), self.len())
        };
    }
}

impl<T> Push for Option<T> where T: Push {
    fn push(&self, cxt: &LuaContext) {
        match self {
            &Some(ref p) => {
                p.push(cxt)
            }
            &None => {
                unsafe {
                    ffi::lua_pushnil(cxt.handle)
                }
            }
        }
    }
}

macro_rules! tuple_push {
    ($($name:ident)+) => (
        impl<$($name: Push),*> Push for ($($name,)*) {
            fn push(&self, cxt: &LuaContext) {
                #![allow(non_snake_case)]
                let &($(ref $name,)*) = self;
                $($name.push(cxt);)*
            }
        }
    );
}

tuple_push!(A);
tuple_push!(A B);
tuple_push!(A B C);
tuple_push!(A B C D);
tuple_push!(A B C D E);
tuple_push!(A B C D E F);
tuple_push!(A B C D E F G);
tuple_push!(A B C D E F G H);
tuple_push!(A B C D E F G H I);
tuple_push!(A B C D E F G H I J);
tuple_push!(A B C D E F G H I J K);
tuple_push!(A B C D E F G H I J K L);

