extern crate libc;

pub mod ffi;
pub mod push;
pub mod read;
pub mod size;

mod table;

pub use table::Table;

use push::Push;
use read::Read;
use size::Size;

#[derive(Debug, PartialEq, Eq)]
pub struct LuaContext {
    handle: *mut ffi::lua_State,
    owner: bool
}

impl LuaContext {
    pub fn new() -> Self {
        LuaContext {
            handle: unsafe { ffi::luaL_newstate() },
            owner: true,
        }
    }

    pub fn from_state(state: *mut ffi::lua_State) -> Self {
        LuaContext {
            handle: state,
            owner: true,
        }
    }

    pub fn from_state_weak(state: *mut ffi::lua_State) -> Self {
        LuaContext {
            handle: state,
            owner: false,
        }
    }

    /*pub fn load(&mut self, path: std::path::Path) -> Result<(), IoError> {
        let mut f = try!(File::open(path));
        let mut s = String::new();
        try!(f.read_to_string(&mut s));

        unsafe {

        };

        Ok(())
    }*/ 

    /*pub fn eval_file<T>(&mut self, path: std::path::Path) -> Result<Result<T, ()>, IoError> {
        unimplemented!()
    }

    pub fn eval<T>(&mut self, code: &str) -> T {
        unimplemented!()
    }*/
    pub fn peek<'a, T>(&'a self, idx: i32) -> T
                   where T: Read<'a> {
        T::read(self, idx)
    }

    pub fn push<T>(&self, val: T)
                   where T: Push {
        val.push(self);
    }

    pub fn pop<'a, T>(&'a self) -> T
                      where T: Read<'a> + Size {
        let ret = T::read(self, -1);
        if ret.size() > 0 {
            self.pop_discard(1);
        }
        ret
    }

    pub fn pop_discard(&self, idx: i32) {
        unsafe { ffi::lua_pop(self.handle, idx) };
    }

    pub fn remove<'a, T>(&'a self, idx: i32) -> T
                        where T: Read<'a> + Size {
        let ret = T::read(self, idx);
        if ret.size() > 0 {
            self.remove_discard(idx);
        }
        ret
    }

    pub fn remove_discard(&self, idx: i32) {
        unsafe { ffi::lua_remove(self.handle, idx) };    
    }

    pub fn size(&self) -> i32 {
        unsafe { ffi::lua_gettop(self.handle) }
    }

    // TODO: more stuff

}

impl Drop for LuaContext {
    fn drop(&mut self) {
        if self.owner {
            unsafe { ffi::lua_close(self.handle) }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct LuaRef<'a> {
    cxt: &'a LuaContext,
    key: i32,
}

impl<'a> Drop for LuaRef<'a> {
    fn drop(&mut self) {
        unsafe { ffi::luaL_unref(self.cxt.handle, ffi::LUA_REGISTRYINDEX, self.key) }
    }
}

#[derive(Debug, PartialEq)]
pub enum LuaValue<'a> {
    Number(f64),
    String(&'a str),
    Bool(bool),
    Table(Table<'a>),
    /*Function(LuaFunction),
    Userdata,
    Thread,*/
    Nil
}

pub struct nil;

#[macro_export]
macro_rules! push {
    ($cxt:expr, $($arg:expr),*) => (
        $(
            $cxt.push($arg);
        )*
    )
}
