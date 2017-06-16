#![feature(unboxed_closures)]
#![feature(try_trait)]
#![recursion_limit = "1024"]
#![feature(test)]

extern crate test;
#[macro_use]
extern crate error_chain;
extern crate libc;

macro_rules! c_str {
    ($s:expr) => { {
        concat!($s, "\0").as_ptr() as *const i8
    } }
}

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

pub fn pcall_errck<'a>(state: &'a WeakState, ret: libc::c_int) -> Result<()> {
    match ret {
        0 => Ok(()),
        ffi::LUA_ERRRUN => Err(
            ErrorKind::RuntimeError(
                <String as transfer::FromLua<'a>>::read(&state, -1)
                    .chain_err(|| "unable to read error message from stack")?,
            ).into(),
        ),
        ffi::LUA_ERRMEM => Err(ErrorKind::MemoryError.into()),
        ffi::LUA_ERRERR => Err(ErrorKind::ErrorHandler.into()),
        _ => unreachable!(),
    }
}

pub fn arg_unchecked_typeck(state: &WeakState, idx: i32, ty: libc::c_int) {
    unsafe {
        if ffi::lua_type(state.L, idx) != ty {
            ffi::luaL_typerror(state.L, idx, ffi::lua_typename(state.L, ty));
        }
    }
}

pub fn arg_typeck(state: &WeakState, idx: i32, ty: libc::c_int) -> Result<()> {
    unsafe {
        if ffi::lua_type(state.L, idx) != ty {
            return Err(ErrorKind::ArgTypeError(idx, typename(state, idx)).into());
        }

        Ok(())
    }
}

fn typename(state: &WeakState, idx: i32) -> String {
    unsafe {
        match ffi::lua_type(state.L, idx) {
            ffi::LUA_TNONE => "none",
            ffi::LUA_TNIL => "nil",
            ffi::LUA_TBOOLEAN => "bool",
            ffi::LUA_TLIGHTUSERDATA => "lightuserdata",
            ffi::LUA_TNUMBER => "number",
            ffi::LUA_TSTRING => "string",
            ffi::LUA_TTABLE => "table",
            ffi::LUA_TFUNCTION => "function",
            ffi::LUA_TUSERDATA => "userdata",
            ffi::LUA_TTHREAD => "thread",
            _ => "unknown type",
        }.into()
    }
}


pub mod ffi;
pub mod errors;

mod state;
mod transfer;
mod function;
mod stack;
mod table;

pub use errors::*;
pub use state::*;
pub use transfer::*;
pub use table::*;
pub use function::*;
pub use stack::*;

pub struct nil;

#[macro_export]
macro_rules! push {
    ($cxt:expr, $($arg:expr),*) => (
        $(
            $cxt.push($arg);
        )*
    )
}
