use ::{ffi, typename};
use errors::*;
use transfer::{FromLua, FromLuaFunctionStack, LuaSize, ToLua};

use std::ffi::CString;
use std::mem;
use std::slice;

#[derive(Debug)]
pub struct Ref<'a> {
    pub state: &'a WeakState,
    pub key: i32,
}

impl<'a> Drop for Ref<'a> {
    fn drop(&mut self) {
        unsafe { ffi::luaL_unref(self.state.L, ffi::LUA_REGISTRYINDEX, self.key) }
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct WeakState {
    pub L: *mut ffi::lua_State,
}

impl ::std::ops::Deref for State {
    type Target = WeakState;

    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct State {
    pub state: WeakState,
}

fn dump_table(state: &WeakState, indent: i32) {}

fn dump_stack(state: &WeakState, indent: i32) {

    let size = unsafe { ffi::lua_gettop(state.L) };

    unsafe {
        for i in 1..(size + 1) {
            print!("\t{} => ", i);
            match ffi::lua_type(state.L, i) {
                ffi::LUA_TNONE => println!("{}", "none"),
                ffi::LUA_TNIL => println!("{}", "nil"),
                ffi::LUA_TBOOLEAN => println!("bool: {}", ffi::lua_toboolean(state.L, i)),
                ffi::LUA_TLIGHTUSERDATA => println!("{}", "lightuserdata"),
                ffi::LUA_TNUMBER => println!("number: {}", ffi::lua_tonumber(state.L, i)),
                ffi::LUA_TSTRING => println!("string: {:?}", ffi::lua_tostring(state.L, i)),
                ffi::LUA_TTABLE => println!("{}", "table"),
                ffi::LUA_TFUNCTION => println!("{}", "function"),
                ffi::LUA_TUSERDATA => println!("{}", "userdata"),
                ffi::LUA_TTHREAD => println!("{}", "thread"),
                _ => panic!("unknown type"),
            };
        }
    }
}

impl State {
    pub fn new() -> Self {
        State { state: WeakState::new() }
    }
}


extern "C" fn errhandler(stack: ::UncheckedFunctionStack) -> i32 {
    unsafe {
        ffi::patch::traceback::flu_traceback(stack.state.L);
        //ffi::luaL_traceback(stack.state.L, stack.state.L, ffi::lua_tostring(stack.state.L, -1), 0);
    }

    1
}

impl WeakState {
    pub fn new() -> Self {
        let L = unsafe { ffi::luaL_newstate() };
        unsafe { ffi::luaL_openlibs(L) };

        WeakState { L: L }
    }

    #[inline(always)]
    pub fn from_state(state: *mut ffi::lua_State) -> Self {
        WeakState { L: state }
    }

    pub fn eval<'a, T>(&'a self, code: &str) -> Result<T>
    where T: FromLua<'a> + LuaSize
    {
        unsafe {
            //pub fn luaL_loadbuffer(L: *mut lua_State, buf: *const c_char, size: size_t, name: *const c_char) -> c_int;

            (errhandler as ::LuaUncheckedFn).write(self);

            let ret = ffi::luaL_loadbuffer(self.L, code.as_ptr() as _, code.len(), c_str!("<eval>"));
            ::pcall_errck(self, ret)?;

            let ret = ffi::lua_pcall(self.L, 0, T::size(), -2);
            ::pcall_errck(self, ret)?;

            let r = T::read(self, -1);

            ffi::lua_pop(self.L, T::size());

            r
        }
    }

    pub fn get<'a, V>(&'a self, idx: &str) -> Result<V>
    where
        V: FromLua<'a> + LuaSize
    {
        unsafe {
            ffi::patch::flu_getlfield(
                self.L,
                ffi::LUA_GLOBALSINDEX,
                idx.as_ptr() as _,
                idx.len()
            );

            /*ffi::lua_getfield(
                self.L,
                ffi::LUA_GLOBALSINDEX,
                CString::new(idx).unwrap().as_ptr() as _,
            );*/
        }

        let r = V::read(self, -1);

        unsafe {
            ffi::lua_pop(self.L, V::size());
        }

        r
    }

    pub fn set<'a, V>(&'a self, idx: &str, val: V)
    where
        V: ToLua,
    {
        val.write(&self);

        unsafe {
            ffi::patch::flu_setlfield(
                self.L,
                ffi::LUA_GLOBALSINDEX,
                idx.as_ptr() as _,
                idx.len()
            );

            /*ffi::lua_setfield(
                self.L,
                ffi::LUA_GLOBALSINDEX,
                CString::new(idx.into()).unwrap().as_ptr() as _,
            );*/
        }
    }

    pub fn dump(&self) {
        println!("{}", "========== Stack Dump ==========");
        println!("{}", "[");
        dump_stack(self, 0);
        println!("{}", "]");
        println!("{}", "========== Stack Dump ==========");
    }
    // TODO: more stuff
}

impl Drop for State {
    fn drop(&mut self) {
        unsafe { ffi::lua_close(self.state.L) }
    }
}
