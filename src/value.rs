use ::{
    ffi, State, WeakState, Ref
};
use errors::*;
use transfer::{FromLuaFunctionStack, FromLua, ToLua, LuaSize};

use std::ffi::CString;
use std::mem;
use std::slice;

pub enum Value<'a> {
    Table(::Table<'a>)
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
            unimplemented!()

        }
    }

    fn read_arg(state: &'a WeakState, idx: i32) -> Result<Self> {
        unsafe {
            unimplemented!()

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
    fn valid(state: &'a WeakState, idx: i32) -> bool { true }
}