extern crate libc;

pub mod ffi;
pub mod push;
pub mod read;
pub mod size;

mod context;
mod table;
mod value;
mod borrow;

pub use context::LuaContext;
pub use table::Table;
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
