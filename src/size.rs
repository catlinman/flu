use LuaContext;
use LuaRef;

use std::str;

pub trait Size {
    fn size() -> i32;
}

macro_rules! type_size {
    ($ty:ident, $sz:expr) => (
        impl Size for $ty {
            fn size() -> i32 {
                $sz
            }
        }
    )
}

type_size!(bool, 1);

type_size!(i8, 1);
type_size!(i16, 1);
type_size!(i32, 1);

type_size!(f32, 1);
type_size!(f64, 1);

type_size!(String, 1);

impl<'a> Size for &'a str {
    fn size() -> i32 {
        1
    }
}

impl<T> Size for Option<T> where T: Size {
    fn size() -> i32 {
        T::size()
    }
}

impl<'a, T> Size for LuaRef<'a, T> where T: Size {
    fn size() -> i32 {
        0
    }
}
