use ffi;
use LuaContext;

use std::ffi::CString;
use std::slice;
use std::str;
use std::mem;

pub trait Read {
    fn read(cxt: &mut LuaContext, idx: i32) -> Self;
    fn size() -> i32;
}

impl Read for bool {
    fn read(cxt: &mut LuaContext, idx: i32) -> Self {
        unsafe { ffi::lua_toboolean(cxt.handle, idx) > 0 }
    }

    fn size() -> i32 {
        1
    }
}

macro_rules! integer_read {
    ($ty:ident) => (
        impl Read for $ty {
            fn read(cxt: &mut LuaContext, idx: i32) -> Self {
                unsafe { ffi::lua_tointeger(cxt.handle, idx) as Self }
            }

            fn size() -> i32 {
                1
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

            fn size() -> i32 {
                1
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

    fn size() -> i32 {
        1
    }
}

impl Read for String {
    fn read(cxt: &mut LuaContext, idx: i32) -> Self {
        unsafe {
            let slice = {
                let mut size = 0;
                let cs = ffi::lua_tolstring(cxt.handle, idx, &mut size);
                slice::from_raw_parts(cs, size as usize)
            };
            String::from_utf8_lossy(mem::transmute(slice)).into_owned()
        }
    }

    fn size() -> i32 {
        1
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

    fn size() -> i32 {
        1
    }
}

macro_rules! count_exprs {
    () => { 0usize };
    ($e:expr) => { 1usize };
    ($e:expr, $($es:expr),+ $(,)*) => { 1usize + count_exprs!($($es),*) };
}

macro_rules! tuple_read_impl {
    (@void $name:ident $expr:expr) => ($expr);
    (@tail $x:ident) => ();
    (@tail $x:ident $($xs:ident)+) => { tuple_read_impl!($($xs)*) };

    ($($name:ident)+) => {
        {
            $name::read(cxt, idx - 0 $(+ tuple_read_impl!(@void $name 1))*)
            //tuple_read_impl!(@tail $($name)+)
        }
    }
}

macro_rules! tuple_read {
    ($($name:ident)+) => (
        impl<$($name: Read),*> Read for ($($name,)*) {
            fn read(cxt: &mut LuaContext, idx: i32) -> Self {
                //panic!("{:?}", count_exprs!($($name),*))
                (
                    $(cxt.pop_bottom::<$name>(),)*
                    //tuple_read_impl!($name);
                )
            }

            fn size() -> i32 {
                0
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
tuple_read!(A B C D E F G H I J K L);


/*impl<T> Read for (T) where T: Read {
    fn read(cxt: &mut LuaContext, idx: i32) -> Self {
        unsafe {
            
        }
    }
}*/
