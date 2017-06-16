use errors::*;

use std::slice;
use std::mem;
use std::ffi::CString;

use State;
use WeakState;
use Ref;
use ffi;
use typename;

pub trait Value<'a>: Sized {
    fn read(state: &'a WeakState, idx: i32) -> Result<Self>;
    fn write(&self, state: &'a WeakState);
    fn size() -> i32;
}

pub trait FromLua<'a>: Sized {
    fn read(state: &'a WeakState, idx: i32) -> Result<Self>;
}

pub trait FromLuaFunctionStack<'a>: Sized {
    fn read_unchecked_arg(state: &'a WeakState, idx: i32) -> Self;
    fn read_arg(state: &'a WeakState, idx: i32) -> Result<Self>;
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

impl<'a> FromLua<'a> for String {
    fn read(state: &'a WeakState, idx: i32) -> Result<Self> {
        unsafe {
            if ffi::lua_isstring(state.L, idx as _) != 0 {
                let slice = {
                    let mut size = 0;
                    let cs = ffi::lua_tolstring(state.L, idx, &mut size);
                    ffi::lua_pop(state.L, 1);
                    slice::from_raw_parts(cs, size as usize)
                };
                Ok(String::from_utf8_lossy(mem::transmute(slice)).into_owned())
            } else {
                Err(
                    ErrorKind::TypeError("string".into(), typename(state, idx)).into(),
                )
            }
        }
    }
}

impl<'a> FromLuaFunctionStack<'a> for String {
    fn read_unchecked_arg(state: &'a WeakState, idx: i32) -> Self {
        unsafe {
            ::arg_unchecked_typeck(state, idx, ffi::LUA_TSTRING);

            let slice = {
                let mut size = 0;
                let cs = ffi::lua_tolstring(state.L, idx, &mut size);
                slice::from_raw_parts(cs, size as usize)
            };
            String::from_utf8_lossy(mem::transmute(slice)).into_owned()
        }
    }

    fn read_arg(state: &'a WeakState, idx: i32) -> Result<Self> {
        unsafe {
            ::arg_typeck(state, idx, ffi::LUA_TSTRING)?;

            let slice = {
                let mut size = 0;
                let cs = ffi::lua_tolstring(state.L, idx, &mut size);
                slice::from_raw_parts(cs, size as usize)
            };
            Ok(String::from_utf8_lossy(mem::transmute(slice)).into_owned())
        }
    }
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
                CString::new(self.clone()).unwrap().as_ptr() as _,
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
    )
}

number_push!(f32);
number_push!(f64);
