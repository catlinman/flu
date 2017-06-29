use ::{
    ffi, State, WeakState, Ref
};
use errors::*;
use transfer::{FromLuaFunctionStack, FromLua, ToLua, LuaSize};

use std::ffi::CString;
use std::mem;
use std::slice;

pub trait IsTable {}

#[derive(Debug, PartialEq)]
pub struct Table<'a> {
    ptr: Ref<'a>
}

#[derive(Debug)]
pub struct TableContext {
    pub state: WeakState,
    root: i32
}

impl TableContext {
    pub fn get<'a, V>(&'a self, idx: &str) -> Result<V>
        where
            V: FromLua<'a> + LuaSize,
    {
        unsafe {
            ffi::patch::flu_getlfield(
                self.state.L,
                self.root,
                idx.as_ptr() as _,
                idx.len()
            );

            /*ffi::lua_getfield(
                self.state.L,
                self.root,
                CString::new(idx).unwrap().as_ptr() as _,
            );*/
        }

        let r = V::read(&self.state, -1);

        unsafe {
            ffi::lua_pop(self.state.L, V::size());
        }

        r
    }

    pub fn set<'a, V>(&'a self, idx: &'a str, val: V)
        where
            V: ToLua,
    {
        val.write(&self.state);

        unsafe {
            ffi::patch::flu_setlfield(
                self.state.L,
                self.root,
                idx.as_ptr() as _,
                idx.len()
            );

            /*ffi::lua_setfield(
                self.state.L,
                self.root,
                CString::new(idx).unwrap().as_ptr() as _,
            );*/
        }
    }

    pub fn set_meta<T>(&self, table: T)
        where T: ToLua + IsTable
    {
        table.write(&self.state);

        unsafe {
            ffi::lua_setmetatable(self.state.L, self.root);
        }
    }

    /*

    write() -> i32

    let i = V::write(self);
    set(L, i)

    pub fn as_table(&self) -> StackValue<Table> {
        StackValue {
            idx: self.root
        }
    }*/

    //TODO:
    pub fn as_table<'a>(&'a self) -> ::StackValue<Table<'a>> {
        ::StackValue {
            idx: self.root,
            _phantom: ::std::marker::PhantomData
        }
    }
}


pub struct TableInit<F: Fn(TableContext)> {
    func: F
}

impl<'a> IsTable for Table<'a> {}
impl<F: Fn(TableContext)> IsTable for TableInit<F> {}

impl<'a> Table<'a> {
    pub fn new<F>(func: F) -> TableInit<F>
            where F: Fn(TableContext)
    {
        TableInit {
            func: func
        }
    }

    pub fn reference<F>(state: &'a State, func: F) -> Self
        where F: Fn(TableContext)
    {
        Self::new(func).write(&state);

        Table {
            ptr: Ref::read(&state.state, -1).unwrap()
        }
    }
}

impl<'a> ToLua for Table<'a> {
    fn write(&self, state: &WeakState) {
        self.ptr.write(&state);
    }
}

impl<'a, 'b> ToLua for &'b Table<'a> {
    fn write(&self, state: &WeakState) {
        self.ptr.write(&state);
    }
}
impl<'a, 'b> IsTable for &'b Table<'a> {}
impl<'a> IsTable for ::StackValue<'a, Table<'a>> {}


impl<F> ToLua for TableInit<F>
        where F: Fn(TableContext) {
    fn write(&self, state: &WeakState) {
        unsafe {
            ffi::lua_newtable(state.L);

            (self.func)(TableContext { state: WeakState::from_state(state.L), root: ::abs_idx(state.L, -1) });
        }
    }
}

impl<'a> LuaSize for Table<'a> {
    fn size() -> i32 {
        0
    }
}


impl<'a, 'b> FromLua<'a> for Table<'a> {
    fn read(state: &'a WeakState, idx: i32) -> Result<Self> {
        unsafe {
            ::arg_typeck(state, idx, ffi::LUA_TTABLE)?;

            Ok(Table {
                ptr: Ref::read(&state, -1)?
            })
        }
    }
}

impl<'a> FromLuaFunctionStack<'a> for Table<'a> {
    type WithContext = TableContext;

    fn read_unchecked_arg(state: &'a WeakState, idx: i32) -> Self {
        unsafe {
            ::arg_unchecked_typeck(state, idx, ffi::LUA_TTABLE);

            Table {
                ptr: Ref::read(&state, -1).unwrap()
            }
        }
    }

    fn read_arg(state: &'a WeakState, idx: i32) -> Result<Self> {
        unsafe {
            ::arg_typeck(state, idx, ffi::LUA_TTABLE)?;

            Ok(Table {
                ptr: Ref::read(&state, -1)?
            })
        }
    }

    fn with_arg<F, T>(state: &'a WeakState, idx: i32, func: F) -> Result<T>
        where F: Fn(Self::WithContext) -> Result<T>
    {
        unsafe {
            ::arg_typeck(state, idx, ffi::LUA_TTABLE)?;

            (func)(TableContext {
                state: WeakState::from_state(state.L),
                root: ::abs_idx(state.L, idx)
            })
        }
    }

    fn valid(state: &'a WeakState, idx: i32) -> bool { ::arg_typeck(state, idx, ffi::LUA_TTABLE).is_ok() }
}