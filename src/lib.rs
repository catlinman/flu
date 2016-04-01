#![feature(unboxed_closures)]

extern crate libc;

macro_rules! assert_enum {
    (@as_expr $e:expr) => {$e};
    (@as_pat $p:pat) => {$p};
    ($left:expr, $($right:tt)*) => (
        {
            match &($left) {
                assert_enum!(@as_pat &$($right)*(..)) => {},
                _ => {
                    panic!("assertion failed: `(if let left = right(..))` \
                           (left: `{:?}`, right: `{:?}`)",
                           $left,
                           stringify!($($right)*)
                           )
                }
            }
        }
    )
}

pub mod ffi;
pub mod stack;

pub mod collections;

mod context;
mod value;
mod borrow;
mod function;

pub use context::*;
pub use collections::*;
pub use value::*;
pub use borrow::*;
pub use function::*;

pub struct nil;

#[macro_export]
macro_rules! push {
    ($cxt:expr, $($arg:expr),*) => (
        $(
            $cxt.push($arg);
        )*
    )
}
