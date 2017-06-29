use ::{
    ffi, State, WeakState, Ref
};
use errors::*;
use transfer::{FromLuaFunctionStack, FromLua, ToLua, LuaSize};

use std::ffi::CString;
use std::mem;
use std::slice;

#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    Bool(bool),
    Number(f64),
    String(String),
    Table(::Table<'a>),
    Function(::Function<'a>),
    Userdata,
    LightUserdata()
}


pub struct ValueContext {
    pub state: WeakState,
    root: i32
}

impl ValueContext {
    pub fn set_meta<T>(&self, table: T)
        where T: ToLua + ::IsTable
    {
        table.write(&self.state);

        unsafe {
            ffi::lua_setmetatable(self.state.L, self.root);
        }
    }
}

impl<'a, 'b> FromLuaFunctionStack<'a> for Value<'a> {
    type WithContext = ValueContext;

    fn read_unchecked_arg(state: &'a WeakState, idx: i32) -> Self {
        unsafe {
            match ffi::lua_type(state.L, idx) {
                //                ffi::LUA_TNIL => {},
                ffi::LUA_TBOOLEAN =>
                    { return Value::Bool(bool::read_unchecked_arg(state, idx)); },
                //ffi::LUA_TLIGHTUSERDATA => {},
                ffi::LUA_TNUMBER =>
                    { return Value::Number(f64::read_unchecked_arg(state, idx)); },
                ffi::LUA_TSTRING =>
                    { return Value::String(String::read_unchecked_arg(state, idx)); },
                ffi::LUA_TTABLE =>
                    { return Value::Table(::Table::read_unchecked_arg(state, idx)); },
                ffi::LUA_TFUNCTION =>
                    { return Value::Function(::Function::read_unchecked_arg(state, idx)); },
                //ffi::LUA_TUSERDATA => {},
                //ffi::LUA_TTHREAD => {},
                _ => { unreachable!() }
            }
        }
    }

    fn read_arg(state: &'a WeakState, idx: i32) -> Result<Self> {
        unsafe {
            match ffi::lua_type(state.L, idx) {
//                ffi::LUA_TNIL => {},
                ffi::LUA_TBOOLEAN =>
                    { return Ok(Value::Bool(bool::read_arg(state, idx)?)); },
                //ffi::LUA_TLIGHTUSERDATA => {},
                ffi::LUA_TNUMBER =>
                    { return Ok(Value::Number(f64::read_arg(state, idx)?)); },
                ffi::LUA_TSTRING =>
                    { return Ok(Value::String(String::read_arg(state, idx)?)); },
                ffi::LUA_TTABLE =>
                    { return Ok(Value::Table(::Table::read_arg(state, idx)?)); },
                ffi::LUA_TFUNCTION =>
                    { return Ok(Value::Function(::Function::read_arg(state, idx)?)); },
                //ffi::LUA_TUSERDATA => {},
                //ffi::LUA_TTHREAD => {},
                _ =>
                    { return Err(
                        ErrorKind::TypeError("value".into(), ::typename(state, idx)).into(),
                    ) }
            }
        }


    }

    fn with_arg<F, T>(state: &'a WeakState, idx: i32, func: F) -> Result<T>
        where F: Fn(Self::WithContext) -> Result<T>
    {
        unsafe {
            unimplemented!()
        }
    }

    #[inline(always)]
    fn valid(_state: &'a WeakState, _idx: i32) -> bool { true }
}