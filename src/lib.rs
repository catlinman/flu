extern crate libc;

pub mod ffi;
pub mod stack;

pub mod collections;

mod context;
mod value;
mod borrow;

pub use context::LuaContext;
pub use collections::Array;
pub use collections::Table;
pub use value::LuaValue;
pub use borrow::LuaRef;

pub struct nil;

#[macro_export]
macro_rules! push {
    ($cxt:expr, $($arg:expr),*) => (
        $(
            $cxt.push($arg);
        )*
    )
}
