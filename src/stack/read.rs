use Context;
use LuaValue;
use LuaRef;
use ffi;
use nil;

use std::slice;
use std::str;
use std::mem;

pub trait Read<'a> {
    fn read(ctx: &'a Context, idx: i32) -> Self;
    fn check(ctx: &'a Context, idx: i32) -> bool;
}

impl<'a> Read<'a> for bool {
    fn read(ctx: &'a Context, idx: i32) -> Self {
        unsafe {
            ffi::lua_toboolean(ctx.handle, idx) > 0
        }
    }

    fn check(ctx: &'a Context, idx: i32) -> bool {
        unsafe {
            ffi::lua_isboolean(ctx.handle, idx)
        }
    }
}

macro_rules! integer_read {
    ($ty:ident) => (
        impl<'a> Read<'a> for $ty {
            fn read(ctx: &'a Context, idx: i32) -> Self {
                unsafe { ffi::lua_tointeger(ctx.handle, idx) as Self }
            }

            fn check(ctx: &'a Context, idx: i32) -> bool {
                unsafe { ffi::lua_isnumber(ctx.handle, idx) > 0 }
            }
        }
    )
}

integer_read!(i8);
integer_read!(i16);
integer_read!(i32);

macro_rules! number_read {
    ($ty:ident) => (
        impl<'a> Read<'a> for $ty {
            fn read(ctx: &'a Context, idx: i32) -> Self {
                unsafe { ffi::lua_tonumber(ctx.handle, idx) as Self }
            }

            fn check(ctx: &'a Context, idx: i32) -> bool {
                unsafe { ffi::lua_isnumber(ctx.handle, idx) > 0 }
            }
        }
    )
}

number_read!(f32);
number_read!(f64);

impl<'a, 'b> Read<'a> for &'b str {
    fn read(ctx: &'a Context, idx: i32) -> Self {
        unsafe {
            let slice = {
                let mut size = 0;
                let cs = ffi::lua_tolstring(ctx.handle, idx, &mut size);
                slice::from_raw_parts(cs, size as usize)
            };
            str::from_utf8(mem::transmute(slice)).unwrap()
        }
    }

    fn check(ctx: &'a Context, idx: i32) -> bool {
        unsafe {
            ffi::lua_isstring(ctx.handle, idx) > 0
        }
    }
}

impl<'a> Read<'a> for String {
    fn read(ctx: &'a Context, idx: i32) -> Self {
        unsafe {
            let slice = {
                let mut size = 0;
                let cs = ffi::lua_tolstring(ctx.handle, idx, &mut size);
                slice::from_raw_parts(cs, size as usize)
            };
            String::from_utf8_lossy(mem::transmute(slice)).into_owned()
        }
    }

    fn check(ctx: &'a Context, idx: i32) -> bool {
        unsafe {
            ffi::lua_isstring(ctx.handle, idx) > 0
        }
    }
}

impl<'a, T> Read<'a> for Option<T> where T: Read<'a> {
    fn read(ctx: &'a Context, idx: i32) -> Self {
        unsafe {
            match ffi::lua_isnil(ctx.handle, idx) {
                false => Some(ctx.peek::<T>(idx)),
                true => None,
            }
        }
    }

    fn check(ctx: &'a Context, idx: i32) -> bool {
        T::check(ctx, idx) ||
        unsafe {
            ffi::lua_isnil(ctx.handle, idx)
        }
    }
}

/*macro_rules! tuple_read {
    ($($name:ident)+) => (
        impl<'a, $($name: Read<'a>),*> Read<'a> for ($($name,)*) {
            fn read(ctx: &'a Context, idx: i32) -> Self {
                (
                    $(ctx.remove::<$name>(idx),)*
                )
            }

            fn check(ctx: &'a Context, idx: i32) -> bool {
                let mut idx = 0;
                true $(&& $name::check(ctx, { idx += 1; idx }))*
            }
        }
    );
}

tuple_read!(A);
tuple_read!(A B);
tuple_read!(A B C);
tuple_read!(A B C D);
tuple_read!(A B C D E);
tuple_read!(A B C D E F);
tuple_read!(A B C D E F G);
tuple_read!(A B C D E F G H);
tuple_read!(A B C D E F G H I);
tuple_read!(A B C D E F G H I J);
tuple_read!(A B C D E F G H I J K);
tuple_read!(A B C D E F G H I J K L);*/


#[test]
fn read_int() {
    let ctx = Context::new();

    ctx.push(42i8);
    assert_eq!(ctx.pop::<i8>(), 42i8);

    ctx.push(16i16);
    assert_eq!(ctx.pop::<i16>(), 16i16);

    ctx.push(101i32);
    assert_eq!(ctx.pop::<i32>(), 101i32);
}

#[test]
fn read_num() {
    let ctx = Context::new();

    ctx.push(42f64);
    assert_eq!(ctx.pop::<f64>(), 42f64);

    ctx.push(16f32);
    assert_eq!(ctx.pop::<f32>(), 16f32);

    ctx.push(101f32);
    assert_eq!(ctx.pop::<f64>(), 101f64);

    ctx.push(99f64);
    assert_eq!(ctx.pop::<f32>(), 99f32);
}

#[test]
fn read_string() {
    let ctx = Context::new();

    ctx.push(("Hello world!", "Hello rust!".to_string()));

    assert_eq!(ctx.pop::<String>(), "Hello rust!");
    assert_eq!(ctx.pop::<&str>(), "Hello world!");
}

#[test]
fn read_optional() {
    let ctx = Context::new();

    ctx.push(("Hello world!", nil));

    assert_eq!(ctx.pop::<Option<String>>(), None);
    assert_eq!(ctx.pop::<Option<&str>>(), Some("Hello world!"));

    /*push!(&ctx, flu::nil, 5f64, flu::nil);
    assert_eq!(ctx.pop::<(Option<f64>, Option<f64>, Option<f64>)>(), (None, Some(5f64), None));*/
}
