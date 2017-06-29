use errors::*;

use std::slice;
use std::mem;
use std::ffi::CString;

use State;
use WeakState;
use Ref;
use ffi;
use typename;

pub trait FromLua<'a>: Sized {
    fn read(state: &'a WeakState, idx: i32) -> Result<Self>;
}

pub trait FromLuaFunctionStack<'a>: Sized {
    type WithContext = ();

    fn read_unchecked_arg(state: &'a WeakState, idx: i32) -> Self;
    fn read_arg(state: &'a WeakState, idx: i32) -> Result<Self>;
    fn with_arg<F, T>(state: &'a WeakState, idx: i32, func: F) -> Result<T>
        where F: Fn(Self::WithContext) -> Result<T>
    {
        unimplemented!()
    }
    fn valid(state: &'a WeakState, idx: i32) -> bool { false }
}

pub trait ToLua: Sized {
    fn write(&self, state: &WeakState);
}

pub trait LuaSize {
    fn size() -> i32 {
        0
    }
}

impl<'a> FromLua<'a> for () {
    fn read(state: &'a WeakState, idx: i32) -> Result<Self> {
        Ok(())
    }
}

impl LuaSize for () {
    fn size() -> i32 {
        0
    }
}

impl<'a> FromLua<'a> for String {
    fn read(state: &'a WeakState, idx: i32) -> Result<Self> {
        unsafe {
            if ffi::lua_isstring(state.L, idx as _) != 0 {
                let slice = {
                    let mut size = 0;
                    let cs = ffi::lua_tolstring(state.L, idx, &mut size);
                    slice::from_raw_parts(cs as *const u8, size as usize)
                };
                Ok(String::from_utf8_unchecked(slice.to_vec()))
            } else {
                Err(
                    ErrorKind::TypeError("string".into(), typename(state, idx)).into(),
                )
            }
        }
    }
}

impl<'a> FromLuaFunctionStack<'a> for String {
    //type WithContext;

    fn read_unchecked_arg(state: &'a WeakState, idx: i32) -> Self {
        unsafe {
            ::arg_unchecked_typeck(state, idx, ffi::LUA_TSTRING);

            let slice = {
                let mut size = 0;
                let cs = ffi::lua_tolstring(state.L, idx, &mut size);
                slice::from_raw_parts(cs as *const u8, size as usize)
            };
            String::from_utf8_unchecked(slice.to_vec())
        }
    }

    fn read_arg(state: &'a WeakState, idx: i32) -> Result<Self> {
        unsafe {
            ::arg_typeck(state, idx, ffi::LUA_TSTRING)?;

            let slice = {
                let mut size = 0;
                let cs = ffi::lua_tolstring(state.L, idx, &mut size);
                slice::from_raw_parts(cs as *const u8, size as usize)
            };
            Ok(String::from_utf8_unchecked(slice.to_vec()))
        }
    }

    fn valid(state: &'a WeakState, idx: i32) -> bool { ::arg_typeck(state, idx, ffi::LUA_TSTRING).is_ok() }
}


impl LuaSize for String {
    fn size() -> i32 {
        1
    }
}

impl<'a> FromLua<'a> for Ref<'a> {
    fn read(state: &'a WeakState, idx: i32) -> Result<Self> {
        unsafe {
            if ffi::lua_isnil(state.L, idx) {
                Err(
                    ErrorKind::TypeError("function".into(), typename(state, idx)).into(),
                )
            } else {
                let key = ffi::luaL_ref(state.L, ffi::LUA_REGISTRYINDEX);

                Ok(Ref {
                    state: state,
                    key: key,
                })
            }
        }
    }
}

impl<'a> ToLua for Ref<'a> {
    fn write(&self, state: &WeakState) {
        unsafe { ffi::lua_rawgeti(state.L, ffi::LUA_REGISTRYINDEX, self.key) }
    }
}

impl<'a> LuaSize for Ref<'a> {
    fn size() -> i32 {
        1
    }
}

impl ToLua for String {
    fn write(&self, state: &WeakState) {
        unsafe {
            ffi::lua_pushlstring(
                state.L,
                self.as_ptr() as _,
                self.len(),
            )
        }
    }
}

impl<'a> ToLua for &'a str {
    fn write(&self, state: &WeakState) {
        unsafe {
            ffi::lua_pushlstring(state.L, self.as_ptr() as _, self.len());
        }
    }
}

impl<'a> FromLua<'a> for bool {
    fn read(state: &'a WeakState, idx: i32) -> Result<Self> {
        unsafe {
            if ffi::lua_isboolean(state.L, idx as _) {

                Ok(ffi::lua_toboolean(state.L, idx as _) != 0)
            } else {
                Err(ErrorKind::TypeError("number".into(), typename(state, idx)).into())
            }
        }
    }
}

impl<'a> FromLuaFunctionStack<'a> for bool {
    fn read_unchecked_arg(state: &'a WeakState, idx: i32) -> Self {
        ::arg_unchecked_typeck(state, idx, ffi::LUA_TBOOLEAN);

        unsafe { ffi::lua_toboolean(state.L, idx as _) != 0 }
    }

    fn read_arg(state: &'a WeakState, idx: i32) -> Result<Self> {
        ::arg_typeck(state, idx, ffi::LUA_TBOOLEAN)?;

        unsafe { Ok(ffi::lua_toboolean(state.L, idx as _) != 0) }
    }
}

impl ToLua for bool {
    fn write(&self, state: &WeakState) {
        unsafe { ffi::lua_pushboolean(state.L, *self as _) }
    }
}

impl LuaSize for bool {
    fn size() -> i32 {
        1
    }
}

macro_rules! integer_push {
    ($ty:ident) => (
        impl<'a> FromLua<'a> for $ty {
            fn read(state: &'a WeakState, idx: i32) -> Result<Self> {
                unsafe {
                    if ffi::lua_isnumber(state.L, idx as _) != 0 {

                        Ok(ffi::lua_tointeger(state.L, idx as _) as _)
                    } else {
                        Err(ErrorKind::TypeError("number".into(), typename(state, idx)).into())
                    }
                }
            }
        }

        impl<'a> FromLuaFunctionStack<'a> for $ty {
            fn read_unchecked_arg(state: &'a WeakState, idx: i32) -> Self {
                ::arg_unchecked_typeck(state, idx, ffi::LUA_TNUMBER);

                unsafe { ffi::lua_tointeger(state.L, idx as _) as _ }
            }

            fn read_arg(state: &'a WeakState, idx: i32) -> Result<Self> {
                ::arg_typeck(state, idx, ffi::LUA_TNUMBER)?;

                unsafe { Ok(ffi::lua_tointeger(state.L, idx as _) as _) }
            }
        }

        impl ToLua for $ty {
            fn write(&self, state: &WeakState) {
                unsafe { ffi::lua_pushinteger(state.L, *self as ffi::lua_Integer) }
            }
        }

        impl LuaSize for $ty {
            fn size() -> i32 {
                1
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
        impl<'a> FromLua<'a> for $ty {
            fn read(state: &'a WeakState, idx: i32) -> Result<Self> {
                unsafe {
                    if ffi::lua_isnumber(state.L, idx as _) != 0 {

                        Ok(ffi::lua_tonumber(state.L, idx as _) as _)
                    } else {
                        Err(ErrorKind::TypeError("number".into(), typename(state, idx)).into())
                    }
                }
            }
        }

        impl<'a> FromLuaFunctionStack<'a> for $ty {
            fn read_unchecked_arg(state: &'a WeakState, idx: i32) -> Self {
                ::arg_unchecked_typeck(state, idx, ffi::LUA_TNUMBER);

                unsafe { ffi::lua_tonumber(state.L, idx as _) as _ }
            }

            fn read_arg(state: &'a WeakState, idx: i32) -> Result<Self> {
                ::arg_typeck(state, idx, ffi::LUA_TNUMBER)?;

                unsafe { Ok(ffi::lua_tonumber(state.L, idx as _) as _) }
            }
        }

        impl ToLua for $ty {
            fn write(&self, state: &WeakState) {
                unsafe { ffi::lua_pushnumber(state.L, *self as ffi::lua_Number) }
            }
        }

        impl LuaSize for $ty {
            fn size() -> i32 {
                1
            }
        }
    )
}

number_push!(f32);
number_push!(f64);
