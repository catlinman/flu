
use Context;
use LuaRef;
use ffi;
use nil;

use std::ffi::CString;

pub trait Push {
    fn push(&self, ctx: &Context);
}

impl Push for () {
    fn push(&self, _: &Context) {
    }
}

impl Push for nil {
    fn push(&self, ctx: &Context) {
        unsafe {
            ffi::lua_pushnil(ctx.handle)
        }
    }
}

impl Push for bool {
    fn push(&self, ctx: &Context) {
        unsafe {
            ffi::lua_pushboolean(ctx.handle, *self as i32)
        }
    }
}

macro_rules! integer_push {
    ($ty:ident) => (
        impl Push for $ty {
            fn push(&self, ctx: &Context) {
                unsafe { ffi::lua_pushinteger(ctx.handle, *self as ffi::lua_Integer) }
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
            fn push(&self, ctx: &Context) {
                unsafe { ffi::lua_pushnumber(ctx.handle, *self as ffi::lua_Number) }
            }
        }
    )
}

number_push!(f32);
number_push!(f64);

impl<'a> Push for &'a str {
    fn push(&self, ctx: &Context) {
        unsafe {
            ffi::lua_pushlstring(ctx.handle, self.as_ptr() as *const i8, self.len());
        }
    }
}

impl Push for String {
    fn push(&self, ctx: &Context) {
        let value = CString::new(&self[..]).unwrap();
        unsafe {
            ffi::lua_pushlstring(ctx.handle, value.as_ptr(), self.len())
        };
    }
}

impl<T> Push for Option<T> where T: Push {
    fn push(&self, ctx: &Context) {
        match self {
            &Some(ref p) => {
                p.push(ctx)
            }
            &None => {
                unsafe {
                    ffi::lua_pushnil(ctx.handle)
                }
            }
        }
    }
}

macro_rules! tuple_push {
    ($($name:ident)+) => (
        impl<$($name: Push),*> Push for ($($name,)*) {
            fn push(&self, ctx: &Context) {
                #![allow(non_snake_case)]
                let &($(ref $name,)*) = self;
                $($name.push(ctx);)*
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

