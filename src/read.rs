use LuaContext;
use LuaValue;
use LuaRef;
use ffi;
use nil;

use std::slice;
use std::str;
use std::mem;

pub trait Read<'a> {
    fn read(cxt: &'a LuaContext, idx: i32) -> Self;
    fn check(cxt: &'a LuaContext, idx: i32) -> bool;
}

impl<'a> Read<'a> for bool {
    fn read(cxt: &'a LuaContext, idx: i32) -> Self {
        unsafe { ffi::lua_toboolean(cxt.handle, idx) > 0 }
    }

    fn check(cxt: &'a LuaContext, idx: i32) -> bool {
        unsafe { ffi::lua_isboolean(cxt.handle, idx) }
    }
}

macro_rules! integer_read {
    ($ty:ident) => (
        impl<'a> Read<'a> for $ty {
            fn read(cxt: &'a LuaContext, idx: i32) -> Self {
                unsafe { ffi::lua_tointeger(cxt.handle, idx) as Self }
            }

            fn check(cxt: &'a LuaContext, idx: i32) -> bool {
                unsafe { ffi::lua_isnumber(cxt.handle, idx) > 0 }
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
            fn read(cxt: &'a LuaContext, idx: i32) -> Self {
                unsafe { ffi::lua_tonumber(cxt.handle, idx) as Self }
            }

            fn check(cxt: &'a LuaContext, idx: i32) -> bool {
                unsafe { ffi::lua_isnumber(cxt.handle, idx) > 0 }
            }
        }
    )
}

number_read!(f32);
number_read!(f64);

impl<'a, 'b> Read<'a> for &'b str {
    fn read(cxt: &'a LuaContext, idx: i32) -> Self {
        unsafe {
            let slice = {
                let mut size = 0;
                let cs = ffi::lua_tolstring(cxt.handle, idx, &mut size);
                slice::from_raw_parts(cs, size as usize)
            };
            str::from_utf8(mem::transmute(slice)).unwrap()
        }
    }

    fn check(cxt: &'a LuaContext, idx: i32) -> bool {
        unsafe { ffi::lua_isstring(cxt.handle, idx) > 0}
    }
}

impl<'a> Read<'a> for String {
    fn read(cxt: &'a LuaContext, idx: i32) -> Self {
        unsafe {
            let slice = {
                let mut size = 0;
                let cs = ffi::lua_tolstring(cxt.handle, idx, &mut size);
                slice::from_raw_parts(cs, size as usize)
            };
            String::from_utf8_lossy(mem::transmute(slice)).into_owned()
        }
    }

    fn check(cxt: &'a LuaContext, idx: i32) -> bool {
        unsafe { ffi::lua_isstring(cxt.handle, idx) > 0 }
    }
}

impl<'a, T> Read<'a> for Option<T> where T: Read<'a> {
    fn read(cxt: &'a LuaContext, idx: i32) -> Self {
        unsafe {
            match ffi::lua_isnil(cxt.handle, idx) {
                false => Some(cxt.peek::<T>(-1)),
                true  => None,
            }
        }
    }

    fn check(cxt: &'a LuaContext, idx: i32) -> bool {
        T::check(cxt, idx) || unsafe { ffi::lua_isnil(cxt.handle, idx) }
    }
}

/*macro_rules! tuple_read {
    ($($name:ident)+) => (
        impl<'a, $($name: Read<'a>),*> Read<'a> for ($($name,)*) {
            fn read(cxt: &'a LuaContext, idx: i32) -> Self {
                (
                    $(cxt.remove::<$name>(idx),)*
                )
            }

            fn check(cxt: &'a LuaContext, idx: i32) -> bool {
                let mut idx = 0;
                true $(&& $name::check(cxt, { idx += 1; idx }))*
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
tuple_read!(A B C D E F G H I J K L); */


#[test]
fn flu_stack_read_int() {
    let cxt = LuaContext::new();

    cxt.push(42i8);
    assert_eq!(cxt.pop::<i8>(), 42i8);

    cxt.push(16i16);
    assert_eq!(cxt.pop::<i16>(), 16i16);

    cxt.push(101i32);
    assert_eq!(cxt.pop::<i32>(), 101i32);
}

#[test]
fn flu_stack_read_num() {
    let cxt = LuaContext::new();

    cxt.push(42f64);
    assert_eq!(cxt.pop::<f64>(), 42f64);

    cxt.push(16f32);
    assert_eq!(cxt.pop::<f32>(), 16f32);

    cxt.push(101f32);
    assert_eq!(cxt.pop::<f64>(), 101f64);

    cxt.push(99f64);
    assert_eq!(cxt.pop::<f32>(), 99f32);
}

#[test]
fn flu_stack_read_string() {
    let cxt = LuaContext::new();

    cxt.push(("Hello world!", "Hello rust!".to_string()));

    assert_eq!(cxt.pop::<String>(), "Hello rust!");
    assert_eq!(cxt.pop::<&str>(), "Hello world!");
}

#[test]
fn flu_stack_read_optional() {
    let cxt = LuaContext::new();

    cxt.push(("Hello world!", nil));

    assert_eq!(cxt.pop::<Option<String>>(), None);
    assert_eq!(cxt.pop::<Option<&str>>(), Some("Hello world!"));

    /*push!(&cxt, flu::nil, 5f64, flu::nil);
    assert_eq!(cxt.pop::<(Option<f64>, Option<f64>, Option<f64>)>(), (None, Some(5f64), None));*/
}
