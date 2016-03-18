
use LuaValue;
use LuaRef;
use nil;

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

type_size!(nil, 1);
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

macro_rules! tuple_size {
    ($($name:ident)+) => (
        impl<$($name: Size),*> Size for ($($name,)*) {
            fn size() -> i32 {
                0 $(+ $name::size())*
            }
        }
    );
}

tuple_size!(A);
tuple_size!(A B);
tuple_size!(A B C);
tuple_size!(A B C D);
tuple_size!(A B C D E);
tuple_size!(A B C D E F);
tuple_size!(A B C D E F G);
tuple_size!(A B C D E F G H);
tuple_size!(A B C D E F G H I);
tuple_size!(A B C D E F G H I J);
tuple_size!(A B C D E F G H I J K);
tuple_size!(A B C D E F G H I J K L);

